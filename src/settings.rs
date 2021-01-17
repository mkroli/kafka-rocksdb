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

use serde::Deserialize;
use crate::error::KafkaRocksDBResult;
use config::FileFormat;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct RocksDBSettings {
    pub directory: String,
}

#[derive(Debug, Deserialize)]
pub struct PrometheusExporterSettings {
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub topics: Vec<String>,
    pub kafka: BTreeMap<String, String>,
    pub rocksdb: RocksDBSettings,
    pub prometheus: PrometheusExporterSettings,
}

impl Settings {
    fn delete_kafka_environment_settings() -> BTreeMap<String, String> {
        let mut kafka_settings = BTreeMap::new();
        for (ref k, v) in std::env::vars() {
            if let Some(key) = k.strip_prefix("KR_KAFKA_") {
                std::env::remove_var(k);
                let key = key.replace("_", ".").to_lowercase();
                kafka_settings.insert(key, v);
            }
        }
        kafka_settings
    }

    fn override_kafka_settings(&mut self, kafka_settings: BTreeMap<String, String>) {
        for (k, v) in kafka_settings {
            self.kafka.insert(k, v);
        }
    }

    pub fn read(filename: &str) -> KafkaRocksDBResult<Settings> {
        let kafka_settings = Settings::delete_kafka_environment_settings();
        let mut config = config::Config::default();
        config.merge(config::File::with_name(filename).format(FileFormat::Toml).required(false))?;
        config.merge(config::Environment::with_prefix("KR").separator("_"))?;
        let mut settings: Settings = config.try_into()?;
        settings.override_kafka_settings(kafka_settings);
        Ok(settings)
    }
}
