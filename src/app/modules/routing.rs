use crate::app::modules::profiles::controller::routes as profile_routes;

pub fn router() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Routes", |rocket| async {
        rocket.mount("/api/v1/profile", profile_routes())
    })
}
