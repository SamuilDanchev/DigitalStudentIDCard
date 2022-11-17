use ldap3::{LdapConnAsync, Scope, SearchEntry};
use log::{error, info};
use login::{create_token, Expiration, LoginError, LoginUser,verify_token};

use warp::{Filter};

use crate::error;
use crate::error::ApiError;
use crate::user::{LoginResponse, Student, VerifyResponse};

pub fn routes() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
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

    login()
        .or(verify())
        .recover(error::handle_rejection)
        .with(cors)
}

pub fn login() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_login)
}

pub async fn handle_login(_login_user: LoginUser) -> Result<impl warp::Reply, warp::Rejection> {
    let (conn, mut ldap) = LdapConnAsync::new("ldap://localhost:389").await.unwrap();
    ldap3::drive!(conn);
    let result = ldap.simple_bind("cn=admin,dc=dannybbs,dc=com", "admin").await.unwrap().success();
    match result {
        Ok(_) => {
            let filter = format!("(uidNumber=1000)");
            let (rs, _res) = ldap.search(
                "ou=School,dc=dannybbs,dc=com",
                Scope::Subtree,
                &filter,
                vec!["uidNumber", "givenName", "sn", "description", "ou", "gecos", "audio", "mobile", "l"],
            ).await.unwrap().success().unwrap();

            let mut students = Vec::new();

            for entry in rs {
                let e = SearchEntry::construct(entry);
                students.push(Student::entry_to_student(e));
            }

            let student = match students.get(0) {
                None => {
                    Student {
                        uid: 0,
                        firstname: "".to_string(),
                        lastname: "".to_string(),
                        birthday: "".to_string(),
                        school_class: "".to_string(),
                        printed_in: "".to_string(),
                        valid_to: "".to_string(),
                        image: "".to_string(),
                    }
                }
                Some(s) => {
                    let s = s.clone();
                    s
                }
            };
            info!("STUDENT: {:?}", student);
            let token =
                create_token::<&Student>("secret", Expiration::MINUTES(5), Some(&student)).unwrap();
            info!("LOGIN SUCCESSFULL TOKEN: {}", token);

            let res = LoginResponse::create_login_response(token, student);
            Ok(warp::reply::json(&res))
        }
        Err(_) => {
            Err(warp::reject::custom(ApiError::QueryParameterError))
        }
    }
}

pub fn verify() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("verify")
        .and(warp::get())
        .and(warp::header::<String>("Authorization"))
        .and_then(handle_verify)
}

pub async fn handle_verify(token: String) -> Result<impl warp::Reply, warp::Rejection> {
    info!("TOKEN: {}", token);
    match verify_token::<VerifyResponse>(&token, "secret") {
        Ok(token) => {
            Ok(warp::reply::json(&token.claims.payload))
        }
        Err(e) => {
            error!("ERROR: {:?}", e);
            let e = match e {
                LoginError::InvalidSignature => warp::reject::custom(ApiError::InvalidSignature),
                LoginError::InvalidToken => warp::reject::custom(ApiError::InvalidToken),
                LoginError::ExpiredSignature => warp::reject::custom(ApiError::ExpiredSignature),
                _ => warp::reject::custom(ApiError::UnknownError)
            };
            Err(e)
        }
    }
}

