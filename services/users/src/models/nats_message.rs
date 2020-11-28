use bson::{Document, doc};
use nats::Message;
use serde::{Deserialize, Serialize};

use crate::errors::ServerError;
#[derive(Debug, Serialize, Deserialize)]
pub struct NatsRequest {
    pub request_type: String, /// Type of request to topic
    pub request_id: String, /// Id of request
    pub from: String, /// Name of sender
    pub data: Document, /// Data send
    pub status: bool,
    pub status_code: i64,
    pub status_des: String, // Resquest Describle
    pub send_time: u64
}
impl NatsRequest{
    pub fn errorParse()-> Self{
        Self{
            request_type: "".to_string(),
            request_id: "".to_string(),
            from: "".to_string(),
            data: doc! {},
            send_time: 0,
            status: false,
            status_code: -1,
            status_des:"Parse Request From Message Fail".to_string()
        }
    }
}
impl From<Message> for NatsRequest{
    fn from(msg: Message) -> Self{
        serde_json::from_slice(&msg.data).unwrap_or(NatsRequest::errorParse())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NatsResponse {
    pub nats_request: NatsRequest, // Original Request
    pub response_type: String, // Type of response
    pub response_id: String, // Id of response
    pub from: String, // Name of responser
    pub data: Document, // Data response
    pub status: bool, // Status of request
    pub send_time: u64,
    pub status_code: i64, 
    pub status_des: String // Response result Describle
}
impl NatsResponse{
    pub fn errorParse()-> Self{
        Self{
            nats_request: NatsRequest::errorParse(),
            response_type: "".to_string(),
            response_id: "".to_string(),
            from: "".to_string(),
            data: doc!{},
            status: false,
            send_time: 0,
            status_code: -1,
            status_des: "Parse Response From Message Fail".to_string()
        }
    }
}

impl From<Message> for NatsResponse{
    fn from(msg: Message) -> Self{
        serde_json::from_slice(&msg.data).unwrap_or(NatsResponse::errorParse())
    }
}