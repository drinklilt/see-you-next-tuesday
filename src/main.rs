use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::{Duration, Utc};
use http::StatusCode;
use now::DateTimeNow;

//#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}

//#[get("/api/{unit}")]
async fn api(path: web::Path<String>) -> impl Responder {
    let unit = path.into_inner();

    // Get tuesday of current week
    let tuesday = Utc::now().beginning_of_week() + Duration::days(1);
    // If current date is before tuesday, then return millis between current date and tuesday
    // Else return millis between current date and next tuesday
    let delta: Duration = if Utc::now() < tuesday {
        tuesday.signed_duration_since(Utc::now())
        //println!("Next tuesday: {}", tuesday);
    } else {
        (tuesday + Duration::weeks(1)).signed_duration_since(Utc::now())
        //println!("Next tuesday: {}", (tuesday + Duration::weeks(1)));
    };

    match unit.as_str() {
        "ns" => return HttpResponse::Ok().body(delta.num_nanoseconds().unwrap().to_string()),
        "ms" => return HttpResponse::Ok().body(delta.num_milliseconds().to_string()),
        "s" => return HttpResponse::Ok().body(delta.num_seconds().to_string()),
        _ => return HttpResponse::BadRequest().body("Invalid unit"),
    }
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Invalid Request")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let governor = GovernorConfigBuilder::default()
    .per_second(60)
    .burst_size(120)
    .finish()
    .unwrap();
    HttpServer::new(move || {
        App::new()
            .wrap(Governor::new(&governor.clone()))
            .route("/", web::get().to(index))
            .route("/api/{unit}", web::get().to(api))
            .default_service(web::to(not_found))
    })
    .workers(4)
    .bind(("127.0.0.1", 17640))?
    .run()
    .await
}
