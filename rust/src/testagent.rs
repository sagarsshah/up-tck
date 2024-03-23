use std::any::{Any, TypeId};
use std::fmt::Debug;
use std::{string, usize};
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
use std::{io::Write /* , net::TcpStream*/};
use async_std::net;
use base64::Engine;
use prost::bytes::Bytes;
use prost_types::field;
use protobuf::reflect::FieldDescriptor;
use serde::de::DeserializeOwned;
use serde_json::{map, Value};
//use async_std::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::error;
use tokio::task::spawn_local;
//use up_rust::UAttributes;
//use up_rust::UTransport;
use std::io::Read;
//use std::sync::Arc;
//use std::collections::HashMap;
use protobuf::{Message, MessageDyn, SpecialFields};
//use serde::{Deserialize, Serialize};
use up_rust::ulistener::UListener;
use up_rust::{Data, UAttributes, UCode, UMessage, UMessageType, UStatus, UTransport, UUri};
use up_rust::{
    PublishValidator, RequestValidator, ResponseValidator, UAttributesValidator,
    UAttributesValidators, UriValidator,
};



use std::{
    collections::HashMap,
    sync::{atomic::AtomicU64, Arc, Mutex},
};

use serde::{Deserialize, Serialize};
//use serde_json::Value;
//use prost::Message;

#[derive(Debug, Deserialize)]
struct JsonObj {
    attributes: Value,
    payload: Value,
}



use crate::uTransportSocket::UtrasnsportSocket;
use crate::utils::{base64_to_protobuf_bytes, convert_json_to_jsonstring, protobuf_to_base64};
use crate::SEND_COMMAND;

#[derive(Serialize)]
pub struct JsonData {
    action: String,
    message: String,
}
// Define a listener type alias

trait JsonDecoder {
  /*   fn set_value_in_uProtocol(self,  key: &str, value: &[u8]);
    fn field_by_name(self,key:&String)-> Option<FieldDescriptor> ;
    fn mut_field(self, key: &String)-> &mut SpecialFields;
    fn is_some(self,key:&String)-> bool;
    fn set_value(&mut self, key: String, value: String) ;
    //fn get_sub_message(self, field_descriptor: FieldDescriptor) -> JsonUMessage;*/
    
    
}

struct JsonUMessage{
    uMessage: UMessage,
}
struct JsonUURi{
    uUURI: UUri,
}

impl JsonDecoder for JsonUMessage{
/* 
    fn get_sub_message(self, field_descriptor: FieldDescriptor) -> JsonUMessage{
        
   // let test = self.uMessage.descriptor_dyn().fields().map(|f| f.containing_message().new_instance());

   let mut nested_builder = self.uMessage
        .new_builder_for_field(field_descriptor)
        .expect("Failed to create nested builder");
 /*  let NestedMessag = self.uMessage
   .descriptor_dyn()
   .fields()
   .map(|f| f.containing_message().new_instance())
   .next(); // Get the first item in the iterator*/
  JsonUMessage{
     uMessage:  NestedMessag,
       
    }
}
    fn set_value_in_uProtocol( mut self, key: &str, value: &[u8]) {
        match key {
            "attributes" => {
                // Set the value in the "Attributes" field

                self.uMessage.attributes.merge_from_bytes(value);
            },
            "payload" => {
                // Set the value in the "Payload" field
                self.uMessage.payload.merge_from_bytes(value);
            },
            _ => {
                // Handle unknown key or error condition
                println!("Unknown key: {}", key);
            }
        }
    }
    fn field_by_name(self,key:&String)-> Option<FieldDescriptor>{
        self.uMessage.descriptor_dyn().field_by_name(key)
       

    }
    fn is_some(self,key:&String)-> bool{
        self.uMessage.descriptor_dyn().field_by_name(key).is_some()

    }
    fn mut_field(mut self, key: &String)->&mut SpecialFields
    {
        self.uMessage.mut_special_fields_dyn()
      
        //self.uMessage.mut_
    }

    fn set_value(&mut self, key: String, value: String) {
        

        self.uMessage.attributes.merge_from_bytes(value.as_bytes());
    }
 */  
}


//type Listener = Box<dyn Fn(Result<UMessage, UStatus>) + Send + Sync + 'static>;
//#[derive(Clone)]
pub struct SocketTestAgent {
    utransport: UtrasnsportSocket,
    clientsocket: Arc<Mutex<TcpStream>>,
    listner_map: Vec<String>,
}

