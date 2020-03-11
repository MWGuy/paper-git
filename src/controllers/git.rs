use crate::services::git::http::{GitPackType, GitHttpService};
use crate::services::git::repository::GitRepositoryService;

use actix_web::{get, post, web, HttpRequest, HttpResponse, Error};
use actix_web::http::StatusCode;
use futures::StreamExt;
use qstring::QString;
use git2::Repository;

#[get("/{user}/{repo}.git/info/refs")]
pub async fn git_info_refs(req: HttpRequest, info: web::Path<(String, String)>) -> Result<HttpResponse, Error> {
    let qstring = QString::from(req.query_string());
    if !qstring.has("service") {
        return Ok(HttpResponse::new(StatusCode::BAD_REQUEST));
    }

    let path_info = info.into_inner();
    match GitRepositoryService::resolve(format!("{}/{}", path_info.0, path_info.1).as_str()) {
        Some(git_repository) => {
            let pack_type = match qstring.get("service").unwrap() {
                "git-upload-pack" => GitPackType::Upload,
                _ => GitPackType::Receive
            };

            let res = GitHttpService::info_refs(git_repository, pack_type);

            Ok(HttpResponse::build(StatusCode::OK)
                .content_type(res.content_type)
                .body(res.body))
        }
        None => Ok(HttpResponse::build(StatusCode::NOT_FOUND).finish())
    }
}

#[post("/{user}/{repo}.git/git-upload-pack")]
pub async fn git_upload_pack(_req: HttpRequest,
                             mut body: web::Payload,
                             info: web::Path<(String, String)>) -> Result<HttpResponse, Error> {
    git_open_stateless_rpc(body, info, GitPackType::Upload).await
}

#[post("/{user}/{repo}.git/git-receive-pack")]
pub async fn git_receive_pack(_req: HttpRequest,
                              mut body: web::Payload,
                              info: web::Path<(String, String)>) -> Result<HttpResponse, Error> {
    git_open_stateless_rpc(body, info, GitPackType::Receive).await
}

async fn git_open_stateless_rpc(mut body: web::Payload,
                          info: web::Path<(String, String)>,
                          pack: GitPackType) -> Result<HttpResponse, Error> {
    let path_info = info.into_inner();
    match GitRepositoryService::resolve(format!("{}/{}", path_info.0, path_info.1).as_str()) {
        Some(git_repository) => {
            let mut bytes = web::BytesMut::new();
            while let Some(item) = body.next().await {
                bytes.extend_from_slice(&item?);
            }

            let res = GitHttpService::stateless_rpc(
                git_repository,
                pack,
                bytes.to_vec());

            Ok(HttpResponse::build(StatusCode::OK)
                .content_type(res.content_type)
                .body(res.body))
        }
        None => Ok(HttpResponse::build(StatusCode::NOT_FOUND).finish())
    }
}
