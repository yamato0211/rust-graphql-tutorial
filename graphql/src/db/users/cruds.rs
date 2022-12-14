use crate::db::{
    PgPool,
    users::{
        User,
        UserNewForm,
        LoginUserForm,
    },
    schema::users::dsl::*,
};
use super::utils::Jwt;
use actix_web::web::Data;
use anyhow::{Result,Error};
use diesel::{
    debug_query,
    dsl::{
        delete,
        insert_into,
    },
    pg::Pg,
    prelude::*,
};
use log::debug;
use std::string::String;
use pwhash::bcrypt;
use uuid::Uuid;


pub struct Cruds;

impl Cruds {
    // 全てのUserを配列として返す.
    pub fn all_user(pool: &Data<PgPool>) -> Result<Vec<User>> {
        let connection = pool.get().expect("get db connection failed");
        Ok(users.load(&connection).expect("get all user failed"))
    }

    // key_idに合致するUserを返す.
    pub fn find_by_id(pool: &Data<PgPool>, key_id: Uuid) -> Result<User> {
        let connection = pool.get().expect("get db connection failed");
        let query = users.find(key_id);

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_result(&connection).expect("user not found"))
    }


    // new_formを新しい行としてDBに追加し、その行のUserを返す.
    pub fn insert_user(pool: &Data<PgPool>, mut new_form: UserNewForm) -> Result<User> {
        let connection = pool.get().expect("get db connection failed");
        new_form.password = bcrypt::hash(new_form.password).unwrap();
        let query = insert_into(users).values(new_form);

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_result(&connection).expect("insert user failed"))
    }

    pub fn authentication(pool: &Data<PgPool>, login_user: LoginUserForm) -> Result<String, Error> {
        let connection = pool.get().expect("get db connection failed");
        let query = users.filter(email.eq(login_user.email));
        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);
        let user: _ = query.first::<User>(&connection).expect("user not found");
        assert_eq!(bcrypt::verify(login_user.password, &user.password),true, "password is Invalid");
        Ok(Jwt::encode_jwt(&user))
    }

    // idに合致するUserの行をDBから削除し、その行のUserを返す.
    pub fn delete_user(pool: &Data<PgPool>, key_id: Uuid) -> Result<User> {
        let connection = pool.get().expect("get db connection failed");
        let query = delete(users.find(key_id));

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_result(&connection).expect("delete user failed"))
    }
}
