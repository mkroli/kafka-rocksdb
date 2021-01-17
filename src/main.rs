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

extern crate clap;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;

use clap::Clap;
use futures::{future::FutureExt, pin_mut, select};

use crate::error::KafkaRocksDBResult;
use crate::logging::setup_logger;
use crate::prometheus_exporter::PrometheusExporter;
use crate::settings::Settings;

mod consumer;
mod database;
mod error;
mod kafka_rocksdb;
mod kafka_stream_ext;
mod logging;
mod metrics;
mod prometheus_exporter;
mod settings;

#[derive(Clap, Debug)]
#[clap(author, about, version)]
struct CommandLineOptions {
    #[clap(short = 'l', long = "log-to-stdout")]
    log_to_stdout: bool,
    #[clap(
        value_name = "configuration file",
        long_about = "Configuration file to use",
        required = true
    )]
    config_file: String,
}

#[tokio::main]
async fn main() -> KafkaRocksDBResult<()> {
    let opts = CommandLineOptions::parse();
    setup_logger(opts.log_to_stdout)?;
    let settings = Settings::read(&opts.config_file)?;

    metrics::initialize_metrics();
    let prometheus = PrometheusExporter::start(&settings).fuse();

    let kafka_rocksdb = kafka_rocksdb::KafkaRocksDB::new(&settings)?;
    let kafka_rocksdb = kafka_rocksdb.start().fuse();

    pin_mut!(prometheus, kafka_rocksdb);
    select!(
        result = prometheus => result?,
        result = kafka_rocksdb => result?,
    );
    Ok(())
}
