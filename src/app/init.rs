use actix_web::App;
use app::state::AppState;
use app::route::setup_routes;

pub fn initialize(state: AppState) -> App<AppState> {
    setup_routes(App::with_state(state))
}
