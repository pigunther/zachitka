<template>
  <nav class="header navbar navbar-expand-lg navbar-light">
    <div class="container">
      <div class="title">{{title}}</div>
      <!--<router-link :to="{ name: isAuthorized ? 'user' : 'home' }" class="navbar-brand">Voting</router-link>-->
      <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbar" aria-controls="navbar" aria-expanded="false" aria-label="Toggle navigation">
        <span class="navbar-toggler-icon"/>
      </button>
      <div id="navbar" class="collapse navbar-collapse">
        <ul class="navbar-nav">
          <li class="nav-item">
            <router-link :to="{ name: 'user' }" class="nav-link">User</router-link>
          </li>
        </ul>
        <ul class="navbar-nav">
          <li class="nav-item">
            <router-link :to="{ name: 'vote' }" class="nav-link">Vote</router-link>
          </li>
        </ul>
        <ul class="navbar-nav">
          <li class="nav-item">
            <router-link :to="{ name: 'transactions'}" class="nav-link">Transactions</router-link>
          </li>
        </ul>
        <ul class="navbar-nav">
          <li class="nav-item">
            <router-link :to="{ name: 'blockchain' }" class="nav-link">Blockchain</router-link>
          </li>
        </ul>
        <ul v-if="isAuthorized" class="navbar-nav ml-auto">
          <li class="nav-item">
            <a href="#" class="nav-link" @click="logout">Logout</a>
          </li>
        </ul>
      </div>
    </div>
  </nav>
</template>

<script>
  module.exports = {
    name: 'navbar',
    props: {
      title: String
    },
    computed: {
      isAuthorized() {
        return this.$store.state.keyPair !== null;
      }
    },
    methods: {
      logout() {
        this.$store.commit('logout');
        this.$router.push({ name: 'home' });
      }
    },
  };
</script>

<style>

  .navbar {
    padding: 8px 16px;
  }

  .header.navbar {
    font-size: 16px;
  }

  .title {
    font-size: 30px;
  }

  .navbar .navbar-toggler:not(:disabled):not(.disabled) {
    cursor: pointer;
    color: #7494c5;
    background: white;
    border-color: white;
  }

  .navbar-light .navbar-nav .nav-link {
    color: #7494c5;
    text-transform: capitalize;
  }

  .navbar-nav {
    background: #040b26;
  }

  .navbar-light .navbar-nav .nav-link:hover {
    color: #7494c591;
  }
</style>
