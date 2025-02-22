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

use futures::task::{Context, Poll};
use futures::{Future, Stream, pin_mut};
use tokio::signal::unix::{Signal, SignalKind};

pub struct Signals {
    signals: Vec<Signal>,
}

pub fn signals(signal_kinds: &[SignalKind]) -> std::io::Result<Signals> {
    let mut signals = vec![];
    for sk in signal_kinds {
        let signal = tokio::signal::unix::signal(*sk)?;
        signals.push(signal);
    }
    Ok(Signals { signals })
}

impl Stream for Signals {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        for signal in self.signals.iter_mut() {
            let s = signal.recv();
            pin_mut!(s);
            if s.poll(cx).is_ready() {
                return Poll::Ready(Some(()));
            }
        }
        Poll::Pending
    }
}
