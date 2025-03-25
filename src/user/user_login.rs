use rocket::{post, serde::json::Json, State, response::status::Forbidden};
use sqlx::{query, query_as, PgPool};
use bcrypt::verify;
use cuid::cuid2;

use crate::models::user::{Credentials, LoginResponse, User, Role};

#[post("/user_login", format = "json", data = "<credentials>")]
pub async fn user_login(pool: &State<PgPool>, credentials: Json<Credentials>) -> Result<Json<LoginResponse>, Forbidden<String>> {
    let credentials = credentials.into_inner();

    // Получаем user из базы данных
    let user: Option<User> = query_as::<_, User>(r#"SELECT * FROM users WHERE username = $1"#)
        .bind(&credentials.username)
        .fetch_optional(pool.inner())
        .await
        .map_err(|_| Forbidden("Не смог найти user".to_string()))?;

    let user = match user {
        Some(h) => h,
        None => return Err(Forbidden("Не существует такого пользователя".to_string())),
    };
    // проверяем наличие роли у данного пользователя
    let role: Option<Role> = query_as::<_, Role>(r#"SELECT * FROM roles WHERE user_guid = $1 and name = $2;"#)
        .bind(&user.guid)
        .bind(&credentials.role)
        .fetch_optional(pool.inner())
        .await
        .map_err(|_| Forbidden("Не смог найти role".to_string()))?;

    let role = match role {
        Some(h) => h,
        None => return Err(Forbidden("У данного пользователя нет такой доступной роли".to_string())),
    };
  
    // теперь проверяем хеш пароля
    if let Ok(valid) = verify(credentials.password.as_bytes(), &user.password_hash) {
        if valid {
            // Генерируем новый UUID для токена
            let token = cuid2();

            // Обновляем строку в базе данных с новым токеном
            query(r#"UPDATE roles SET token = $1 WHERE user_guid = $2"#)
                .bind(&token)
                .bind(&user.guid)
                .execute(pool.inner())
                .await
                .map_err(|_| Forbidden("Не смог update token".to_string()))?;

            Ok(Json(LoginResponse {
                message: "Logged in successfully".to_string(),
                token,
                role: role.name.to_string(),
            }))
        } else {
           Err(Forbidden("Неправильный пароль".to_string()))
        }
    } else {
        Err(Forbidden("Неправильный пароль".to_string()))
    }
}
