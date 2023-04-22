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

use anyhow::{anyhow, Result};
use clap::{ArgGroup, Parser};
use hex_slice::AsHex;
use rocksdb::{IteratorMode, Options, DB};

use kafka_rocksdb::logging::setup_logger;
use schema_registry_converter::blocking::avro::AvroDecoder;
use schema_registry_converter::blocking::schema_registry::SrSettings;

#[derive(Parser, Debug)]
#[clap(author, about, version, group = ArgGroup::new("output"))]
struct CommandLineOptions {
    #[clap(group = "output", name = "binary", long, short, help = "Output as Hex")]
    output_binary: bool,
    #[clap(group = "output", name = "text", long, short, help = "Output as Text")]
    output_text: bool,
    #[clap(group = "output", name = "avro", long, short, help = "Output as Avro")]
    output_avro: Option<String>,
    #[clap(
        value_name = "database directory",
        help = "RocksDB database directory",
        required = true
    )]
    db_directory: String,
}

impl CommandLineOptions {
    fn output(&self, data: &[u8]) -> Result<String> {
        if self.output_text {
            Ok(String::from_utf8_lossy(data).to_string())
        } else if let Some(ref url) = self.output_avro {
            let sr_settings = SrSettings::new(url.clone());
            let decoder = AvroDecoder::new(sr_settings);
            let result = decoder.decode(Some(data)).map_err(|e| anyhow!("{e}"))?;
            Ok(format!("{:?}", result.value))
        } else {
            Ok(format!("{:02X}", data.plain_hex(false)))
        }
    }
}

fn list_db(options: CommandLineOptions) -> Result<()> {
    let db_options = Options::default();
    let cfs = DB::list_cf(&db_options, &options.db_directory)?;
    let db = DB::open_cf_as_secondary(
        &db_options,
        &options.db_directory,
        &options.db_directory,
        &cfs,
    )?;
    db.try_catch_up_with_primary()?;
    for cf in cfs {
        println!("ColumnFamily: {}", &cf);
        let cfh = db.cf_handle(&cf).unwrap();
        for row in db.iterator_cf(cfh, IteratorMode::Start) {
            match row {
                Ok((k, v)) => println!("{}: {}", options.output(&k)?, options.output(&v)?),
                Err(e) => println!("Failed to read row: {e}"),
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = CommandLineOptions::parse();
    setup_logger()?;
    Ok(list_db(opts)?)
}
