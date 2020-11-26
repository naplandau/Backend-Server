
use crate::core::rabbit_queue::*;
use futures::StreamExt;
use lapin::options::*;

pub async fn rabbit_server(rabbit_fac1: RabbitPool) {
    let rabbit = rabbit_fac1.to_owned();
    let conn = rabbit.get().await.unwrap();
    actix_rt::spawn(async move { queue_consume(conn).await });
    let conn1 = rabbit.get().await.unwrap();
    actix_rt::spawn(async move {
        queue_consume1(conn1).await;
    });
}

async fn queue_consume(conn: RabbitConnection) {
    let mut consumer = RabbitServer::create_consumer(&conn, "ha_qu_test", "test")
        .await
        .unwrap();
    while let Some(delivery) = consumer.next().await {
        let (channel, delivery) = delivery.expect("error in consumer");
        println!("[{}] consume messsage: {:?}", "ha_qu_test", delivery.data);
        channel
            .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
            .await
            .expect("ack");
    }
}
async fn queue_consume1(conn: RabbitConnection) {
    let mut consumer = RabbitServer::create_consumer(&conn, "ha_qu_test1", "test")
        .await
        .unwrap();
    while let Some(delivery) = consumer.next().await {
        let (channel, delivery) = delivery.expect("error in consumer");
        println!("[{}] consume messsage: {:?}", "ha_qu_test_1", delivery.data);
        channel
            .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
            .await
            .expect("ack");
    }
}


