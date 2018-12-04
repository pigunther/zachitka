<template>
  <div class="container1">
    <navbar title="Transactions"/>

    <tabs>
      <tab v-for="(transaction, index) in reverseTransactions" :is-active="index === 0" :key="transaction.hash"
           :title="transaction.message_id === 2 ? 'User' :
                  transaction.message_id === 0 && transaction.body.from === keyPair.publicKey ? 'Vote out' :
                  transaction.message_id === 0 && transaction.body.to === keyPair.publicKey ? 'Votes in' : ''"
      >
        <transaction class="transaction" :hash="transaction.hash">
        </transaction>
      </tab>
    </tabs>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  import { mapState } from 'vuex';
  import Modal from '../components/Modal.vue';
  import Navbar from '../components/Navbar.vue';
  import Spinner from '../components/Spinner.vue';
  import Tabs from '../components/Tabs.vue';
  import Tab from '../components/Tab.vue';
  import Transaction from '../pages/Transaction.vue';

  module.exports = {
    components: {
      Modal,
      Navbar,
      Spinner,
      Tabs,
      Tab,
      Transaction
    },
    data() {
      return {
        name: '',
        cand: '',
        votes: '',
        receiver: '',
        amountToTransfer: '',
        isSpinnerVisible: false,
        transactions: [],
        location: {},
        transaction: {},
        status: {},
        type: ''
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

          this.transactions = data.transactions;
          this.isSpinnerVisible = false;
        } catch (error) {
          this.isSpinnerVisible = false;
          this.$notify('error', error.toString());
        }
      },
    },
    mounted() {
      this.$nextTick(function() {
        this.loadUser();
      });
    }
  };
</script>
<style>
  .transaction {
    display: block;
    width: 100%;
    height: 200px;
  }
</style>


//todo - не грузить постоянно юзера в юзерах и транзакциях
