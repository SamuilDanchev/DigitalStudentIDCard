use crate::lib_error::LoginError;

///Possible expiration times
pub enum Expiration {
    SECONDS(i64),
    MINUTES(i64),
    HOURS(i64),
    DAYS(i64),
    WEEKS(i64),
}

pub(crate) fn expiration_date(exp: Expiration) -> Result<usize, LoginError> {
    let exp = match exp {
        Expiration::SECONDS(n) => chrono::Duration::seconds(n),
        Expiration::MINUTES(n) => chrono::Duration::minutes(n),
        Expiration::HOURS(n) => chrono::Duration::hours(n),
        Expiration::DAYS(n) => chrono::Duration::days(n),
        Expiration::WEEKS(n) => chrono::Duration::weeks(n),
    };

    match chrono::Utc::now().checked_add_signed(exp) {
        Some(dt) => Ok(dt.timestamp() as usize),
        _ => Err(LoginError::TimeConvertError),
    }
}
