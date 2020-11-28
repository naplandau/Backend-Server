use crate::app::modules::*;
use crate::models::nats_message::*;
use crate::models::*;
use crate::nats_broker::*;
pub async fn nats_server(nats_conn: NatsConnection) {
    create_users_topic("my.subject".to_string(), nats_conn).await;
}

async fn create_users_topic(topic: String, nats_conn: NatsConnection) {
    match NatsServer::create_response_subcriber(nats_conn, topic.to_owned(), "".to_string()).await {
        Ok(sub) => {
            sub.with_handler(move |msg| {
                let nats_res = NatsRequest::from(msg.clone());
                let res = futures::executor::block_on(create_users(nats_res.into()));
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
async fn get_users_topic(topic: String, nats_conn: NatsConnection) {
    match NatsServer::create_response_subcriber(nats_conn, topic.to_owned(), "".to_string()).await {
        Ok(sub) => {
            sub.with_handler(move |msg| {
                let nats_res = NatsRequest::from(msg.clone());
                let res = futures::executor::block_on(create_users(nats_res.into()));
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
        let email = doc.get_str("email").unwrap_or("");
        let password = doc.get_str("password").unwrap_or("");
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
