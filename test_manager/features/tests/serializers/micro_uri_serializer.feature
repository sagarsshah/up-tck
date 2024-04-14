# -------------------------------------------------------------------------
#
# Copyright (c) 2024 General Motors GTO LLC
#
# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied.  See the License for the
# specific language governing permissions and limitations
# under the License.
# SPDX-FileType: SOURCE
# SPDX-FileCopyrightText: 2024 General Motors GTO LLC
# SPDX-License-Identifier: Apache-2.0
#
# -------------------------------------------------------------------------
Feature: UUri Micro Serialization

  Scenario Outline: Testing uuri micro serializer
    Given "<uE1>" creates data for "micro_serialize_uri"
      And sets "authority.id" to "<authority_id>"
      And sets "authority.name" to "<authority_name>"
      And sets "entity.id" to "<entity_id>"
      And sets "entity.name" to "<entity_name>"
      And sets "entity.version_major" to "<entity_version_major>"
      And sets "resource.id" to "<resource_id>"
      And sets "resource.name" to "<resource_name>"
      And sets "resource.instance" to "<resource_instance>"
      And sets "resource.message" to "<resource_message>"
    
    When sends "micro_serialize_uri" request
    Then receives micro serialized uri "<expected_bytes_as_base64_str>"

    Examples:
      | uE1    | authority_id       | authority_name | entity_id | entity_name | resource_id | resource_name | resource_instance | resource_message | entity_version_major | expected_bytes_as_base64_str |
      | python |                    |                |           | neelam      |             | rpc           | test              |                  | 0                    | <empty>                      |
      | python |                    |                | 1         | neelam      |             |               |                   |                  | 0                    | <empty>                      |
      | python |                    |                |           | neelam      | 1           |               |                   |                  | 1                    | <empty>                      |
      | python |                    |                | 1         | neelam      | 1           | test          |                   |                  | 0                    | AQAAAQABAAA= |
      | python |                    |                | 1         | neelam      | 1           |               |                   |                  | 0                    | AQAAAQABAAA= |
      | python |                    |                | 1         | neelam      | 1           |               |                   |                  | 1                    | AQAAAQABAQA= |
      | python |                    |                | 1         | neelam      | 1           | rpc           | test              |                  | 0                    | AQAAAQABAAA= |
      | python |                    |                | 2         | neelam      | 3           | test          |                   |                  | 1                    | AQAAAwACAQA= |
      | python |                    |                | 0         | neelam      | 0           | test          | front             |                  | 0                    | AQAAAAAAAAA= |
      | python |                    |                | 100       | neelam      | 300         | test          | front             |                  | 1                    | AQABLABkAQA= |
      | python |                    |                | 255       | neelam      | 255         | test          | front             | Test             | 0                    | AQAA/wD/AAA= |
      | python |                    |                | 256       | neelam      | 256         | test          | front             | Test             | 1                    | AQABAAEAAQA= |
      | python |BYTES:unique id 1234| vcu.my_car_vin | 29999     |             | 99          |               |                   |                  | 254                  | AQMAY3Uv/gAOdW5pcXVlIGlkIDEyMzQ= |

      | java   |                    |                |           | neelam      |             | rpc           | test              |                  | 0                    | <empty>                      |
      | java   |                    |                | 1         | neelam      |             |               |                   |                  | 0                    | <empty>                      |
      | java   |                    |                |           | neelam      | 1           |               |                   |                  | 1                    | <empty>                      |
      | java   |                    |                | 1         | neelam      | 1           | test          |                   |                  | 0                    | AQAAAQABAAA= |
      | java   |                    |                | 1         | neelam      | 1           |               |                   |                  | 0                    | AQAAAQABAAA= |
      | java   |                    |                | 1         | neelam      | 1           |               |                   |                  | 1                    | AQAAAQABAQA= |
      | java   |                    |                | 1         | neelam      | 1           | rpc           | test              |                  | 0                    | AQAAAQABAAA= |
      | java   |                    |                | 2         | neelam      | 3           | test          |                   |                  | 1                    | AQAAAwACAQA= |
      | java   |                    |                | 0         | neelam      | 0           | test          | front             |                  | 0                    | AQAAAAAAAAA= |
      | java   |                    |                | 100       | neelam      | 300         | test          | front             |                  | 1                    | AQABLABkAQA= |
      | java   |                    |                | 255       | neelam      | 255         | test          | front             | Test             | 0                    | AQAA/wD/AAA= |
      | java   |                    |                | 256       | neelam      | 256         | test          | front             | Test             | 1                    | AQABAAEAAQA= |
      | java   |BYTES:unique id 1234| vcu.my_car_vin | 29999     |             | 99          |               |                   |                  | 254                  | AQMAY3Uv/gAOdW5pcXVlIGlkIDEyMzQ= |