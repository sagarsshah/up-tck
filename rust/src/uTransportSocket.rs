/*
 * Copyright (c) 2023 General Motors GTO LLC
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


use async_std::io;
use async_trait::async_trait;

use protobuf::{Message, MessageDyn};
use std::net::TcpStream as TcpStreamSync;
use std::{
    //clone,
    io::{Read, Write},
    thread,
};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use std::{
    collections::HashMap,
    sync::{atomic::AtomicU64, Arc, Mutex},
   // time::Duration,
};

use up_rust::{
    transport::{datamodel::UTransport, validator::Validators},
    uprotocol::{
        Data, UAttributes, UCode, UMessage, UMessageType, UStatus, UUri,
    },
    uri::validator::UriValidator,
};

use crate::constants::BYTES_MSG_LENGTH;
use crate::constants::DISPATCHER_ADDR;


pub type UtransportListener = Box<dyn Fn(Result<UMessage, UStatus>) + Send + Sync + 'static>;

pub trait UtransportExt {
    
    async fn socket_init(&mut self);
    fn _handle_publish_message(&mut self, umsg: UMessage);
    async fn read_socket(&mut self, buffer: &mut [u8]) -> io::Result<usize>;
}

pub struct UtrasnsportSocket {
    socket: Arc<Mutex<TcpStream>>, //TcpStream,
    socket_sync: TcpStreamSync,
    listner_map: Arc<Mutex<HashMap<String, Vec<Arc<UtransportListener>>>>>,
}
impl Clone for UtrasnsportSocket {
    fn clone(&self) -> Self {
        UtrasnsportSocket {
            socket: self.socket.clone(),
            socket_sync: self
                .socket_sync
                .try_clone()
                .expect("issue in cloning sync socket"),
            listner_map: self.listner_map.clone(),
        }
    }
}

impl UtrasnsportSocket {
    pub async fn new() -> Self {
        let socket_connecton = TcpStream::connect(DISPATCHER_ADDR).await.unwrap();
        let socket_sync: TcpStreamSync =
            TcpStreamSync::connect(DISPATCHER_ADDR).expect("issue in connecting  sync socket");
        let socket = Arc::new(Mutex::new(socket_connecton));
        UtrasnsportSocket {
            socket,
            socket_sync,
            listner_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl UtransportExt for UtrasnsportSocket {
  
    async fn socket_init(&mut self) {
        loop {
            // Receive data from the socket
            let mut buffer: [u8; BYTES_MSG_LENGTH] = [0; BYTES_MSG_LENGTH];

            let bytes_read = match self.read_socket(&mut buffer).await {
                Ok(bytes_read) => bytes_read,
                Err(e) => {
                    // Handle socket errors (e.g., connection closed)
                    eprintln!("Socket error: {}", e);
                    break;
                }
            };

            // Check if no data is received
            if bytes_read == 0 {
                continue;
            }

            let mut umessage = UMessage::new(); // Assuming UMessage is a protobuf-generated message type

            if let Err(err) = umessage.merge_from_bytes(&buffer) {
                eprintln!("Error deserializing UMessage: {}", err);
            } else {
                eprint!("data seems to be correct!");
            }

            match umessage.attributes.type_.enum_value() {
            
                Ok(mt) => match mt {
                    UMessageType::UMESSAGE_TYPE_PUBLISH | UMessageType::UMESSAGE_TYPE_REQUEST => {
                        self._handle_publish_message(umessage);
                       // Ok(())
                       ()
                    }
                    UMessageType::UMESSAGE_TYPE_UNSPECIFIED => (),//Err("Umessage type unspecified".to_string()),
                    UMessageType::UMESSAGE_TYPE_RESPONSE => (),//Err("umessage type reponse not implemented".to_string()),
                },
                Err(_) => (),//Err("invalid arguments".to_string()),
          
            }
        }
    }

    async fn read_socket(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let mut socket = self
            .socket
            .lock()
            .expect("error access dispatcher socket connection");
        socket.read(buffer).await
    }

    fn _handle_publish_message(&mut self, umsg: UMessage) {
        let topic_b = umsg.attributes.source.to_string().as_bytes().to_vec();

        if let Some(tests) = self
            .listner_map
            .lock()
            .unwrap()
            .get(&umsg.attributes.source.to_string())
        {
            for listner in tests {
                listner(Ok(umsg.clone()));
            }
        }
    }
}

#[async_trait]
impl UTransport for UtrasnsportSocket {
    /// Sends a message using this transport's message exchange mechanism.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to send. The `type`, `source` and`sink` properties of the [`crate::UAttributes`] contained
    ///   in the message determine the addressing semantics:
    ///   * `source` - The origin of the message being sent. The address must be resolved. The semantics of the address
    ///     depends on the value of the given [attributes' type](crate::UAttributes::type_) property .
    ///     * For a [`PUBLISH`](crate::UMessageType::UMESSAGE_TYPE_PUBLISH) message, this is the topic that the message should be published to,
    ///     * for a [`REQUEST`](crate::UMessageType::UMESSAGE_TYPE_REQUEST) message, this is the *reply-to* address that the sender expects to receive the response at, and
    ///     * for a [`RESPONSE`](crate::UMessageType::UMESSAGE_TYPE_RESPONSE) message, this identifies the method that has been invoked.
    ///   * `sink` - For a `notification`, an RPC `request` or RPC `response` message, the (resolved) address that the message
    ///     should be sent to.
    ///
    /// # Errors
    ///
    /// Returns an error if the message could not be sent.
    async fn send(&self, message: UMessage) -> Result<(), UStatus> {
        let mut socket_clone = self
            .socket_sync
            .try_clone()
            .expect("dispatcher socket connection cloning failed");

        let umsg_serialized = message.to_string().as_bytes().to_vec().clone();
        // Acquire the lock on the mutex to access the TcpStream
        // Perform the write operation on the TcpStream
        let _payload = *message
            .payload
            .0
            .ok_or(UStatus::fail_with_code(UCode::INVALID_ARGUMENT, "Invalid uPayload").clone())?;
        let attributes = *message.attributes.0.ok_or(
            UStatus::fail_with_code(UCode::INVALID_ARGUMENT, "Invalid uAttributes").clone(),
        )?;

        // Check the type of UAttributes (Publish / Request / Response)
        match attributes
            .type_
            .enum_value()
            .map_err(|_| UStatus::fail_with_code(UCode::INTERNAL, "Unable to parse type"))?
        {
            UMessageType::UMESSAGE_TYPE_PUBLISH => {
                Validators::Publish
                    .validator()
                    .validate(&attributes)
                    .map_err(|e| {
                        UStatus::fail_with_code(
                            UCode::INVALID_ARGUMENT,
                            format!("Wrong Publish UAttributes {e:?}"),
                        )
                    })?;

                match socket_clone.write_all(&umsg_serialized) {
                    Ok(_) => Err(UStatus::ok()),
                    Err(_) => Err(UStatus::fail_with_code(
                        UCode::UNAVAILABLE,
                        "Dispatcher communication issue",
                    )),
                }
            }
            UMessageType::UMESSAGE_TYPE_REQUEST => {
                Validators::Request
                    .validator()
                    .validate(&attributes)
                    .map_err(|e| {
                        UStatus::fail_with_code(
                            UCode::INVALID_ARGUMENT,
                            format!("Wrong Request UAttributes {e:?}"),
                        )
                    })?;
                match socket_clone.write_all(&umsg_serialized) {
                    Ok(_) => Err(UStatus::ok()),
                    Err(_) => Err(UStatus::fail_with_code(
                        UCode::UNAVAILABLE,
                        "Dispatcher communication issue",
                    )),
                }
            }
            UMessageType::UMESSAGE_TYPE_RESPONSE => {
                Validators::Response
                    .validator()
                    .validate(&attributes)
                    .map_err(|e| {
                        UStatus::fail_with_code(
                            UCode::INVALID_ARGUMENT,
                            format!("Wrong Response UAttributes {e:?}"),
                        )
                    })?;
                match socket_clone.write_all(&umsg_serialized) {
                    Ok(_) => Err(UStatus::ok()),
                    Err(_) => Err(UStatus::fail_with_code(
                        UCode::UNAVAILABLE,
                        "Dispatcher communication issue",
                    )),
                }
            }
            UMessageType::UMESSAGE_TYPE_UNSPECIFIED => Err(UStatus::fail_with_code(
                UCode::INVALID_ARGUMENT,
                "Wrong Message type in UAttributes",
            )),
        }
    }

    /// Receives a message from the transport.
    ///
    /// # Arguments
    ///
    /// * `topic` - The topic to receive the message from.
    ///
    /// # Errors
    ///
    /// Returns an error if no message could be received. Possible reasons are that the topic does not exist
    /// or that no message is available from the topic.
    async fn receive(&self, topic: UUri) -> Result<UMessage, UStatus> {
        Err(UStatus::fail_with_code(
            UCode::UNIMPLEMENTED,
            "Not implemented",
        ))
    }

    /// Registers a listener to be called for each message that is received on a given address.
    ///
    /// # Arguments
    ///
    /// * `address` - The (resolved) address to register the listener for.
    /// * `listener` - The listener to invoke.
    ///
    /// # Returns
    ///
    /// An identifier that can be used for [unregistering the listener](Self::unregister_listener) again.
    ///
    /// # Errors
    ///
    /// Returns an error if the listener could not be registered.
    async fn register_listener(
        &self,
        topic: UUri,
        listener: Box<dyn Fn(Result<UMessage, UStatus>) + Send + Sync + 'static>,
    ) -> Result<String, UStatus> {
        let listener = Arc::new(listener);
        // self.listner_map.lock().unwrap().insert(topic.to_string(),listener);
        if topic.authority.is_some() && topic.entity.is_none() && topic.resource.is_none() {
            // This is special UUri which means we need to register for all of Publish, Request, and Response
            // RPC response
            Err(UStatus::fail_with_code(
                UCode::UNIMPLEMENTED,
                "Not implemented",
            ))
        } else {
            // Do the validation
            UriValidator::validate(&topic)
                .map_err(|_| UStatus::fail_with_code(UCode::INVALID_ARGUMENT, "Invalid topic"))?;

            if UriValidator::is_rpc_response(&topic) {
                self.listner_map
                    .lock()
                    .unwrap()
                    .entry(topic.to_string())
                    .and_modify(|Listener| Listener.push(listener.clone()))
                    .or_insert_with(|| vec![listener]);

                Ok("register listner successful".to_string())
            } else if UriValidator::is_rpc_method(&topic) {
                self.listner_map
                    .lock()
                    .unwrap()
                    .entry(topic.to_string())
                    .and_modify(|Listener| Listener.push(listener.clone()))
                    .or_insert_with(|| vec![listener]);
                Ok("register listner successful".to_string())
            } else {
                self.listner_map
                    .lock()
                    .unwrap()
                    .entry(topic.to_string())
                    .and_modify(|Listener| Listener.push(listener.clone()))
                    .or_insert_with(|| vec![listener]);
                Ok("register listner successful".to_string())
            }
        }
    }

    /// Unregisters a listener for a given topic.
    ///
    /// Messages arriving on this topic will no longer be processed by this listener.
    ///
    /// # Arguments
    ///
    /// * `topic` - Resolved topic uri where the listener was registered originally.
    /// * `listener` - Identifier of the listener that should be unregistered.
    ///
    /// # Errors
    ///
    /// Returns an error if the listener could not be unregistered, for example if the given listener does not exist.
    async fn unregister_listener(&self, topic: UUri, listener: &str) -> Result<(), UStatus> {
        let topic_serialized = topic.to_string().as_bytes().to_vec(); // Assuming SerializeToString returns Result<Vec<u8>, _>

        //  if let Some(listeners) = self.topic_to_listener.get_mut(&topic_serialized) {
        //    if listeners. > 1 {
        //      if let Some(index) = listeners.iter().position(|x| *x == listener) {
        //        listeners.remove(index);
        //  }
        //} else {
        //   self.topic_to_listener.remove(&topic_serialized);
        // }
        // }

        // fn remove_callback(&mut self, uri: &str, callback: Callback) {
        //if let Some(callbacks) = self.map.get_mut(uri) {
        //  callbacks.retain(|&cb| cb != callback);
        //if callbacks.is_empty() {
        //  self.map.remove(uri);
        // }
        //}
        //Err(UStatus{code: up_rust::uprotocol::UCode::OK.into(), message: "OK",details:todo!(),special_fields:todo!()})
        Err(UStatus {
            code: up_rust::uprotocol::UCode::OK.into(),
            message: Some("OK".to_string()), // Convert &str to String and wrap it into Some
            details: todo!(),
            special_fields: todo!(),
        })
        //UStatus { code: up_rust::UCode::OK, message: "OK", details: todo!(), special_fields: todo!() }
    }
}
