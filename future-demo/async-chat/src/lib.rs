use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod utils;

// 从客户端发送到服务端的数据包
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum FromClient {
    Join { group_name: Arc<String> },
    Post {
        group_name: Arc<String>,
        message: Arc<String>,
    }
}

// 从服务端发送到客户端的数据包
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum FromServer {
    Message {
        group_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),
}

#[test]
fn test_fromclient_json() {
    use std::sync::Arc;

    let from_client = FromClient::Post {
        group_name: Arc::new("Dogs".to_string()), 
        message: Arc::new("Samoyeds rock!".to_string()), 
    };
    let json = serde_json::to_string(&from_client).unwrap();
    assert_eq!(json, r#"{"Post":{"group_name":"Dogs","message":"Samoyeds rock!"}}"#);
    assert_eq!(serde_json::from_str::<FromClient>(&json).unwrap(), from_client);
}
