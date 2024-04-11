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
use protobuf::MessageField;
use serde_json::Value;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use up_rust::{UCode, UListener, UUIDBuilder};
use up_rust::{UMessage, UStatus, UTransport};

use std::io::{Read, Write};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde::Serialize;

use crate::u_transport_socket::UtrasnsportSocket;
use crate::utils::{convert_json_to_jsonstring, WrapperUMessage, WrapperUUri};
use crate::*;

#[derive(Serialize)]
pub struct JsonResponseData {
    data: HashMap<String, String>,
    action: String,
    ue: String,
}

pub struct SocketTestAgent {
    utransport: UtrasnsportSocket,
    clientsocket: Arc<Mutex<TcpStreamSync>>,
    clientsocket_to_tm: Arc<Mutex<TcpStreamSync>>,
    listner_map: Vec<String>,
}
#[async_trait]
impl UListener for SocketTestAgent {
    async fn on_receive(&self, msg: UMessage) {
        dbg!("Listener onreceived");
        let json_message = JsonResponseData {
            action: "onReceive".to_owned(),
            data: HashMap::new(),
            ue: "rust".to_string(),
        };
        //json_message.data = msg.clone().to_string().to_value().to_string();

        <SocketTestAgent as Clone>::clone(&self)
            .send_to_tm(json_message)
            .await;
    }

    async fn on_error(&self, _err: UStatus) {
        todo!();
    }
}

