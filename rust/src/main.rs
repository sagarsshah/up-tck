/*
 * Copyright (c) 2024 General Motors GTO LLC
 *
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 * SPDX-FileType: SOURCE
 * SPDX-FileCopyrightText: 2023 General Motors GTO LLC
 * SPDX-License-Identifier: Apache-2.0
 */

mod constants;
mod utils;
mod uTransportSocket;
use crate::constants::*;
use crate::uTransportSocket::UtransportExt;
//use protobuf::Message;
use prost::Message;
use testagent::SocketTestAgent;
use uTransportSocket::UtrasnsportSocket;
mod testagent;
use tokio::runtime::Runtime;
use tokio::net::TcpStream;
use up_rust::UMessage;
use prost::bytes::Bytes;
use serde_json::{map, Value};


fn main() {
   
    let json_str = r#"
    {
        "attributes": {
            "priority": "UPRIORITY_CS1",
            "type": "UMESSAGE_TYPE_PUBLISH",
            "id": "12345"
        },
        "payload": {
            "format": "UPAYLOAD_FORMAT_PROTOBUF_WRAPPED_IN_ANY"
        }
    }
"#;
let result = json_to_protobuf(json_str)?;
  // Check if deserialization was successful
  match result {
    Ok(protobuf_message) => {
        println!("Deserialized protobuf message: {:?}", protobuf_message);
        // Add assertions or further processing here
    }
    Err(err) => {
        eprintln!("Error: {}", err);
        // Handle error here
    }
}

    let transport = UtrasnsportSocket::new();
    let rt = Runtime::new().unwrap();
    rt.block_on(async { 
        let test_agent_socket = TcpStream::connect(TEST_MANAGER_ADDR).await.unwrap();
        
        let mut transport_socket =transport.await;
        transport_socket.socket_init().await;        
        let agent = SocketTestAgent::new(test_agent_socket, transport_socket);
                 
    

        agent.await.receive_from_tm().await;
    
      
          
    });


}


use prost::Message as _;

//use serde_json::{json, Value};
use std::error::Error;
struct MyMessage{

    test: dyn Message,
}
pub fn json_to_protobuf(json_str: &str) -> Result<Box<dyn prost::Message>, Box<dyn std::error::Error>> {
//pub fn json_to_protobuf(json_str: &str) -> Result<MyMessage, Box<dyn std::error::Error>> {
//pub fn json_to_protobuf(json_str: &str) -> Result< MyMessage, Box<dyn Error>> {
    // Parse JSON string into a Value
    let json_obj: Value = serde_json::from_str(json_str)?;

    // Serialize Value back into bytes
    let bytes = serde_json::to_vec(&json_obj)?;

    // Wrap bytes in a Bytes object
    let bytes = Bytes::from(bytes);

    // Decode bytes into the protobuf message
    let protobuf_message = MyMessage::decode(&bytes)?;

    Ok(protobuf_message)
}

 /* 
pub fn json_to_protobuf<T>( json_str: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: prost::Message + Default,
{
      // Parse JSON string into a Value
      let json_obj: Value = serde_json::from_str(json_str)?;

      // Serialize Value back into bytes
      let bytes = serde_json::to_vec(&json_obj)?;
  
  // Wrap bytes in a Bytes object
  let bytes = Bytes::from(bytes);

      // Decode bytes into the protobuf message
      let protobuf_message = T::decode(bytes)?;
  
      Ok(protobuf_message)
}*/