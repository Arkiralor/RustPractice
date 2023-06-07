//! Implementations of existing traits for various models and schema.
//!

use crate::libs::structures::schema::User;
use std::fmt;

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ \"id\": \"{id}\", \"username\": \"{username}\", \"email\": \"{email}\", \"dateOfBirth\": \"{date_of_birth}\" , \"gender\": \"{gender}\" }}" , id=self.id, username=self.username, email=self.email, date_of_birth=self.date_of_birth, gender=self.gender)
    }
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ \"id\": \"{id}\", \"username\": \"{username}\", \"email\": \"{email}\", \"dateOfBirth\": \"{date_of_birth}\" , \"gender\": \"{gender}\" }}" , id=self.id, username=self.username, email=self.email, date_of_birth=self.date_of_birth, gender=self.gender)
    }
}
