use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use serde::Serialize;

use crate::{database::DBCONNECTION, diesel, models::item::item::Item};
use diesel::prelude::*;

use crate::{
    schema::to_do,
    to_do::{enums::TaskStatus, structs::base::Base, to_do_factory, ItemTypes},
};

#[derive(Debug, Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_array_buff = Vec::new();
        let mut done_array_buff = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Pending(packed) => pending_array_buff.push(packed.super_struct),
                ItemTypes::Done(packed) => done_array_buff.push(packed.super_struct),
            }
        }
        let pending_count = pending_array_buff.len() as i8;
        let done_count = done_array_buff.len() as i8;
        ToDoItems {
            pending_items: pending_array_buff,
            done_items: done_array_buff,
            pending_item_count: pending_count,
            done_item_count: done_count,
        }
    }
    pub fn get_state(user_id: i32) -> ToDoItems {
        let connection = DBCONNECTION.db_connection.get().unwrap();

        let items = to_do::table
            .filter(to_do::columns::user_id.eq(&user_id))
            .order(to_do::columns::id.asc())
            .load::<Item>(&connection)
            .unwrap();
        let mut buffer = Vec::with_capacity(items.len());

        for item in items {
            let status = TaskStatus::from_string(item.status.as_str().to_string());
            let item = to_do_factory(&item.title, status);
            buffer.push(item);
        }
        ToDoItems::new(buffer)
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
