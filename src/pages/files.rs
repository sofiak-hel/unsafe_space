use crate::config::Config;
use crate::mime::MimeTypes;

use actix_web::{
    http::header,
    web,
    web::{Data, HttpResponse, ServiceConfig},
    Responder,
};
use std::{fs::File, io::Read};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/{filename}").route(web::get().to(get)));
}

pub async fn get(
    filename: web::Path<String>,
    config: Data<Config>,
    mimetypes: Data<MimeTypes>,
) -> impl Responder {
    let filepath = config.static_path.join(filename.as_str());
    let mimetype = filepath
        .extension()
        .map(|ext| ext.to_str().map(|ext| mimetypes.mimetype(ext)))
        .flatten()
        .flatten();

    if let Ok(mut file) = File::open(&filepath) {
        let mut bytes = Vec::new();
        if let Ok(_) = file.read_to_end(&mut bytes) {
            let mut response = HttpResponse::Ok();
            if let Some(mimetype) = mimetype {
                response.header(header::CONTENT_TYPE, mimetype.as_str());
            }
            response.body(bytes)
        } else {
            HttpResponse::InternalServerError().body("Failed to read file")
        }
    } else {
        HttpResponse::NotFound().body("File not found")
    }
}
