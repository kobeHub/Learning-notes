use actix_http::http;
use std::sync::Mutex;
use listenfd::ListenFd;
use actix_web::{web, guard, get, App, HttpResponse, HttpServer, Responder};

async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .header("X-TEST", "Jack Ma")
        .header(http::header::CONTENT_TYPE, "application/json")
        .json("The InnoHub API is heathy!")
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .body("Welcome to InnoHub API!\n")
}

struct MetaInfo<'a> {
    name: &'a str,
    counter: usize,
}

// 所有 route 见共享数据以 web::Data 进行存储，其内部`Data` 类型使用了
// `Arc` 如果所使用的数据类型实现了`Send + Sync`可以使用 `web::Data::new`
// 来避免双`Arc`
#[get("/name")]
async fn name<'a>(data: web::Data<Mutex<MetaInfo<'a>>>) -> String {
    // `web::data` 实现了`Deref` trait， 属于智能指针，可以使用解引用强制多态
    let mut data = data.lock().unwrap();
    data.counter += 1;
    format!("App name:{}, call `/api/name` times:{}\n",
              data.name, data.counter)
}

// Configure use `web::App` configure method to set config
// among modules

// The config can be loaded in modules FnOnce(&mut web::ServiceConfig)
fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok().body("Scoped test is healty\n")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
    );
}

// The config can be loaded in modules
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{contents}")
            .guard(guard::Header("Test-params", "true"))
            .route(web::get().to(|info: web::Path<String>| {
                HttpResponse::Ok()
                    .body(format!("Test params successfully: {}\n", info))
            }))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
    );
}

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    let mut listenfd = ListenFd::from_env();
    let data = web::Data::new(Mutex::new(MetaInfo{ name: "Actix-demo", counter: 0}));

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone()) // register data
            .configure(config)
            .service(
                web::scope("/api")
                // use guard to set route or scope filter
                .guard(guard::Header("Token-access", "justatoken"))
                .service(index)
                // 由于Scope并没有指定唯一的资源实体，所以需要path参数指定
                // 唯一URI，然后指定相应的router
                .route("/hello", web::get().to(hello))
                .service(name)
            )
            .service(
                web::scope("/app")
                    // use a fn_guard to filter
                    .guard(guard::fn_guard(
                        |req| req.headers()
                            .contains_key("content-type")
                    ))
                    .service(name)
            )
            .service(
                web::scope("/client")
                    .configure(scoped_config)
            )
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8088")?
    };

    server.run().await
}
