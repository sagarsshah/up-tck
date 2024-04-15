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

#[path = "../../../up_client_socket/rust/u_transport_socket.rs"]
pub mod u_transport_socket;
 
mod utils;

use std::thread;

use crate::constants::*;
use testagent::SocketTestAgent;
use u_transport_socket::{UtransportExt, UtransportSocket};
mod testagent;
use tokio::runtime::Runtime;
use std::net::TcpStream as TcpStreamSync;

#[tokio::main]
async fn main() {
    let test_agent_socket: TcpStreamSync =
    TcpStreamSync::connect(TEST_MANAGER_ADDR).expect("issue in connecting  sync socket");
    let test_agent_socket_to_tm: TcpStreamSync =
    TcpStreamSync::connect(TEST_MANAGER_ADDR).expect("issue in connecting  sync socket");

    let mut transport_socket = UtransportSocket::new();
    let transport_socket_clone = transport_socket.clone();

    tokio::spawn(async move {
        // you could add explicit error return from socket_init if you wanted to handle this here
        transport_socket.socket_init();
    });

    let agent = SocketTestAgent::new(test_agent_socket,test_agent_socket_to_tm, transport_socket_clone);
    agent.lock().await.receive_from_tm().await;
}
