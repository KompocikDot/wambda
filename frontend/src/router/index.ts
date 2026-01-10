import { createRouter, createWebHistory } from 'vue-router'
import MainPage from '../pages/Main.vue'
import CreateFunction from '../pages/CreateFunction.vue'

const routes = [
  {
    path: '/',
    component: MainPage,
  },
  {
    path: '/create',
    component: CreateFunction,
  },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

export default router
