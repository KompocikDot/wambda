import { createRouter, createWebHistory } from 'vue-router'
import MainPage from '../pages/MainPage.vue'
import DashboardPage from '@/pages/DashboardPage.vue'
import FunctionsPage from '@/pages/FunctionsPage.vue'

const routes = [
  {
    path: '/',
    component: MainPage,
    name: 'main',
  },
  {
    path: '/dashboard',
    component: DashboardPage,
    name: 'dashboard',
  },
  {
    path: '/functions',
    component: FunctionsPage,
    name: 'functions',
  },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

export default router
