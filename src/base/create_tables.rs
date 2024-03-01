use sqlx::postgres::PgPool;
use std::fs;
use std::path::Path;

pub async fn create_tables(pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut conn = pool.acquire().await?;

    // Путь к папке с SQL-файлами
    let sql_dir = Path::new("src/base/sql");

    // Проверяем, существует ли папка
    if sql_dir.exists() {
        // Получаем список всех файлов в папке
        for entry in fs::read_dir(sql_dir)? {
            let path = entry?.path();
            if path.is_file() {
                // Читаем содержимое файла
                let sql = fs::read_to_string(&path)?;
                // Выполняем SQL-запрос
                sqlx::query(&sql).execute(&mut *conn).await?;
            }
        }
    }

    Ok(())
}
