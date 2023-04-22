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
use hyper::header::CONTENT_TYPE;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use prometheus::{Encoder, TextEncoder};
use std::error::Error;

use crate::settings::Settings;

type GenericError = Box<dyn Error + Send + Sync>;
type GenericResult<T> = std::result::Result<T, GenericError>;

pub struct PrometheusExporter {}

impl PrometheusExporter {
    fn metrics() -> GenericResult<Response<Body>> {
        let encoder = TextEncoder::new();
        let metric_families = prometheus::gather();
        let mut buffer = vec![];
        encoder.encode(&metric_families, &mut buffer)?;

        let response = Response::builder()
            .status(200)
            .header(CONTENT_TYPE, encoder.format_type())
            .body(Body::from(buffer))?;

        Ok(response)
    }

    fn not_found() -> GenericResult<Response<Body>> {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Not found".into())?)
    }

    async fn routes(req: Request<Body>) -> GenericResult<Response<Body>> {
        Ok(match (req.method(), req.uri().path()) {
            (&Method::GET, "/metrics") => PrometheusExporter::metrics()?,
            _ => PrometheusExporter::not_found()?,
        })
    }

    pub async fn start(config: &Settings) -> Result<()> {
        let addr = config.prometheus.address.parse()?;
        let service = make_service_fn(|_| async {
            Ok::<_, GenericError>(service_fn(PrometheusExporter::routes))
        });
        let server = Server::bind(&addr).serve(service);
        Ok(server.await?)
    }
}
