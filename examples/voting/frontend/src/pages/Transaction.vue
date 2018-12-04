<template>
  <div>
    <navbar v-if="ownPage" title="Transaction"/>

    <div class="container2">
      <nav v-if="location.block_height" class="breadcrumb_container" aria-label="breadcrumb">
        <ol class="breadcrumb">
          <li class="breadcrumb-item">
            <!--Blockchain-->
            <router-link :to="{ name: 'blockchain' }">Blockchain</router-link>
          </li>
          <li class="breadcrumb-item">
            <!--Block {{-->
            <!--location.block_height }}-->
            <router-link :to="{ name: 'block', params: { height: location.block_height } }">Block {{
              location.block_height }}
            </router-link>
          </li>
          <li class="breadcrumb-item active" aria-current="page">Transaction {{ hash }}</li>
        </ol>
      </nav>

      <div class="card-container">
        <div class="card-row important">
          <div>Hash:</div>
          <span class="hint">(click to copy)</span>
          <div :id="hash" @click="copyText(hash)">{{hash}}</div>
        </div>
        <div class="card-row" v-if="location.block_height">
          <div>Block:</div>
          <div>
            <router-link :to="{ name: 'block', params: { height: location.block_height } }">{{ location.block_height
              }}
            </router-link>
          </div>
        </div>

        <div class="card-row" v-if="type">
          <div>Type:</div>
          <div>{{type}}</div>
        </div>

        <div class="card-row" v-if="status && status.type">
          <div>Status:</div>
          <div>{{status.type}}</div>
        </div>

        <div class="card-row" v-if="transaction.protocol_version !== undefined">
          <div>Protocol version:</div>
          <div>{{transaction.protocol_version}}</div>
        </div>

        <div class="card-row" v-if="transaction.network_id !== undefined">
          <div>Network ID:</div>
          <div>{{transaction.network_id}}</div>
        </div>

        <div class="card-row" v-if="transaction.service_id !== undefined">
          <div>Service ID:</div>
          <div>{{transaction.service_id}}</div>
        </div>

        <div class="card-row" v-if="transaction.message_id !== undefined">
          <div>Message ID:</div>
          <div>{{transaction.message_id}}</div>
        </div>


        <div class="card-row" v-if="transaction.signature !== undefined">
          <div>Signature:</div>
          <div>{{transaction.signature}}</div>
        </div>

        <div class="card-row" v-if="transaction.body">
          <div>Body:</div>
          <div>
            {{ JSON.stringify(transaction.body, null, 2) }}
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
    name: 'transaction',
    components: {
      Navbar,
      Spinner
    },
    props: {
      hash: String,
      ownPage: false
    },
    data() {
      return {
        transaction: {},
        location: {},
        status: {},
        type: '',
        isSpinnerVisible: false
      };
    },
    methods: {
      async loadTransaction() {
        this.isSpinnerVisible = true;

        try {
          const data = await this.$blockchain.getTransaction(this.hash);
          this.transaction = data.content;
          this.location = data.location;
          this.status = data.status;
          this.type = data.type;
          this.isSpinnerVisible = false;
        } catch (error) {
          this.isSpinnerVisible = false;
          this.$notify('error', error.toString());
        }
      },
      copyText(id) {

        let thisEl = document.getElementById(id);

        let textarea = document.createElement('textarea');
        textarea.id = 'temp_element';
        textarea.style.height = 0;
        document.body.appendChild(textarea);
        textarea.value = thisEl.innerText;
        let selector = document.querySelector('#temp_element');
        thisEl.style["border-color"] = "black";
        setTimeout(() => thisEl.style["border-color"] = "#92dd54", 500);
        selector.select();
        document.execCommand('copy');
        document.body.removeChild(textarea);

      }
    },
    mounted() {
      this.$nextTick(function () {
        this.loadTransaction();
      });
    }
  };
</script>
<style>
  .breadcrumb {
    word-break: break-word;
    background-color: #92dd5436;
  }

  .container2 {
    width: calc(100% - 15px * 2);
    min-height: 1px;
    margin: 15px 15px 0 15px;
  }
</style>
