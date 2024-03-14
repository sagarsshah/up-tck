mod constants;
mod utils;
use crate::constants::*; 
use testagent::SocketTestAgent;
use uTransportSocket::UtrasnsportSocket;
//use std::io::{Read, Write};
//use std::thread;
//use up_rust::{UMessage, UStatus};

//use up_rust::UMessageType;

//use up_rust::UUri;
mod testagent;
mod uTransportSocket;
//use uprotocol::proto::umessage_pb2::{UMessage};
//use uprotocol::proto::ustatus_pb2::{UStatus, UCode};
//use uprotocol::transport::ulistener::UListener;
//use test_agent::transport_layer::TransportLayer;
//use test_agent::testagent::SocketTestAgent;

//use std::io::*;

use std::io::{self, Write};
use std::net::{TcpStream, Shutdown};
//use std::net::TcpStream;
use crate::utils::{convert_json_to_jsonstring, convert_str_to_bytes, send_socket_data};
use serde_json::Value;
use serde::{Serialize, Deserialize};








   

fn main() {
    struct JsonSdkname {
        sdk_name: String,
        
    }
    let transport:UtrasnsportSocket;
    let test_agent_socket = TcpStream::connect(TEST_MANAGER_ADDR).expect("Failed to connect to Test Manager");
    let mut agent = SocketTestAgent::new(test_agent_socket,transport);
    agent.TM_receive_thread();
    let json_sdk_name = JsonSdkname{sdk_name: String::from("Python"),};
    //let serde_value:Value = serde_json::to_value(&json_sdk_name).unwrap();

    let json_message_str = convert_json_to_jsonstring(&json_sdk_name);
    let message = convert_str_to_bytes(&json_message_str);
    println!("Sending SDK name to Test Manager Directly!");
        if let Err(err) = send_socket_data(test_agent_socket, &message) {
            eprintln!("Error sending message: {}", err);
        }

}
