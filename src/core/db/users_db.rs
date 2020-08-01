use actix::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};

use crate::core::db::db::{PgPool};
use crate::app::modules::cb_users::*;
use crate::core::models::users::*;
use crate::server::handlers::errors::*;
use crate::server::handlers::{HASHER, PWD_SCHEME_VERSION};


/// Get all users
pub fn get_all(pool: &PgPool) -> Result<UsersResponse, Error> {
    use super::super::models::schema::users::dsl::users;

    let conn = pool.get()?;
    let all_users = users.load(&conn)?;

    Ok(all_users.into())
}

/// Find a user by the user's id or error out
pub fn find_by_id(pool: &PgPool, user_id: Uuid) -> Result<UserResponse, Error> {
    use super::super::models::schema::users::dsl::{id, users};

    let not_found = format!("User {} not found", user_id);
    let conn = pool.get()?;
    let user = users
        .filter(id.eq(user_id))
        .first::<User>(&conn)
        .map_err(|_| Error::NotFound(not_found))?;

    Ok(user.into())
}

/// Find a user by the user's authentication information (email + password)
/// Return an Unauthorized error if it doesn't match
pub fn find_by_auth(
    pool: &PgPool,
    user_email: &str,
    user_password: &str,
) -> Result<UserResponse, Error> {
    use super::super::models::schema::users::dsl::{email, password, users};

    let conn = pool.get()?;
    let user = users
        .filter(email.eq(user_email.to_string()))
        .filter(password.eq(user_password.to_string()))
        .first::<User>(&conn)
        .map_err(|_| Error::Unauthorized("Invalid login".into()))?;
    Ok(user.into())
}

/// Create a new user
pub fn create(pool: &PgPool, new_user: &NewUser) -> Result<UserResponse, Error> {
    use super::super::models::schema::users::dsl::users;

    let conn = pool.get()?;
    diesel::insert_into(users).values(new_user).execute(&conn)?;
    Ok(new_user.clone().into())
}

/// Update a user
pub fn update(pool: &PgPool, update_user: &UpdateUser) -> Result<UserResponse, Error> {
    use super::super::models::schema::users::dsl::{id, users};

    let conn = pool.get()?;
    diesel::update(users)
        .filter(id.eq(update_user.id.clone()))
        .set(update_user)
        .execute(&conn)?;
    find(&pool, Uuid::parse_str(&update_user.id)?)
}

/// Delete a user
pub fn delete(pool: &PgPool, user_id: Uuid) -> Result<(), Error> {
    use super::super::models::schema::users::dsl::{id, users};

    let conn = pool.get()?;
    diesel::delete(users)
        .filter(id.eq(user_id.to_string()))
        .execute(&conn)?;
    Ok(())
}

impl From<NewUser> for User {
    fn from(user: NewUser) -> Self {
        User {
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            phone_number: user.phone_number,
            dob: user.dob,
            avatar: user.avatar,
            time_zone: user.time_zone,
            status: user.status,
            confirm_code: user.confirm_code,
            confirm_code_created_time_dt: user.confirm_code_created_time_dt,
            role: user.role,
            roles: user.roles,
            password: HASHER.hash(&user.password).unwrap() ,
            created_by: user.created_by,
            created_time_dt: Utc::now().naive_utc(),
            updated_by: user.updated_by,
            updated_time_dt: Utc::now().naive_utc(),
        }
    }
}