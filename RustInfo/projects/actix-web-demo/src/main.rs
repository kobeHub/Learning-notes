use actix_http::http;
use bytes::Bytes;
use std::sync::Mutex;
use serde::Serialize;
use serde_json;
use serde::Deserialize;
use futures::future::{ready, ok, Ready};
use futures::stream::once;
use listenfd::ListenFd;
use actix_web::{web, guard, get, post};
use actix_web::{App, Error, Either, HttpRequest,
                HttpResponse, HttpServer, Responder, Result};

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

#[get("/object")]
async fn sim_obj() -> impl Responder {
    SimObj("A simple tuple struct")
}

#[post("/users")]
async fn create_users(info: web::Json<User>) -> impl Responder {
    info.into_inner()
}

#[post("/stream")]
async fn from_stream() -> HttpResponse {
    let body = once(ok::<_, Error>(Bytes::from_static(b"Jack Ma")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[post("status")]
async fn diff_status(info: web::Json<Current>) -> RegisteredResult {
    if info.value > 100 {
        Either::A(HttpResponse::BadRequest().body("The task is done!"))
    } else {
        Either::B(Ok("The task is doing..."))
    }
}

#[derive(Serialize)]
struct SimObj(&'static str);

impl Responder for SimObj {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
                 .content_type("application/json")
                 .body(body)
        ))
    }
}

type RegisteredResult = Either<HttpResponse, Result<&'static str, Error>>;

#[derive(Deserialize)]
struct Current {
    value: u32,
}

#[derive(Deserialize, Serialize)]
struct User {
    id: usize,
    name: String,
    sex: char,
}

impl Responder for User {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
                 .content_type("application/json")
                 .body(body)
        ))
    }
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
                .service(sim_obj)
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
                    .service(create_users)
                    .service(name)
                    .service(from_stream)
                    .service(diff_status)
            )
            .service(
                web::scope("/client")
                    .configure(scoped_config)
                    .service(diff_status)
            )
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8088")?
    };

    server.run().await
}
