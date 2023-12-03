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
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use axum_extra::headers::ContentType;
use axum_extra::TypedHeader;
use prometheus::{Encoder, TextEncoder};
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::settings::Settings;

pub struct PrometheusExporter {}

impl PrometheusExporter {
    async fn metrics() -> Response {
        let encoder = TextEncoder::new();
        let metric_families = prometheus::gather();
        let mut result = vec![];
        let result = match encoder.encode(&metric_families, &mut result) {
            Ok(()) => Ok((TypedHeader(ContentType::text_utf8()), result)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        };
        result.into_response()
    }

    pub async fn start(config: &Settings) -> Result<()> {
        let addr: SocketAddr = config.prometheus.address.parse()?;
        let app = Router::new().route("/metrics", get(PrometheusExporter::metrics));
        let listener = TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}
