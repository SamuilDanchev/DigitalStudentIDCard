use ldap3::SearchEntry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Student {
    pub uid: i32,
    pub firstname: String,
    pub lastname: String,
    pub birthday: String,
    pub school_class: String,
    pub printed_in: String,
    pub valid_to: String,
    pub image: String,
}

impl Student {
    pub fn entry_to_student(entry: SearchEntry) -> Self {
        Self {
            uid: entry.attrs.get("uidNumber").unwrap()[0].parse().unwrap(),
            firstname: entry.attrs.get("givenName").unwrap()[0].to_string(),
            lastname: entry.attrs.get("sn").unwrap()[0].to_string(),
            birthday: entry.attrs.get("audio").unwrap()[0].to_string(),
            school_class: entry.attrs.get("ou").unwrap()[0].to_string(),
            printed_in: entry.attrs.get("mobile").unwrap()[0].to_string(),
            valid_to: entry.attrs.get("l").unwrap()[0].to_string(),
            image: entry.attrs.get("gecos").unwrap()[0].to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub student: Student
}

impl LoginResponse {
    pub fn create_login_response(token: String, student: Student) -> Self {
        Self {
            token,
            student
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyResponse {
    pub firstname: String,
    pub lastname: String
}
