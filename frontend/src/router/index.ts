import { createRouter, createWebHistory } from 'vue-router'
import MainPage from '../pages/MainPage.vue'
import DashboardPage from '@/pages/DashboardPage.vue'
import FunctionsPage from '@/pages/FunctionsPage.vue'
import RunsPage from '@/pages/RunsPage.vue'
import SettingsPage from '@/pages/SettingsPage.vue'

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
  { path: '/runs', component: RunsPage, name: 'runs' },
  { path: '/settings', component: SettingsPage, name: 'settings' },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

export default router
