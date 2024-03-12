use actix_web::{get,post,put,delete,web,Responder,HttpResponse};
use crate::{AppState,TodolistEntry};
use super::models::{CreateEntryData,UpdateEnrtyData};

#[get("/todolist/entries")]
async fn get_entries(data:web::Data<AppState>) -> impl Responder{
    HttpResponse::Ok().json(data.todolist_entries.lock().unwrap().to_vec())
}

#[post("todolist/entries")]
async fn create_entry(data:web::Data<AppState>,param_obj:web::Json<CreateEntryData>) -> impl Responder{
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    let mut max_id:i32 = 0; 
    for i in 0..todolist_entries.len(){
        if todolist_entries[i].id > max_id {
            max_id = todolist_entries[i].id;
        }
    }
        todolist_entries.push(TodolistEntry{
            id : max_id + 1,
            title : param_obj.title.clone(),
            description : param_obj.description.clone()
        });
        HttpResponse::Ok().json(todolist_entries.to_vec())
}

#[put("todolist/entries/{id}")]
async fn update_entry(data:web::Data<AppState>,path:web::Path<i32>,param_obj:web::Json<UpdateEnrtyData>) -> impl Responder{
    let id = path.into_inner();
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    for i in 0..todolist_entries.len(){
        if todolist_entries[i].id == id {
            todolist_entries[i].title = param_obj.title.clone();
            todolist_entries[i].description = param_obj.description.clone();
            break;
        }
    }
    HttpResponse::Ok().json(todolist_entries.to_vec())
}

#[delete("todolist/entries/{id}")]
async fn delete_entry(data:web::Data<AppState>,path:web::Path<i32>) -> impl Responder{
    let mut todolist_entries = data.todolist_entries.lock().unwrap();

    let id = path.into_inner();
    *todolist_entries = todolist_entries.to_vec().into_iter().filter(|x| x.id!=id).collect();

    HttpResponse::Ok().json(todolist_entries.to_vec())
}

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(get_entries)
    .service(create_entry)
    .service(update_entry)
    .service(delete_entry);
}