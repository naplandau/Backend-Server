use diesel::prelude::*;
use uuid::Uuid;
use crate::config::config::CONFIG;
use crate::core::models::users_mongo;
use crate::core::models::api_response;
use crate::app::modules::cb_users::*;
use crate::core::db::db::PgPool;
use crate::core::models::schema::users::dsl::*;
use crate::core::models::users::*;
use crate::server::handlers::errors::*;

// pub fn find_all(pool: &PgPool) -> Result<UsersResponse, Error> {
//    let conn = pool.get()?;
//    let all_users = users.load(&conn)?;

//    Ok(all_users.into())
// }

// pub fn find(pool: &PgPool, user_id: String) -> Result<UserResponse, Error> {
//    let not_found = format!("not found");
//    let conn = pool.get()?;
//    let user = users
//       .filter(id.eq(user_id))
//       .first::<User>(&conn)
//       .map_err(|_| Error::NotFound(not_found))?;

//    Ok(user.into())
// }

// /// Find a user by the user's authentication information (email + password)
// /// Return an Unauthorized error if it doesn't match
// // pub fn find_by_auth(pool: &PgPool, user_email: &str, user_password: &str) -> Result<UserResponse, Error>
// //  {
// //     let conn = pool.get()?;
// //     let user = users
// //         .filter(email.eq(user_email.to_string()))
// //         .filter(password.eq(user_password.to_string()))
// //         .first::<User>(&conn)
// //         .map_err(|_| Error::Unauthorized("Invalid login".into()))?;
// //     Ok(user.into())
// //  }

// pub fn create(pool: &PgPool, user: &User) -> Result<UserResponse, Error> {
//    let conn = pool.get()?;
//    diesel::insert_into(users).values(user).execute(&conn)?;

//    Ok(UserResponse::from(user.clone()).into())
// }

// pub fn update(pool: &PgPool, update_user: &User) -> Result<UserResponse, Error> {
//    let conn = pool.get()?;
//    diesel::update(users)
//       .filter(id.eq(update_user.id.clone()))
//       .set(update_user)
//       .execute(&conn)?;

//    find(&pool, update_user.id.clone())
// }

// pub fn delete(pool: &PgPool, user_id: Uuid) -> Result<(), Error> {
//    let conn = pool.get()?;
//    diesel::delete(users)
//       .filter(id.eq(user_id.to_string()))
//       .execute(&conn)?;

//    Ok(())
// }