use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use askama_actix::Template;
use env_logger::{Builder, Env};

#[get("")]
async fn app() -> impl Responder {
  HttpResponse::Ok().body("Hello!")
}

#[derive(Template)]
#[template(path = "new.html")]
struct NewTemplate<'a> {
  name: &'a str,
}

#[get("/new")]
async fn new_workout() -> impl Responder {
  NewTemplate { name: "ijsef" }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  Builder::from_env(Env::default().default_filter_or("info")).init();

  HttpServer::new(|| {
    App::new()
      .wrap(Logger::default())
      .service(web::scope("/app").service(app).service(new_workout))
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
