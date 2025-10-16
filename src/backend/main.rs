use actix_web::*;
use serde::*;
use app_lib::*;

#[derive(Serialize)]
struct ConnectionObject {
    ok: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(get_connection)
            .route("/hey", web::get().to(manual_hello))
            .default_service(web::to(error404))
    })
    .bind((BACKEND_ADDR, BACKEND_PORT))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    let daemon = establish_connection();
    let mut body = String::new();
    body = "Hello world!".to_string();
    if daemon == false {
        body = "No Connection to daemon!".to_string();
    }
    HttpResponse::Ok().body(body) 
}

#[get("/connection")]
async fn get_connection() -> impl Responder {
    let daemon_connected = establish_connection();
    let response = ConnectionObject {
        ok: daemon_connected,
    };

    web::Json(response)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn error404() -> impl Responder {
    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body("<h1>404: Page not found!</h1>")
}