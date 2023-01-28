use crate::player::player_get_action;
use crate::GameAppData;
use axum::routing::get;
use axum::Router;

pub fn player_routes() -> Router<GameAppData> {
    Router::new().route(
        "/teams/{team_slug}/players/{player_id}",
        get(player_get_action),
    )
}
