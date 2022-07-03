// Licensed to the LF AI & Data foundation under one
// or more contributor license agreements. See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership. The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::error::{Error, Result};
use crate::proto::milvus::milvus_service_client::MilvusServiceClient;
use tonic::codegen::StdError;
use tonic::transport::Channel;

pub struct Client {
    client: MilvusServiceClient<Channel>,
}

impl Client {
    pub async fn new<D>(dst: D) -> Result<Self>
    where
        D: std::convert::TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        match MilvusServiceClient::connect(dst).await {
            Ok(i) => Ok(Self { client: i }),
            Err(e) => Err(Error::Grpc(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    use crate::error::Result;

    #[tokio::test]
    async fn test_create_client() -> Result<()> {
        const URL: &str = "http://localhost:19530";
        match Client::new(URL).await {
            Ok(_) => return Result::<()>::Ok(()),
            Err(e) => panic!("Error is {}.", e),
        }
    }
    #[tokio::test]
    async fn test_create_client_wrong_url() -> Result<()> {
        const URL: &str = "http://localhost:9999";
        match Client::new(URL).await {
            Ok(_) => panic!("Should fail due to wrong url."),
            Err(_) => return Result::<()>::Ok(()),
        }
    }
    #[tokio::test]
    async fn test_create_client_wrong_fmt() -> Result<()> {
        const URL: &str = "9999";
        match Client::new(URL).await {
            Ok(_) => panic!("Should fail due to wrong format url."),
            Err(_) => return Result::<()>::Ok(()),
        }
    }
}
