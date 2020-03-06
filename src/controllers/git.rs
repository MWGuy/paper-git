use crate::services::git::{GitService, GitPackType};

use actix_web::{get, post, web, HttpRequest, HttpResponse, Error};
use actix_web::http::StatusCode;
use futures::StreamExt;
use qstring::QString;

#[get("/{user}/{repo}.git/info/refs")]
pub async fn git_info_refs(req: HttpRequest/*, info: web::Path<(String, String)>*/) -> Result<HttpResponse, Error> {
    println!("{:?}", req);

    let qstring = QString::from(req.query_string());
    if !qstring.has("service") {
        return Ok(HttpResponse::new(StatusCode::BAD_REQUEST))
    }

    let pack_type = match qstring.get("service").unwrap() {
        "git-upload-pack" => GitPackType::Upload,
        _ => GitPackType::Receive
    };

    let git_req = GitService { repository: String::from("/home/mwguy/test_repo") };
    let res = git_req.info_refs(pack_type);

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(res.content_type)
        .body(res.body))
}

#[post("/{user}/{repo}.git/git-upload-pack")]
pub async fn git_upload_pack(req: HttpRequest, mut body: web::Payload/*, info: web::Path<(String, String)>*/) -> Result<HttpResponse, Error> {
    println!("{:?}", req);

    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    let git_req = GitService { repository: String::from("/home/mwguy/test_repo") };
    let res = git_req.stateless_rpc(GitPackType::Upload, bytes.to_vec());

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(res.content_type)
        .body(res.body))
}

#[post("/{user}/{repo}.git/git-receive-pack")]
pub async fn git_receive_pack(req: HttpRequest, mut body: web::Payload/*, info: web::Path<(String, String)>*/) -> Result<HttpResponse, Error> {
    println!("{:?}", req);

    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    let git_req = GitService { repository: String::from("/home/mwguy/test_repo") };
    let res = git_req.stateless_rpc(GitPackType::Receive, bytes.to_vec());

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(res.content_type)
        .body(res.body))
}
