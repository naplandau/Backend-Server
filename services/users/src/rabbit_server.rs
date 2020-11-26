use crate::core::rabbit_queue::*;
use lapin::message::DeliveryResult;
use lapin::options::*;
use std::str;
pub async fn rabbit_server(rabbit_fac1: RabbitPool) {
    let rabbit = rabbit_fac1.to_owned();
    let conn = rabbit.get().await.unwrap();
    let consumer = RabbitServer::create_consumer(&conn, "ha_qu_test", "my_consumer")
        .await
        .unwrap();

    consumer.set_delegate(move |delivery: DeliveryResult| async move {
        match delivery {
            Ok(delivery_ops) => {
                if let Some((_channel, delivery)) = delivery_ops {
                    delivery.ack(BasicAckOptions::default()).await.expect("");
                    let data = match str::from_utf8(&delivery.data) {
                        Ok(v) => v,
                        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                    };

                    println!(
                        "Acknowledge message {} : {:?}",
                        delivery.delivery_tag, data
                    );
                }
            }
            Err(e) => println!("Consumer fail to get msg: {:?}", e),
        }
    });
}

async fn queue_consume(conn: RabbitConnection) {
    let consumer = RabbitServer::create_consumer(&conn, "ha_qu_test", "test")
        .await
        .unwrap();
    consumer.set_delegate(move |delivery: DeliveryResult| async move {
        let delivery = delivery.expect("error caught in in consumer");
        if let Some((_channel, deliveries)) = delivery {
            deliveries.ack(BasicAckOptions::default()).await.expect("");
            println!(
                "Acknowledge message {} : {:?}",
                deliveries.delivery_tag, deliveries.data
            );
        }
    });
}
async fn queue_consume1(conn: RabbitConnection) {
    let consumer = RabbitServer::create_consumer(&conn, "ha_qu_test1", "test")
        .await
        .unwrap();
    consumer.set_delegate(move |delivery: DeliveryResult| async move {
        let delivery = delivery.expect("error caught in in consumer");
        if let Some((_channel, deliveries)) = delivery {
            deliveries.ack(BasicAckOptions::default()).await.expect("");
            println!(
                "Acknowledge message {} : {:?}",
                deliveries.delivery_tag, deliveries.data
            );
        }
    });
}
