use actix_files as fs;
use actix_web::{web, App, HttpServer, Responder, web::Data};
use tera::{Context, Tera};

async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("name", "User");  // готовим передачу значений в html
    let rendered = tera.render("index.html", &context).unwrap();  // пихаем html + контекст
    actix_web::HttpResponse::Ok().body(rendered)
}

// пример если не нужны переменные котекстные
async fn about(tera: web::Data<Tera>) -> impl Responder {
    let rendered = tera.render("about.html", &Context::new()).unwrap();
    actix_web::HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("src/templates/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .service(fs::Files::new("/static", "./src/static"))
            .app_data(Data::new(tera.clone()))
            .route("/", web::get().to(index))
            .route("/about", web::get().to(about))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
