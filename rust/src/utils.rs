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


use crate::constants::*; 
use serde_json::Value;
use serde::{Serialize, Deserialize};
use std::io::{Read, Write};
//use std::net::TcpStream;
use protobuf::Message;

use std::net::TcpStream;

//use uprotocol::cloudevent::serialize::base64protobufserializer::Base64ProtobufSerializer;
//use uprotocol::logger::logger;
//use uprotocol::utils::constants::BYTES_MSG_LENGTH;

pub fn send_socket_data(stream:&mut TcpStream, msg: &[u8]) -> std::io::Result<()> {
    stream.write_all(msg)?;
    Ok(())
}

pub fn receive_socket_data(stream:&mut TcpStream) -> std::io::Result<Vec<u8>> {
    let mut buffer = vec![0; BYTES_MSG_LENGTH];
    stream.read_exact(&mut buffer)?;
    Ok(buffer)
}


// Define a function to convert a Protocol Buffers message to a Base64-encoded string
pub fn protobuf_to_base64<T: Message>(obj: &T) -> String {
    // Serialize the Protocol Buffers message to bytes
    let serialized_bytes = obj.write_to_bytes().expect("Failed to serialize message");

    // Encode the bytes to Base64
    let base64_str = base64::encode(&serialized_bytes);

    // Return the Base64-encoded string
    base64_str
}

// Define a function to convert a Base64-encoded string to Protocol Buffers bytes
pub fn base64_to_protobuf_bytes(base64str: &str) -> Result<Vec<u8>, base64::DecodeError> {
    // Decode the Base64-encoded string to bytes
    let decoded_bytes = base64::decode(base64str)?;

    // Return the decoded bytes
    Ok(decoded_bytes)
}



pub fn convert_bytes_to_string(data: &[u8]) -> String {
    String::from_utf8_lossy(data).into_owned()
}

pub fn convert_jsonstring_to_json(jsonstring: &str) -> Value {
    serde_json::from_str(jsonstring).unwrap()
}


pub fn convert_json_to_jsonstring<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value).expect("Failed to convert to JSON string")
}
//pub fn convert_json_to_jsonstring(j: &Value) -> String {
  //  j.to_string()
//}

pub fn convert_str_to_bytes(string: &str) -> Vec<u8> {
    string.as_bytes().to_vec()
}


#[cfg(test)]
mod tests {
    //use std::any::Any;
    use super::*;
    use protobuf::well_known_types::any::Any;
    use std::net::TcpListener;
    

    #[test]
   // use std::net::TcpListener;
    fn test_send_receive_socket_data() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to socket");
        let addr = listener.local_addr().expect("Failed to get local address");

        // Spawn a thread to accept incoming connections
        std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("Failed to accept connection");
            let mut received_data = vec![0; 10];
            receive_socket_data(&mut stream).unwrap().copy_from_slice(&mut received_data);
            assert_eq!(received_data, b"HelloWorld");
        });

        // Connect to the listener's address
        let mut stream = TcpStream::connect(addr).expect("Failed to connect to server");

        // Send data
        send_socket_data(&mut stream, b"HelloWorld").unwrap();
    }

   
    #[test]
    //use std::net::TcpListener;
    fn test_protobuf_to_base64_and_base64_to_protobuf_bytes() {
        // Create a sample Protocol Buffers Any message
        let mut any = Any::new();
        
        any.type_url= "example.com/MyMessage".to_string();
        any.value= vec![1, 2, 3, 4, 5];

        // Convert the message to a Base64-encoded string
        let base64_str = protobuf_to_base64(&any);

        // Decode the Base64-encoded string to bytes
        let decoded_bytes = base64_to_protobuf_bytes(&base64_str).unwrap();

        println!("any: {:x?}", any.write_to_bytes().unwrap());
        println!("Decoded bytes: {:x?}", decoded_bytes);
        // Check if the decoded bytes match the original message bytes
        assert_eq!(any.write_to_bytes().unwrap(), decoded_bytes);
       
    }
    #[test]
    fn test_convert_bytes_to_string() {
        let data = vec![104, 101, 108, 108, 111]; // "hello" in bytes
        let result = convert_bytes_to_string(&data);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_convert_jsonstring_to_json() {
        let jsonstring = r#"{"key": "value"}"#;
        let result = convert_jsonstring_to_json(jsonstring);
        assert_eq!(result["key"], "value");
    }

    #[test]
    fn test_convert_json_to_jsonstring() {
        let json = serde_json::json!({"key": "value"});
        let result = convert_json_to_jsonstring(&json);
        assert_eq!(result, r#"{"key":"value"}"#);
    }

    #[test]
    fn test_convert_str_to_bytes() {
        let string = "hello";
        let result = convert_str_to_bytes(string);
        assert_eq!(result, vec![104, 101, 108, 108, 111]); // "hello" in bytes
    }

    // Write more test cases for other functions...
}





