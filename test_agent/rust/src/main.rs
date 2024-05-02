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

// #[path = "../../../up_client_socket/rust/u_transport_socket.rs"]
// pub mod u_transport_socket;

mod utils;

use std::{sync::Arc, thread};

use crate::constants::TEST_MANAGER_ADDR;
use testagent::{ListenerHandlers, SocketTestAgent};
use utransport_socket::UTransportSocket;
mod testagent;
use log::error;
use std::net::TcpStream; //as TcpStreamSync;
use tokio::runtime::Runtime;

// fn main() {
//     let handle = thread::spawn(|| {
//         // Create a new Tokio runtime
//         let Ok(rt) = Runtime::new() else {
//             eprintln!("Error creating runtime");
//             return;
//         };

//         let test_agent = match TcpStream::connect(TEST_MANAGER_ADDR) {
//             Ok(socket) => socket,
//             Err(err) => {
//                 error!("Error connecting test agent socket: {}", err);

//                 return;
//             }
//         };

//         let ta_to_tm_socket = match TcpStream::connect(TEST_MANAGER_ADDR) {
//             Ok(socket) => socket,
//             Err(err) => {
//                 error!("Error connecting test agent socket: {}", err);

//                 return;
//             }
//         };

//         let foo_listner_socket_to_tm = match TcpStream::connect(TEST_MANAGER_ADDR) {
//             Ok(socket) => socket,
//             Err(err) => {
//                 error!("Error connecting foo listner socket: {}", err);

//                 return;
//             }
//         };

//         rt.block_on(async {
//             // Spawn a Tokio task to connect to TEST_MANAGER_ADDR asynchronously

//             let u_transport = match UTransportSocket::new() {
//                 Ok(socket) => {
//                     // The function call succeeded
//                     dbg!("socket trasport create successfully");
//                     socket
//                 }
//                 Err(err) => {
//                     // The function call failed with an error
//                     error!("socket trasport create failed: {}", err);
//                     return;
//                 }
//             };

//             let foo_listener = Arc::new(ListenerHandlers::new(foo_listner_socket_to_tm));
//             let agent = SocketTestAgent::new(test_agent, foo_listener);
//             //agent.clone().receive_from_tm().await;
//             agent
//                 .clone()
//                 .receive_from_tm(u_transport, ta_to_tm_socket)
//                 .await;
//         });
//     });

//     handle.join().unwrap();
// }

fn connect_to_socket(addr: &str, port: u16) -> Result<TcpStream, Box<dyn std::error::Error>> {
    let socket_addr = format!("{}:{}", addr, port);
    match TcpStream::connect(socket_addr) {
        Ok(socket) => Ok(socket),
        Err(err) => {
            error!("Error connecting socket: {}", err);
            Err(Box::new(err))
        }
    }
}

async fn connect_and_receive() -> Result<(), Box<dyn std::error::Error>> {
    let test_agent = connect_to_socket(TEST_MANAGER_ADDR.0, TEST_MANAGER_ADDR.1)?;
    let ta_to_tm_socket = connect_to_socket(TEST_MANAGER_ADDR.0, TEST_MANAGER_ADDR.1)?;
    let foo_listener_socket_to_tm = connect_to_socket(TEST_MANAGER_ADDR.0, TEST_MANAGER_ADDR.1)?;

    let u_transport = UTransportSocket::new()?;
    dbg!("Socket transport created successfully");

    let foo_listener = Arc::new(ListenerHandlers::new(foo_listener_socket_to_tm));
    let agent = SocketTestAgent::new(test_agent, foo_listener);
    agent
        .clone()
        .receive_from_tm(u_transport, ta_to_tm_socket)
        .await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handle = thread::spawn(|| {
        let rt = Runtime::new().expect("Error creating runtime");
        match rt.block_on(connect_and_receive()) {
            Ok(_) => (),
            Err(err) => eprintln!("Error occurred: {}", err),
        };
    });

    handle.join().unwrap();
    Ok(())
}