impl UListener for SocketTestAgent {
    fn on_receive(&self, result: Result<UMessage, UStatus>) {
        println!("Listener onreceived");
        let mut json_message = JsonData {
            action: "onReceive".to_owned(),
            message: "None".to_string(),
        };
        match result {
            Ok(message) => json_message.message = protobuf_to_base64(&message),
            Err(status) => println!("Received error status: {}", status),
        }
        <SocketTestAgent as Clone>::clone(&self).send_to_tm(json_message);
    }
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



 
pub fn json_to_protobuf<T>(self, json_str: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: prost::Message + Default,
{
      // Parse JSON string into a Value
      let json_obj: Value = serde_json::from_str(json_str)?;

      // Serialize Value back into bytes
      let bytes = serde_json::to_vec(&json_obj)?;
  
  // Wrap bytes in a Bytes object
  let bytes = Bytes::from(bytes);

      // Decode bytes into the protobuf message
      let protobuf_message = T::decode(bytes)?;
  
      Ok(protobuf_message)
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
            let umsg_base64 = json_msg["message"].clone();
            let protobuf_serialized_data =
                base64_to_protobuf_bytes(&umsg_base64).expect("received data from TM is corrupt"); // Implement this function according to your logic

            let mut umsg = UMessage::new(); // Assuming UMessage is a protobuf-generated message type

            // Assuming umsg_serialized is the byte array obtained from SerializeToString()
            if let Err(err) = umsg.merge_from_bytes(&protobuf_serialized_data) {
                eprintln!("Error deserializing UMessage: {}", err);
            } else {
                eprint!("data seems to be correct!");
            }

            let status = match action.as_str() {
                "SEND_COMMAND" => match self.utransport.send(umsg).await {
                    Ok(_) => {
                        println!("message sent successfully");
                        ()
                    }
                    Err(_status) => {
                        println!("failed to send message");
                        ()
                    }
                },

                "REGISTER_LISTENER_COMMAND" => {
                    let cloned_listener = Arc::clone(&arc_self);
                    //let cloned_listener_data: UListener = Box::new(move |result: Result<UMessage, UStatus>| { <SocketTestAgent as Clone>::clone(&cloned_listener).on_receive(result); });
                    //let listener = Arc::new(MyListener);
                    let _ = self.utransport.register_listener(
                        umsg.attributes.source.clone().unwrap(),
                        &cloned_listener,
                    );
                    ()
                } // Assuming listener can be cloned

                "UNREGISTER_LISTENER_COMMAND" => {
                    let cloned_listener = Arc::clone(&arc_self);
                    // let cloned_listener_data: Listener = Box::new(move |result: Result<UMessage, UStatus>| { <SocketTestAgent as Clone>::clone(&cloned_listener).on_receive(result); });
                    let _ = self.utransport.unregister_listener(
                        umsg.attributes.source.clone().unwrap(),
                        &cloned_listener,
                    );
                    ()
                } // Assuming listener can be cloned

                _ => {
                    || UStatus {
                        code: UCode::OK.into(),
                        message: Some("Unknown action".to_string()),
                        details: todo!(),
                        special_fields: todo!(),
                    };
                } // Modify with appropriate handling
            };

            let _status_clone = status.clone();
            let base64_str = serde_json::to_string(&status).unwrap();
            let _json_message = JsonData {
                action: "uStatus".to_owned(),
                message: base64_str,
            };
            // Err(e) => {
            //   eprintln!("Error receiving data: {}", e);
            //   break;
            // }
        }
    }

    async fn inform_tm_ta_starting(self) {
        #[derive(Serialize)]
        struct JsonSdkname {
            sdk_name: String,
        }
        let json_sdk_name = JsonSdkname {
            sdk_name: String::from("Rust"),
        };
        //infor TM that rust TA is running
        println!("Sending SDK name to Test Manager Directly!");
        let json_message_str = convert_json_to_jsonstring(&json_sdk_name);
        let message = json_message_str.as_bytes();

        let socket_clone = self.clientsocket.clone();
        //socket_clone.write_all(message);
        let _ = socket_clone
            .lock()
            .expect("error in sending data to TM")
            .write_all(message);
    }

