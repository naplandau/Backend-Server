use bson::{doc};
use nats::Message;
use serde::{Deserialize, Serialize};
use serde_json::{Value as Json};



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NatsRequest {
    pub request_type: String, /// Type of request to topic
    pub request_id: String, /// Id of request
    pub from: String, /// Name of sender
    pub data: Json, /// Data send
    pub status: bool,
    pub status_code: i64,
    pub status_des: String, // Resquest Describle
    pub send_time: i64
}
impl NatsRequest{
    pub fn error_parse()-> Self{
        Self{
            request_type: "".to_string(),
            request_id: "".to_string(),
            from: "".to_string(),
            data: json!({}),
            send_time: 0,
            status: false,
            status_code: -999,
            status_des:"Parse Request From Message Fail".to_string()
        }
    }
}
impl From<Message> for NatsRequest{
    fn from(msg: Message) -> Self{
        serde_json::from_slice(&msg.data).unwrap()//.unwrap_or(NatsRequest::errorParse())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NatsResponse {
    pub nats_request: NatsRequest, // Original Request
    pub response_type: String, // Type of response
    pub response_id: String, // Id of response
    pub from: String, // Name of responser
    pub data: Json, // Data response
    pub status: bool, // Status of request
    pub send_time: i64,
    pub status_code: i64, 
    pub status_des: String // Response result Describle
}
impl NatsResponse{
    pub fn error_parse()-> Self{
        Self{
            nats_request: NatsRequest::error_parse(),
            response_type: "".to_string(),
            response_id: "".to_string(),
            from: "".to_string(),
            data: json!({}),
            status: false,
            send_time: 0,
            status_code: -999,
            status_des: "Parse Response From Message Fail".to_string()
        }
    }
}

impl From<Message> for NatsResponse{
    fn from(msg: Message) -> Self{
        serde_json::from_slice(&msg.data).unwrap()//unwrap_or(NatsResponse::errorParse())
    }
}