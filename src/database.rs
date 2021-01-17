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

use crate::error::{KafkaRocksDBError, KafkaRocksDBResult};
use crate::settings::Settings;
use rocksdb::DB;

pub struct Database {
    db: DB,
}

impl Database {
    pub fn new(config: &Settings) -> KafkaRocksDBResult<Database> {
        let mut options = rocksdb::Options::default();
        options.create_if_missing(true);
        options.create_missing_column_families(true);
        let cfs: Vec<rocksdb::ColumnFamilyDescriptor> = config
            .topics
            .iter()
            .map(|t| {
                let options = rocksdb::Options::default();
                rocksdb::ColumnFamilyDescriptor::new(t, options)
            })
            .collect();
        let db = DB::open_cf_descriptors(&options, &config.rocksdb.directory, cfs)?;
        Ok(Database { db })
    }

    pub fn update(&self, topic: &str, key: &[u8], value: Option<&[u8]>) -> KafkaRocksDBResult<()> {
        let cf = self
            .db
            .cf_handle(topic)
            .ok_or_else(|| KafkaRocksDBError::ColumnFamilyNotFound(topic.to_string()))?;
        match value {
            Some(value) => self.db.put_cf(cf, key, value)?,
            None => self.db.delete_cf(cf, key)?,
        }
        Ok(())
    }
}
