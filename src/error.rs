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

use thiserror::Error;

#[derive(Error, Debug)]
pub enum KafkaRocksDBError {
    #[error("syslog failed")]
    SyslogError(#[from] syslog::Error),
    #[error("failed to set logger")]
    SetLoggerError(#[from] log::SetLoggerError),
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("failed to parse the configuration")]
    ConfigError(#[from] config::ConfigError),
    #[error("Kafka error")]
    KafkaError(#[from] rdkafka::error::KafkaError),
    #[error("RocksDB error")]
    RocksDBError(#[from] rocksdb::Error),
    #[error("RocksDB column family not found")]
    ColumnFamilyNotFound(String),
    #[error("failed to parse address")]
    AddressParseError(#[from] std::net::AddrParseError),
    #[error("Hyper error")]
    HyperError(#[from] hyper::Error),
    #[cfg(feature = "schema_registry")]
    #[error("Reqwest Error")]
    ReqwestError(#[from] reqwest::Error),
    #[cfg(feature = "schema_registry")]
    #[error("Schema Registry Error")]
    SchemaRegistryError(schema_registry_converter::error::SRCError),
}

pub type KafkaRocksDBResult<T> = Result<T, KafkaRocksDBError>;
