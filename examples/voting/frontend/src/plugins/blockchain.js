import * as Exonum from 'exonum-client';
import axios from 'axios';

const TRANSACTION_URL = '/api/services/voting/v1/wallets/transaction';
const TRANSACTION_EXPLORER_URL = '/api/explorer/v1/transactions?hash=';
const PER_PAGE = 10;
const PROTOCOL_VERSION = 0;
const SERVICE_ID = 128;
const TX_TRANSFER_ID = 0;
//const TX_ISSUE_ID = 1
const TX_WALLET_ID = 2;
const TableKey = Exonum.newType({
  fields: [
    {name: 'service_id', type: Exonum.Uint16},
    {name: 'table_index', type: Exonum.Uint16}
  ]
});
const Wallet = Exonum.newType({
  fields: [
    {name: 'pub_key', type: Exonum.PublicKey},
    {name: 'name', type: Exonum.String},
    {name: 'balance', type: Exonum.Uint64},
    {name: 'cand', type: Exonum.Uint64},
    {name: 'votes', type: Exonum.Uint64},
    {name: 'history_len', type: Exonum.Uint64},
    {name: 'history_hash', type: Exonum.Hash},
    {name: 'ended', type: Exonum.Bool},
    {name: 'winner', type: Exonum.String}
  ]
});
const TransactionMetaData = Exonum.newType({
  fields: [
    {name: 'tx_hash', type: Exonum.Hash},
    {name: 'execution_status', type: Exonum.Bool}
  ]
});

function TransferTransaction() {
  return Exonum.newMessage({
    protocol_version: PROTOCOL_VERSION,
    service_id: SERVICE_ID,
    message_id: TX_TRANSFER_ID,
    fields: [
      {name: 'from', type: Exonum.PublicKey},
      {name: 'to', type: Exonum.PublicKey},
      {name: 'ended', type: Exonum.Bool},
      {name: 'seed', type: Exonum.Uint64}
    ]
  });
}


function CreateTransaction() {
  return Exonum.newMessage({
    protocol_version: PROTOCOL_VERSION,
    service_id: SERVICE_ID,
    message_id: TX_WALLET_ID,
    fields: [
      {name: 'pub_key', type: Exonum.PublicKey},
      {name: 'name', type: Exonum.String},
      {name: 'cand', type: Exonum.Uint64}
    ]
  });
}

function getTransaction(id) {
  switch (id) {
    case TX_TRANSFER_ID:
      return new TransferTransaction();
//    case TX_ISSUE_ID:
//      return new IssueTransaction()
    case TX_WALLET_ID:
      return new CreateTransaction();
    default:
      throw new Error('Unknown transaction ID has been passed');
  }
}

function getOwner(transaction) {
  switch (transaction.message_id) {
    case TX_TRANSFER_ID:
      return transaction.body.from;
//    case TX_ISSUE_ID:
//      return transaction.body.pub_key
    case TX_WALLET_ID:
      return transaction.body.pub_key;
    default:
      throw new Error('Unknown transaction ID has been passed');
  }
}

