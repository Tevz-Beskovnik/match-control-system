use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, Serialize, Default, ToSchema)]
pub struct DisplayConnectQuery {
    /// sequential number of the display connected (for purpouses of figuring out what field its on)
    #[schema(example = 1)]
    pub display_number: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct TeamInfo {
    pub name: Option<String>,
    pub country: Option<String>,
    pub flag_url: Option<String>,
    pub member_one: Option<String>,
    pub member_two: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct DisplayMatchInfo {
    pub team_one: TeamInfo,
    pub team_two: TeamInfo,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct DisplayScore {
    pub tema_one: Option<i32>,
    pub team_two: Option<i32>,
}
