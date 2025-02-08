use actix_web::{delete, get, patch, post, web, HttpResponse};
use sqlx::MySqlPool;
use log::{info, error};

use crate::models::{User, NewUser, UpdateUser};
use crate::errors::AppError;

#[post("/users")]
async fn create_user(
    pool: web::Data<MySqlPool>,
    user: web::Json<NewUser>,
) -> Result<HttpResponse, AppError> {
    info!("Mencoba menambahkan user baru: {:?}", user);
    let mut transaction = pool.begin().await.map_err(AppError::DatabaseError)?;

    let result = sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind(&user.name)
        .bind(&user.email)
        .execute(&mut *transaction)
        .await;

    match result {
        Ok(result) => {
            info!("Berhasil menambah user: {:?}", result);
            let user_id = result.last_insert_id();

            let user = sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = ?")
                .bind(user_id)
                .fetch_one(&mut *transaction)
                .await
                .map_err(AppError::DatabaseError)?;

            transaction.commit().await.map_err(AppError::DatabaseError)?;
            Ok(HttpResponse::Ok().json(user))
        }
        Err(err) => {
            error!("Gagal menambahkan user: {}", err); 
            transaction.rollback().await.map_err(AppError::DatabaseError)?;
            Err(AppError::DatabaseError(err))
        }
    }
}

#[get("/users/{id}")]
async fn get_user_by_id(
    pool: web::Data<MySqlPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let result = sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = ?")
        .bind(*id)
        .fetch_optional(&**pool)
        .await
        .map_err(AppError::DatabaseError)?;

    match result {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(AppError::NotFound), // Gunakan AppError::NotFound
    }
}

#[get("/users")]
async fn get_users(pool: web::Data<MySqlPool>) -> Result<HttpResponse, AppError> {
    let result = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(&**pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(HttpResponse::Ok().json(result))
}

#[patch("/users/{id}")]
async fn update_user(
    pool: web::Data<MySqlPool>,
    id: web::Path<i32>,
    user: web::Json<UpdateUser>,
) -> Result<HttpResponse, AppError> {
    let mut query = sqlx::query_builder::QueryBuilder::new("UPDATE users SET ");
    let mut updates = Vec::new();

    if let Some(_name) = &user.name {
        updates.push("name = ?"); // Placeholder ? dulu
    }
    if let Some(_email) = &user.email {
        updates.push("email = ?"); // Placeholder ? dulu
    }

    if updates.is_empty() {
        return Ok(HttpResponse::BadRequest().body("No fields to update"));
    }

    query.push(updates.join(", "));
    query.push(" WHERE id = ?"); // Placeholder ? untuk id

    let mut query = query.build(); // Build query sebelum bind values

    if let Some(name) = &user.name {
        query = query.bind(name); // Bind values setelah build
    }
    if let Some(email) = &user.email {
        query = query.bind(email); // Bind values setelah build
    }

    query = query.bind(*id); // Bind id setelah WHERE

    query
        .execute(&**pool)
        .await
        .map_err(AppError::DatabaseError)?;

    let result = sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = ?")
        .bind(*id)
        .fetch_one(&**pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(HttpResponse::Ok().json(result))
}

#[delete("/users/{id}")]
async fn delete_user(
    pool: web::Data<MySqlPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(*id)
        .execute(&**pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(HttpResponse::Ok().finish())
}

// FOR TESTING ONLY
// Input 10000 data to database
#[post("/insert")]
async fn insert_10k_user(
    pool: web::Data<MySqlPool>
) -> Result<HttpResponse, AppError> {
    for _n in 0..10000 {
        let name = format!("User {}", _n);
        let email = format!("email{}@mail.com", _n);
        let _result = sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
            .bind(&name)
            .bind(&email)
            .execute(&**pool)
            .await
            .map_err(AppError::DatabaseError)?;
    }

    Ok(HttpResponse::Ok().json("ok"))
}
