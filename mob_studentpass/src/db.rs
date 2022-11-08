use std::convert::Infallible;
use std::sync::Arc;
use ldap3::{Ldap, LdapConnAsync};

use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Pool, Postgres,
};
use warp::Filter;

pub struct Db {
    pub pool: Pool<Postgres>,
}

impl Db {
    pub async fn establish_db_connection() -> Self {
        let options = PgConnectOptions::new()
            .host("192.168.91.241")
            .port(5432)
            .username("postgres")
            .password("admin")
            .database("nbs");

        let pool = PgPoolOptions::new().connect_with(options).await.unwrap();

        Self { pool }
    }
}

pub fn with_db(db: Arc<Db>) -> impl Filter<Extract = (Arc<Db>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
