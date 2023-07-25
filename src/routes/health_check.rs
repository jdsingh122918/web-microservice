use actix_web::get;
use actix_web::HttpResponse;
use actix_web::Responder;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
