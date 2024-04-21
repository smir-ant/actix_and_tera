use actix_files as fs;
use actix_web::{web, App, HttpServer, Responder, web::Data};
use tera::{Context, Tera};

async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("name", "User");
    let rendered = tera.render("index.html", &context).unwrap();
    actix_web::HttpResponse::Ok().body(rendered)
}

// Новый обработчик для /about
async fn about(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("name", "User");
    let rendered = tera.render("about.html", &context).unwrap();
    actix_web::HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("src/templates/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .service(fs::Files::new("/static", "./src/static"))  // Добавьте эту строку
            .app_data(Data::new(tera.clone()))  // Измените эту строку
            .route("/", web::get().to(index))
            .route("/about", web::get().to(about))  // Новый маршрут
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}