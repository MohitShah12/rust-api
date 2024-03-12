use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize,Serialize};
use std::sync::Mutex;

mod todolist;
use todolist::services;


struct AppState{
    todolist_entries : Mutex<Vec<TodolistEntry>>
}

#[derive(Serialize,Deserialize,Clone)]
struct TodolistEntry{
    id:i32,
    description:String,
    title:String
}

#[get("/")]
async fn index() -> String{
    "Everything is Fine!!".to_string()
}



// struct  User{
//     id : u32,
//     name : String,
//     age : u32
// }

// async fn index() -> impl Responder{
//     HttpResponse::Ok().body("Welcome")
// }

// async fn get_users() -> impl Responder{
//     let users = vec![User{
//         id:1,
//         name:"Alice".to_string(),
//         age:30
//     },
//     User{
//         id:2,
//         name:"Bob".to_string(),
//         age:28
//     }
//     ];
//     HttpResponse::Ok().json(users)
// }

// async fn get_user(path: web::Path<(u32,)>) -> impl Responder{
//     let user_id = path.into_inner().0;
//     println!("user Id {}",user_id);
//     let user = User{
//         id:user_id,
//         name:"Alice".to_string(),
//         age:30
//     };
//     HttpResponse::Ok().json(user)
// }

#[actix_web::main]
  async fn main() -> std::io::Result<()>{
      println!("server is running");

    let app_data = web::Data::new(AppState{
        todolist_entries:Mutex::new(vec![])
    });
    HttpServer::new(move ||{
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
    // println!("Server is running on port 3000.....");
    // HttpServer::new(||{
    //     App::new()
    //         .route("/",web::get().to(index))
    //         // .route("/users",web::get().to(get_users))
    //         // .route("/user/{id}",web::get().to(get_user))
    // })
    // .bind("127.0.0.1:3000")?
    // .run()
    // .await
}
