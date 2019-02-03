// Copyright 2018 The Grin Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// File Output 'plugin' implementation
use std::fs::File;
use std::io::{Read, Write};

use crate::core::libtx::slate::Slate;
use crate::libwallet::{Error, ErrorKind};
use crate::{WalletCommAdapter, WalletConfig};
use serde_json as json;
use std::collections::HashMap;

use failure::ResultExt;

#[derive(Clone)]
pub struct FileWalletCommAdapter {}

impl FileWalletCommAdapter {
	/// Create
	pub fn new() -> Box<dyn WalletCommAdapter> {
		Box::new(FileWalletCommAdapter {})
	}
}

impl WalletCommAdapter for FileWalletCommAdapter {
	fn supports_sync(&self) -> bool {
		false
	}

	fn send_tx_sync(&self, _dest: &str, _slate: &Slate) -> Result<Slate, Error> {
		unimplemented!();
	}

	fn send_tx_async(&self, dest: &str, slate: &Slate) -> Result<(), Error> {
		let mut pub_tx = File::create(dest).context(ErrorKind::SlateSendError)?;
		pub_tx
			.write_all(json::to_string(&slate).unwrap().as_bytes())
			.context(ErrorKind::SlateSendError)?;
		pub_tx.sync_all().context(ErrorKind::SlateSendError)?;
		Ok(())
	}
	/// Cannot send slate
	fn receive_tx_async(&self, params: &str) -> Result<Slate, Error> {
		let mut pub_tx_f = File::open(params).context(ErrorKind::SlateSendError)?;
		let mut content = String::new();
		pub_tx_f
			.read_to_string(&mut content)
			.context(ErrorKind::SlateSendError)?;
		Ok(json::from_str(&content).map_err(|err| ErrorKind::Format(err.to_string()))?)
	}

	fn listen(
		&self,
		_params: HashMap<String, String>,
		_config: WalletConfig,
		_passphrase: &str,
		_account: &str,
		_node_api_secret: Option<String>,
	) -> Result<(), Error> {
		unimplemented!();
	}
}
