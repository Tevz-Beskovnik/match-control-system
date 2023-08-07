use std::sync::Arc;

use crate::brodcaster::MessageHandler;

#[derive(Clone, Debug)]
pub struct AppState {
    pub message_handler: Arc<MessageHandler>,
}
