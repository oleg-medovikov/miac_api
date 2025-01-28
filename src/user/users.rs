use rocket::{get, serde::json::Json, State, response::status::Custom, http::Status};
use sqlx::{PgPool, query_as};
use crate::models::user::User;


#[get("/users")]
pub async fn get_users(pool: &State<PgPool>) -> Result<Json<Vec<User>>, Custom<String>> {
    let query = r#"SELECT 
    guid, tg_id, username, '' as password_hash, fio, date_create   
    from users"#;

    let users: Vec<User> = match query_as::<_, User>(query)
        .fetch_all(pool.inner())
        .await
    {
        Ok(events) => events,
        Err(e) => {
            return Err(Custom(Status::InternalServerError, format!("Database error: {}", e),));
        }
    };

    Ok(Json(users))
}
