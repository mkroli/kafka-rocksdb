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

use std::pin::Pin;

use futures::Stream;
use futures::task::{Context, Poll};
use pin_project::pin_project;
use rdkafka::Message;
use rdkafka::consumer::Consumer;
use rdkafka::consumer::StreamConsumer;
use rdkafka::error::KafkaError;
use rdkafka::message::BorrowedMessage;

#[pin_project]
pub struct StoreOffsets<'a, T>
where
    T: Stream,
{
    #[pin]
    stream: T,
    consumer: &'a StreamConsumer,
}

#[pin_project]
pub struct TryStoreOffsets<'a, T>
where
    T: Stream,
{
    #[pin]
    stream: T,
    consumer: &'a StreamConsumer,
}

macro_rules! poll_store_offsets {
    ( $( $return_error:expr )* ) => {
        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            let this = self.project();
            let polled = this.stream.poll_next(cx);
            if let Poll::Ready(Some(Ok(ref msg))) = polled {
                if let Err(e) = this.consumer.store_offset(msg.topic(), msg.partition(), msg.offset()) {
                    log::warn!("Failed to store offset: {}", e);
                    $(
                        $return_error
                        return Poll::Ready(Some(Err(e.into())))
                    )*
                }
            }
            polled
        }
    };
}

impl<'a, T, E> Stream for StoreOffsets<'a, T>
where
    T: Stream<Item = Result<BorrowedMessage<'a>, E>>,
    E: From<KafkaError>,
{
    type Item = T::Item;

    poll_store_offsets!({});
}

impl<'a, T, E> Stream for TryStoreOffsets<'a, T>
where
    T: Stream<Item = Result<BorrowedMessage<'a>, E>>,
{
    type Item = T::Item;

    poll_store_offsets!();
}

impl<'a, S: ?Sized, E> KafkaStreamExt<'a> for S where
    S: Stream<Item = Result<BorrowedMessage<'a>, E>>
{
}

pub trait KafkaStreamExt<'a>: Stream {
    fn try_store_offsets<C>(self, consumer: C) -> TryStoreOffsets<'a, Self>
    where
        Self: Sized,
        C: Into<&'a StreamConsumer>,
    {
        TryStoreOffsets {
            consumer: consumer.into(),
            stream: self,
        }
    }
}
