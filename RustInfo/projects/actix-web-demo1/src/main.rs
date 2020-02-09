use actix_web::{get, post, web, error};
use actix_web::{App, FromRequest, HttpServer, Responder,
                HttpRequest, HttpResponse, Result};
use serde::Deserialize;
use listenfd::ListenFd;
use std::cell::Cell;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[get("/")]
async fn index() -> Option<String> {
    Some("The http server is heathy!".to_owned())
}

#[post("/users/{user_id}/{friend}")]
async fn create_users(path: web::Path<(u32, String)>,
                      info: web::Json<UserAddi>) -> Result<String> {
    Ok(format!("The user details:\t id:{}, friend:{}, additions: {}",
    path.0, path.1, info))
}

#[get("/users/{user_id}/{friend}")]
async fn get_users(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend")
        .unwrap().parse().unwrap();
    let id: u32 = req.match_info().query("user_id").parse().unwrap();
    let path = req.path();
    let query = req.query_string();
    if let Some(app_data) = req.app_data::<u32>() {
        Ok(format!("Welcome {}, id:{}, path:{}, query:{}, global data: {}",
                   name, id, path, query, app_data))
    } else {
        Ok(format!("Welcome {}, id:{}, path:{}, query:{}, global data: None",
                   name, id, path, query))
    }
}

#[post("forms")]
async fn create_forms(form: web::Form<Data>) -> Result<String> {
    Ok(format!("The contents of the form: {}", form.content))
}

async fn create_annos(info: web::Json<Data>) -> Result<String> {
    Ok(format!("A annos create: {}", info.content))
}

#[get("/count")]
async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!("count: {}", data.count.load(Ordering::Relaxed))
}

#[post("/addone")]
async fn add_one(data: web::Data<AppState>) -> impl Responder {
//    let count = data.count.get();
    data.count.fetch_add(1, Ordering::Relaxed);

    format!("After add one: {}", data.count.load(Ordering::Relaxed))
}

#[derive(Deserialize)]
struct UserAddi {
    sex: char,
    age: u8,
    address: String,
}

#[derive(Deserialize)]
struct Data {
    content: String,
}

#[derive(Clone)]
struct AppState {
    count: Arc<AtomicUsize>,
}

use std::fmt;

impl fmt::Display for UserAddi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(sex: {}, age: {}, address: {})", self.sex, self.age, self.address)
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut sys = ListenFd::from_env();
    let data = AppState {
        count: Arc::new(AtomicUsize::new(0)),
    };

    let mut server = HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .app_data(12)
            .service(web::scope("/api")
                     //.data(data.clone())
                     .service(index)
                     .service(create_users)
                     .service(get_users)
                     .service(create_forms)
                     .service(
                         web::resource("/annos")
                             .app_data(
                                 // 设置json extractor 配置
                                 web::Json::<Data>::configure(|cfg| {
                                     cfg.limit(4096).error_handler(|err, _req| {
                                         // 处理错误情形
                                         error::InternalError::from_response(
                                             err,
                                             HttpResponse::Conflict().body("The contents are too large!"),
                                         )
                                         .into()
                                     })
                                 })
                             )
                        .route(web::post().to(create_annos))
                     )
                     .service(show_count)
                     //.service(add_one)
            )
            .service(
                web::scope("/app")
                    .service(add_one)
            )
    });

    server = if let Some(l) = sys.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8077")?
    };
    server.run().await
}
