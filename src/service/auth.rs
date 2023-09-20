use axum::http::header::ToStrError;
use crate::api::dao::entity::AuthReq;
use crate::api::dao::entity::AuthResp;
use crate::api::dao;
use axum::Json;
use chrono::Utc;
use sqlx::Error;
use uuid::Uuid;
use crate::api::service::common_entity::Resp;

pub(crate) mod common_entity {
    include!("../entity/common.rs");
}

pub async fn login(req: AuthReq) -> Resp<AuthResp> {
    let result = dao::query_user_by_account(req);

    let mut resp = common_entity::Resp {
        data: None,
        message: None,
    };

    match result.await {
        Ok(user) => {
            let uuid = Uuid::new_v4();
            let uuid_string = uuid.to_simple().to_string();
            let _uuid = uuid_string.clone();
            let res = AuthResp {
                token: uuid_string,
            };


            match dao::update_token(user, _uuid).await {
                None => {
                    resp.data = Some(res);
                }
                Some(err) => {
                    resp.message = Some(err.to_string());
                }
            }
        }
        Err(err) => {
            resp.message = Some(err.to_string())
        }
    }

    return resp;
}


pub async fn auth(mut token: String) -> bool {
    if token.contains("Bearer") {
        token = token.replace("Bearer ", "");
    }
    let result = dao::query_user_by_token(token);

    return match result.await {
        Ok(user) => {
            let current_date = Utc::now();
            if current_date > user.token_expire.unwrap() {
                return false;
            }
            true
        }
        Err(_) => {
            false
        }
    };
}

