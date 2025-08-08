use actix_web::{web, HttpResponse};
use sea_orm::{EntityTrait, Set, ActiveModelTrait, DatabaseConnection};
use uuid::Uuid;
use crate::models::user::{ActiveModel, Entity as User, Model};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

pub async fn create_user(
    db: web::Data<DatabaseConnection>,
    new_user: web::Json<NewUser>,
) -> HttpResponse {
    let user = ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(new_user.name.clone()),
        email: Set(new_user.email.clone()),
    };

    match user.insert(db.get_ref()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_users(db: web::Data<DatabaseConnection>) -> HttpResponse {
    match User::find().all(db.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
