use crate::controllers::git::{git_info_refs, git_upload_pack, git_receive_pack};
use actix_web::{HttpServer, App};
use std::env;
use dotenv::dotenv;
use simple_logger;
use log::info;

mod services;
mod controllers;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init().unwrap();
    dotenv().ok();

    info!("Starting paper git");

    let host = env::var("GIT_HOST").expect("GIT_HOST is not defined");
    let port = env::var("GIT_PORT").expect("GIT_PORT is not defined");


    HttpServer::new(|| App::new()
        .service(git_info_refs)
        .service(git_upload_pack)
        .service(git_receive_pack))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
