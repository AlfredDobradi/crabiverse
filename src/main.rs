use std::{fs, collections::HashMap};
use env_logger::Env;

use actix_web::{error, web, App, HttpServer, HttpResponse, middleware::Logger};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod handlers;
mod config;
mod database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let user_pem = fs::read_to_string("./barveyhirdman.pem").expect("failed to read pem file");

    let mut config_map = HashMap::<String, String>::new();
    config_map.insert(String::from("user_name"), String::from("barveyhirdman"));
    config_map.insert(String::from("base_url"), String::from("localhost"));
    config_map.insert(String::from("user_pem"), String::from(user_pem));
    let config = config::AppConfig::from(config_map);

    let cert = "localhost";

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(format!("{}-key.pem", &cert), SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(format!("{}.pem",cert)).unwrap();
    
    HttpServer::new(move || {
        let json_config = web::JsonConfig::default()
          .limit(4096);
        // Scope for the /.well-known prefix
        App::new()
        .app_data(web::Data::new(config.clone()))
        .app_data(json_config)
            .service(
                web::scope("/.well-known").route("/webfinger", web::get().to(handlers::handle_webfinger))
            )
            .route("/inbox", web::get().to(handlers::handle_not_implemented))
            .route("/users/{username}", web::get().to(handlers::handle_user_profile))
            .route("/users/{username}/following", web::get().to(handlers::handle_not_implemented))
            .route("/users/{username}/followers", web::get().to(handlers::handle_not_implemented))
            .route("/users/{username}/inbox", web::get().to(handlers::handle_not_implemented))
            .route("/users/{username}/outbox", web::get().to(handlers::handle_outbox))
            .route("/users/{username}/collections/featured", web::get().to(handlers::handle_not_implemented))
            .route("/users/{username}/collections/tags", web::get().to(handlers::handle_not_implemented))
            .route("/post/new", web::post().to(handlers::handle_new_post))
            .wrap(Logger::default())

    })
    .bind_openssl("127.0.0.1:443", builder)?
    .run()
    .await
}



/*
{
  "subject": "acct:admin@mastodon.local",
  "aliases": [
    "http://mastodon.local/@admin",
    "http://mastodon.local/users/admin"
  ],
  "links": [
    {
      "rel": "http://webfinger.net/rel/profile-page",
      "type": "text/html",
      "href": "http://mastodon.local/@admin"
    },
    {
      "rel": "self",
      "type": "application/activity+json",
      "href": "http://mastodon.local/users/admin"
    },
    {
      "rel": "http://ostatus.org/schema/1.0/subscribe",
      "template": "http://mastodon.local/authorize_interaction?uri={uri}"
    }
  ]
}
*/