use actix_web::{App, HttpServer, Responder, web};

#[actix_web::main] //Macro que permite funciones asíncronas
async fn main() -> std::io::Result<()> {
    HttpServer::new(
        /*closure*/ || App::new().route("/", web::get().to(test)),
    )
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}

async fn test() -> impl Responder {
    "Hallo"
}
