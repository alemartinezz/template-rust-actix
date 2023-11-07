use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::models::user::{User, NewUser};
use crate::DbPool;

pub async fn get_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> impl Responder {
    // Implement your handler here
    HttpResponse::Ok().body("Get user")
}

pub async fn create_user(pool: web::Data<DbPool>, user: web::Json<NewUser>) -> impl Responder {
    // Implement your handler here
    HttpResponse::Created().body("Create user")
}
