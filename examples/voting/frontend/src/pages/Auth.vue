<template>
  <div>
    <div class="container1">
      <div class="header auth">Authorization</div>
      <tabs>
        <tab :is-active="true" title="Register">
          <form class="form" @submit.prevent="register">
            <div class="form-group">
              <label class="control-label">Name:</label>
              <input v-model="name" type="text" class="form-control" placeholder="Enter name" maxlength="260"
                     required>
            </div>
            <div>
              <div class="form-group group-check">
                <input id="candidateCheck" type="checkbox" class="form-check-input">
                <label class="form-check-label candidateLbl" for="candidateCheck">Candidate</label>
              </div>
              <button type="submit" class="btn btn-lg btn-block btn-primary">Register</button>
            </div>
          </form>
        </tab>
        <tab title="Log in">
          <form class="form" @submit.prevent="login">
            <div class="form-group">
              <label class="control-label">Secret key:</label>
              <input v-model="secretKey" type="text" class="form-control" placeholder="Enter secret key" required>
            </div>
            <button type="submit" class="btn btn-lg btn-block btn-primary">Log in</button>
          </form>
        </tab>
      </tabs>
    </div>

    <modal :visible="isModalVisible" title="Use has been created" action-btn="Log in" @close="closeModal"
           @submit="proceed">
      <div class="alert alert-warning" role="alert">Find your secret key in the user tab. Save it in a safe place. You will need it to log in to
      the demo next time.
      </div>
    </modal>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  import Tab from '../components/Tab.vue';
  import Tabs from '../components/Tabs.vue';
  import Modal from '../components/Modal.vue';
  import Spinner from '../components/Spinner.vue';

  module.exports = {
    components: {
      Tabs,
      Tab,
      Modal,
      Spinner
    },
    data() {
      return {
        name: '',
        secretKey: '',
        keyPair: {},
        isModalVisible: false,
        isSpinnerVisible: false
      };
    },
    methods: {
      login() {
        if (!this.$validateHex(this.secretKey, 64)) {
          return this.$notify('error', 'Invalid secret key is passed');
        }

        this.isSpinnerVisible = true;

        this.$store.commit('login', {
          publicKey: this.secretKey.substr(64),
          secretKey: this.secretKey
        });

        this.$nextTick(function () {
          this.$router.push({ name: 'user' });
        });
      },

      async register() {
        if (!this.name) {
          return this.$notify('error', 'The name is a required field');
        }

        let candidate = '0';
        let checkedValue = $('#candidateCheck:checked').val();
        if (checkedValue === 'on') candidate = '1';
//        console.log('check: ', checkedValue, ' candidate: ', candidate);

        this.isSpinnerVisible = true;
        this.keyPair = this.$blockchain.generateKeyPair();

        try {
          await this.$blockchain.createWallet(this.keyPair, this.name, candidate);
          this.name = '';
          this.isSpinnerVisible = false;
          this.isModalVisible = true;
        } catch (error) {
          console.log(error.toString());
          this.isSpinnerVisible = false;
          this.$notify('error', error.toString());
        }
      },

      closeModal() {
        this.isModalVisible = false;
      },

      proceed() {
        this.isModalVisible = false;

        this.$store.commit('login', this.keyPair);

        this.$nextTick(function () {
          this.$router.push({ name: 'user' });
        });
      }
    }
  };
</script>

<style>
  .header {
    width: 100%;
    background: #040b26;
    color: white;
    display: flex;
    justify-content: center;
    font-size: 30px;
    text-transform: uppercase;
    font-weight: 700;
  }

  .header.auth {
    height: 60px;
    line-height: 60px;
  }

  .container1 {
    background: #f0f0f0;
    height: 100%;
    overflow-x: hidden;
  }

  .group-check {
    margin-left: 20px;
  }

  .form {
    height: 100%;
    display: flex;
    padding: 40px;
    flex-direction: column;
    justify-content: space-between;
  }

  .btn.btn-primary {
    background: #92dd54;
    border: 1px solid #3d7a0b;
  }

  .alert-warning {
    font-size: 20px;
    color: black;
    background-color: #92dd542e;
    border: none;
  }
</style>
