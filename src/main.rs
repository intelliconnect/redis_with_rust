use actix_web::{web, App, HttpResponse, HttpServer};
use redis::AsyncCommands;

mod errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let rclient = redis::Client::open("redis://host.docker.internal").unwrap();
    println!("starting server...");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(rclient.clone()))
            .route("/", web::get().to(get_operations))
            .route("/set", web::get().to(set_operations))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

async fn get_operations(client: web::Data<redis::Client>) -> Result<HttpResponse, errors::Myerror> {
    let mut conn = client.get_async_connection().await?;
    let val: (Option<String>, Option<String>) = conn.get(&["ping", "ping"]).await?;
    let languages: Vec<String> = conn.lrange("languages", 0, -1).await?;
    println!("{}, {}, {:#?}", &val.0.unwrap(), &val.1.unwrap(), languages);
    Ok(HttpResponse::Ok().body(languages.join(",")))
}

async fn set_operations(client: web::Data<redis::Client>) -> Result<HttpResponse, errors::Myerror> {
    let mut conn = client.get_async_connection().await?;
    let _ = conn.lpush("languages", &["rust", "javascript"]).await?;
    let _ = conn.set("ping", "PONG").await?;
    Ok(HttpResponse::Ok().body("done"))
}
