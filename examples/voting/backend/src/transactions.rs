// Copyright 2018 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Voting transactions.

// Workaround for `failure` see https://github.com/rust-lang-nursery/failure/issues/223 and
// ECR-1771 for the details.
#![allow(bare_trait_objects)]

use exonum::{
    blockchain::{ExecutionError, ExecutionResult, Transaction}, crypto::{CryptoHash, PublicKey},
    messages::Message, storage::Fork,
};

use schema::Schema;
use VOTING_SERVICE_ID;
use INITIAL_BALANCE;

/// Error codes emitted by wallet transactions during execution.
#[derive(Debug, Fail)]
#[repr(u8)]
pub enum Error {
    /// Wallet already exists.
    ///
    /// Can be emitted by `CreateWallet`.
    #[fail(display = "Wallet already exists")]
    WalletAlreadyExists = 0,

    /// Sender doesn't exist.
    ///
    /// Can be emitted by `Transfer`.
    #[fail(display = "Sender doesn't exist")]
    SenderNotFound = 1,

    /// Receiver doesn't exist.
    ///
    /// Can be emitted by `Transfer` or `Issue`.
    #[fail(display = "Receiver doesn't exist")]
    ReceiverNotFound = 2,

    /// Insufficient currency amount.
    ///
    /// Can be emitted by `Transfer`.
    #[fail(display = "Insufficient currency amount")]
    InsufficientCurrencyAmount = 3,
}

impl From<Error> for ExecutionError {
    fn from(value: Error) -> ExecutionError {
        let description = format!("{}", value);
        ExecutionError::with_description(value as u8, description)
    }
}

transactions! {
    /// Transaction group.
    pub WalletTransactions {
        const SERVICE_ID = VOTING_SERVICE_ID;

        /// Transfer `amount` of the currency from one wallet to another.
        struct Transfer {
            /// `PublicKey` of sender's wallet.
            from:    &PublicKey,
            /// `PublicKey` of receiver's wallet.
            to:      &PublicKey,
            ///f
            ended: bool,
            /// Auxiliary number to guarantee [non-idempotence][idempotence] of transactions.
            ///
            /// [idempotence]: https://en.wikipedia.org/wiki/Idempotence
            seed:    u64,
        }

        /// Issue `amount` of the currency to the `wallet`.
        struct Issue {
            /// `PublicKey` of the wallet.
            pub_key:  &PublicKey,
            /// Issued amount of currency.
            amount:  u64,
            /// Auxiliary number to guarantee [non-idempotence][idempotence] of transactions.
            ///
            /// [idempotence]: https://en.wikipedia.org/wiki/Idempotence
            seed:    u64,
        }

        /// Create wallet with the given `name`.
        struct CreateWallet {
            /// `PublicKey` of the new wallet.
            pub_key: &PublicKey,
            /// Name of the new wallet.
            name:    &str,
			/// Candidate
			cand:    u64,
        }
    }
}


impl Transaction for Transfer {
    fn verify(&self) -> bool {
        println!("go to verify Transfer Transaction");
        {
            println!("in verify Transfer Transaction: .{}", self.ended());
            println!("in verify Transfer Transaction: .{}", self.from());
            println!("in verify Transfer Transaction: .{}", self.verify_signature(self.from()));

        }
        self.verify_signature(self.from()) //add ability to transfer to urself
    }

    fn execute(&self, fork: &mut Fork) -> ExecutionResult {
        if self.ended() {
            println!("execute end_process transactoin for transfer");
            let mut schema = Schema::new(fork);

            let hash = self.hash();

            schema.end_process(&hash);

            println!("ok end process");

            Ok(())
        } else {
            println!("execute transaction for transfer");
            let mut schema = Schema::new(fork);

            let from = self.from();
            let to = self.to();
            let hash = self.hash();

            let sender = schema.wallet(from).ok_or(Error::SenderNotFound)?;

            let receiver = schema.wallet(to).ok_or(Error::ReceiverNotFound)?;

            if sender.balance() < INITIAL_BALANCE {
                Err(Error::InsufficientCurrencyAmount)?
            }

            if to != from {
                schema.decrease_wallet_balance(sender, &hash);
                schema.increase_wallet_votes(receiver, &hash);
            } else {
                schema.decrease_wallet_balance_increase_wallet_votes(sender, &hash);
            }

            println!("execute ok");

            Ok(())
        }
    }
}

impl Transaction for Issue {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, fork: &mut Fork) -> ExecutionResult {
        let mut schema = Schema::new(fork);
        let pub_key = self.pub_key();
        let hash = self.hash();

        if let Some(wallet) = schema.wallet(pub_key) {
            schema.increase_wallet_votes(wallet, &hash);
            Ok(())
        } else {
            Err(Error::ReceiverNotFound)?
        }
    }
}

impl Transaction for CreateWallet {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, fork: &mut Fork) -> ExecutionResult {
        let mut schema = Schema::new(fork);
        let pub_key = self.pub_key();
        let hash = self.hash();

        if schema.wallet(pub_key).is_none() {
            let name = self.name();
			let cand = self.cand();
            schema.create_wallet(pub_key, name, cand, &hash);
            Ok(())
        } else {
            Err(Error::WalletAlreadyExists)?
        }
    }
}
