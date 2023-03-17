use std::{env, future::Future, path::Path, pin::Pin};

use actix_files::NamedFile;
use actix_web::{error, get, middleware::Logger, web, App, Error, FromRequest, HttpServer, Result};
use log::info;

struct Apikey(String);

impl FromRequest for Apikey {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let apikey_req = req
            .headers()
            .get("x-api-key")
            .ok_or_else(|| error::ErrorBadRequest("X-API-KEY header not set"))
            .and_then(|hdr| {
                hdr.to_str()
                    .and_then(|hdr| Ok(hdr.to_owned()))
                    .map_err(|_| error::ErrorBadRequest("invalid x-api-key header"))
            });

        let apikey = env::var("APIKEY").expect("APIKEY not set");

        Box::pin(async move {
            match apikey_req {
                Ok(apikey_req) if apikey_req == apikey => Ok(Apikey(apikey_req.to_owned())),
                Ok(_) => Err(error::ErrorUnauthorized("provided api key incorrect")),
                Err(err) => Err(err),
            }
        })
    }
}

#[get("/{filename:.*}")]
async fn file(path: web::Path<String>, _apikey: Apikey) -> Result<NamedFile> {
    let directory = "./files";
    let user_path = path.into_inner();
    let user_path = sanitize_filename::sanitize(user_path);
    let path = Path::new(directory).join(user_path);

    info!("Reading file from {:?}", path);

    Ok(NamedFile::open_async(path).await?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let socket = env::var("SOCKET").expect("SOCKET not set");

    info!("Starting server on {}", socket);

    HttpServer::new(move || App::new().wrap(Logger::default()).service(file))
        .bind(socket)?
        .run()
        .await
}
