//! Custom `struct` definitions go here.

use chrono::{Date, DateTime, Datelike, Duration, NaiveDate, Utc};
use pwhash::{bcrypt, unix};
use serde::{Deserialize, Serialize};
use serde_json::{to_string, to_string_pretty, Result, Value};
use std::env;
use uuid::Uuid;

use crate::libs::constants::{
    DEFAULT_DATE_OF_BIRTH_STR, DEFAULT_EMAIL, DEFAULT_GENDER, DEFAULT_USERNAME, GENDER_CHOICES,
    SEC_IN_YEAR,
};

/// User struct; used to hold information about dummy users.
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    // prithoo: We ABSOLUTELY do NOT want to SErialize the password of a user.
    password: String,
    pub date_of_birth: NaiveDate,
    pub gender: String,
}

impl User {
    pub fn hash_password(mut password: String) -> String {
        //! Hash the user's password.
        let key = match env::var("SECRET_KEY") {
            Ok(val) => val,
            Err(_) => panic!("SECRET_KEY not set."),
        };

        let hashed_password = match bcrypt::hash(password) {
            Ok(val) => val,
            Err(s) => panic!("Error hashing password: {}", s),
        };

        return hashed_password;
    }

    pub fn verify_password(&self, password: &str) -> bool {
        //! Verify a given password against a user's stored password.
        let key = match env::var("SECRET_KEY") {
            Ok(val) => val,
            Err(_) => panic!("SECRET_KEY not set."),
        };

        return bcrypt::verify(password, self.password());
    }

    pub fn password(&self) -> &str {
        //! Get a user's hashed password as it's otherwise a private value.
        &self.password
    }

    pub fn create(
        u: Option<&str>,
        e: Option<&str>,
        p: Option<&str>,
        d: Option<&str>,
        g: Option<&str>,
    ) -> Self {
        //! Create a new instance of a user.
        //!
        //! Auto-hashes the given password or if it is not provided, sets the default password and salts/hashes it.
        //!
        //! args:
        //!     - u: username
        //!     - e: email
        //!     - p: password
        //!     - d: date_of_birth
        //!     - g: gender
        let id: Uuid = Uuid::new_v4();

        let username: String = match u {
            Some(val) => val.to_string(),
            None => panic!("`username` is a mandatory field."),
        };

        let email: String = match e {
            Some(val) => val.to_string(),
            None => panic!("`email` is a mandatory field."),
        };

        let password: String = match p {
            Some(val) => User::hash_password(val.to_string()),
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
        //! Show the user's details as a string.
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

    pub fn age(&self) -> u64 {
        //! Get the user's age in years.
        let today: Date<Utc> = Utc::today();
        let dob: Date<Utc> = Date::from_utc(self.date_of_birth, Utc);

        let actual_age = match (today - dob).to_std() {
            Ok(val) => val.as_secs_f64() / SEC_IN_YEAR,
            Err(s) => panic!("{}", s),
        };

        return actual_age as u64;
    }
}
