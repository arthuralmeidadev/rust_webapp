#[macro_use]
extern crate rocket;
mod routes;
use routes::public;
use routes::protected;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![public::get_all_products])
        .mount("/admin", routes![protected::get_admin])
}
