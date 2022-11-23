#[macro_use]
extern crate serde_derive;
extern crate serde_qs as qs;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

struct AppConfig {
    user_name: String,
    user_pem: String,
    base_url: String,
}

#[derive(Deserialize)]
struct Resource {
    resource: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let user = "barveyhirdman";
    let base_url = "localhost";

    HttpServer::new(move || {
        // Scope for the /.well-known prefix
        App::new()
            .app_data(web::Data::new(AppConfig {
                user_name: String::from(user),
                user_pem: String::from(""),
                base_url: String::from(base_url),
            }))
            .service(
                web::scope("/.well-known").route("/webfinger", web::get().to(handle_webfinger))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn handle_webfinger(cfg: web::Data<AppConfig>, info: web::Query<Resource>) -> impl Responder {
    let query = match info.resource.strip_prefix("acct:") {
        Some(valid) => {
            let account = format!("{}@{}", cfg.user_name, cfg.base_url);
            if account != valid {
                return HttpResponse::NotFound().body("not found");
            }

            HttpResponse::Ok().body(format!("hi {}", valid))
        },
        None => HttpResponse::InternalServerError().body("internal server error")
    };
    query
}