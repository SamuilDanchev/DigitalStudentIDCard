use std::sync::Arc;
use jsonwebtoken::TokenData;
use ldap3::{LdapConnAsync, Scope, SearchEntry};
use log::{error, info};
use serde::{Deserialize, Serialize};
use login::{create_token, Expiration, LoginError, LoginUser, signin, User, verify_token};

use warp::Filter;

use crate::db::{Db, with_db};
use crate::error;
use crate::error::ApiError;
use crate::user::Student;

pub fn routes(
    db: Arc<Db>,
) -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "Access-Control-Allow-Origin",
            "authorization",
            "Content-Type",
        ])
        .allow_methods(vec!["POST", "GET"]);

    login(db.clone())
        .or(token(db.clone()))
        .or(student_id_card())
        .recover(error::handle_rejection)
        .with(cors)
}

pub fn student_id_card() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("studentID")
        .and(warp::get())
        .and(warp::query())
        .and_then(fetch_student_id_card)
}

#[derive(Deserialize)]
pub struct SearchQuery {
    student: Option<String>,
}

//Token mitschicken ?
pub async fn fetch_student_id_card(query: SearchQuery) -> Result<impl warp::Reply, warp::Rejection> {
    match query.student {
        None => {
            Err(warp::reject::custom(error::ApiError::QueryParameterError))
        }
        Some(v) => {
            let _user = "dredenius@dannybbs.com";
            let filter = format!("(mail={})", v);

            let (conn, mut ldap) = LdapConnAsync::new("ldap://localhost:389").await.unwrap();
            ldap3::drive!(conn);
            let result = ldap.simple_bind("cn=admin,dc=dannybbs,dc=com", "admin_pass").await.unwrap().success();
            match result {
                Ok(_) => {
                    let (rs, _res) = ldap.search(
                        "ou=School,dc=dannybbs,dc=com",
                        Scope::Subtree,
                        &filter,
                        vec!["uidNumber", "givenName", "sn", "description", "ou"]
                    ).await.unwrap().success().unwrap();

                    for entry in rs {
                        let mut a = SearchEntry::construct(entry);
                        let s = Student::entry_to_student(a);
                        println!("{:?}", s);
                    }
                    Ok("ldl")
                }
                Err(_) => {
                    Err(warp::reject::custom(ApiError::QueryParameterError))
                }
            }
            /*Ok(warp::reply::json(&format!("{}", v)))*/
        }
    }
}

pub fn login(
    db: Arc<Db>,
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("login")
        .and(warp::post())
        .and(with_db(db))
        .and(warp::body::json())
        .and_then(verify_login)
}

pub async fn verify_login(
    db: Arc<Db>,
    user: LoginUser,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", user);
    match signin(&db.pool, user).await {
        Ok(user) => {
            let token =
                create_token::<User>("secret", Expiration::HOURS(5), Option::from(user)).unwrap();
            info!("LOGIN SUCCESSFULL TOKEN: {}", token);
            Ok(warp::reply::json(&token))
        }
        _ => {
            info!("LOGIN FAILED");
            Err(warp::reject::custom(error::ApiError::CredentialsError))
        }
    }
}

pub fn token(db: Arc<Db>) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("verifyToken")
        .and(warp::get())
        .and(with_db(db))
        .and(warp::header::<String>("Authorization"))
        .and_then(handle_verify_token)
}

pub async fn handle_verify_token(db: Arc<Db>, token: String) -> Result<impl warp::Reply, warp::Rejection> {
    match verify_token::<User>(token, "secret") {
        Ok(data) => {
            match data.claims.payload {
                None => Err(warp::reject::custom(ApiError::TokenError)),
                Some(payload) => Ok(payload.loginname)
            }
        }
        Err(err) => {
            error!("{err:#?}");
            Err(warp::reject::custom(ApiError::TokenNotFoundError))
        }
    }
}
