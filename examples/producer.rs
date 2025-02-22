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

use anyhow::{Result, anyhow};
use apache_avro::types::Value;
use clap::{ArgGroup, Parser};
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use schema_registry_converter::blocking::avro::AvroEncoder;
use schema_registry_converter::blocking::schema_registry::SrSettings;
use schema_registry_converter::schema_registry_common::SubjectNameStrategy;
use tokio::time::Duration;

use kafka_rocksdb::logging::setup_logger;

#[derive(Parser, Debug)]
#[clap(author, about, version, group = ArgGroup::new("output"))]
struct CommandLineOptions {
    #[clap(long, short = 'k')]
    kafka_bootstrap_servers: String,
    #[clap(long, short = 't')]
    kafka_topic: String,
    #[clap(group = "output", name = "binary", long, help = "Output as Hex")]
    output_binary: bool,
    #[clap(group = "output", name = "text", long, help = "Output as Text")]
    output_text: bool,
    #[clap(group = "output", name = "avro", long, help = "Output as Avro")]
    output_avro: Option<String>,
}

impl CommandLineOptions {
    fn register_schemas(&self, url: &str) -> Result<()> {
        let client = reqwest::blocking::Client::new();
        for kv in vec!["key", "value"] {
            client
                .post(&format!("{}/subjects/{}-{}/versions", url, self.kafka_topic, kv))
                .header("Content-Type", "application/vnd.schemaregistry.v1+json")
                .body(r#"{"schema": "{\"type\": \"record\", \"name\": \"test\", \"fields\": [{\"name\": \"key\", \"type\": \"string\"}]}"}"#)
                .send()?;
        }
        Ok(())
    }

    async fn output(&self, topic: &str, key: bool) -> Result<Vec<u8>> {
        if self.output_text {
            Ok("test".as_bytes().into())
        } else if let Some(ref url) = self.output_avro {
            self.register_schemas(url)?;
            let sr_settings = SrSettings::new(url.clone());
            let encoder = AvroEncoder::new(sr_settings);
            let strategy = SubjectNameStrategy::TopicNameStrategy(String::from(topic), key);
            let bytes = encoder
                .encode(
                    vec![("key", Value::String(String::from("value")))],
                    &strategy,
                )
                .map_err(|e| anyhow!("{e}"))?;
            Ok(bytes)
        } else {
            Ok(vec![1, 2, 3])
        }
    }
}

async fn produce(opts: CommandLineOptions) -> Result<()> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &opts.kafka_bootstrap_servers)
        .create()?;
    let key = opts.output(&opts.kafka_topic, true).await?;
    let value = opts.output(&opts.kafka_topic, false).await?;
    let record = FutureRecord::to(&opts.kafka_topic)
        .key(&key)
        .payload(&value);
    producer
        .send(record, Duration::from_secs(5))
        .await
        .map_err(|e| e.0)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = CommandLineOptions::parse();
    setup_logger()?;
    produce(opts).await
}
