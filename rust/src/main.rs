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
use crate::constants::*;
use crate::uTransportSocket::UtransportExt;
use testagent::SocketTestAgent;
use uTransportSocket::UtrasnsportSocket;
mod testagent;
mod uTransportSocket;

use std::io::{self, Write};
use std::net::{Shutdown, /*TcpStream*/};
use std::thread;

use crate::utils::{convert_json_to_jsonstring, convert_str_to_bytes, send_socket_data};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::io::{AsyncReadExt,AsyncWriteExt};
use tokio::net::TcpStream;

fn main() {
   
    let transport = UtrasnsportSocket::new();
    let rt = Runtime::new().unwrap();
    rt.block_on(async { 
        let mut test_agent_socket = TcpStream::connect(TEST_MANAGER_ADDR).await.unwrap();
        
        let mut transport_socket =transport.await;
        transport_socket.socket_init();        
        let mut agent = SocketTestAgent::new(test_agent_socket, transport_socket);
                 

        agent.await.receive_from_tm();
       // agent.await.inform_tm_ta_starting();   
          
    });
}
