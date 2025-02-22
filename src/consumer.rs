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

use anyhow::Result;
use rdkafka::ClientConfig;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{Consumer, DefaultConsumerContext, MessageStream, StreamConsumer};

use crate::settings::Settings;

pub struct KafkaConsumer {
    consumer: StreamConsumer,
}

fn kafka_client_config(config: &Settings) -> ClientConfig {
    let mut client_config = ClientConfig::default();
    client_config.set_log_level(RDKafkaLogLevel::Debug);
    client_config.set("auto.offset.reset", "earliest");
    client_config.set("enable.auto.commit", "true");
    client_config.set("enable.auto.offset.store", "false");
    for (k, v) in config.kafka.iter() {
        client_config.set(k, v);
    }
    client_config
}

impl KafkaConsumer {
    pub fn new(config: &Settings) -> Result<KafkaConsumer> {
        let consumer: StreamConsumer<DefaultConsumerContext> =
            kafka_client_config(config).create()?;
        let topics: Vec<&str> = config.topics.iter().map(|s| s.as_str()).collect();
        consumer.subscribe(&topics)?;
        Ok(KafkaConsumer { consumer })
    }

    pub fn start(&self) -> MessageStream<'_, DefaultConsumerContext> {
        self.consumer.stream()
    }
}

impl<'a> From<&'a KafkaConsumer> for &'a StreamConsumer {
    fn from(kc: &'a KafkaConsumer) -> Self {
        &kc.consumer
    }
}
