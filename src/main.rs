use actix_files as fs;
use actix_web::{web, App, HttpServer, Responder, web::Data};
use tera::{Context, Tera};

async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("name", "User");  // готовим передачу значений в html
    let rendered = tera.render("index.html", &context).unwrap();  // склеиваем html и контекст
    actix_web::HttpResponse::Ok().body(rendered)  // отдаём результат
}

// пример если не нужны переменные котекстные
async fn about(tera: web::Data<Tera>) -> impl Responder {
    let rendered = tera.render("about.html", &Context::new()).unwrap();  // шаблонизацией отрисовываем, но контекст пустой передаём
    actix_web::HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("src/templates/**/*").unwrap();  // указываем, что в шабллнизаии учавствуют все теймплатесы

    HttpServer::new(move || {
        App::new()
            .service(fs::Files::new("/static", "./src/static"))  // Actix-web не автоматически обслуживает статические файлы из директории шаблонов. Вам нужно явно указать где искать статические файлы. p.s. работает на все html
            .app_data(Data::new(tera.clone()))  // подключаем шаблонизацию, опять-таки на всех html
            .route("/", web::get().to(index))
            .route("/about", web::get().to(about))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
