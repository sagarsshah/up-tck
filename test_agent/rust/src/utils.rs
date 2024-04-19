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

use log::error;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use up_rust::{
    Data, UAttributes, UAuthority, UCode, UEntity, UMessage, UMessageType, UPayload,
    UPayloadFormat, UPriority, UResource, UUri, UUID,
};

use protobuf::{Enum, MessageField, SpecialFields};

pub fn convert_json_to_jsonstring<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value).expect("Failed to convert to JSON string")
}

#[derive(Debug, Default)]
pub struct WrapperUUri(pub UUri);
impl<'de> Deserialize<'de> for WrapperUUri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;
        let mut _authority = UAuthority::new();
        let mut _uuri: UUri = UUri::new();
        let mut _entity = UEntity::new();
        let mut _resource = UResource::new();
        //update authority
        _authority.name = value
            .get("authority")
            .and_then(|authority| authority.get("name"))
            .and_then(|name| name.as_str())
            .map(String::from);

            if let Some(authority_value) = value.get("authority") {
                if let Some(name_value) = authority_value.get("name") {
                    if let Some(name_str) = name_value.as_str() {
                        _authority.name = Some(String::from(name_str));
                    } else {
                        error!("Error: Name field is not a string in Authority")
                    }
                } 
            } 
            


        if let Some(_authority_number_ip) = value
            .get("authority")
            .and_then(|authority| authority.get("number"))
            .and_then(|number| number.get("ip"))
        {
            _authority.number = Some(up_rust::Number::Ip(
                _authority_number_ip.to_string().as_bytes().to_vec(),
            ))
        } else if let Some(_authority_number_id) = value
            .get("authority")
            .and_then(|authority| authority.get("number"))
            .and_then(|number| number.get("id"))
        {
            _authority.number = Some(up_rust::Number::Id(
                _authority_number_id.to_string().as_bytes().to_vec(),
            ))
        };

   
        if let Some(entity) = value
            .get("entity")
            .and_then(|entity| entity.get("name"))
        {
            if let Some(name) = entity.as_str() {
                _entity.name = name.to_owned();
            } else {
           
                error!("Error: Name field is not a string in entity");
                
            }
        };


        if let Some(entity) = value.get("entity").and_then(|entity| entity.get("id")) {
            if let Ok(_entity_id_parsed) = entity
                .clone()
                .as_str()
                .expect("not a string")
                .parse::<u32>()
            {
                _entity.id = Some(_entity_id_parsed);
            }
        };

        if let Some(entity) = value
            .get("entity")
            .and_then(|entity| entity.get("version_major").and_then(|v| v.as_str()))
        {
            // Attempt to parse the string to u32
            _entity.version_major = Some(entity.parse::<u32>().unwrap_or_else(|_| {
                // Handle the error here, for now, just use 0 as default value
                0
            }))
        };

        if let Some(entity) = value
            .get("entity")
            .and_then(|entity| entity.get("version_minor").and_then(|v| v.as_str()))
        {
            
            _entity.version_minor = Some(entity.parse::<u32>().unwrap_or_else(|_| {
            
                0
            }))
        };

        _entity.special_fields = SpecialFields::default();

        if let Some(resource) = value
            .get("resource")
            .and_then(|resource| resource.get("name"))
        {
            if let Some(name) = resource.as_str() {
                _resource.name = name.to_owned();
            } else {
           
                error!("Error: Name field is not a string in resource");
                
            }
        };

        if let Some(resource_value) = value.get("resource") {
            if let Some(instance_value) = resource_value.get("instance") {
                if let Some(instance_str) = instance_value.as_str() {
                    _resource.instance = Some(instance_str.to_owned());
                } else {
                    error!("Error: instance field is not a string in resource");
                }
            } 
        } 
        if let Some(resource_value) = value.get("resource") {
            if let Some(message_value) = resource_value.get("message") {
                if let Some(message_str) = message_value.as_str() {
                    _resource.message = Some(message_str.to_owned());
                } else {
                    error!("Error: message field is not a string in resource");
                }
            } 
        }

        if let Some(resource_value) = value.get("resource") {
            if let Some(id_value) = resource_value.get("id") {
                if let Some(id_str) = id_value.as_str() {
                    if let Ok(parsed_id) = id_str.parse::<u32>() {
                        _resource.id = Some(parsed_id);
                    } else {
                        error!("Error: id field parsing to u32");
                    }
                } else {
                    error!("Error: id field is not string");
                }
            } 
        } 


    

        if !(_authority.get_name() == None && _authority.number == None) {
            dbg!("authority is not default");
            _uuri.authority = MessageField(Some(Box::new(_authority)));
        }
        _uuri.entity = MessageField(Some(Box::new(_entity)));
        _uuri.resource = MessageField(Some(Box::new(_resource)));

        Ok(WrapperUUri(_uuri))
    }
}
#[derive(Default)]
pub struct WrapperUAttribute(pub UAttributes);
impl<'de> Deserialize<'de> for WrapperUAttribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;
        let mut _uattributes = UAttributes::new();

        if let Some(priority_value) = value.get("priority") {
            if let Some(priority_str) = priority_value.as_str() {
                _uattributes.priority = UPriority::from_str(priority_str).unwrap_or_else(|| {
                    // Handle the case where the conversion fails
                    error!("Deserialize: Something wrong with priority field");
                    UPriority::UPRIORITY_UNSPECIFIED
                }).into();
            } else {
                error!("pririty value is not string!")
            }
        } else {
            error!("pririty value not available!")
        }
      

        dbg!("_uattributes.priority: {:?}", _uattributes.priority.clone());

        if let Some(type_value) = value.get("type") {
            if let Some(type_str) = type_value.as_str() {
                _uattributes.type_ = UMessageType::from_str(type_str).unwrap_or_else(|| {
                    // Handle the case where the conversion fails
                    error!("Deserialize: Something wrong with type field");
                    UMessageType::UMESSAGE_TYPE_UNSPECIFIED
                }).into();
            } else {
                error!("type value is not string!")
            }
        } else {
            error!("type value not available!")
        }

        dbg!("_uattributes.type_: {:?}", _uattributes.type_.clone());

        if let Some(source_value) = value.get("source") {
            if let Some(wrapper_uri) =
                serde_json::from_value::<WrapperUUri>(source_value.clone()).ok()
            {
                _uattributes.source = MessageField(Some(Box::new(wrapper_uri.0)));
            }
        };
        if let Some(sink_value) = value.get("sink") {
            if let Some(wrapper_uri) =
                serde_json::from_value::<WrapperUUri>(sink_value.clone()).ok()
            {
                _uattributes.sink = MessageField(Some(Box::new(wrapper_uri.0)));
            }
        };

        let mut ___id = UUID::new();
        if let Some(resource) = value.get("id").and_then(|resource| resource.get("lsb")) {
            if let Some(id_str) = resource.as_str() {
                if let Ok(parsed_id) = id_str.parse::<u64>() {
                    ___id.lsb = parsed_id;
                } else {
                    error!("Error: Failed to parse _id_lsb as u64");
                }
            } else {
                error!("Error: _id_lsb is not a string");
            }
        };
        if let Some(resource) = value.get("id").and_then(|resource| resource.get("msb")) {
            if let Some(id_str) = resource.as_str() {
                if let Ok(parsed_id) = id_str.parse::<u64>() {
                    ___id.msb = parsed_id;
                } else {
                    error!("Error: Failed to parse _id_msb as u64");
                }
            } else {
                error!("Error: _id_msb is not a string");
            }
        };

        _uattributes.id = MessageField(Some(Box::new(___id)));

        if let Some(_ttl) = value.get("ttl").and_then(|ttl| ttl.as_str()) {
            if let Ok(parsed_ttl) = _ttl.parse::<u32>() {
                _uattributes.ttl = parsed_ttl.into();
            } else {
                error!("Error: Failed to parse _ttl as u32");
            }
        };

        if let Some(_permission_level) = value
            .get("permission_level")
            .and_then(|permission_level| permission_level.as_str())
        {
            if let Ok(parsed_permission_level) = _permission_level.parse::<u32>() {
                _uattributes.permission_level = Some(parsed_permission_level.into());
            } else {
                error!("Error: Failed to parse permission_level as u32");
            }
        };

        if let Some(commstatus_value) = value.get("commstatus") {
            if let Some(commstatus_str) = commstatus_value.as_str() {
             _uattributes.commstatus   = Some(UCode::from_str(commstatus_str).unwrap().into());
            } else {
            error!("commstatus value is not string");
            }
        }

     dbg!(
            " _uattributes.commstatus: {:?}",
            _uattributes.commstatus.clone()
        );

        let mut ___reqid = UUID::new();
        if let Some(resource) = value.get("reqid").and_then(|resource| resource.get("lsb")) {
            if let Some(id_str) = resource.as_str() {
                if let Ok(parsed_id) = id_str.parse::<u64>() {
                    ___reqid.lsb = parsed_id;
                } else {
                    eprintln!("Error: Failed to parse _id_lsb as u64");
                }
            } else {
                eprintln!("Error: _id_lsb is not a string");
            }
        };
        if let Some(resource) = value.get("reqid").and_then(|resource| resource.get("msb")) {
            if let Some(id_str) = resource.as_str() {
                if let Ok(parsed_id) = id_str.parse::<u64>() {
                    ___reqid.msb = parsed_id;
                } else {
                    eprintln!("Error: Failed to parse _id_msb as u64");
                }
            } else {
                eprintln!("Error: _id_msb is not a string");
            }
        };
        _uattributes.reqid = MessageField(Some(Box::new(___reqid)));

        if let Some(_token) = value.get("token") {
            if let Some(token_str) = _token.as_str() {
                _uattributes.token = Some(token_str.to_owned());
            } else {
                error!("Error: token is not a string");
            }
        };
        if let Some(_traceparent) = value.get("traceparent") {
            if let Some(traceparent_str) = _traceparent.as_str() {
                _uattributes.traceparent = Some(traceparent_str.to_owned());
            } else {
                error!("Error: traceparent is not a string");
            }
        };

        // special field //todo
        let _special_fields = SpecialFields::default();

        if _special_fields.ne(&SpecialFields::default()) {
            _uattributes.special_fields = _special_fields;
        }

        Ok(WrapperUAttribute(_uattributes))
    }
}
#[derive(Default)]
pub struct WrapperUPayload(pub UPayload);
impl<'de> Deserialize<'de> for WrapperUPayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;
        let _format = match value.get("format") {
            Some(_format) => UPayloadFormat::from_str(
                _format
                    .as_str()
                    .expect("Deserialize:something wrong with _type field"),
            ),
            None => Some(UPayloadFormat::UPAYLOAD_FORMAT_UNSPECIFIED),
        };

        let _length = match value.get("length") {
            Some(_length) => _length
                .as_str()
                .unwrap_or_else(|| panic!("Deserialize: something wrong with commstatus field"))
                .parse::<i32>()
                .expect("commstatus parsing error"),
            None => 0,
        };

        let _data = match value.get("value") {
            Some(_data) => Data::Value(
                serde_json::to_vec(_data).expect("error in converting data value to vector"),
            ),
            None => Data::Reference(0),
        };

        Ok(WrapperUPayload(UPayload {
            length: Some(_length),
            format: _format.unwrap().into(),
            data: _data.into(),
            special_fields: Default::default(),
        }))
    }
}

