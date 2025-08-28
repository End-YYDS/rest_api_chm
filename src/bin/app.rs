use actix_web::HttpServer;
use rest_api_chm::configure_app;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Your code here
    HttpServer::new(|| {
        actix_web::App::new()
            .configure(configure_app)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}
