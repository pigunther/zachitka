<template>
  <div class="blockchain-main">
    <navbar title="Blockchain"/>

    <div class="container2 blockchain">
      <div class="blockchain-card-header">Latest blocks</div>

      <div class="blockchain-card">
        <div>Block height</div>
        <div>Transactions count</div>
      </div>
      <div class="blockchain-card" v-for="(block) in blocks" :key="block.height">
        <div>
          <router-link :to="{ name: 'block', params: { height: block.height } }">{{ block.height }}</router-link>
        </div>
        <div>{{ block.tx_count }}</div>
      </div>


      <div class="card-body text-center">
        <a href="#" class="btn btn-primary" @click.prevent="loadMore">Load older blocks</a>
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
    data() {
      return {
        isSpinnerVisible: false,
        blocks: []
      };
    },
    methods: {
      async loadBlocks(latest) {
        this.isSpinnerVisible = true;

        try {
          const data = await this.$blockchain.getBlocks(latest);
          this.blocks = this.blocks.concat(data.blocks);

          this.isSpinnerVisible = false;
        } catch (error) {
          this.isSpinnerVisible = false;
          this.$notify('error', error.toString());
        }
      },
      loadMore() {
        this.loadBlocks(this.blocks[this.blocks.length - 1].height - 1);
      }
    },
    mounted() {
      this.$nextTick(function () {
        this.loadBlocks();
      });
    }
  };
</script>
<style>
  .blockchain-main {
    background: #f0f0f0;
  }

  .blockchain-card-header {
    font-size: 24px;
    line-height: 24px;
    word-break: keep-all;
    height: 40px;
    margin: 5px;
  }

  .blockchain-card {
    display: flex;
    height: 40px;
    flex-direction: row;
    border: 1px solid #0000000d;
    border-radius: 5px;
    border-top: 0;
    margin-top: 5px;
    background: white;
  }

  .blockchain-card div {
    flex-basis: 50%;
    line-height: 30px;
    margin: 5px;
  }
</style>