    async fn send_to_tm(self, json_message: JsonData) {
        let json_message_str = convert_json_to_jsonstring(&json_message);
        let message = json_message_str.as_bytes();
        let socket_clone = self.clientsocket.clone();
        let _ = socket_clone
            .lock()
            .expect("error in sending data to TM")
            .write_all(message);
    }
    fn close_connection(&self) {
        let _ = self
            .clientsocket
            .lock()
            .expect("error in sending data to TM")
            .shutdown();
    }


/* 
    
    fn populate_fields(self,json_obj: &mut HashMap<String, Value>, proto_obj: &mut dyn JsonDecoder) -> Result<(), Box<dyn std::error::Error>> {
        // let descriptor = proto_obj.descriptor();
         for (key, value) in json_obj.iter_mut() {
             if let Some(field) = proto_obj.field_by_name(key) {
                 if let Some(s) = value.as_str() {
                     if s.starts_with("BYTES:") {
                         let byte_string = s.trim_start_matches("BYTES:");
                         let byte_value = base64::decode(byte_string)?;
                         proto_obj.set_value_in_uProtocol(key, &byte_value)
                     } else {
                         self.set_field_value(key,proto_obj, field, value)?;
                     }
              //   } else //if let Some(sub_message) = proto_obj.store(proto_obj.mut_field(key).fmt(f)) {
                 //} else if let Some(sub_message) = proto_obj.field::<dyn Message>(field) {
                    // if let Some(map) = value.as_object_mut() {
                //        {
                  //      {
                    //     self.populate_fields(map, sub_message)?;
                    // }
                 }
             }
         }
         Ok(())
     }


    pub fn dict_to_proto(self,mut parent_json_obj: HashMap<String, Value>, parent_proto_obj: &mut dyn JsonDecoder) -> Result<(), Box<dyn std::error::Error>> {
        self.populate_fields(&mut parent_json_obj, parent_proto_obj)?;
        Ok(())
    }
    

    
    fn set_field_value(self, key:&String, proto_obj: &mut dyn JsonDecoder, field: FieldDescriptor, value: &mut Value) -> Result<(), Box<dyn std::error::Error>> {
       
       let typeId = field.type_id();
        match typeId {
        
            id if id == TypeId::of::<i32>() => {
                let int_value = value.as_i64().ok_or("Invalid value for Int32 field")? as i32;
                let update_value = int_value.to_string().as_bytes();
                proto_obj.set_value_in_uProtocol(key, update_value);
            }
            id if id == TypeId::of::<i64>() => {
                let int_value = value.as_i64().ok_or("Invalid value for Int64 field")?;
                let update_value = int_value.to_string().as_bytes();
                proto_obj.set_value_in_uProtocol(key, update_value);
            }
            id if id == TypeId::of::<f32>() => {
                let float_value = value.as_f64().ok_or("Invalid value for Float field")? as f32;
                let update_value = float_value.to_string().as_bytes();
                proto_obj.set_value_in_uProtocol(key, update_value);
            }
            id if id == TypeId::of::<f64>() => {
                let double_value = value.as_f64().ok_or("Invalid value for Double field")?;
                let update_value = double_value.to_string().as_bytes();
                proto_obj.set_value_in_uProtocol(key, update_value);
            }
            id if id == TypeId::of::<bool>() => {
                let bool_value = value.as_bool().ok_or("Invalid value for Bool field")?;
                let update_value = bool_value.to_string().as_bytes();
                proto_obj.set_value_in_uProtocol(key, update_value);
            }
            id if id == TypeId::of::<String>() => {
                let string_value = value.as_str().ok_or("Invalid value for String field")?;
                let update_value = string_value.to_string().as_bytes();
                proto_obj.set_value_in_uProtocol(key, update_value);
            }
            id if id == TypeId::of::<dyn JsonDecoder>() => {
                if let Some(map) = value.as_object_mut() {
                   // let nested_builder = proto_obj.descriptor().field::<dyn Message>(field).map(|f| f.message_descriptor().new_instance().unwrap());
                  // let nested_builder = proto_obj.get_json_obj();
                    if let Some(sub_message) = proto_obj.get_json_obj() {
                        self.populate_fields(map, sub_message)?;
                        proto_obj.set_field(field.number() as usize, sub_message);
                    }
                }
            }
            _ => {
                // Handle other types as needed
            }
        }
        Ok(())
    }

*/
  /*   
    #[cfg(test)]
    mod tests {
        use super::*;
        use prost::Message;
        use serde_json::{json, Value};
        use std::collections::HashMap;
    
        // Define a simple message struct for testing
        #[derive(Clone, Debug, PartialEq, Message)]
        struct TestMessage {
            #[prost(int32, tag = "1")]
            pub int_field: i32,
            #[prost(string, tag = "2")]
            pub string_field: String,
        }
    
        #[test]
        fn test_dict_to_proto() {
            // Create a HashMap representing JSON data
            let mut json_obj = HashMap::new();
            json_obj.insert("int_field".to_string(), json!(42));
            json_obj.insert("string_field".to_string(), json!("test_string"));
    
            // Create a mutable TestMessage instance
            let mut proto_obj = TestMessage {
                int_field: 0,
                string_field: String::new(),
            };
    
            // Call the dict_to_proto function
            assert!(dict_to_proto(json_obj, &mut proto_obj).is_ok());
    
            // Check if the TestMessage instance is properly populated
            assert_eq!(proto_obj.int_field, 42);
            assert_eq!(proto_obj.string_field, "test_string");
        }
    }
*/
  
/* 
    fn dict_to_proto1<T>(&mut self, parent_json_obj: &serde_json::Map<String, serde_json::Value>, parent_proto_obj: &mut T){

        fn populate_fields(self, json_obj: &serde_json::Map<String, serde_json::Value>, proto_obj: &mut T) {
            for (key, value) in json_obj {
                if let Some(value_str) = value.as_str() {
                    if value_str.starts_with("BYTES:") {
                        let bytes_value = value_str.trim_start_matches("BYTES:").as_bytes();
    
                        self.set_value_in_proto(proto_obj,key,bytes_value);
                    } else if let Some(field) = proto_obj.descriptor().field_by_name(key) {
    
                        if field.message.is_some() {
                            // Recursively update the nested message object
                            if let Some(nested_message) = proto_obj.mut_field(key) {
                                populate_fields(value.as_object().unwrap(), nested_message);
                            }
                        } else {
                           // Set scalar value
                           proto_obj.set_field(key, value.parse().expect("Failed to parse scalar value"));
                            }
                        }
                    }
                }
            }
        
    
    populate_fields(self, parent_json_obj, parent_proto_obj);
    //proto_obj
    }*/


/* 
    fn dict_to_proto1<T>(&mut self, parent_json_obj: &serde_json::Map<String, serde_json::Value>, parent_proto_obj: &dyn JsonDecoder)
 //   where
   //     T: ProtoObject, // Assume ProtoObject is a trait defining required methods
    {
        // Define the inner function to recursively populate fields
        fn populate_fields<T>(json_obj: &serde_json::Map<String, serde_json::Value>, proto_obj: &dyn JsonDecoder)
        //where
          //  T: ProtoObject, // Assume ProtoObject is a trait defining required methods
        {
            for (key, value) in json_obj { 
                if let Some(value_str) = value.as_str() {
                    if value_str.starts_with("BYTES:") {
                        let bytes_value = value_str.trim_start_matches("BYTES:").as_bytes();
                        proto_obj.set_value_in_uProtocol(key,bytes_value);
                    } else if let Some(field) = proto_obj.field_by_name(key) {
                        if proto_obj.is_some(key){
                            // Recursively update the nested message object
                            if let Some(nested_message) = proto_obj.mut_field(key) {
                                populate_fields(value.as_object().unwrap(), nested_message);
                            }
                        } else {
                            // Set scalar value
                            scalar_value = value.as_str();
                            proto_obj.set_value_in_uProtocol(key,); // Cloning the value to avoid ownership issues
                           
                        }
                    }
                }
            }
        }
    
        // Call the inner function to populate fields
        populate_fields::<T>(parent_json_obj, parent_proto_obj);
    }
    
*/
}

