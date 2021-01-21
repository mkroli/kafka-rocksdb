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

use futures::channel::oneshot;
use futures::stream::TakeUntil;
use futures::Stream;
use futures::StreamExt;
use tokio::signal::unix::SignalKind;

use crate::signals::signals;

#[cfg(unix)]
async fn termination_signal() -> std::io::Result<()> {
    signals(&[SignalKind::interrupt(), SignalKind::terminate()])?
        .into_future()
        .await;
    Ok(())
}

#[cfg(windows)]
async fn termination_signal() -> std::io::Result<()> {
    tokio::signal::ctrl_c().await
}

pub trait StreamSignalExt<S: Stream>: Stream + Sized {
    fn until_termination(self) -> TakeUntil<Self, oneshot::Receiver<()>> {
        let (snd, rcv) = oneshot::channel();
        tokio::spawn(async {
            let _ = termination_signal().await;
            let _ = snd.send(());
        });
        self.take_until(rcv)
    }
}

impl<S> StreamSignalExt<S> for S where S: Stream {}
