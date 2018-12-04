<template>
  <div class="tabs-container">
    <ul class="nav nav-tabs">
      <!-- eslint-disable-next-line vue/require-v-for-key -->
      <li v-for="tab in tabs" class="nav-item">
        <a :class="{ 'active': current === tab }" href="#" class="nav-link" @click="changeTab(tab)">
          {{ tab.title }}
        </a>
      </li>
    </ul>

    <div class="tab-content">
      <slot @mount="addTab"/>
    </div>
  </div>
</template>

<script>
  module.exports = {
    name: 'tabs',
    data() {
      return {
        tabs: [],
        current: null
      };
    },
    methods: {
      addTab(tab) {
        this.tabs.push(tab);
        if (tab.active === true) {
          this.current = tab;
        }
      },

      changeTab(tab) {
        this.current = tab;
        this.tabs.forEach(value => {
          value.active = value === tab;
        });
      }
    }
  };
</script>

<style>
  .tabs-container {
    height: calc(100vh - 60px);
    display: flex;
    flex-direction: column;
  }

  .tab-content {
    flex: 1 0;
  }

  .nav-tabs {
    min-height: 40px;
    border-bottom: 1px solid #040b26;
    display: flex;
    justify-content: space-between;
    background: #040b26;
  }

  .nav-tabs .nav-link {
    border: none;
    height: 40px;
    color: #7494c5;
    background: white;
  }

  .nav-tabs .nav-item.show .nav-link,
  .nav-tabs .nav-link.active {
    color: white;
    background-color: #92dd54;
    border: none;
    text-transform: capitalize;
  }

  a {
    color: #7494c5;
  }

</style>
//todo - слишком низка кнопка регистрации на мобилке
