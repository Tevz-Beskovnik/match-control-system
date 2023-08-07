use anyhow::anyhow;

use crate::{logic::logic_controller::LogicController, structs::DisplayMatchInfo};

impl LogicController {
    pub async fn display_set_teams(
        &self,
        teams: DisplayMatchInfo,
        display_number: i32,
    ) -> Result<(), anyhow::Error> {
        let json = match serde_json::to_string(&teams) {
            Ok(data) => data,
            Err(err) => return Err(anyhow!(err)),
        };

        self.message_handler
            .send_message(display_number, json)
            .await?;

        Ok(())
    }
}
