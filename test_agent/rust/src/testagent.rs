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
use serde_json::Value;
use up_rust::{Data, UCode, UListener};
use up_rust::{UMessage, UStatus, UTransport};

use std::io::{Read, Write};
use std::{
    collections::HashMap,
    sync::Arc,
};
use tokio::sync::Mutex;

use serde::Serialize;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task;

use crate::utils::{convert_json_to_jsonstring, WrapperUMessage, WrapperUUri};
use crate::*;

#[derive(Serialize)]
pub struct JsonResponseData {
    data: HashMap<String, String>,
    action: String,
    ue: String,
}

#[derive(Clone)]
pub struct SocketTestListener {
    sender_to_test_agent: Sender<JsonResponseData>
}

impl SocketTestListener {
    pub fn new(sender_to_test_agent: Sender<JsonResponseData>) -> Self {
        Self {
            sender_to_test_agent
        }
    }
}

#[derive(Clone)]
pub struct SocketTestAgent {
    utransport: UtransportSocket,
    listener: Arc<Mutex<Arc<SocketTestListener>>>,
    clientsocket: Arc<Mutex<TcpStreamSync>>,
    clientsocket_to_tm: Arc<Mutex<TcpStreamSync>>,
}

#[async_trait]
impl UListener for SocketTestListener {
    
    async fn on_receive(&self, msg: UMessage) {
        dbg!("OnReceive called");

        let __payload = match &msg.payload.data {
            Some(data) => {
                // Now we have access to the Data enum
                match data {Data::Reference(reference)=>{reference.to_string()}Data::Value(value)=> {let value_str=String::from_utf8_lossy(value);value_str.to_string()},
    _ => "none".into(), }
            }
            None => {
                println!("No data available");
                "none".into()
            }
        };

        let Ok(value_to_payload_str) = serde_json::to_string(&HashMap::from([
            ("value".to_string(), __payload.to_string())
        ])) else {
            // error!("unable to stringify hashmap");
            return;
        };

        let Ok(payload_to_value_str) = serde_json::to_string(&HashMap::from([
            ("payload".to_string(), value_to_payload_str)
        ])) else {
            // error!("unable to stringify hashmap");
            return;
        };

        let data = HashMap::from([
            ("data".to_string(), payload_to_value_str)
        ]);

        let json_message = JsonResponseData {
            action: constants::RESPONSE_ON_RECEIVE.to_owned(),
            data,
            ue: "rust".to_string(),
        };

        match self.sender_to_test_agent.send(json_message).await {
            Ok(_) => { /* log something here if you like */ }
            Err(e) => {
                // error!("Unable to send data to test agent!");
            }
        }
    }

    async fn on_error(&self, _err: UStatus) {
        todo!();
    }
}

impl SocketTestAgent {
    pub fn new(test_clientsocket: TcpStreamSync, test_clientsocket_to_tm : TcpStreamSync, utransport: UtransportSocket) -> Arc<Mutex<Self>> {
        let clientsocket = Arc::new(Mutex::new(test_clientsocket));
        let clientsocket_to_tm = Arc::new(Mutex::new(test_clientsocket_to_tm));
        let (tx, rx) = channel(100);
        let listener = Arc::new(Mutex::new(Arc::new(SocketTestListener::new(tx))));

        let myself = Arc::new(Mutex::new(Self {
            utransport,
            listener,
            clientsocket,
            clientsocket_to_tm,
        }));

        let myself_for_listening = myself.clone();
        task::spawn_blocking(move || {
            Self::listen_for_sends_to_tm(rx, myself_for_listening)
        });

        myself
    }

    fn sanitize_input_string(input: &str) -> String {
        input.chars()
            .map(|c| {
                match c {
                    '\x00'..='\x1F' => Self::escape_control_character(c),
                    _ => c.to_string(),
                }
            })
            .collect()
    }
     
    fn escape_control_character(c: char) -> String {
        let escaped = format!("\\u{:04x}", c as u32);
        escaped
    }
    pub async fn receive_from_tm(&self) {

        let mut socket = self.clientsocket.lock().await;
        let listener_clone_for_loop = self.listener.clone();

        loop {
            let listener_top_of_loop = listener_clone_for_loop.clone();
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
            let cleaned_json_string = Self::sanitize_input_string(&recv_data_str).replace("BYTES:", "");
            let json_msg: Value = serde_json::from_str(&cleaned_json_string.to_string()).expect("issue in from str"); // Assuming serde_json is used for JSON serialization/deserialization
            let action = json_msg["action"].clone();
            let json_data_value = json_msg["data"].clone();
            
            let json_str_ref = action.as_str().expect("issue in converting value to string");

            let status = match json_str_ref {
                SEND_COMMAND => {
                    let wu_message: WrapperUMessage =
                        serde_json::from_value(json_data_value).unwrap(); // convert json to UMessage
                    let  u_message = wu_message.0;
                    action_str = constants::SEND_COMMAND;
                    self.utransport.send(u_message).await
                }

                REGISTER_LISTENER_COMMAND => {
                    let cloned_listener = listener_top_of_loop.lock().await.clone();

                    let wu_uuri: WrapperUUri = serde_json::from_value(json_data_value).unwrap(); // convert json to UMessage
                    let u_uuri = wu_uuri.0;
                    action_str = constants::REGISTER_LISTENER_COMMAND;
                    self.utransport
                        .register_listener(
                            u_uuri,
                            cloned_listener,
                        )
                        .await
                }

                UNREGISTER_LISTENER_COMMAND => {
                    let cloned_listener = listener_top_of_loop.lock().await.clone();
                    let wu_uuri: WrapperUUri = serde_json::from_value(json_data_value).unwrap(); // convert json to UMessage
                    let u_uuri = wu_uuri.0;
                    action_str = constants::UNREGISTER_LISTENER_COMMAND;
                    self.utransport
                        .unregister_listener(
                            u_uuri,
                            cloned_listener
                        )
                        .await
                }

                _ => Ok(())
            };

            // Create an empty HashMap to store the fields of the message
            let mut status_dict:HashMap<String, String> = HashMap::new();

            match status {
                Ok(()) => {
                    let status = UStatus::default();
                    status_dict.insert("message".to_string(), status.message.clone().unwrap_or_default());
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

     async fn inform_tm_ta_starting(self) {
        let sdk_init = r#"{"ue":"rust","data":{"SDK_name":"rust"},"action":"initialize"}"#;

        //inform TM that rust TA is running
        dbg!("Sending SDK name to Test Manager!");
        let message = sdk_init.as_bytes();

        let socket_clone = self.clientsocket.clone();
        let _ = socket_clone
            .lock()
            .await
            .write_all(message);
    }

    async fn listen_for_sends_to_tm(mut receiver_from_listener: Receiver<JsonResponseData>, myself: Arc<Mutex<SocketTestAgent>>) {
        let self_clone = myself.clone();
        task::spawn(async move {
            while let Some(json_response_data) = receiver_from_listener.recv().await {
                let myself = self_clone.clone();
                myself.lock().await.send_to_tm(json_response_data).await;
            }
        });
    }

    async fn send_to_tm(&self, json_message: JsonResponseData) {

        let json_message_str = convert_json_to_jsonstring(&json_message);
        let message = json_message_str.as_bytes();
        
        let socket_clone = self.clientsocket_to_tm.clone();
        let _result = socket_clone
            .try_lock()
            .expect("error in sending data to TM")
            .write_all(message);
    }

    pub fn close_connection(&self) {
           todo!();
    }
}
