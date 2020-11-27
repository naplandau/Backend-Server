use crate::nats_broker::*;
pub async fn nats_server(nat_fac: NatsConnection) {
    let sub = NatsServer::create_response_subcriber(nat_fac, "my.subject".to_string())
        .await
        .expect("Create Subcriber fail");
    sub.with_handler(move |msg| {
        println!("Received {}", &msg);
        msg.respond("Responsed Success")
    });
}
async fn nats_topic_1(topic: String, nats_conn: NatsConnection) {
    let sub = NatsServer::create_response_subcriber(nats_conn, topic.to_owned())
        .await
        .expect(
            ("Create subcriber for ".to_string() + topic.to_owned().as_str() + " fail").as_str(),
        );
    sub.with_handler(move |msg| {
        println!("Received {}", &msg);
        msg.respond("Responsed Success")
    });
}
async fn create_users_topic(topic: String, nats_conn: NatsConnection) {
    match NatsServer::create_response_subcriber(nats_conn, topic.to_owned()).await {
        Ok(sub) => {
            sub.with_handler(move |msg| {
                println!("Received {}", &msg);
                msg.respond("Responsed Success")
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
