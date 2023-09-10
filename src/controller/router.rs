use std::{result, time};
use axum::Json;


mod dao {
    include!("../dao/user_db.rs");
}

// which calls one of these handlers
pub async fn user() -> Json<Vec<dao::entity::User>> {
    let result = dao::query_user();
    match result.await {
        Ok(users) => Json(users),
        Err(_) => Json(vec![]),  // 返回一个空的用户列表或根据实际情况处理错误
    }
}

pub async fn get_foo() -> String {
    String::from("get:foo")
}

pub async fn post_foo() -> String {
    String::from("post:foo")
}

pub async fn foo_bar() -> String {
    String::from("foo:bar")
}