#[derive(Debug, Default)]

pub struct WrapperUMessage(pub UMessage);

impl<'de> Deserialize<'de> for WrapperUMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;

        let wattributes = match value.get("attributes") {
            Some(attributes) => {
                serde_json::from_value::<WrapperUAttribute>(attributes.clone()).unwrap_or_default()
            }
            None => WrapperUAttribute(UAttributes::default()),
        };

        let wpayload = match value.get("payload") {
            Some(payload) => {
                serde_json::from_value::<WrapperUPayload>(payload.clone()).unwrap_or_default()
            }
            None => WrapperUPayload(UPayload::default()),
        };

        Ok(WrapperUMessage(UMessage {
            attributes: Some(wattributes.0).into(),
            payload: Some(wpayload.0).into(),
            special_fields: Default::default(),
        }))
    }
}

pub fn escape_control_character(c: char) -> String {
    let escaped = format!("\\u{:04x}", c as u32);
    escaped
}

pub fn sanitize_input_string(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '\x00'..='\x1F' => escape_control_character(c),
            _ => c.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_convert_json_to_jsonstring() {
        let json = serde_json::json!({"key": "value"});
        let result = convert_json_to_jsonstring(&json);
        assert_eq!(result, r#"{"key":"value"}"#);
    }
}

// use prost::Message; // Import the prost crate for protobuf message handling
use std::{default, fmt::Debug};

// Function to serialize any protobuf message to JSON string
// fn protobuf_to_json<M: Message>(message: &M) -> Result<String, serde_json::Error> {
//     // Serialize the protobuf message to bytes
//     let bytes = message.encode_to_vec();

//     // Deserialize the bytes into a JSON value
//     let json_value = serde_json::from_slice(&bytes)?;

//     // Serialize the JSON value into a JSON string
//     let json_string = serde_json::to_string(&json_value)?;

//     Ok(json_string)
// }
