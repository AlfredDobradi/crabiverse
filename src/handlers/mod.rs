use serde::Deserialize;
use serde_json::json;
use actix_web::{web, HttpResponse, Responder};

use crate::database::{get_posts, create_post};

#[derive(Deserialize)]
pub struct Resource {
    resource: String
}

#[derive(Deserialize)]
pub struct NewPost {
    title: String,
    body: String
}

pub async fn handle_webfinger(cfg: web::Data<crate::config::AppConfig>, info: web::Query<Resource>) -> impl Responder {
    let query = match info.resource.strip_prefix("acct:") {
        Some(valid) => {
            let account = format!("{}@{}", cfg.user_name(), cfg.base_url());
            if account != valid {
                return HttpResponse::NotFound().body("not found");
            }

            let response = json!({
                "subject": info.resource,
                "aliases": [
                    cfg.self_url(),
                    cfg.profile_url()
                ],
                "links": [
                    {
                        "rel": "https://webfinger.net/rel/profile-page",
                        "type": "text/html",
                        "href": cfg.profile_url()
                    },
                    {
                        "rel": "self",
                        "type": "application/activity+json",
                        "href": cfg.self_url()
                    },
                ]
            });

            HttpResponse::Ok().insert_header(("Content-Type", "application/jrd+json")).body(response.to_string())
        },
        None => HttpResponse::BadRequest().body("missing acct resource")
    };
    query
}

pub async fn handle_user_profile(cfg: web::Data<crate::config::AppConfig>, path: web::Path<String>) -> impl Responder {
    let username = path.into_inner();
    if username != cfg.user_name() {
        return HttpResponse::NotFound().body("not found");
    }

    let response = json!({
        "@context": [
          "https://www.w3.org/ns/activitystreams",
          "https://w3id.org/security/v1",
        ],
        "id": cfg.self_url(),
        "type": "Person",
        "preferredUsername": cfg.user_name(),
        "url": cfg.profile_url(),
        "published": "2022-11-24T00:00:00Z",
        "publicKey": {
          "id": format!("{}#main-key", cfg.self_url()),
          "owner": cfg.self_url(),
          "publicKeyPem": cfg.user_pem()
        },
        "manuallyApprovesFollowers": true,
        "discoverable": true,

        "following": format!("{}/following", cfg.self_url()),
        "followers": format!("{}/followers", cfg.self_url()),
        "inbox":  format!("{}/inbox", cfg.self_url()),
        "outbox":  format!("{}/outbox", cfg.self_url()),
        "featured": format!("{}/collections/featured", cfg.self_url()),
        "featuredTags": format!("{}/collections/tags", cfg.self_url()),
        "endpoints": {
            "sharedInbox": format!("https://{}/inbox", cfg.base_url())
        }
      });

      HttpResponse::Ok().insert_header(("Content-Type", "application/jrd+json")).body(response.to_string())
      
}

pub async fn handle_not_implemented(_cfg: web::Data<crate::config::AppConfig>, _path: web::Path<String>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}

pub async fn handle_outbox(cfg: web::Data<crate::config::AppConfig>, path: web::Path<String>) -> impl Responder {
    let username = path.into_inner();
    if username != cfg.user_name() {
        return HttpResponse::NotFound().body("not found");
    }

    let r = match get_posts() {
        Ok(posts) => posts,
        Err(_) => {
            let response = json!({
                "status": "error",
            });
            return HttpResponse::InternalServerError().insert_header(("Content-Type", "application/jrd+json")).body(response.to_string())
        },
    };

    let response = json!({
        "@context": "https://www.w3.org/ns/activitystreams",
        "summary": format!("{}'s entries", cfg.user_name()),
        "type": "OrderedCollection",
        "totalItems": 1,
        "orderedItems": r
    });

    HttpResponse::Ok().insert_header(("Content-Type", "application/jrd+json")).body(response.to_string())
}

pub async fn handle_new_post(query: web::Json<NewPost>) -> impl Responder {
    match create_post(&query.title, &query.body) {
        Ok(_) => {
            let response = json!({
                "status": "ok"
            });
            HttpResponse::Ok().insert_header(("Content-Type", "application/jrd+json")).body(response.to_string())
        },
        Err(_) => {
            let response = json!({
                "status": "error",
            });
            HttpResponse::InternalServerError().insert_header(("Content-Type", "application/jrd+json")).body(response.to_string())
        }
    }

}