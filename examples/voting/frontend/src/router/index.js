import Vue from 'vue'
import Router from 'vue-router'
import AuthPage from '../pages/Auth.vue'
import WalletPage from '../pages/Wallet.vue'
import BlockchainPage from '../pages/Blockchain.vue'
import UserPage from '../pages/User.vue'
import VotePage from '../pages/Vote.vue'
import BlockPage from '../pages/Block.vue'
import TransactionPage from '../pages/Transaction.vue'
import TransactionsPage from '../pages/Transactions.vue'

Vue.use(Router);

export default new Router({
  routes: [
    {
      path: '/',
      name: 'home',
      component: AuthPage
    },
    {
      path: '/blockchain',
      name: 'blockchain',
      component: BlockchainPage
    },
    {
      path: '/user',
      name: 'user',
      component: UserPage
    },
    {
      path: '/vote',
      name: 'vote',
      component: VotePage
    },
    {
      path: '/transactions',
      name: 'transactions',
      component: TransactionsPage
    },
    {
      path: '/block/:height',
      name: 'block',
      component: BlockPage,
      props: true
    },
    {
      path: '/transaction/:hash',
      name: 'transaction',
      component: TransactionPage,
      props: true
    }
  ]
})
