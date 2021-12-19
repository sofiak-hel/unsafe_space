use crate::auth::Identity;
use crate::db::Database;
use actix_web::{web::Data, HttpRequest, HttpResponse, Responder};

pub async fn get(req: HttpRequest, database: Data<Database>) -> impl Responder {
    Identity::clear_session(&req, HttpResponse::Found(), &database)
        .header("location", "/")
        .finish()
}
