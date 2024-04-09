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

use async_trait::async_trait;
use log::kv::ToValue;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use up_rust::UListener;
use up_rust::{ UMessage,  UStatus, UTransport};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde::Serialize;

use crate::uTransportSocket::UtrasnsportSocket;
use crate::utils::{
     convert_json_to_jsonstring,  WrapperUMessage,
    WrapperUUri,
};


#[derive(Serialize)]
pub struct JsonResponseData {
    message: String,
    action: String,
    ue: String,
}
// Define a listener type alias

pub struct SocketTestAgent {
    utransport: UtrasnsportSocket,
    clientsocket: Arc<Mutex<TcpStream>>,
    listner_map: Vec<String>,
   
}
#[async_trait]
impl UListener for SocketTestAgent {
    async fn on_receive(&self, msg: UMessage){
        println!("Listener onreceived");
        let mut json_message = JsonResponseData {
            action: "onReceive".to_owned(),
            message: "None".to_string(),
            ue: "rust".to_string(),
        };
        json_message.message = msg.clone().to_string().to_value().to_string();
        
        <SocketTestAgent as Clone>::clone(&self).send_to_tm(json_message).await;
    }

    async fn on_error(&self, _err: UStatus){todo!();}
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

impl SocketTestAgent {
    pub async fn new(test_clientsocket: TcpStream, utransport: UtrasnsportSocket) -> Self {
        let socket = Arc::new(Mutex::new(test_clientsocket));
        let clientsocket = socket;
        SocketTestAgent {
            utransport,
            clientsocket,
            listner_map: Vec::new(),
         
        }
    }

    pub async fn receive_from_tm(&mut self) {
        // Clone Arc to capture it in the closure

        let arc_self = Arc::new(self.clone());
        <SocketTestAgent as Clone>::clone(&self)
            .inform_tm_ta_starting()
            .await;
        let mut socket = self.clientsocket.lock().expect("error accessing TM server");

        loop {
            let mut recv_data = [0; 1024];

            let bytes_received = match socket.read(&mut recv_data).await {
                Ok(bytes_received) => bytes_received,
                Err(e) => {
                    // Handle socket errors (e.g., connection closed)
                    eprintln!("Socket error: {}", e);
                    break;
                }
            };
            // Check if no data is received
            if bytes_received == 0 {
                continue;
            }

            let recv_data_str: std::borrow::Cow<'_, str> =
                String::from_utf8_lossy(&recv_data[..bytes_received]);
            let json_msg: HashMap<String, String> = serde_json::from_str(&recv_data_str).unwrap(); // Assuming serde_json is used for JSON serialization/deserialization
            let action = json_msg["action"].clone();
            let json_data_string = json_msg["data"].clone();
            let json_data_value = serde_json::from_str(&json_data_string).unwrap();
            println!("json data received: {:?}", json_data_value);

            let status = match action.as_str() {
                "SEND_COMMAND" => {
                    let wu_message: WrapperUMessage =
                        serde_json::from_value(json_data_value).unwrap(); // convert json to UMessage
                    println!("\n\n Send UMessage received from TM: {:?} \n", wu_message);
                    let u_message = wu_message.0;

                    self.utransport.send(u_message).await
                }

                "REGISTER_LISTENER_COMMAND" => {
                    let cloned_listener = Arc::clone(&arc_self);
             
                    let wu_uuri: WrapperUUri = serde_json::from_value(json_data_value).unwrap(); // convert json to UMessage
                    println!("\n\n Send UUri received from TM: {:?} \n", wu_uuri);
                    let u_uuri = wu_uuri.0;
                    self.utransport
                        .register_listener(
                            u_uuri,
                            Arc::clone(&cloned_listener) as Arc<dyn UListener> /*&cloned_listener*/,
                        )
                        .await
                } // Assuming listener can be cloned

                "UNREGISTER_LISTENER_COMMAND" => {
                    let cloned_listener = Arc::clone(&arc_self);
                    let wu_uuri: WrapperUUri = serde_json::from_value(json_data_value).unwrap(); // convert json to UMessage
                    println!("\n\n Send UUri received from TM: {:?} \n", wu_uuri);
                    let u_uuri = wu_uuri.0;
                    self.utransport
                        .unregister_listener(u_uuri,Arc::clone(&cloned_listener) as Arc<dyn UListener> /*&cloned_listener*/)
                        .await
                } // Assuming listener can be cloned

                _ => Ok({
                    ()
                }), // Modify with appropriate handling
            };

            let _status_clone = status
                .clone()
                .to_owned()
                .err()
                .unwrap()
                .to_string()
                .to_value()
                .to_string();
            let _json_message = JsonResponseData {
                action: "uStatus".to_owned(),
                message: _status_clone.to_owned(),
                ue: "rust".to_owned(),
            };
            <SocketTestAgent as Clone>::clone(&self).send_to_tm(_json_message).await;
        }
    }

    async fn inform_tm_ta_starting(self) {
        let sdk_init = r#"{"ue":"rust","data":{"SDK_name":"rust"},"action":"initialize"}"#;

        //infor TM that rust TA is running
        println!("Sending SDK name to Test Manager!");
        let message = sdk_init.as_bytes();

        let socket_clone = self.clientsocket.clone();
        //socket_clone.write_all(message);
        let _ = socket_clone
            .lock()
            .expect("error in sending data to TM")
            .write_all(message);
    }

    async fn send_to_tm(self, json_message: JsonResponseData) {
        let json_message_str = convert_json_to_jsonstring(&json_message);
        let message = json_message_str.as_bytes();
        let socket_clone = self.clientsocket.clone();
        let _ = socket_clone
            .lock()
            .expect("error in sending data to TM")
            .write_all(message);
    }
    pub fn close_connection(&self) {
        let _ = self
            .clientsocket
            .lock()
            .expect("error in sending data to TM")
            .shutdown();
    }
}
