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
use crate::constants::TEST_MANAGER_ADDR;
use testagent::{ListenerHandlers, SocketTestAgent};
use up_rust::{Number, UAuthority, UEntity, UTransport};
use utransport_socket::UTransportSocket;
mod testagent;
use log::error;
use std::{env, net::TcpStream, sync::Arc, thread}; //as TcpStreamSync;
use tokio::runtime::Runtime;
use up_client_zenoh::UPClientZenoh;
use zenoh::config::Config;

fn connect_to_socket(addr: &str, port: u16) -> Result<TcpStream, Box<dyn std::error::Error>> {
    let socket_addr = format!("{addr}:{port}");
    match TcpStream::connect(socket_addr) {
        Ok(socket) => Ok(socket),
        Err(err) => {
            error!("Error connecting socket: {}", err);
            Err(Box::new(err))
        }
    }
}

async fn connect_and_receive(transport_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let test_agent = connect_to_socket(TEST_MANAGER_ADDR.0, TEST_MANAGER_ADDR.1)?;
    let ta_to_tm_socket = connect_to_socket(TEST_MANAGER_ADDR.0, TEST_MANAGER_ADDR.1)?;
    let foo_listener_socket_to_tm = connect_to_socket(TEST_MANAGER_ADDR.0, TEST_MANAGER_ADDR.1)?;

    let u_transport: Box<dyn UTransport> = if "zenoh" == transport_name {
        let uauthority = UAuthority {
            name: Some("MyAuthName".to_string()),
            number: Some(Number::Id(vec![1, 2, 3, 4])),
            ..Default::default()
        };
        let uentity = UEntity {
            name: "default.entity".to_string(),
            id: Some(u32::from(rand::random::<u16>())),
            version_major: Some(1),
            version_minor: None,
            ..Default::default()
        };
        dbg!("zenoh transport created successfully");

        Box::new(
            UPClientZenoh::new(Config::default(), uauthority, uentity)
                .await
                .unwrap(),
        )
    } else {
        dbg!("Socket transport created successfully");
        Box::new(UTransportSocket::new()?)
    };

    let u_transport_ref: &dyn UTransport = &*u_transport;

    let foo_listener = Arc::new(ListenerHandlers::new(foo_listener_socket_to_tm));
    let agent = SocketTestAgent::new(test_agent, foo_listener);
    agent
        .clone()
        .receive_from_tm(u_transport_ref, ta_to_tm_socket)
        .await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handle = thread::spawn(|| {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            eprintln!("Transport Name: {} <your_string>", args[0]);
            std::process::exit(1);
        }

        let transport_name = &args[1].clone(); // Assuming the string is the first argument

        println!("Transport Name: {}", transport_name);

        let rt = Runtime::new().expect("error creating run time");

        match rt.block_on(connect_and_receive(&transport_name)) {
            Ok(_) => (),
            Err(err) => eprintln!("Error occurred: {}", err),
        };
    });

    if let Err(err) = handle.join() {
        eprintln!("Error joining thread: {err:?}");
        std::process::exit(1);
    } else {
        dbg!("Successfully joined thread");
    }
    Ok(())
}
