/*
 * Copyright 2021 Michael Krolikowski
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use lazy_static::lazy_static;
use prometheus::{IntCounter, opts, register_int_counter};

lazy_static! {
    pub static ref MESSAGES: IntCounter =
        register_int_counter!(opts!("messages", "Number of messages received.")).unwrap();
}

pub fn initialize_metrics() {
    MESSAGES.reset();
}
