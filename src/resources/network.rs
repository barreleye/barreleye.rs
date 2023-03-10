use chrono::NaiveDateTime as DateTime;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{Barreleye, Blockchain, Endpoint, Env, Response};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkData {
	name: Option<String>,
	env: Option<Env>,
	blockchain: Option<Blockchain>,
	chain_id: Option<u64>,
	block_time_ms: Option<u64>,
	rpc_endpoints: Option<Vec<String>>,
	rps: Option<u32>,
	is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
	name: String,
	env: Env,
	blockchain: Blockchain,
	chain_id: u64,
	block_time_ms: u64,
	rpc_endpoints: Vec<String>,
	rps: u32,
	is_active: bool,
	created_at: DateTime,
}

impl Network {
	pub async fn create(
		client: &Barreleye,
		name: &str,
		env: Env,
		blockchain: Blockchain,
		chain_id: u64,
		block_time_ms: u64,
		rpc_endpoints: Vec<&str>,
		rps: u32,
	) -> Response<Self> {
		client
			.post::<Self>(
				Method::POST,
				Endpoint::Networks,
				json!({
					"name": name.to_string(),
					"env": env,
					"blockchain": blockchain,
					"chainId": chain_id,
					"blockTimeMs": block_time_ms,
					"rpcEndpoints": rpc_endpoints,
					"rps": rps,
				}),
			)
			.await
	}

	pub async fn list(client: &Barreleye) -> Response<Vec<Self>> {
		client.get::<Vec<Self>>(Endpoint::Networks, &[]).await
	}

	pub async fn get(client: &Barreleye, id: &str) -> Response<Self> {
		client.get::<Self>(Endpoint::Network(id.to_string()), &[]).await
	}

	pub async fn update(client: &Barreleye, id: &str, data: NetworkData) -> Response<()> {
		client.post::<()>(Method::PUT, Endpoint::Network(id.to_string()), json!(data)).await
	}

	pub async fn delete(client: &Barreleye, id: &str) -> Response<()> {
		client.post::<()>(Method::DELETE, Endpoint::Network(id.to_string()), json!(null)).await
	}
}
