<template>
  <div class="container1">
    <navbar title="User"/>

    <div class="card-container">
      <div class="card-row">
        <div>Name:</div>
        <div>{{name}}</div>
      </div>
      <div class="card-row important">
        <div>Public key:</div>
        <span class="hint">(click to copy)</span>
        <div id="publicKey" @click="copyText('publicKey')">{{keyPair.publicKey}}</div>
      </div>
      <div class="card-row important">
        <div>Secret key:</div>
        <span class="hint">(click to copy)</span>
        <div id="secretKey" @click="copyText('secretKey')">{{keyPair.secretKey}}</div>
      </div>
      <div class="card-row">
        <div>Voted:</div>
        <div>{{balance}}</div>
      </div>
      <div class="card-row" v-if="isCandidate">
        <div>Votes:</div>
        <div>{{votes}}</div>
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
        votes: '',
        candidateWallets: [],
        isSpinnerVisible: false,
        variants: [

        ],
        isCandidate: false
      };
    },
    computed:
      mapState({
        keyPair: state => state.keyPair
      }),
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

          this.name = data.wallet.name;
          console.log("data", data);

          this.isCandidate = data.wallet.cand > 0;

          this.isCandidate ? this.votes = data.wallet.votes : this.votes = 'n/a';
          this.isSpinnerVisible = false;
        } catch (error) {
          this.isSpinnerVisible = false;
          this.$notify('error', error.toString());
        }
      },

      copyText(id) {
        console.log(id);
        console.log("should copy");
        console.log($('#'+id));

        let thisEl = document.getElementById(id);

        // Create a new textarea element and give it id='temp_element'
        let textarea = document.createElement('textarea');
        textarea.id = 'temp_element';
        // Optional step to make less noise on the page, if any!
        textarea.style.height = 0;
        // Now append it to your page somewhere, I chose <body>
        document.body.appendChild(textarea);
        // Give our textarea a value of whatever inside the div of id=containerid
        textarea.value = thisEl.innerText;
        // Now copy whatever inside the textarea to clipboard
        let selector = document.querySelector('#temp_element');
        thisEl.style["border-color"] = "black";
        setTimeout(() => thisEl.style["border-color"] = "#92dd54", 500);
        selector.select();
        document.execCommand('copy');
        // Remove the textarea
        document.body.removeChild(textarea);

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
  .card-container {
    margin: 15px;
  }

  .card-row {
    display: flex;
    flex-direction: row;
    overflow-wrap: break-word;
    margin-bottom: 30px;
    word-break: break-word;
    overflow-x: hidden;
  }

  .card-row div:first-child {
    font-size: 24px;
    line-height: 24px;
    word-break: keep-all;
    margin: 5px;
  }

  .card-row div:nth-child(2) {
    line-height: 28px;
    margin: 5px 5px 5px 20px;
    color: #000c;
    white-space: pre-line;
  }

  .important {
    flex-wrap: wrap;
  }

  .hint {
    font-size: 12px;
    color: rgba(0, 0, 0, 0.48);
    line-height: 35px;
    margin-left: 5px;
  }

  .important div:nth-child(3) {
    max-width: 100%;
    display: block;
    background: #92dd54;
    border: 1px solid #92dd54;
    border-radius: 5px;
    margin-top: 10px;
    margin-left: 0;
  }
</style>
