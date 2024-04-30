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
use testagent::{FooListener, SocketTestAgent};
use utransport_socket::UTransportSocket;
mod testagent;
use log::error;
use std::net::TcpStream; //as TcpStreamSync;
use tokio::runtime::Runtime;

fn main() {
    let handle = thread::spawn(|| {
        // Create a new Tokio runtime
        let Ok(rt) = Runtime::new() else {
            eprintln!("Error creating runtime");
            return;
        };

        let test_agent_socket = match TcpStream::connect(TEST_MANAGER_ADDR) {
            Ok(socket) => socket,
            Err(err) => {
                error!("Error connecting test agent socket: {}", err);

                return;
            }
        };

        let test_agent_socket_to_tm = match TcpStream::connect(TEST_MANAGER_ADDR) {
            Ok(socket) => socket,
            Err(err) => {
                error!("Error connecting test agent socket: {}", err);

                return;
            }
        };

        rt.block_on(async {
            // Spawn a Tokio task to connect to TEST_MANAGER_ADDR asynchronously

            let transport_socket = match UTransportSocket::new() {
                Ok(socket) => {
                    // The function call succeeded
                    dbg!("socket trasport create successfully");
                    socket
                }
                Err(err) => {
                    // The function call failed with an error
                    error!("socket trasport create failed: {}", err);
                    return;
                }
            };

            //   let transport_socket_clone = transport_socket.clone();

            // Spawn a blocking task within the runtime
            //    let blocking_task = tokio::task::spawn_blocking(move || {
            // println!("calling socket_init..");
            //  transport_socket.socket_init();

            // match transport_socket.socket_init(){
            //     Ok(_) => {
            //         // The function call succeeded
            //         dbg!("socket trasport initilized successfully");
            //     }
            //     Err(err) => {
            //         // The function call failed with an error
            //         error!("socket trasport initilized failed: {}", err);
            //       return;
            //     }
            // }

            //  });

            // Don't wait for the blocking task to finish
            // tokio::spawn(async move {
            //     if let Err(err) = blocking_task.await {
            //         dbg!("Error in socket_init: {}", err);
            //         return;
            //     }
            //     dbg!("socket_init completed successfully");
            // });
            let foo_listener = Arc::new(FooListener::new(test_agent_socket_to_tm));
            let agent = SocketTestAgent::new(test_agent_socket, foo_listener);
            //agent.clone().receive_from_tm().await;
            agent.clone().receive_from_tm(transport_socket).await;
        });
    });

    handle.join().unwrap();
}
