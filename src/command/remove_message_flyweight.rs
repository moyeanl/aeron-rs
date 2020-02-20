/*
 * Copyright 2020 UT OVERSEAS INC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::command::correlated_message_flyweight::{CorrelatedMessageDefn, CorrelatedMessageFlyweight};
use crate::concurrent::atomic_buffer::AtomicBuffer;
use crate::utils::types::Index;

/**
* Control message for removing a Publication or Subscription.
*
* <p>
* 0                   1                   2                   3
* 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
* +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
* |                         Client ID                             |
* |                                                               |
* +---------------------------------------------------------------+
* |                    Command Correlation ID                     |
* |                                                               |
* +---------------------------------------------------------------+
* |                       Registration ID                         |
* |                                                               |
* +---------------------------------------------------------------+
*/

#[repr(C, packed(4))]
#[derive(Copy, Clone)]
struct RemoveMessageDefn {
    correlated_message: CorrelatedMessageDefn,
    registration_id: i64,
}

const REMOVE_MESSAGE_LENGTH: Index = std::mem::size_of::<RemoveMessageDefn>() as Index;

struct RemoveMessageFlyweight {
    correlated_message_flyweight: CorrelatedMessageFlyweight,
    m_struct: RemoveMessageDefn,
}

impl RemoveMessageFlyweight {
    pub fn new(buffer: AtomicBuffer, offset: Index) -> Self {
        let correlated_message_flyweight = CorrelatedMessageFlyweight::new(buffer, offset);
        let m_struct = correlated_message_flyweight.flyweight.get::<RemoveMessageDefn>(0);
        Self {
            correlated_message_flyweight,
            m_struct,
        }
    }

    #[inline]
    pub fn registration_id(&self) -> i64 {
        self.m_struct.registration_id
    }

    #[inline]
    pub fn set_registration_id(&mut self, value: i64) {
        self.m_struct.registration_id = value;
    }

    #[inline]
    pub const fn length(&self) -> Index {
        REMOVE_MESSAGE_LENGTH
    }
}