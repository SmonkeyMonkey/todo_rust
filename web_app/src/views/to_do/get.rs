use actix_web::Responder;

use crate::{json_serilization::to_do_items::ToDoItems, jwt::JwToken};

pub async fn get(token: JwToken) -> impl Responder {
    ToDoItems::get_state(token.user_id)
}