/*
    fn dict_to_proto(json_obj: HashMap<String, String>, mut proto_obj: my_proto::MyProto) -> my_proto::MyProto {
        fn populate_fields(json_obj: HashMap<String, String>, proto_obj: &mut my_proto::MyProto) {
            for (key, value) in json_obj {
                if value.starts_with("BYTES:") {
                    let value = value.replace("BYTES:", "");
                    let bytes_value = base64::decode(&value).expect("Failed to decode base64 string");
                    proto_obj.set_field(key, bytes_value);
                } else if let Some(field) = proto_obj.descriptor().field_by_name(&key) {
                    if field.is_message() {
                        // Recursively update the nested message object
                        let mut nested_proto_obj = proto_obj.get_or_init(key);
                        populate_fields(json_obj, &mut nested_proto_obj);
                    } else {
                        // Set scalar value
                        proto_obj.set_field(key, value.parse().expect("Failed to parse scalar value"));
                    }
                }
            }
        }

        populate_fields(json_obj, &mut proto_obj);

        proto_obj
    }



    

//use prost::Message;



}
//fn handle_send_command(json_msg: serde_json::Value, transport: &mut Transport) -> Result<(), prost::EncodeError> {
  //  let mut umsg = UMessage::default();
   // dict_to_proto(&json_msg["data"], &mut umsg);
   // transport.send(umsg)
//}*/
