<template>
  <div>
    <navbar/>

    <div class="container">
      <div class="row">
        <div class="col-sm-12">
          <div class="card mt-5">
            <div class="card-header">Latest blocks</div>
            <ul class="list-group list-group-flush">
              <li class="list-group-item font-weight-bold">
                <div class="row">
                  <div class="col-sm-6">Block height</div>
                  <div class="col-sm-6">Transactions count</div>
                </div>
              </li>
              <li v-for="(block) in blocks" :key="block.height" class="list-group-item">
                <div class="row">
                  <div class="col-sm-6">
                    <router-link :to="{ name: 'block', params: { height: block.height } }">{{ block.height }}</router-link>
                  </div>
                  <div class="col-sm-6">{{ block.tx_count }}</div>
                </div>
              </li>
            </ul>
            <div class="card-body text-center">
              <a href="#" class="btn btn-primary" @click.prevent="loadMore">Load older blocks</a>
            </div>
          </div>
        </div>
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

//            this.blocks = data.blocks;
//v-if="block.tx_count"
/*            let cnt = 0;
            let arr = [];
            let found = 0;
            while (found < 10) {
                for (let i in this.blocks) {
                    if (this.blocks[i].tx_count != 0) {
                        arr.push(this.blocks[i]);
                        found++;
                        console.log('true');
                    }
                }
//                console.log(cnt);
                cnt++;
                if (cnt > 1000) break;
                let newdata = await this.$blockchain.getBlocks(this.blocks[this.blocks.length - 1].height - 1);
                this.blocks = newdata.blocks;
            }
            console.log('arr:', arr);
            this.blocks = arr;

//            console.log('latest:', latest);
            console.log('this.blocks:', this.blocks);
*/
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
      this.$nextTick(function() {
        this.loadBlocks();
      });
    }
  };
</script>
