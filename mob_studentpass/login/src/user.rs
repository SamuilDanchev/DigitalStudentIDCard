use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};

use crate::lib_error::LoginError;

///Expected form when logging in
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginUser {
    pub loginname: String,
    pub password: String,
}

///Return value if registration was successful
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub loginname: String,
    active: bool,
}

///Checks if the user exists, if it exists it is returned
pub async fn signin(pool: &Pool<Postgres>, user: LoginUser) -> Result<User, LoginError> {
    let user = sqlx::query_as::<_, User>(
        "\
     SELECT id, name, loginname, active FROM user_user \
     where loginname = $1 AND \
     password = decode($2, 'hex'); \
    ",
    )
    .bind(user.loginname.to_uppercase())
    .bind(format!("{:?}", md5::compute(user.password)))
    .fetch_one(pool)
    .await?;

    Ok(user)
}
