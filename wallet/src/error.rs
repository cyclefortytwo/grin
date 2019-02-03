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

//! Implementation specific error types
use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};

/// Error definition
#[derive(Debug)]
pub struct Error {
	pub inner: Context<ErrorKind>,
}

/// Wallet errors, mostly wrappers around underlying crypto or I/O errors.
#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
	/// Secp Error
	#[fail(display = "Secp error")]
	Secp,

	/// Filewallet error
	#[fail(display = "Wallet data error: {}", _0)]
	FileWallet(&'static str),

	/// Error when formatting json
	#[fail(display = "IO error")]
	IO,

	/// Error when formatting json
	#[fail(display = "Serde JSON error")]
	Format,

	/// Error originating from hyper.
	#[fail(display = "Hyper error")]
	Hyper,

	/// Error originating from hyper uri parsing.
	#[fail(display = "Uri parsing error")]
	Uri,

	/// Attempt to use duplicate transaction id in separate transactions
	#[fail(display = "Duplicate transaction ID error")]
	DuplicateTransactionId,

	/// Wallet seed already exists
	#[fail(display = "Wallet seed file exists: {}", _0)]
	WalletSeedExists(String),

	/// Wallet seed doesn't exist
	#[fail(display = "Wallet seed doesn't exist error")]
	WalletSeedDoesntExist,

	/// Enc/Decryption Error
	#[fail(display = "Enc/Decryption error (check password?)")]
	Encryption,

	/// BIP 39 word list
	#[fail(display = "BIP39 Mnemonic (word list) Error")]
	Mnemonic,

	#[fail(display = "Cannot instantiate walllet")]
	CannotInstantiateWalllet,

	#[fail(display = "Cannot start listening")]
	ListenError,

	#[fail(display = "Account error")]
	AccountError(String),

	#[fail(display = "Cannot init wallet")]
	CannotInitWallet,

	#[fail(display = "Cannot send slate")]
	CannotSendSlate,

	#[fail(display = "Cannot send invoice")]
	CannotSendInvoice,

	#[fail(display = "Cannot receive slate")]
	CannotReceiveSlate,

	#[fail(display = "Cannot finalize slate")]
	CannotFinalizeSlate,

	#[fail(display = "Cannot derive keychain")]
	CannotDeriveKeychain,

	#[fail(display = "Cannot repair wallet")]
	WalletRepairError,

	#[fail(display = "Cannot restore wallet")]
	WalletRestoreError,

	#[fail(display = "Cannot cancel transaction")]
	CannotCancelTransaction,

	#[fail(display = "Cannot repost transaction")]
	CannotRepostTransaction,

	#[fail(display = "Cannot get transaction list")]
	CannotGetTransactions,

	#[fail(display = "Cannot get outputs list")]
	CannotGetOutputs,

	#[fail(display = "Cannot get info")]
	CannotGetInfo,

	/// Command line argument error
	#[fail(display = "{}", _0)]
	ArgumentError(String),

	/// Other
	#[fail(display = "Generic error: {}", _0)]
	GenericError(String),
}

impl Fail for Error {
	fn cause(&self) -> Option<&dyn Fail> {
		self.inner.cause()
	}

	fn backtrace(&self) -> Option<&Backtrace> {
		self.inner.backtrace()
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		Display::fmt(&self.inner, f)
	}
}

impl Error {
	/// get kind
	pub fn kind(&self) -> ErrorKind {
		self.inner.get_context().clone()
	}
}

impl From<ErrorKind> for Error {
	fn from(kind: ErrorKind) -> Error {
		Error {
			inner: Context::new(kind),
		}
	}
}

impl From<Context<ErrorKind>> for Error {
	fn from(inner: Context<ErrorKind>) -> Error {
		Error { inner: inner }
	}
}

impl From<Context<String>> for Error {
	fn from(inner: Context<String>) -> Error {
		ErrorKind::GenericError(inner.get_context().clone()).into()
	}
}
