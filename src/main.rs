use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet_and_insult(req: HttpRequest) -> impl Responder{
    let name = req.match_info().get("name").unwrap_or("world");
    let insult = req.match_info().get("insult").unwrap_or("IDIOT");
    format!("Hello {} \nI am an{} ! ", &name, &insult)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet_and_insult))
            .route("/{name}", web::get().to(greet_and_insult))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}