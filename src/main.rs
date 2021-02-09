#![feature(decl_macro)]

#[macro_use]
extern crate diesel;

mod schema;

use crate::schema::todo;
use rocket::{ get, self, routes, post, put };
use rocket_contrib::{ json::Json, databases::{ database, diesel::{ PgConnection } } };
use diesel::{ Queryable, Insertable, prelude::* };
use serde_derive::{ Serialize, Deserialize };

#[database("postgres")]
struct DbCOnn(PgConnection);


#[derive(Queryable, Serialize)]
struct Todo {
    id: i32,
    title: String,
    checkout: bool
}

#[derive(Insertable, Deserialize)]
#[table_name="todo"]
struct newTodo {
    title: String,
}

#[get("/")]
fn get_todos(conn: DbCOnn) -> Json<Vec<Todo>> {
    let todos = todo::table
        .order(todo::columns::id.desc())
        .load::<Todo>(&*conn)
        .unwrap();
    
    Json(todos)
}

#[post("/", data = "<new_todo>")]
fn create_todo(conn: DbCOnn, new_todo: Json<newTodo>) -> Json<Todo> {
    let result = diesel::insert_into(todo::table)
        .values(&new_todo.0)
        .get_result(&*conn)
        .unwrap();
    
    Json(result)
}

#[put("/<id>")]
fn check_todo(conn: DbCOnn, id: i32) -> Json<Todo> {
    let target = todo::table 
        .filter(todo::columns::id.eq(id));

    let result = diesel::update(target)
        .set(todo::columns::checked.eq(true))
        .get_result(&*conn)
        .unwrap();

    Json(result)
}

#[get("/")]
fn hello() -> &'static str {
    "Hello World"
}

#[get("/<name>")]
fn hello_name(name: String) -> String {
    format!("Hello {}!", name)
}

fn main() {
    rocket::ignite()
        .attach(DbCOnn::fairing())
        .mount("/hello", routes![hello, hello_name])
        .mount("/todos", routes![get_todos, create_todo, check_todo])
        .launch();
}
