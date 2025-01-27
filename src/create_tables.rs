use sqlx::{Executor, PgPool};
use std::fs;
use std::path::Path;

pub async fn create_tables(pool: &PgPool) -> Result<(), sqlx::Error> {
    let sql_files = vec![
        "users_table.sql",
        "roles_table.sql"
    ];

    for file_name in sql_files {
        let file_path = format!("src/sql/{}", file_name);
        println!("Выполняем SQL file: {}", &file_name );
        let sql = fs::read_to_string(Path::new(&file_path))
            .unwrap_or_else(|_| panic!("Не смог прочитать SQL file: {}", file_path));
        pool.execute(&*sql).await?;
    }

    Ok(())
}
