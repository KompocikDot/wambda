import { createRouter, createWebHistory } from 'vue-router'
import MainPage from '../pages/MainPage.vue'
import DashboardPage from '@/pages/dashboard/DashboardPage.vue'
import FunctionsPage from '@/pages/dashboard/functions/FunctionsPage.vue'
import RunsPage from '@/pages/dashboard/runs/RunsPage.vue'
import SettingsPage from '@/pages/dashboard/SettingsPage.vue'
import SingleFunctionPage from '@/pages/dashboard/functions/SingleFunctionPage.vue'
import NotFoundPage from '@/pages/404Page.vue'
import SingleRunPage from '@/pages/dashboard/runs/SingleRunPage.vue'
import LoginPage from '@/pages/auth/LoginPage.vue'
import RegisterPage from '@/pages/auth/RegisterPage.vue'
import { useAuth } from '@/composables/useAuth'

const routes = [
  {
    path: '/',
    component: MainPage,
    name: 'main',
  },
  { path: '/login', component: LoginPage, name: 'login' },
  { path: '/register', component: RegisterPage, name: 'register' },
  {
    path: '/dashboard',
    component: DashboardPage,
    name: 'dashboard',
    meta: { requiresAuth: true },
  },
  {
    path: '/functions',
    component: FunctionsPage,
    name: 'functions',
    meta: { requiresAuth: true },
  },
  {
    path: '/functions/:uuid',
    component: SingleFunctionPage,
    name: 'singleFunction',
    meta: { requiresAuth: true },
  },
  { path: '/runs', component: RunsPage, name: 'runs', meta: { requiresAuth: true } },
  {
    path: '/runs/:id',
    component: SingleRunPage,
    name: 'singleRun',
    meta: { requiresAuth: true },
  },
  { path: '/settings', component: SettingsPage, name: 'settings', meta: { requiresAuth: true } },

  { path: '/not-found', component: NotFoundPage, name: 'notFound' },
  { path: '/:pathMatch(.*)', redirect: '/not-found' },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

router.beforeEach((to) => {
  const { isAuthenticated } = useAuth()

  if (to.meta.requiresAuth && !isAuthenticated.value) {
    return {
      name: 'login',
      query: { redirect: to.fullPath },
    }
  }
})

export default router
