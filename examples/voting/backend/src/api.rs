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

use exonum::{
    api::{self, ServiceApiBuilder, ServiceApiState},
    blockchain::{self, BlockProof, Transaction, TransactionSet}, crypto::{Hash, PublicKey},
    helpers::Height, node::TransactionSend, storage::{ListProof, MapProof},
};

use transactions::WalletTransactions;
use wallet::Wallet;
use {Schema, VOTING_SERVICE_ID};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::Write;

/// Describes the query parameters for the `get_wallet` endpoint.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct WalletQuery {
    /// Public key of the queried wallet.
    pub pub_key: PublicKey,
}

/// Response to an incoming transaction returned by the REST API.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    /// Hash of the transaction.
    pub tx_hash: Hash,
}

/// Proof of existence for specific wallet.
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletProof {
    /// Proof of the whole database table.
    pub to_table: MapProof<Hash, Hash>,
    /// Proof of the specific wallet in this table.
    pub to_wallet: MapProof<PublicKey, Wallet>,
}

/// Wallet history.
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletHistory {
    /// Proof of the list of transaction hashes.
    pub proof: ListProof<Hash>,
    /// List of above transactions.
    pub transactions: Vec<WalletTransactions>,
}

/// Wallet information.
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletInfo {
    /// Proof of the last block.
    pub block_proof: BlockProof,
    /// Proof of the appropriate wallet.
    pub wallet_proof: WalletProof,
    /// History of the appropriate wallet.
    pub wallet_history: Option<WalletHistory>,
    /// Candidates.
    pub candidates: Vec<WalletProof>, //Vec<MapProof<PublicKey, Wallet>>,
    pub winner: String
}

/// Public service API description.
#[derive(Debug, Clone, Copy)]
pub struct PublicApi;

impl PublicApi {
    /// Endpoint for getting a single wallet.
    pub fn wallet_info(state: &ServiceApiState, query: WalletQuery) -> api::Result<WalletInfo> {
        println!("in wallet_info in api");
        let snapshot = state.snapshot();
        let general_schema = blockchain::Schema::new(&snapshot);
        let currency_schema = Schema::new(&snapshot);

        let max_height = general_schema.block_hashes_by_height().len() - 1;

        let block_proof = general_schema
            .block_and_precommits(Height(max_height))
            .unwrap();

        let to_table: MapProof<Hash, Hash> =
            general_schema.get_proof_to_service_table(VOTING_SERVICE_ID, 0);

        let to_wallet: MapProof<PublicKey, Wallet> =
            currency_schema.wallets().get_proof(query.pub_key);

        let wallet_proof = WalletProof {
            to_table,
            to_wallet,
        };

        let wallet = currency_schema.wallet(&query.pub_key);

        let wallet_history = wallet.map(|_| {
            let history = currency_schema.wallet_history(&query.pub_key);
            let proof = history.get_range_proof(0, history.len());

            let transactions: Vec<WalletTransactions> = history
                .iter()
                .map(|record| general_schema.transactions().get(&record).unwrap())
                .map(|raw| WalletTransactions::tx_from_raw(raw).unwrap())
                .collect::<Vec<_>>();

            WalletHistory {
                proof,
                transactions,
            }
        });

		let walets = currency_schema.wallets();
//		let mut ckeys = Vec::new();
		let mut candidates = Vec::new();

		for val in walets.values() {
			if val.cand() == 1 {
//				ckeys.push(*val.pub_key());
				let to_wallet: MapProof<PublicKey, Wallet> = currency_schema.wallets().get_proof(*val.pub_key());
				let to_table: MapProof<Hash, Hash> = general_schema.get_proof_to_service_table(VOTING_SERVICE_ID, 0);
				let cwallet_proof = WalletProof {
					to_table,
					to_wallet,
				};
				candidates.push(cwallet_proof);
			}
		}
//		println!("mcandidates: {:?}\n", candidates);

//		let candidate_keys: Vec<PublicKey> = ckeys.iter().cloned().collect::<Vec<PublicKey>>();
        println!("before ok in api");
        //todo - write your own file path
        let mut f = File::open("/Users/pigunther/workspace/zachitka/exonum/examples/voting/backend/output.vtk").expect("Unable to open file");

        let mut winner = String::new();
        f.read_to_string(&mut winner)
            .expect("something went wrong reading the file");

        Ok(WalletInfo {
            block_proof,
            wallet_proof,
            wallet_history,
			candidates,
            winner
        })
    }

    /// Endpoint for handling voting transactions.
    pub fn post_transaction(state: &ServiceApiState, query: WalletTransactions, ) -> api::Result<TransactionResponse> {
        println!("start post_transaction");
        let transaction: Box<dyn Transaction> = query.into();
        let tx_hash = transaction.hash();
        state.sender().send(transaction)?;
        //println!("TransactionResponse: {:?}, {:?}", transaction, tx_hash);
        println!("post_transaction ok");
        Ok(TransactionResponse { tx_hash })
    }

    /// Wires the above endpoint to public scope of the given `ServiceApiBuilder`.
    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint("v1/wallets/info", Self::wallet_info)
            .endpoint_mut("v1/wallets/transaction", Self::post_transaction);
    }
}
