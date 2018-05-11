//
// Copyright (C) 2018 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use nom::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct RxStatusEventLog {
    pub word: u32, //TODO: enum
    pub bit: u32,
    pub event: u32, //TODO: enum
    pub description: String,
}

impl RxStatusEventLog {
    pub fn new(raw: Vec<u8>) -> Option<Self> {
        match parse_rxstatusevent(&raw) {
            Ok(conv) => Some(conv.1),
            _ => None,
        }
    }
}

named!(parse_rxstatusevent(&[u8]) -> RxStatusEventLog,
    do_parse!(
        word: le_u32 >>
        bit: le_u32 >>
        event: le_u32 >>
        description: take!(32) >>
        (RxStatusEventLog {
            word,
            bit,
            event,
            description: ::std::str::from_utf8(description).unwrap().to_owned(),
            }
        )
    )
);
