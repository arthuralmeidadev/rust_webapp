#[get("/")]
pub async fn get_all_products() -> &'static str {
    "All products:"
}