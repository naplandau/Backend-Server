use crate::app::modules::*;
use crate::models::nats_message::*;
use crate::models::*;
use crate::nats_broker::*;

use chrono::Utc;
use std::collections::HashMap;
pub async fn nats_server(nats_conn: NatsConnection) {
    create_users_topic("user.create".to_owned(), nats_conn).await;
}

async fn create_users_topic(topic: String, nats_conn: NatsConnection) {
    match NatsServer::create_response_subcriber(nats_conn, topic.to_owned(), "".to_string()).await {
        Ok(sub) => {
            sub.with_handler(move |msg| {
                let nats_req = NatsRequest::from(msg.clone());
                let res = futures::executor::block_on(create_users(nats_req.to_owned().into()));
                let nats_res = match res {
                    Ok(user) => {
                        resp_nats(
                        nats_req,
                        "resp_create_user".to_owned(),
                        serde_json::to_value(&user).unwrap(),
                        true,
                        0,
                        "Ok".to_owned(),
                    )
                },
                    Err(e) => resp_nats(
                        nats_req,
                        "create_user".to_owned(),
                        json!({}),
                        false,
                        -1,
                        e.to_string(),
                    ),
                };
                msg.respond(serde_json::to_string(&nats_res).unwrap())
            });
        }
        Err(e) => {
            println!(
                "[NATS][FAIL] Create subcriber for topic:`{}` fail | {}",
                topic, e
            );
        }
    }
}
async fn get_users_topic(topic: String, nats_conn: NatsConnection) {
    match NatsServer::create_response_subcriber(nats_conn, topic.to_owned(), "".to_string()).await {
        Ok(sub) => {
            sub.with_handler(move |msg| {
                let nats_res = NatsRequest::from(msg.clone());
                let res = futures::executor::block_on(get_users(nats_res.into()));
                // let res_data = serde_json::to_string(&res.unwrap()).unwrap();
                msg.respond(serde_json::to_string(&res.unwrap()).unwrap())
            });
        }
        Err(e) => {
            println!(
                "[NATS][FAIL] Create subcriber for topic:`{}` fail | {}",
                topic, e
            );
        }
    }
}

impl From<NatsRequest> for Register {
    fn from(nas_req: NatsRequest) -> Self {
        let doc = nas_req.data;
        let email = doc["email"].as_str().unwrap_or("").to_owned();
        let password = doc["password"].as_str().unwrap_or("").to_owned();
        Self {
            email: if email == "" {
                None
            } else {
                Some(email.to_string())
            },
            password: if password == "" {
                None
            } else {
                Some(password.to_string())
            },
        }
    }
}
impl From<NatsRequest> for HashMap<String, String> {
    fn from(_nats_req: NatsRequest) -> Self {
        HashMap::new()
    }
}

fn resp_nats(
    nats_req: NatsRequest,
    resp_type: String,
    data: serde_json::Value,
    status: bool,
    status_code: i64,
    status_des: String,
) -> NatsResponse {
    let now = Utc::now().timestamp();
    NatsResponse {
        nats_request: nats_req,
        response_type: resp_type.to_owned(),
        response_id: resp_type.to_owned() + &now.to_string(),
        from: "User Service".to_owned(),
        data: data,
        status: status,
        send_time: now,
        status_code: status_code,
        status_des: status_des,
    }
}
