<template>
  <div class="container1">
    <navbar title="Vote"/>

    <div class="card-body">
      <form id="vote" @submit.prevent="transfer">
        <div class="form-group">
          <label class="d-block">Select a candidate:</label>
          <div v-for="variant in variants" :key="variant.key" class="votes-container">
            <input :id="variant.key"
                   :value="variant.key"
                   :checked="defcandidate === variant.name"
                   v-model="receiver"
                   type="radio"
                   @change="variantChange"
            >
            <span class="checkmark"></span>
            <label :for="variant.key" class="label-container">
              <div>{{ variant.name }}</div>
              <div>{{ variant.votesAmount }}</div>
            </label>
          </div>
        </div>
        <button id="btnvote" type="submit" class="btn btn-primary">{{ btnVoteText }}</button>
      </form>

      <div class="end-container">
        <button id="btnend" type="button" class="btn btn-primary" @click="endVoting()">End voting for all</button>

        <div class="card-row" v-if="winner">
          <div>Winners:</div>
          <!--<div>{{winner}}</div>-->
          <div v-for="winn in winnerArr">{{winn}}</div>

          <!--winnerArr-->
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
        votes: '',
        defcandidate: 'none',
        receiver: '',
        isSpinnerVisible: false,
        variants: [

        ],
        btnVoteText: 'Vote',
        btnVotedText: 'You have voted',
        isCandidate: false,
        winner: '',
        winnerArr: []
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
      async loadUser() {
        if (this.keyPair === null) {
          this.$store.commit('logout');
          this.$router.push({ name: 'home' });
          return;
        }

        this.isSpinnerVisible = true;

        try {
          const data = await this.$blockchain.getUser(this.keyPair.publicKey);
          console.log("data", data);
          data.candidateWallets.forEach((item) => {
            this.variants.push({ key: item.pub_key, name: item.name, votesAmount: "Votes: " + item.votes });
          });

          this.name = data.wallet.name;
          this.winner = data.winner;
          this.winnerArr = this.winner.split(";");

          console.log("winner ---", this.winner);
          // console.log("winner.length ---", this.winner.length);
          // this.winner = data.winner;

          if (data.wallet.balance > 0) {
            this.balance = 'No';
            $('#btnvote').prop('disabled', false);
          } else {
            this.balance = 'Yes';
            $('#btnvote').prop('disabled', true);
            this.btnVoteText = this.btnVotedText;
          }


          if (this.winner && this.winner.length) {
            $('#btnvote').prop('disabled', true);
            $('#btnend').prop('disabled', true);

          }
          console.log(this.balance, "|",this.receiver, "|", this.receiver === '  ', this.receiver.length, this.receiver.charCodeAt(0), this.receiver.charCodeAt(1));
//          data.wallet.balance > 0 ? this.balance = 'No' : this.balance = 'Yes';

          this.isCandidate = data.wallet.cand > 0;

          this.transactions = data.transactions;
          this.isSpinnerVisible = false;
        } catch (error) {
          this.isSpinnerVisible = false;
          this.$notify('error', error.toString());
        }
      },
      async transfer() {
        console.log("|",this.receiver, "|");
        if (!this.$validateHex(this.receiver)) {
        }

        if (this.winner && this.winner.length) {
          return this.$notify('error', 'Sorry. The voting is over.');
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

          if (data.wallet.balance > 0) {
            this.balance = 'No';
            $('#btnvote').prop('disabled', false);
          }
          else {
            this.balance = 'Yes';
            $('#btnvote').prop('disabled', true);
            this.btnVoteText = this.btnVotedText;
          }

          this.isSpinnerVisible = false;
          this.$notify('success', 'Your voting has been successfully recorded.');
          window.location.reload();
        } catch (error) {
          this.isSpinnerVisible = false;
          this.$notify('error', error.toString());
        }
      },
      async endVoting() {

        if (this.winner && this.winner.length) {
          return this.$notify('error', 'Sorry. The voting is over.');
        } else {
          this.isSpinnerVisible = true;
          const seed = this.$blockchain.generateSeed();
          try {

            console.log("await this.$blockchain.endProcess", this.keyPair, this.receiver, seed);
            await this.$blockchain.endProcess(this.keyPair, this.receiver, seed);
            this.isSpinnerVisible = false;
            this.$notify('success', 'Ended successful.');
          } catch (error) {
            this.isSpinnerVisible = false;
            this.$notify('error', error.toString());
          }
        }

      },
      variantChange() {
        if (this.receiver === '' || this.balance === 'Yes') {
          $('#btnvote').prop('disabled', true);
          this.btnVoteText = this.btnVotedText;
        }
        else {
          $('#btnvote').prop('disabled', false);
        }
      },
    },
    created() {
      this.receiver = "";
    },
    mounted() {
      this.$nextTick(function() {
        this.loadUser();
      });
    }
  };
</script>

<style>
  /* Customize the label (the container) */
  .votes-container {
    display: block;
    position: relative;
    padding-left: 35px;
    margin-bottom: 12px;
    cursor: pointer;
    font-size: 22px;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
  }

  /* Hide the browser's default radio button */
  .votes-container input {
    position: absolute;
    opacity: 0;
    cursor: pointer;
    height: 0;
    width: 0;
  }

  /* Create a custom radio button */
  .checkmark {
    position: absolute;
    top: 5px;
    left: 0;
    height: 25px;
    width: 25px;
    background-color: white;
    border-radius: 50%;
  }

  /* On mouse-over, add a grey background color */
  .votes-container:hover input ~ .checkmark {
    background-color: #ccc;
  }

  /* When the radio button is checked, add a blue background */
  .votes-container input:checked ~ .checkmark {
    background-color: #92dd54;
  }

  /* Create the indicator (the dot/circle - hidden when not checked) */
  .checkmark:after {
    content: "";
    position: absolute;
    display: none;
  }

  /* Style the indicator (dot/circle) */
  .votes-container .checkmark:after {
    top: 9px;
    left: 9px;
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: white;
  }

  /* Show the indicator (dot/circle) when checked */
  .votes-container input:checked ~ .checkmark:after {
    display: block;
  }

  .label-container {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    overflow: hidden;
    word-break: break-all;
    flex-wrap: wrap;
    text-overflow: ellipsis;
  }

  .end-container {
    margin-top: 20px;
  }

  .end-container .card-row {
    flex-wrap: wrap;
  }

  .end-container .card-row div:not(:first-child) {
    height: 30px;
    line-height: 30px;
    align-self: center;
    margin-right: 20px;
  }
</style>
