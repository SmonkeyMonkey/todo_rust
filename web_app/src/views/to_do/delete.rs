use actix_web::{web, HttpResponse};

use crate::database::DB;
use crate::diesel;
use diesel::prelude::*;

use crate::models::item::item::Item;
use crate::{
    json_serilization::{to_do_item::ToDoItem, to_do_items::ToDoItems},
    jwt::JwToken,
    schema::to_do,
};

pub async fn delete(to_do_item: web::Json<ToDoItem>, token: JwToken, db: DB) -> HttpResponse {
    let items = to_do::table
        .filter(to_do::columns::title.eq(&to_do_item.title.as_str()))
        .filter(to_do::columns::user_id.eq(&token.user_id))
        .order(to_do::columns::id.asc())
        .load::<Item>(&db.connection)
        .unwrap();
    let _ = diesel::delete(&items[0]).execute(&db.connection);
    HttpResponse::Ok().json(ToDoItems::get_state(token.user_id))
}