impl Clone for SocketTestAgent {
    fn clone(&self) -> Self {
        SocketTestAgent {
            utransport: self.utransport.clone(), // Assuming UtrasnsportSocket implements Clone
            clientsocket: self.clientsocket.clone(),
            clientsocket_to_tm:self.clientsocket_to_tm.clone(),
            listner_map: self.listner_map.clone(), // Clone Vec<String>
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

impl SocketTestAgent {
    pub fn new(test_clientsocket: TcpStreamSync, test_clientsocket_to_tm : TcpStreamSync, utransport: UtrasnsportSocket) -> Self {
        let socket = Arc::new(Mutex::new(test_clientsocket));
        let socket_to_tm = Arc::new(Mutex::new(test_clientsocket_to_tm));
        let clientsocket = socket;
        let clientsocket_to_tm = socket_to_tm;
        SocketTestAgent {
            utransport,
            clientsocket,
            clientsocket_to_tm,
            listner_map: Vec::new(),
        }
    }


    fn sanitize_input_string(self, input: &str) -> String {
        input.chars()
            .map(|c| {
                match c {
                    '\x00'..='\x1F' => self.clone().escape_control_character(c),
                    _ => c.to_string(),
                }
            })
            .collect()
    }
     
    fn escape_control_character(self,c: char) -> String {
        let escaped = format!("\\u{:04x}", c as u32);
        escaped
    }
    pub async fn receive_from_tm(&mut self) {
        // Clone Arc to capture it in the closure

        let arc_self = Arc::new(self.clone());
        self.clone().inform_tm_ta_starting();
        let mut socket = self.clientsocket.lock().expect("error accessing TM server");

        loop {
            let mut recv_data = [0; 2048];

            let bytes_received = match socket.read(&mut recv_data) {
                Ok(bytes_received) => bytes_received,
                Err(e) => {
                    // Handle socket errors (e.g., connection closed)
                    dbg!("Socket error: {}", e);
                    break;
                }
            };
            // Check if no data is received
            if bytes_received == 0 {
                continue;
            }

            let recv_data_str: std::borrow::Cow<'_, str> =
            String::from_utf8_lossy(&recv_data[..bytes_received]);
            let mut action_str = "";
            let cleaned_json_string = self.clone().sanitize_input_string(&recv_data_str).replace("BYTES:", "");
            let json_msg: Value = serde_json::from_str(&cleaned_json_string.to_string()).expect("issue in from str"); // Assuming serde_json is used for JSON serialization/deserialization
            let action = json_msg["action"].clone();
            let json_data_value = json_msg["data"].clone();
            dbg!("JSONMSG");
            dbg!(json_msg);
            
            let json_str_ref = action.as_str().expect("issue in converting value to string");
            dbg!("json data json_str_ref: {:?}", json_str_ref);

            let status = match json_str_ref {
                SEND_COMMAND => {
                    let mut wu_message: WrapperUMessage =
                        serde_json::from_value(json_data_value).unwrap(); // convert json to UMessage
                        dbg!( wu_message.0.clone());
                    let  u_message = wu_message.0;
                    action_str = constants::SEND_COMMAND;
                    self.utransport.send(u_message).await
                }

                REGISTER_LISTENER_COMMAND => {
                    let cloned_listener = Arc::clone(&arc_self);
                    // dbg!("Callable?");
                    // arc_self.on_receive(UMessage::default()).await;
                    // dbg!("Callable!");

                    let wu_uuri: WrapperUUri = serde_json::from_value(json_data_value).unwrap(); // convert json to UMessage
                    dbg!( wu_uuri.0.clone());
                    let u_uuri = wu_uuri.0;
                    action_str = constants::REGISTER_LISTENER_COMMAND;
                    self.utransport
                        .register_listener(
                            u_uuri,
                            Arc::clone(&cloned_listener) as Arc<dyn UListener>, /*&cloned_listener*/
                        )
                        .await
                } // Assuming listener can be cloned

                UNREGISTER_LISTENER_COMMAND => {
                    let cloned_listener = Arc::clone(&arc_self);
                    let wu_uuri: WrapperUUri = serde_json::from_value(json_data_value).unwrap(); // convert json to UMessage
                    dbg!( wu_uuri.0.clone());
                    let u_uuri = wu_uuri.0;
                    action_str = constants::UNREGISTER_LISTENER_COMMAND;
                    self.utransport
                        .unregister_listener(
                            u_uuri,
                            Arc::clone(&cloned_listener) as Arc<dyn UListener>, /*&cloned_listener*/
                        )
                        .await
                } // Assuming listener can be cloned

                _ => Ok({ () }), // Modify with appropriate handling
            };


            // Create an empty HashMap to store the fields of the message
            let mut status_dict:HashMap<String, String> = HashMap::new();

            match status {
                Ok(()) => {
                    // Handle the case when status is Ok
                    let status = UStatus::default(); // Replace this with your actual status object
                    status_dict.insert("message".to_string(), status.message.clone().unwrap_or_default());
                    //status_dict.insert("code".to_string(), status.code);
                    // Add more fields as needed
                }
                Err(u_status) => {
                    // Handle the case when status is an error
                    // Convert the error message to a string and insert it into the HashMap
                    status_dict.insert("message".to_string(), u_status.message.clone().unwrap_or_default());
                    let enum_string = match u_status.get_code() {
                        UCode::OK => "OK",
                        UCode::INTERNAL => "INTERNAL",
                        UCode::ABORTED => "ABORTED",
                        UCode::ALREADY_EXISTS => "ALREADY_EXISTS",
                        UCode::CANCELLED => "CANCELLED",
                        UCode::DATA_LOSS => "DATA_LOSS",
                        UCode::DEADLINE_EXCEEDED => "DEADLINE_EXCEEDED",
                        UCode::FAILED_PRECONDITION => "FAILED_PRECONDITION",
                        UCode::INVALID_ARGUMENT => "INVALID_ARGUMENT",
                        UCode::NOT_FOUND => "NOT_FOUND",
                        UCode::OUT_OF_RANGE => "OUT_OF_RANGE",
                        UCode::PERMISSION_DENIED => "PERMISSION_DENIED",
                        UCode::RESOURCE_EXHAUSTED => "RESOURCE_EXHAUSTED",
                        UCode::UNAUTHENTICATED => "UNAUTHENTICATED",
                        UCode::UNAVAILABLE => "UNAVAILABLE",
                        UCode::UNIMPLEMENTED => "UNIMPLEMENTED",
                        UCode::UNKNOWN => "UNKNOWN"
                    };
                    status_dict.insert("code".to_string(), enum_string.to_string());
                }
            }

            let _json_message = JsonResponseData {
                action: action_str.to_owned(),
                data: status_dict.to_owned(),
                ue: "rust".to_owned(),
            };


            <SocketTestAgent as Clone>::clone(&self)
                .send_to_tm(_json_message)
                .await;
        }
        self.close_connection();
    }

     fn inform_tm_ta_starting(self) {
        let sdk_init = r#"{"ue":"rust","data":{"SDK_name":"rust"},"action":"initialize"}"#;

        //infor TM that rust TA is running
        dbg!("Sending SDK name to Test Manager!");
        let message = sdk_init.as_bytes();

        let socket_clone = self.clientsocket.clone();
        //socket_clone.write_all(message);
        let  retun_value = socket_clone
            .lock()
            .expect("error in sending data to TM")
            .write_all(message);
    }

    async fn send_to_tm(self, json_message: JsonResponseData) {

        let json_message_str = convert_json_to_jsonstring(&json_message);
        let message = json_message_str.as_bytes();
        let socket_clone = self.clientsocket_to_tm.clone();
        let result = socket_clone
            .try_lock()
            .expect("error in sending data to TM")
            .write_all(message);
    }
    pub fn close_connection(&self) {
           todo!();
    }
}
