pub use crate::app::lib::*;
pub use crate::core::models::users::{Claims, Login, Register, UpdateUser, Confirmation, User, ADMIN_DOC};
pub use crate::utils::handlers::hasher::{hash_validation, HASHER};
pub use crate::utils::handlers::jwt::{generate_jwt, validate_jwt};
pub fn get_sub_field(doc: &Document) -> Document {
    let mut new_doc = doc.clone();
    let keys = vec![
        "password",
        "created_by",
        "created_time_dt",
        "updated_by",
        "updated_time_dt",
    ];
    for key in keys.iter() {
        new_doc.remove(key);
    }
    new_doc
}
