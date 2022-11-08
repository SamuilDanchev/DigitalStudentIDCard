use ldap3::SearchEntry;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Student {
    uid: i32,
    firstname: String,
    lastname: String,
    birthday: String,
    school_class: String,
    printed_in: String,
    valid_to: String,
}

impl Student {
    pub fn entry_to_student(entry: SearchEntry) -> Self {
        println!("{:?}", entry);
        Self {
            uid: entry.attrs.get("uidNumber").unwrap()[0].parse().unwrap(),
            firstname: entry.attrs.get("givenName").unwrap()[0].to_string(),
            lastname: entry.attrs.get("sn").unwrap()[0].to_string(),
            birthday: "".to_string(),
            school_class: entry.attrs.get("ou").unwrap()[0].to_string(),
            printed_in: "".to_string(),
            valid_to: "".to_string()
        }
    }
}
