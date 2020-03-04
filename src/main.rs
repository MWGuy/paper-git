mod git;

use actix_web::{get, post, web, App, HttpServer, HttpRequest, HttpResponse, Error};
use actix_web::http::StatusCode;
use futures::StreamExt;
use git::{SmartGitRequest, GitPackType};
use qstring::QString;

#[get("/{user}/{repo}.git/info/refs")]
async fn git_info_refs(req: HttpRequest/*, info: web::Path<(String, String)>*/) -> Result<HttpResponse, Error> {
    println!("{:?}", req);

    let qstring = QString::from(req.query_string());
    if !qstring.has("service") {
        return Ok(HttpResponse::new(StatusCode::from_u16(500).expect("status code")))
    }

    let pack_type = match qstring.get("service").unwrap() {
        "git-upload-pack" => GitPackType::Upload,
        _ => GitPackType::Receive
    };

    let git_req = SmartGitRequest { repository: String::from("/home/mwguy/test_repo") };
    let res = git_req.info_refs(pack_type);

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(res.content_type)
        .body(res.body))
}

#[post("/{user}/{repo}.git/git-upload-pack")]
async fn git_upload_pack(req: HttpRequest, mut body: web::Payload/*, info: web::Path<(String, String)>*/) -> Result<HttpResponse, Error> {
    println!("{:?}", req);

    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    let git_req = SmartGitRequest { repository: String::from("/home/mwguy/test_repo") };
    let res = git_req.stateless_rpc(GitPackType::Upload, bytes.to_vec());

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(res.content_type)
        .body(res.body))
}

#[post("/{user}/{repo}.git/git-receive-pack")]
async fn git_receive_pack(req: HttpRequest, mut body: web::Payload/*, info: web::Path<(String, String)>*/) -> Result<HttpResponse, Error> {
    println!("{:?}", req);

    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    let git_req = SmartGitRequest { repository: String::from("/home/mwguy/test_repo") };
    let res = git_req.stateless_rpc(GitPackType::Receive, bytes.to_vec());

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(res.content_type)
        .body(res.body))
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(git_info_refs)
        .service(git_upload_pack)
        .service(git_receive_pack))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
