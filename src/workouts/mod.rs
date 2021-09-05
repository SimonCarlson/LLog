use actix_web::{guard, web};

pub mod models;
pub mod views;

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/app")
      .service(views::create::new_workout)
      .service(views::create::create_new_workout)
      .service(views::edit::update_workout)
      .service(
        web::resource("/workout/{id}")
          .name("view_workout")
          .guard(guard::Get())
          .route(web::get().to(views::edit::view_workout)),
      ),
  );
}
