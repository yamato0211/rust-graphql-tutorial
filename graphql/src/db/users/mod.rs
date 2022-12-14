use crate::db::schema::users;

mod cruds;
mod utils;
use uuid::Uuid;
pub use cruds::Cruds;
pub use utils::Jwt;
// Identifiable: この構造体がDBのテーブルであることを示す.
// Queryable: この構造体がDBに問い合わせることができることを示す.
// Clone: おまけ.
#[derive(Clone, Identifiable, Queryable)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String
}

// Insertable: この構造体がDBに新しい行を挿入できることを示す.
#[derive(Insertable)]
#[table_name = "users"]
pub struct UserNewForm {
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Queryable, Clone)]
pub struct LoginUserForm {
    pub email: String,
    pub password: String
}

