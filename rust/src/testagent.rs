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
use std::{io::Write/* , net::TcpStream*/};
//use async_std::sync::Mutex;
use tokio::runtime::Runtime;
use tokio::io::{AsyncReadExt,AsyncWriteExt};
use tokio::net::TcpStream;
use std::io::Read;
//use std::sync::Arc;
//use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use protobuf::{Message, MessageDyn};
use up_rust::{
    rpc::RpcMapper, transport::datamodel::UTransport, uprotocol::{
        umessage, UCode, UMessage, UStatus, UUri 
    }
};
use std::{
    collections::HashMap,
    sync::{atomic::AtomicU64, Arc, Mutex},
   // time::Duration,
};

use crate::uTransportSocket::UtrasnsportSocket;
use crate::utils::{base64_to_protobuf_bytes, protobuf_to_base64, send_socket_data,convert_json_to_jsonstring};

#[derive(Serialize)]
pub struct JsonData {
    action: String,
    message: String,
}
// Define a listener type alias


type Listener = Box<dyn Fn(Result<UMessage, UStatus>) + Send + Sync + 'static>;
//#[derive(Clone)]
pub struct SocketTestAgent {
    utransport: UtrasnsportSocket,
    clientsocket:Arc<Mutex<TcpStream>>,
    listner_map: Vec<String>,
}

impl Clone for SocketTestAgent {
    fn clone(&self) -> Self {
        SocketTestAgent {
            utransport: self.utransport.clone(), // Assuming UtrasnsportSocket implements Clone
            clientsocket: self.clientsocket.clone(),
            listner_map: self.listner_map.clone(), // Clone Vec<String>
        }
    }
    
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

impl SocketTestAgent  {
   pub async fn new(test_clientsocket: TcpStream, utransport: UtrasnsportSocket) -> Self {
       
    let socket = Arc::new(Mutex::new(test_clientsocket));
        let clientsocket= socket;
          
        SocketTestAgent {
            utransport,
            clientsocket,
            listner_map: Vec::new(),
          
        }
    }

async fn on_receive(self,result:Result<UMessage, UStatus>) {
    println!("Listener onreceived");
    let mut json_message = JsonData {
        action: "onReceive".to_owned(),
        message: "None".to_string(),
    };
    match result {
        Ok(message) => json_message.message = protobuf_to_base64(&message),
        Err(status)=> println!("Received error status: {}", status),
     }
     self.send_to_tm(json_message);
 
}
    

    pub async  fn receive_from_tm (&mut self){
         // Clone Arc to capture it in the closure
        
        let arc_self = Arc::new(self.clone());  
        <SocketTestAgent as Clone>::clone(&self).inform_tm_ta_starting().await;
        let mut socket = self
        .clientsocket
        .lock()
        .expect("error accessing TM server");
    
      
        loop {
            let mut recv_data = [0; 1024];
            
            let bytes_received = match socket.read(&mut recv_data).await{
            Ok(bytes_received) => bytes_received,
            Err(e) => {
                // Handle socket errors (e.g., connection closed)
                eprintln!("Socket error: {}", e);
                break;
            }};
              // Check if no data is received
            if bytes_received == 0 {
                continue;
            }
        

          /*   match recv_result {
                Ok(bytes_received) => {
                    if bytes_received == 0 {
                        println!("Closing TA Client Socket");
                        break;
                    }*/
                    let recv_data_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&recv_data[..bytes_received]);
                    let json_msg: HashMap<String, String> = serde_json::from_str(&recv_data_str).unwrap(); // Assuming serde_json is used for JSON serialization/deserialization
                    let action = json_msg["action"].clone();
                    let umsg_base64 = json_msg["message"].clone();
                    let protobuf_serialized_data = base64_to_protobuf_bytes(&umsg_base64).expect("received data from TM is corrupt"); // Implement this function according to your logic

                    
                let mut umsg = UMessage::new(); // Assuming UMessage is a protobuf-generated message type

                    // Assuming umsg_serialized is the byte array obtained from SerializeToString()
                    if let Err(err) = umsg.merge_from_bytes(&protobuf_serialized_data) {
                       eprintln!("Error deserializing UMessage: {}", err);
                    } else {
                       eprint!("data seems to be correct!");                       
                    } 

   
                    let status = match action.as_str() {
                        "SEND_COMMAND" => {
                            match self.utransport.send(umsg).await{
                                Ok(_) =>{println!("message sent successfully");()}
                                Err(_status)=>{println!("failed to send message");()}
                            }
                        },
                        
                        "REGISTER_LISTENER_COMMAND" => {
                            let cloned_listener = Arc::clone(&arc_self);
                            let cloned_listener_data: Listener = Box::new(move |result: Result<UMessage, UStatus>| { <SocketTestAgent as Clone>::clone(&cloned_listener).on_receive(result); });
                            let _ = self.utransport.register_listener(umsg.attributes.source.clone().unwrap(),cloned_listener_data);
                            ()
                        }, // Assuming listener can be cloned
                       
                       "UNREGISTER_LISTENER_COMMAND" => {
                        let _ = self.utransport.unregister_listener(umsg.attributes.source.clone().unwrap(),&self.listner_map[0]);
                        ()
                        }, // Assuming listener can be cloned

                        _ => {
                            ||UStatus { code: UCode::OK.into(), message: Some("Unknown action".to_string()), details: todo!(), special_fields: todo!() };
                        }, // Modify with appropriate handling
                    };


                  let _status_clone= status.clone();  
                 let base64_str  = serde_json::to_string(&status).unwrap();
                    let _json_message = JsonData{
                        action:"uStatus".to_owned(),
                        message: base64_str, 
                             };
               // Err(e) => {
                 //   eprintln!("Error receiving data: {}", e);
                 //   break;
               // }
            }
        
    }
    
  
  async fn inform_tm_ta_starting(self) {


 #[derive(Serialize)]
    struct JsonSdkname {
        sdk_name: String,
    }
    let json_sdk_name = JsonSdkname {
        sdk_name: String::from("Rust"),
    };   
        //infor TM that rust TA is running
        println!("Sending SDK name to Test Manager Directly!");
        let json_message_str = convert_json_to_jsonstring(&json_sdk_name);
        let message = json_message_str.as_bytes();

        let socket_clone = self.clientsocket.clone();
        //socket_clone.write_all(message);   
        let _ = socket_clone.lock().expect("error in sending data to TM").write_all(message);         
  
    }

    async fn send_to_tm(self, json_message:JsonData) {
   
        let json_message_str = convert_json_to_jsonstring(&json_message);
        let message = json_message_str.as_bytes();
        let socket_clone = self.clientsocket.clone();
        let _ = socket_clone.lock().expect("error in sending data to TM").write_all(message);         
  
    }
   fn close_connection(&self) {
        let _ = self.clientsocket.lock().expect("error in sending data to TM").shutdown();
    }
}
