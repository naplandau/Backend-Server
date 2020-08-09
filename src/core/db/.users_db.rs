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

// pub fn update(pool: &PgPool, update_user: &User) -> Result<UserResponse, Error> {
//    let conn = pool.get()?;
//    diesel::update(users)
//       .filter(id.eq(update_user.id.clone()))
//       .set(update_user)
//       .execute(&conn)?;

//    find(&pool, update_user.id.clone())
// }