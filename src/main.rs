use clap::Parser;
use std::{net::TcpListener, sync::Arc};
use utoipa_swagger_ui::SwaggerUi;

use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use match_control_system::{
    arguments::Arguments, brodcaster::MessageHandler, routes::display::connect_display,
    structs::AppState,
};
use utoipa::OpenApi;

struct Application {
    server: Server,
}

impl Application {
    fn build() -> Result<Self, anyhow::Error> {
        let args: Arguments = Arguments::parse();

        let message_handler = MessageHandler::create();

        let listener = TcpListener::bind(format!("{}:{}", args.ip, args.port))
            .expect("Failed to create listener");

        let server = create_server(message_handler, listener)?;

        Ok(Self { server })
    }

    async fn run_until_stoped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

fn create_server(
    message_handler: Arc<MessageHandler>,
    listener: TcpListener,
) -> Result<Server, anyhow::Error> {
    #[derive(OpenApi)]
    #[openapi(
        paths(match_control_system::routes::display::connect_display),
        components(schemas(match_control_system::structs::display::DisplayConnectQuery)),
        tags(
            (name = "Match control system", description = "System for match controll entire backend for the whole system")
        )
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(connect_display)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .app_data(web::Data::new(AppState {
                message_handler: Arc::clone(&message_handler),
            }))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let app = Application::build()?;

    app.run_until_stoped().await?;

    Ok(())
}
