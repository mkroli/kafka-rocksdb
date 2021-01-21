# kafka-rocksdb
Creates a RocksDB database from Kafka topics.

## Overview
kafka-rocksdb will consume one or more [Kafka](https://kafka.apache.org) topics and update a local [RocksDB](https://rocksdb.org/) from the events.
It will create Column-Families for each topic with the same name.
The message's key will be used as key in RocksDB.
Messages without key will be ignored.
The message's value will be used as value in RocksDB.
An empty (null) value will delete the record in RocksDB.
The RocksDB database can then be used as a [Secondary instance](https://github.com/facebook/rocksdb/wiki/Secondary-instance) by any other application.

## Installation
```shell
cargo install --git https://github.com/mkroli/kafka-rocksdb.git
```

## Configuration
```toml
topics = ["test"]

[kafka]
"group.id" = "kafka_rocksdb"
"bootstrap.servers" = "localhost:9092"

[rocksdb]
"directory" = "./db"

[prometheus]
"address" = "0.0.0.0:9184"
```

## Usage
```
% target/release/kafka-rocksdb --help
kafka-rocksdb 0.2.0
Michael Krolikowski <mkroli@yahoo.de>


USAGE:
    kafka-rocksdb [FLAGS] <configuration file>

ARGS:
    <configuration file>
            Configuration file to use

FLAGS:
    -h, --help
            Prints help information

    -l, --log-to-stdout
            

    -V, --version
            Prints version information
```
