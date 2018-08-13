use actix_web::App;
use actix_web::http::Method;
use app::api::{users};
use app::state::AppState;

pub fn setup_routes(app: App<AppState>) -> App<AppState> {
    app
        .route("/users", Method::GET, users::list::list)
}
