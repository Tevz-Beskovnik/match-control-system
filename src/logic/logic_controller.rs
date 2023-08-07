use std::sync::Arc;

use crate::{brodcaster::MessageHandler, database::database::Database};

#[derive(Debug, Clone)]
pub struct LogicController {
    pub message_handler: Arc<MessageHandler>,
    pub database: Arc<Database>,
}