module.exports = {
  install(Vue) {
    Vue.prototype.$blockchain = {
      generateKeyPair() {
        return Exonum.keyPair();
      },

      generateSeed() {
        return Exonum.randomUint64();
      },

      createWallet(keyPair, name, cand) {
        // Describe transaction
        const transaction = new CreateTransaction();

        // Transaction data
        const data = {
          pub_key: keyPair.publicKey,
          name: name,
          cand: cand
        };

        // Sign transaction
        const signature = transaction.sign(keyPair.secretKey, data);
        console.log('signature=', signature);
        console.log('keyPair=', keyPair);
        console.log('data=', data);

        // Send transaction into blockchain
        return transaction.send(TRANSACTION_URL, TRANSACTION_EXPLORER_URL, data, signature);
      },

      transfer(keyPair, receiver, seed) {
        // Describe transaction
        const transaction = new TransferTransaction();

        // Transaction data
        const data = {
          from: keyPair.publicKey,
          to: receiver,
          ended: false,
          seed: seed
        };

        // Sign transaction
        const signature = transaction.sign(keyPair.secretKey, data);
        console.log("signature in transfer transaction:", signature);
        // Send transaction into blockchain
        return transaction.send(TRANSACTION_URL, TRANSACTION_EXPLORER_URL, data, signature);
      },

      endProcess(keyPair, receiver, seed) {
        // Describe transaction
        const transaction = new TransferTransaction();

        // Transaction data
        const data = {
          from: keyPair.publicKey,
          to: receiver,
          seed: seed,
          ended: true
        };

        // Sign transaction
        const signature = transaction.sign(keyPair.secretKey, data);
        console.log("signature in end transaction:", signature);

        // Send transaction into blockchain
        return transaction.send(TRANSACTION_URL, TRANSACTION_EXPLORER_URL, data, signature);
      },

      getUser(publicKey) {
        return axios.get('/api/services/configuration/v1/configs/actual').then(response => {
          // actual list of public keys of validators
          const validators = response.data.config.validator_keys.map(validator => {
            return validator.consensus_key;
          });

          return axios.get(`/api/services/voting/v1/wallets/info?pub_key=${publicKey}`)
            .then(response => response.data)
            .then(data => {

              console.log(data);

              if (!Exonum.verifyBlock(data.block_proof, validators)) {
                throw new Error('Block can not be verified');
              }

              // find root hash of table with wallets in the tree of all tables
              const tableKey = TableKey.hash({
                service_id: SERVICE_ID,
                table_index: 0
              });
              const tableProof = new Exonum.MapProof(data.wallet_proof.to_table, Exonum.Hash, Exonum.Hash);
              if (tableProof.merkleRoot !== data.block_proof.block.state_hash) {
                throw new Error('Wallets table proof is corrupted');
              }
              const walletsHash = tableProof.entries.get(tableKey);
              if (typeof walletsHash === 'undefined') {
                throw new Error('Wallets table not found');
              }

              // find wallet in the tree of all wallets
              const walletProof = new Exonum.MapProof(data.wallet_proof.to_wallet, Exonum.PublicKey, Wallet);
              console.log(walletProof, walletsHash);
              if (walletProof.merkleRoot !== walletsHash) {
                throw new Error('Wallet proof is corrupted');
              }
              const wallet = walletProof.entries.get(publicKey);
              if (typeof wallet === 'undefined') {
                throw new Error('Wallet not found');
              }
              const winner = data.winner;

              console.log(
                wallet.history_hash,
                wallet.history_len,
                data.wallet_history.proof,
                [0, wallet.history_len],
                TransactionMetaData
              );
              // get transactions
              const transactionsMetaData = Exonum.merkleProof(
                wallet.history_hash,        //rootHash
                wallet.history_len,         //count
                data.wallet_history.proof,  //blockProof
                [0, wallet.history_len],    //range
                TransactionMetaData         //type
              );

              if (data.wallet_history.transactions.length !== transactionsMetaData.length) {
                // number of transactions in wallet history is not equal
                // to number of transactions in array with transactions meta data
                throw new Error('Transactions can not be verified');
              }

              // validate each transaction
              let transactions = [];
              for (let i in data.wallet_history.transactions) {
                let transaction = data.wallet_history.transactions[i];

                // get transaction definition
                let Transaction = getTransaction(transaction.message_id);

                // get transaction owner
                const owner = getOwner(transaction);

                // add a signature to the transaction definition
                Transaction.signature = transaction.signature;

                // validate transaction hash
                if (Transaction.hash(transaction.body) !== transactionsMetaData[i]) {
                  throw new Error('Invalid transaction hash has been found');
                }

                // validate transaction signature
                if (!Transaction.verifySignature(transaction.signature, owner, transaction.body)) {
                  throw new Error('Invalid transaction signature has been found');
                }

                // add transaction to the resulting array
                transactions.push(Object.assign({
                  hash: transactionsMetaData[i]
                }, transaction));
              }

              let candidateWallets = [];

              for (let i in data.candidates) {
                const ctableProof = new Exonum.MapProof(data.candidates[i].to_table, Exonum.Hash, Exonum.Hash);
                if (ctableProof.merkleRoot !== data.block_proof.block.state_hash) {
                  console.log('Candidates wallets table proof is corrupted');
                  throw new Error('Candidates wallets table proof is corrupted');

                }
                const cwalletsHash = ctableProof.entries.get(tableKey);
                if (typeof cwalletsHash === 'undefined') {
                  console.log('Candidates wallets table not found');
                  throw new Error('Candidates wallets table not found');
                }

                const cwalletProof = new Exonum.MapProof(data.candidates[i].to_wallet, Exonum.PublicKey, Wallet);

                if (cwalletProof.merkleRoot !== cwalletsHash) {
                  console.log('Candidates wallet proof is corrupted');
                  throw new Error('Candidates wallet proof is corrupted');

                }
                const cWallet = cwalletProof.entries.values().next().value;
                candidateWallets.push(cWallet);
                if (typeof cWallet === 'undefined') {
                  console.log('Candidates wallet not found');
                  throw new Error('Wallet not found');

                }
              }
//              console.log('candidate keys: ', data.candidate_keys);
//              console.log('candidate wallets: ', candidateWallets);

              return {
                block: data.block_proof.block,
                wallet: wallet,
                transactions: transactions,
                candidateWallets: candidateWallets,
                winner: winner
              };
            });
        });
      },


      getBlocks(latest) {
        const suffix = !isNaN(latest) ? '&latest=' + latest : '';
        return axios.get(`/api/explorer/v1/blocks?count=${PER_PAGE}${suffix}`).then(response => response.data);//.then(data => {
//            return {
//                data: data
//            }
//        })
      },

      getBlock(height) {
        return axios.get(`/api/explorer/v1/block?height=${height}`).then(response => response.data);
      },

      getTransaction(hash) {
        return axios.get(`/api/explorer/v1/transactions?hash=${hash}`).then(response => response.data);
      }
    };
  }
};
