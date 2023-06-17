#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, user!"
}

#[get("/")]
fn admin_index() -> &'static str {
    "Hello, admin!"
}

// #[get("/coolroute/<>")]

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/admin", routes![admin_index])
}