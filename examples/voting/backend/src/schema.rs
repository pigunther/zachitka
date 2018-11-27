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

//! Voting database schema.

use exonum::{
    crypto::{Hash, PublicKey}, storage::{Fork, ProofListIndex, ProofMapIndex, Snapshot},
};

use wallet::Wallet;
use INITIAL_BALANCE;
use INITIAL_VOTES;


/// Database schema for the voting.
#[derive(Debug)]
pub struct Schema<T> {
    view: T,
}

impl<T> AsMut<T> for Schema<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.view
    }
}

impl<T> Schema<T>
where
    T: AsRef<dyn Snapshot>,
{
    /// Creates a new schema from the database view.
    pub fn new(view: T) -> Self {
        Schema { view }
    }

    /// Returns `ProofMapIndex` with wallets.
    pub fn wallets(&self) -> ProofMapIndex<&T, PublicKey, Wallet> {
        ProofMapIndex::new("voting.wallets", &self.view)
    }

    /// Returns history of the wallet with the given public key.
    pub fn wallet_history(&self, public_key: &PublicKey) -> ProofListIndex<&T, Hash> {
        ProofListIndex::new_in_family("voting.wallet_history", public_key, &self.view)
    }

    /// Returns wallet for the given public key.
    pub fn wallet(&self, pub_key: &PublicKey) -> Option<Wallet> {
        self.wallets().get(pub_key)
    }

    /// Returns the state hash of voting service.
    pub fn state_hash(&self) -> Vec<Hash> {
        vec![self.wallets().merkle_root()]
    }
}

/// Implementation of mutable methods.
impl<'a> Schema<&'a mut Fork> {
    /// Returns mutable `ProofMapIndex` with wallets.
    pub fn wallets_mut(&mut self) -> ProofMapIndex<&mut Fork, PublicKey, Wallet> {
        ProofMapIndex::new("voting.wallets", &mut self.view)
    }

    /// Returns history for the wallet by the given public key.
    pub fn wallet_history_mut(
        &mut self,
        public_key: &PublicKey,
    ) -> ProofListIndex<&mut Fork, Hash> {
        ProofListIndex::new_in_family("voting.wallet_history", public_key, &mut self.view)
    }

    /// Increase votes of the wallet and append new record to its history.
    ///
    /// Panics if there is no wallet with given public key.
    pub fn increase_wallet_votes(&mut self, wallet: Wallet, transaction: &Hash) {
        let wallet = {
            let mut history = self.wallet_history_mut(wallet.pub_key());
            history.push(*transaction);
            let history_hash = history.merkle_root();
            let votes = wallet.votes();
            wallet.set_vote(votes + INITIAL_BALANCE, &history_hash)
        };
        self.wallets_mut().put(wallet.pub_key(), wallet.clone());
    }

    /// Decrease balance of the wallet and append new record to its history.
    /// Means that voter voted.
    /// Panics if there is no wallet with given public key.
    pub fn decrease_wallet_balance(&mut self, wallet: Wallet/*, amount: u64*/, transaction: &Hash) {
        let wallet = {
            let mut history = self.wallet_history_mut(wallet.pub_key());
            history.push(*transaction);
            let history_hash = history.merkle_root();
            wallet.set_balance(0, &history_hash)
        };
        self.wallets_mut().put(wallet.pub_key(), wallet.clone());
    }

    /// Create new wallet and append first record to its history.
    pub fn create_wallet(&mut self, key: &PublicKey, name: &str, cand: u64, transaction: &Hash) {
        let wallet = {
            let mut history = self.wallet_history_mut(key);
            history.push(*transaction);
            let history_hash = history.merkle_root();
            Wallet::new(key, name, INITIAL_BALANCE, cand, INITIAL_VOTES, history.len(), &history_hash)
        };
		println!("Create the wallet: {:?}", wallet);
		println!("transaction: {:?}", transaction);
        self.wallets_mut().put(key, wallet);
    }
}
