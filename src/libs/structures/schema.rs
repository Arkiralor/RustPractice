//! Custom `struct` definitions go here.

use chrono::{Date, DateTime, Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{to_string, to_string_pretty, Result, Value};
use std::env;
use uuid::Uuid;

use crate::libs::constants::{
    DEFAULT_DATE_OF_BIRTH_STR, DEFAULT_EMAIL, DEFAULT_GENDER, DEFAULT_USERNAME, GENDER_CHOICES,
};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    password: String,
    pub date_of_birth: NaiveDate,
    pub gender: String,
}

impl User {
    pub fn create(
        u: Option<&str>,
        e: Option<&str>,
        p: Option<&str>,
        d: Option<&str>,
        g: Option<&str>,
    ) -> Self {
        let id: Uuid = Uuid::new_v4();

        let username: String = match u {
            Some(val) => val.to_string(),
            None => DEFAULT_USERNAME.to_string(),
        };

        let email: String = match e {
            Some(val) => val.to_string(),
            None => DEFAULT_EMAIL.to_string(),
        };

        let password: String = match p {
            Some(val) => val.to_string(),
            None => {
                println!("No password provided, using default password.");
                env::var("DEFAULT_PASSWORD")
                    .expect("DEFAULT_PASSWORD not set.")
                    .to_string()
            }
        };

        let date_of_birth: NaiveDate = match d {
            Some(val) => NaiveDate::parse_from_str(val, "%Y-%m-%d").unwrap(),
            None => NaiveDate::parse_from_str(DEFAULT_DATE_OF_BIRTH_STR, "%Y-%m-%d").unwrap(),
        };

        let gender: String = match g {
            Some(val) => {
                if !GENDER_CHOICES.contains(&val) {
                    panic!(
                        "Invalid gender choice, valid options are: {:?}",
                        GENDER_CHOICES
                    );
                } else {
                    val.to_string()
                }
            }
            None => DEFAULT_GENDER.to_string(),
        };

        Self {
            id,
            username,
            email,
            password,
            date_of_birth,
            gender,
        }
    }

    pub fn display(&self) {
        //! Show the user's details.
        println!("{}", self);
    }

    pub fn serialize(&self) -> String {
        //! Convert the user to a JSON string.
        let serialized = match to_string_pretty(&self) {
            Ok(val) => val,
            Err(s) => panic!("Error serializing user: {}", s),
        };

        return serialized;
    }

    pub fn age(&self) -> i32 {
        //! Get the user's age.
        let today: Date<Utc> = Utc::today();
        let dob: Date<Utc> = Date::from_utc(self.date_of_birth, Utc);

        today.year() - dob.year()
    }
}
