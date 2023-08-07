use std::{collections::HashMap, sync::Arc, time::Duration};

use actix_web::rt::time::interval;
use actix_web_lab::sse::{self, ChannelStream, Sse};
use anyhow::{anyhow, Error};
use futures::lock::Mutex;

#[derive(Debug, Clone)]
pub struct Client {
    client_stream: sse::Sender,
    client_id: i32,
}

#[derive(Debug, Clone, Default)]
pub struct OpenStreams {
    pub active_streams: HashMap<i32, Client>,
}

#[derive(Debug)]
pub struct MessageHandler {
    pub streams: Mutex<OpenStreams>,
}

impl MessageHandler {
    pub fn create() -> Arc<Self> {
        let message_handler = Arc::new(MessageHandler {
            streams: Mutex::new(OpenStreams::default()),
        });

        Self::spawn_ping(Arc::clone(&message_handler));

        message_handler
    }

    pub fn spawn_ping(message_handler: Arc<Self>) {
        actix_web::rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));

            loop {
                interval.tick().await;
                MessageHandler::drop_closed_connections(&message_handler).await;
            }
        });
    }

    pub async fn drop_closed_connections(&self) {
        let clients = self.streams.lock().await.active_streams.clone();
        let mut connected_clients: HashMap<i32, Client> = HashMap::default();

        for (_, client) in clients {
            if client
                .client_stream
                .send(sse::Event::Comment("Ping".into()))
                .await
                .is_ok()
            {
                connected_clients.insert(client.client_id, client.clone());
            }
        }

        self.streams.lock().await.active_streams = connected_clients;
    }

    pub async fn add_client(&self, client_id: i32) -> Sse<ChannelStream> {
        let (sender, channel_stream) = sse::channel(10);

        let _ = sender.send(sse::Event::Comment("connected".into())).await;

        self.streams.lock().await.active_streams.insert(
            client_id,
            Client {
                client_stream: sender,
                client_id,
            },
        );

        channel_stream
    }

    pub async fn send_message(&self, client_id: i32, message: String) -> Result<(), Error> {
        let clients = self.streams.lock().await.active_streams.clone();

        match clients.contains_key(&client_id.clone()) {
            true => {
                let client = clients.get(&client_id).unwrap();

                if client
                    .client_stream
                    .send(sse::Event::Comment(message.into()))
                    .await
                    .is_err()
                {
                    return Err(anyhow!("Failed to send message to client"));
                }
            }
            false => {
                return Err(anyhow!(
                    "Client with id {} does not appear to exist",
                    client_id
                ))
            }
        }

        Ok(())
    }

    pub async fn is_connected(&self, client_id: i32) -> bool {
        let clients = self.streams.lock().await.active_streams.clone();

        clients.contains_key(&client_id)
    }
}
