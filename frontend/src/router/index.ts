import { createRouter, createWebHistory } from 'vue-router'
import MainPage from '../pages/MainPage.vue'
import DashboardPage from '@/pages/DashboardPage.vue'
import FunctionsPage from '@/pages/FunctionsPage.vue'
import RunsPage from '@/pages/RunsPage.vue'
import SettingsPage from '@/pages/SettingsPage.vue'
import SingleFunctionPage from '@/pages/SingleFunctionPage.vue'
import NotFoundPage from '@/pages/404Page.vue'
import SingleRunPage from '@/pages/SingleRunPage.vue'

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
  {
    path: '/functions/:id',
    component: SingleFunctionPage,
    name: 'singleFunction',
  },
  { path: '/runs', component: RunsPage, name: 'runs' },
  {
    path: '/runs/:id',
    component: SingleRunPage,
    name: 'singleRun',
  },

  { path: '/settings', component: SettingsPage, name: 'settings' },
  { path: '/not-found', component: NotFoundPage, name: 'notFound' },
  { path: '/:pathMatch(.*)', redirect: '/not-found' },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

export default router
