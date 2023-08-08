use actix_web::{get, web, Responder, post};

use crate::structs::{AppState, DisplayConnectQuery, TeamInfo, DisplayMatchInfo};

#[utoipa::path(
    get,
    path = "/connect_display",
    responses(
        (status = 200, description = "Display succesfuly connected")
    ),
    params(
        ("display_number", description = "Number of display that is connecting")
    )
)]
#[get("/display/connect")]
pub async fn connect_display(
    // i hate that you can use paperclip here
    query: web::Query<DisplayConnectQuery>,
    state: web::Data<AppState>,
) -> impl Responder {
    state.message_handler.add_client(query.display_number).await
}

#[post("/display/team")]
pub async fn update_team_info(
    query: web::Query<DisplayMatchInfo>,
    state: web::Data<AppState>,
) ->  {
    
}