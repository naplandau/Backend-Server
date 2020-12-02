use super::lib::*;
mod token;
mod authenticate;

pub async fn create_users(req: Register) -> Result<User, ServerError> {
    let now = Instant::now();
    match req.validate() {
        Ok(_) => {
            let email = req.email.to_owned().unwrap_or_default();
            let find_res = users_db::find_by_email(email.to_owned()).await.unwrap();
            match find_res {
                Some(_) => Err(ServerError::Conflict),
                None => {
                    let user_save: User = req.to_owned().into();
                    match users_db::insert(user_save.to_owned()).await {
                        Ok(_id) => {
                            println!(
                                "[MODULES] | [CREATE_USER] | [{:?}]",
                                now.elapsed().as_secs_f32()
                            );
                            Ok(user_save)
                        }
                        Err(e) => {
                            println!(
                                "[MODULES] | [CREATE_USER] | [{:?}]",
                                now.elapsed().as_secs_f32()
                            );
                            Err(ServerError::from(e))
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!(
                "[MODULES] | [CREATE_USER] | [{:?}]",
                now.elapsed().as_secs_f32()
            );
            Err(ServerError::from(e))
        }
    }
}
pub async fn get_users(_query: HashMap<String, String>) -> Result<Vec<User>, ServerError> {
    let now = Instant::now();
    let option = Some(
        FindOptions::builder()
            //.sort(doc! {"title":1})
            .build(),
    );
    let filter = Some(doc! {});
    match users_db::find_all(filter, option).await {
        Ok(vec) => {
            println!(
                "[MODULES] | [GET_USERS] | [{:?}]",
                now.elapsed().as_secs_f32()
            );
            Ok(vec)
        }
        Err(e) => {
            println!(
                "[MODULES] | [GET_USERS] | [{:?}]",
                now.elapsed().as_secs_f32()
            );
            Err(ServerError::from(e))
        }
    }
}
pub async fn get_user(id: String) -> Result<User, ServerError> {
    let now = Instant::now();
    match users_db::find_by_id(id.to_string()).await {
        Ok(ops) => match ops {
            Some(user) => {
                println!(
                    "[MODULES] | [GET_USER] | [{:?}]",
                    now.elapsed().as_secs_f32()
                );
                Ok(user)
            }
            None => {
                println!(
                    "[MODULES] | [GET_USER] | [{:?}]",
                    now.elapsed().as_secs_f32()
                );
                Err(ServerError::NoContent)
            }
        },
        Err(e) => {
            println!(
                "[MODULES] | [GET_USER] | [{:?}]",
                now.elapsed().as_secs_f32()
            );
            Err(ServerError::from(e))
        }
    }
}
/// Now return old document before update
pub async fn update_user(req: UpdateUser, id: String) -> Result<User, ServerError> {
    let now = Instant::now();
    match req.validate() {
        Ok(_) => {
            let find_res = users_db::find_by_id(id.to_string()).await.unwrap_or(None);
            match find_res {
                Some(user) => {
                    let update_doc = bson::to_document(&req.to_owned()).unwrap();
                    let _execs = users_db::update(user, update_doc).await;
                    match _execs {
                        Ok(user) => {
                            println!(
                                "[MODULES] | [UPDATE_USER] | [{:?}]",
                                now.elapsed().as_secs_f32()
                            );
                            Ok(user)
                        }
                        Err(e) => {
                            println!(
                                "[MODULES] | [UPDATE_USER] | [{:?}]",
                                now.elapsed().as_secs_f32()
                            );
                            Err(ServerError::from(e))
                        }
                    }
                }
                None => {
                    println!(
                        "[MODULES] | [UPDATE_USER] | [{:?}]",
                        now.elapsed().as_secs_f32()
                    );
                    Err(ServerError::NoContent)
                }
            }
        }
        Err(e) => {
            println!("Validate error: {:?}", e);
            println!(
                "[MODULES] | [UPDATE_USER] | [{:?}]",
                now.elapsed().as_secs_f32()
            );
            Err(ServerError::from(e))
        }
    }
}
pub async fn delete_user(id: String) -> Result<User, ServerError> {
    let now = Instant::now();
    match users_db::find_by_id(id.to_owned()).await.unwrap_or(None) {
        Some(_) => match users_db::delete_by_id(id.to_owned()).await {
            Ok(ops) => match ops {
                Some(user) => {
                    println!(
                        "[MODULES] | [DELETE_USER] | [{:?}]",
                        now.elapsed().as_secs_f32()
                    );
                    Ok(user)
                }
                None => {
                    println!(
                        "[MODULES] | [DELETE_USER] | [{:?}]",
                        now.elapsed().as_secs_f32()
                    );
                    Err(ServerError::NoContent)
                }
            },
            Err(_) => {
                println!(
                    "[MODULES] | [DELETE_USER] | [{:?}]",
                    now.elapsed().as_secs_f32()
                );
                Err(ServerError::InternalServerError)
            }
        },
        None => {
            println!(
                "[MODULES] | [DELETE_USER] | [{:?}]",
                now.elapsed().as_secs_f32()
            );
            Err(ServerError::NoContent)
        }
    }
}
pub async fn delete_users() -> Result<i64, ServerError> {
    let now = Instant::now();
    match users_db::delete_all().await {
        Ok(deleted) => {
            println!(
                "[MODULES] | [DELETE_USERS] | [{:?}]",
                now.elapsed().as_secs_f32()
            );
            Ok(deleted)
        }
        Err(_) => {
            println!(
                "[MODULES] | [DELETE_USERS] | [{:?}]",
                now.elapsed().as_secs_f32()
            );
            Err(ServerError::InternalServerError)
        }
    }
}

impl From<Register> for User {
    fn from(register: Register) -> Self {
        let current_time = Utc::now();
        User {
            id: String::from("user_") + &Uuid::new_v4().to_simple().to_string(),
            email: register.email.unwrap_or_default().to_owned(),
            password: register.password.unwrap_or_default().to_owned(),
            first_name: None,
            last_name: None,
            phone_number: None,
            role: "USER".to_owned(),
            created_by: "".to_owned(),
            created_time_dt: current_time.timestamp(),
            updated_by: "".to_owned(),
            updated_time_dt: current_time.timestamp(),
            status: 1,
        }
    }
}
