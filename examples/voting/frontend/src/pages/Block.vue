<template>
  <div class="container2-main">
    <navbar title="Block"/>

    <div class="container2 block">
      <div class="container2-row">
          <nav aria-label="breadcrumb">
            <ol class="breadcrumb">
              <li class="breadcrumb-item">
                <router-link :to="{ name: 'blockchain' }">Blockchain</router-link>
              </li>
              <li class="breadcrumb-item active" aria-current="page">Block {{ height }}</li>
            </ol>
          </nav>

          <div class="card-row">
            <div>Hash</div>
            <div v-for="(transaction) in transactions">
              <router-link :to="{ name: 'transaction', params: { hash: transaction, ownPage: true } }">{{ transaction }}</router-link>
            </div>
          </div>

          <div  v-if="!transactions" class="card-row">
            <div></div>
            <div>There are no transactions in the block</div>
          </div>

          <nav class="mt-5" aria-label="Nearby blocks navigation">
            <ul class="pagination justify-content-center">
              <li class="page-item">
                <router-link :to="{ name: 'block', params: { height: previous } }" class="page-link">&larr; Previous block</router-link>
              </li>
              <li class="page-item">
                <router-link :to="{ name: 'block', params: { height: next } }" class="page-link">Next block &rarr;</router-link>
              </li>
            </ul>
          </nav>
      </div>
    </div>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  import Navbar from '../components/Navbar.vue';
  import Spinner from '../components/Spinner.vue';

  module.exports = {
    components: {
      Navbar,
      Spinner
    },
    props: {
      height: String
    },
    data() {
      return {
        block: {},
        transactions: [],
        isSpinnerVisible: false
      };
    },
    computed: {
      previous() {
        return (parseInt(this.height) - 1).toString();
      },
      next() {
        return (parseInt(this.height) + 1).toString();
      }
    },
    watch: {
      height() {
        this.loadBlock();
      }
    },
    methods: {
      async loadBlock() {
        this.isSpinnerVisible = true;

        try {
          const data = await this.$blockchain.getBlock(this.height);
          this.block = data.block;
          this.transactions = data.txs;
          this.isSpinnerVisible = false;
        } catch (error) {
          this.isSpinnerVisible = false;
          this.$notify('error', error.toString());
        }
      }
    },
    mounted() {
      this.$nextTick(function() {
        this.loadBlock();
      });
    }
  };
</script>
<style>
  .container2.block .card-row div:not(:first-child) {
    line-height: 28px;
    margin-left: 20px;
    color: #000c;
    white-space: pre-line;
  }

  .page-link {
    color: #7494c5;
    border: 1px solid #7494c55e;
  }

  .container2.block {
    flex: 1;
  }

  .container2-main {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: #f0f0f0;
  }

  .page-item .page-link {
    background: #fafafa;
  }
</style>
