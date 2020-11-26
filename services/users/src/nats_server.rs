use crate::core::nats_broker::*;
pub async fn nats_server(nat_fac: NatsConnection) {
    let sub = NatsServer::create_response_subcriber(nat_fac, "my.subject".to_string())
        .await
        .expect("Create Subcriber fail");
    sub.with_handler(move |msg| {
        println!("Received {}", &msg);
        msg.respond("Responsed Success")
    });
}