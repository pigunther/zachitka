<template>
  <div>
    <navbar/>

    <div class="container">
      <div class="row">
        <div class="col-md-6">
          <div class="card mt-5">
            <div class="card-header">User summary</div>
            <ul class="list-group list-group-flush">
              <li class="list-group-item">
                <div class="row">
                  <div class="col-sm-3"><strong>Name:</strong></div>
                  <div class="col-sm-9">{{ name }}</div>
                </div>
              </li>
              <li class="list-group-item">
                <div class="row">
                  <div class="col-sm-3"><strong>Public key:</strong></div>
                  <div class="col-sm-9"><code>{{ keyPair.publicKey }}</code></div>
                </div>
              </li>
              <li class="list-group-item">
                <div class="row">
                  <div class="col-sm-3"><strong>Voted:</strong></div>
                  <div class="col-sm-9">{{ balance }}<!--span v-numeral="balance" /--></div>
                </div>
              </li>
              <li class="list-group-item candidate-item">
                <div class="row">
                  <div class="col-sm-3"><strong>Votes:</strong></div>
                  <div class="col-sm-9">{{ votes }}<!--span v-numeral="balance" /--></div>
                </div>
              </li>
            </ul>
          </div>

          <div class="card mt-5">
            <div class="card-header">Transactions</div>
            <ul class="list-group list-group-flush">
              <li class="list-group-item font-weight-bold">
                <div class="row">
                  <div class="col-sm-12">Description</div>
                </div>
              </li>
              <!-- eslint-disable-next-line vue/require-v-for-key -->
              <li v-for="transaction in reverseTransactions" class="list-group-item">
                <div class="row">
                  <div class="col-sm-12">
                    <router-link :to="{ name: 'transaction', params: { hash: transaction.hash } }">
                      <span v-if="transaction.message_id === 2">Wallet created</span>
                      <span v-else-if="transaction.message_id === 1">
                        <strong v-numeral="transaction.body.amount"/> funds added
                      </span>
                      <span v-else-if="transaction.message_id === 0 && transaction.body.from === keyPair.publicKey">
                        Voted
                      </span>
                      <span v-else-if="transaction.message_id === 0 && transaction.body.to === keyPair.publicKey">
                        Vote received
                      </span>
                    </router-link>
                  </div>
                </div>
              </li>
            </ul>
          </div>
        </div>
        <div class="col-md-6">

          <div class="card mt-5">
            <div class="card-header">Vote</div>
            <div class="card-body">
              <form id="vote" @submit.prevent="transfer">
                <div class="form-group">
                  <label class="d-block">Select a candidate:</label>
                  <div v-for="variant in variants" :key="variant.key" class="form-check form-check">
                    <input :id="variant.key"
                           :value="variant.key"
                           :checked="defcandidate == variant.name"
                           v-model="receiver"
                           class="form-check-input"
                           type="radio"
                           @change="variantChange"
                    >
                    <label :for="variant.key" class="form-check-label">{{ variant.name }}</label>
                  </div>
                </div>
                <button id="btnvote" type="submit" class="btn btn-primary">{{ btnVoteText }}</button>
              </form>
            </div>
          </div>
        </div>
      </div>
    </div>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  import { mapState } from 'vuex';
  import Modal from '../components/Modal.vue';
  import Navbar from '../components/Navbar.vue';
  import Spinner from '../components/Spinner.vue';

  module.exports = {
    components: {
      Modal,
      Navbar,
      Spinner
    },
    data() {
      return {
        name: '',
        balance: '',
        cand: '',
        votes: '',
        candidateWallets: [],
        defcandidate: 'none',
        receiver: '',
        amountToTransfer: '',
        isSpinnerVisible: false,
        transactions: [],
        variants: [
        ],
        btnVoteText: 'Vote',
        btnVotedText: 'You have voted'
      };
    },
    computed: Object.assign({
      reverseTransactions() {
        return this.transactions.slice().reverse();
      }
    },
    mapState({
      keyPair: state => state.keyPair
    })),
    methods: {
      variantChange() {
        if (this.receiver == '' || this.balance == 'Yes') {
          $('#btnvote').prop('disabled', true);
          this.btnVoteText = this.btnVotedText;
        }
        else {
          $('#btnvote').prop('disabled', false);
        }
      },

      async loadUser() {
        if (this.keyPair === null) {
          this.$store.commit('logout');
          this.$router.push({ name: 'home' });
          return;
        }

        this.isSpinnerVisible = true;

        try {
          const data = await this.$blockchain.getUser(this.keyPair.publicKey);

          for (let i in data.candidateWallets) {
            let vname = data.candidateWallets[i].name + ' ..... Votes: ' + data.candidateWallets[i].votes;
            this.variants.push({ key: data.candidateWallets[i].pub_key, name: vname });
          }
        
          this.name = data.wallet.name;

          if (data.wallet.balance > 0) {
            this.balance = 'No';
            $('#btnvote').prop('disabled', false);
          }
          else {
            this.balance = 'Yes';
            $('#btnvote').prop('disabled', true);
            this.btnVoteText = this.btnVotedText;
          }
//          data.wallet.balance > 0 ? this.balance = 'No' : this.balance = 'Yes';

          if (this.receiver == '') {
            $('#btnvote').prop('disabled', true);
          }
          else {
            $('#btnvote').prop('disabled', false);
          }

          this.cand = data.wallet.cand;
          this.cand > 0 ? this.isCandidate = true : false;

          if (!this.isCandidate) $('.candidate-item').addClass('hidden');
        
          this.cand > 0 ? this.votes = data.wallet.votes : this.votes = 'n/a';
          this.transactions = data.transactions;
          this.isSpinnerVisible = false;
        } catch (error) {
          this.isSpinnerVisible = false;
          this.$notify('error', error.toString());
        }
      },
      async transfer() {
        if (!this.$validateHex(this.receiver)) {
          return this.$notify('error', 'Invalid public key is passed');
        }

        // if (this.receiver === this.keyPair.publicKey) {
        //   return this.$notify('error', 'Can not vote for yourself');
        // }

        this.isSpinnerVisible = true;

        const seed = this.$blockchain.generateSeed();

          console.log('seed: ', seed);

        try {
          await this.$blockchain.transfer(this.keyPair, this.receiver, seed);
          const data = await this.$blockchain.getUser(this.keyPair.publicKey);
          console.log("data", data);
          if (data.wallet.balance > 0) {
            this.balance = 'No';
            $('#btnvote').prop('disabled', false);
          }
          else {
            this.balance = 'Yes';
            $('#btnvote').prop('disabled', true);
            this.btnVoteText = this.btnVotedText;
          }
          
          this.transactions = data.transactions;
          this.isSpinnerVisible = false;
          this.$notify('success', 'Your voting has been successfully recorded.');
        } catch (error) {
          this.isSpinnerVisible = false;
          this.$notify('error', error.toString());
        }
      }
    },
    mounted() {
      this.$nextTick(function() {
        this.loadUser();
      });
    }
  };
</script>

<style>
    .hidden {
      display: none;
    }
</style>
