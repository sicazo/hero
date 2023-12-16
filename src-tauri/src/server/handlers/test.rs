use actix_web::get;

#[get("/api/test")]
pub async fn handle() -> actix_web::Result<String> {
    Ok("test reponse".to_string())
}
