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
use futures::StreamExt;
use rdkafka::Message;

use crate::consumer::KafkaConsumer;
use crate::database::Database;
use crate::kafka_stream_ext::KafkaStreamExt;
use crate::settings::Settings;
use crate::stream_signal_ext::StreamSignalExt;

pub struct KafkaRocksDB {
    consumer: KafkaConsumer,
    db: Database,
}

impl KafkaRocksDB {
    pub fn new(config: &Settings) -> Result<KafkaRocksDB> {
        let consumer = KafkaConsumer::new(config)?;
        let db = Database::new(config)?;
        Ok(KafkaRocksDB { consumer, db })
    }

    pub async fn start(&self) -> Result<()> {
        self.consumer
            .start()
            .map(|msg| {
                let msg = msg?;
                crate::metrics::MESSAGES.inc();
                if let Some(key) = msg.key() {
                    match self.db.update(msg.topic(), key, msg.payload()) {
                        Ok(_) => Ok(msg),
                        Err(e) => {
                            log::error!("Failed to update RocksDB: {}", e);
                            Err(e)
                        }
                    }
                } else {
                    log::error!("Ignoring message without key: {:?}", msg);
                    Ok(msg)
                }
            })
            .try_store_offsets(&self.consumer)
            .until_termination()
            .for_each(|_| async {})
            .await;
        Ok(())
    }
}

impl Drop for KafkaRocksDB {
    fn drop(&mut self) {
        log::info!("Shutting down");
    }
}
