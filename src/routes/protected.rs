use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Admin {
    name: &'static str,
    age: i8,
    id: &'static str,
}

impl Admin {
    fn new(name: &'static str, age: i8, id: &'static str) -> Admin {
        Self { name, age, id }
    }
}

#[get("/")]
pub async fn get_admin() -> Json<Admin> {
    Json(Admin::new("Arthur", 20, "89071236781263"))
}