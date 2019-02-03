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

//! Error types for libwallet

use failure::{Backtrace, Context, Fail};
use std::env;
use std::fmt::{self, Display};

/// Error definition
#[derive(Debug, Fail)]
pub struct Error {
	inner: Context<ErrorKind>,
}

/// Wallet errors, mostly wrappers around underlying crypto or I/O errors.
#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
	/// Not enough funds
	#[fail(
		display = "Not enough funds. Required: {}, Available: {}",
		needed_disp, available_disp
	)]
	NotEnoughFunds {
		/// available funds
		available: u64,
		/// Display friendly
		available_disp: String,
		/// Needed funds
		needed: u64,
		/// Display friendly
		needed_disp: String,
	},

	/// Fee dispute
	#[fail(
		display = "Fee dispute: sender fee {}, recipient fee {}",
		sender_fee_disp, recipient_fee_disp
	)]
	FeeDispute {
		/// sender fee
		sender_fee: u64,
		/// display friendly
		sender_fee_disp: String,
		/// recipient fee
		recipient_fee: u64,
		/// display friendly
		recipient_fee_disp: String,
	},

	/// Fee Exceeds amount
	#[fail(
		display = "Fee exceeds amount: sender amount {}, recipient fee {}",
		sender_amount_disp, recipient_fee
	)]
	FeeExceedsAmount {
		/// sender amount
		sender_amount: u64,
		/// display friendly
		sender_amount_disp: String,
		/// recipient fee
		recipient_fee: u64,
		/// display friendly
		recipient_fee_disp: String,
	},

	/// API Error
	#[fail(display = "Client Callback Error: {}", _0)]
	ClientCallback(String),

	/// Secp Error
	#[fail(display = "Secp error")]
	Secp,

	/// Callback implementation error conversion
	#[fail(display = "Trait Implementation error")]
	CallbackImpl(&'static str),

	/// Wallet backend error
	#[fail(display = "Wallet store error: {}", _0)]
	Backend(String),

	/// Callback implementation error conversion
	#[fail(display = "Restore Error")]
	Restore,

	/// An error in the format of the JSON structures exchanged by the wallet
	#[fail(display = "JSON format error: {}", _0)]
	Format(String),

	/// Error when contacting a node through its API
	#[fail(display = "Node API error")]
	Node,

	/// Error contacting wallet API
	#[fail(display = "Wallet Communication Error: {}", _0)]
	WalletComms(String),

	/// Error originating from hyper.
	#[fail(display = "Hyper error")]
	Hyper,

	/// Error originating from hyper uri parsing.
	#[fail(display = "Uri parsing error")]
	Uri,

	/// Signature error
	#[fail(display = "Signature error")]
	Signature(&'static str),

	/// Attempt to use duplicate transaction id in separate transactions
	#[fail(display = "Duplicate transaction ID error")]
	DuplicateTransactionId,

	/// Wallet seed already exists
	#[fail(display = "Wallet seed exists error")]
	WalletSeedExists,

	/// Wallet seed doesn't exist
	#[fail(display = "Wallet seed doesn't exist error")]
	WalletSeedDoesntExist,

	/// Wallet seed doesn't exist
	#[fail(display = "Wallet seed decryption error")]
	WalletSeedDecryption,

	/// Transaction doesn't exist
	#[fail(display = "Transaction {} doesn't exist", _0)]
	TransactionDoesntExist(String),

	/// Transaction already rolled back
	#[fail(display = "Transaction {} cannot be cancelled", _0)]
	TransactionNotCancellable(String),

	/// Cancellation error
	#[fail(display = "Cancellation Error: {}", _0)]
	TransactionCancellationError(&'static str),

	/// Cancellation error
	#[fail(display = "Tx dump Error: {}", _0)]
	TransactionDumpError(&'static str),

	/// Attempt to repost a transaction that's already confirmed
	#[fail(display = "Transaction already confirmed error")]
	TransactionAlreadyConfirmed,

	/// Transaction has already been received
	#[fail(display = "Transaction {} has already been received", _0)]
	TransactionAlreadyReceived(String),

	/// Attempt to repost a transaction that's not completed and stored
	#[fail(display = "Transaction building not completed: {}", _0)]
	TransactionBuildingNotCompleted(u32),

	/// Invalid BIP-32 Depth
	#[fail(display = "Invalid BIP32 Depth (must be 1 or greater)")]
	InvalidBIP32Depth,

	/// Attempt to add an account that exists
	#[fail(display = "Account Label '{}' already exists", _0)]
	AccountLabelAlreadyExists(String),

	/// Cannot send slate
	#[fail(display = "Cannot send slate")]
	SlateSendError,

	/// Cannot receive slate
	#[fail(display = "Cannot receive slate")]
	SlateReceiveError,

	/// Cannot finalize slate
	#[fail(display = "Cannot finalize slate")]
	SlateFinalizeError,

	/// Cannot add output to slate
	#[fail(display = "Cannot add output to slate")]
	SlateAddOutputError,

	/// Cannot add input to slate
	#[fail(display = "Cannot add input to slate")]
	SlateAddInputError,

	/// Cannot verify slate message
	#[fail(display = "Cannot verify slate message")]
	SlateMessageVerificationError,

	/// Cannot get stored transaction
	#[fail(display = "Cannot get stored transaction")]
	CannotGetStoredTransaction,

	/// Cannot store transaction
	#[fail(display = "Cannot store transaction")]
	CannotStoreTransaction,

	/// Cannot save transaction in file
	#[fail(display = "Cannot save transaction in file")]
	CannotDumpTransaction,

	/// Cannot derive key
	#[fail(display = "Cannot derive key")]
	CannotDeriveKeys,

	/// Cannot restore wallet
	#[fail(display = "Cannot restore wallet")]
	WalletRestoreError,

	/// Reference unknown account label
	#[fail(display = "Unknown Account Label '{}'", _0)]
	UnknownAccountLabel(String),

	/// Other
	#[fail(display = "Generic error: {}", _0)]
	GenericError(String),
}

impl Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let show_bt = match env::var("RUST_BACKTRACE") {
			Ok(r) => {
				if r == "1" {
					true
				} else {
					false
				}
			}
			Err(_) => false,
		};
		let backtrace = match self.backtrace() {
			Some(b) => format!("{}", b),
			None => String::from("Unknown"),
		};
		let inner_output = format!("{}", self.inner,);
		let backtrace_output = format!("\n Backtrace: {}", backtrace);
		let mut output = inner_output.clone();
		if show_bt {
			output.push_str(&backtrace_output);
		}
		Display::fmt(&output, f)
	}
}

impl Error {
	/// get kind
	pub fn kind(&self) -> ErrorKind {
		self.inner.get_context().clone()
	}
	/// get cause string
	pub fn cause_string(&self) -> String {
		match self.cause() {
			Some(k) => format!("{}", k),
			None => format!("Unknown"),
		}
	}
	/// get cause
	pub fn cause(&self) -> Option<&dyn Fail> {
		self.inner.cause()
	}
	/// get backtrace
	pub fn backtrace(&self) -> Option<&Backtrace> {
		self.inner.backtrace()
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
