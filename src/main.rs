use sqlx::{Connection, SqliteConnection};


fn main() {
    let conn = SqliteConnection::connect("sqlite::memory:").await.unwrap();
}
