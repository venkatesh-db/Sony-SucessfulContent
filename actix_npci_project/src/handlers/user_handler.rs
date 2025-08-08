
use actix_web::{web, HttpResponse};
use crate::app_state::AppState;
use crate::models::user::User;

pub async fn get_user(state: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let user_id = path.into_inner();
    let user = User {
        id: user_id.clone(),
        name: format!("NPCI_{}", user_id),
    };

    // Store in Redis (mock)
    let _ = state.redis.get_connection().unwrap().set(&user_id, &user.name);

    HttpResponse::Ok().json(user)
}