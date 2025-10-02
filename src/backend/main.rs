use actix_web::*;
use app_lib::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .default_service(web::to(error404))
    })
    .bind((BACKEND_ADDR, BACKEND_PORT))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!") 
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