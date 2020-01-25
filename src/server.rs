/*
 * Copyright 2020 Google LLC All Rights Reserved.
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

use std::io::Error;

use slog::{info, o, Logger};

use crate::config::{Config, ConnectionConfig};

/// Server is the UDP server main implementation
pub struct Server {
    log: Logger,
}

impl Server {
    pub fn new(base: Logger) -> Self {
        let log = base.new(o!("source" => "server::Server"));
        return Server { log };
    }

    pub async fn run(self, config: Config) -> Result<(), Error> {
        let Server { log } = self;
        info!(log, "Starting!");
        match config.connections {
            ConnectionConfig::Sender { address, .. } => {
                info!(log, "Sender configuration"; "address" => address)
            }
            ConnectionConfig::Receiver { endpoints } => {
                info!(log, "Receiver configuration"; "endpoints" => endpoints.len())
            }
        };

        return Ok(());
    }
}