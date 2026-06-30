import { createRouter, createWebHistory } from 'vue-router'
import MainPage from '../pages/Main.vue'
import CreateFunctionPage from '../pages/CreateFunction.vue'
import LambdaPage from '../pages/Lambda.vue'

const routes = [
  {
    path: '/',
    component: MainPage,
    name: 'main',
  },
  {
    path: '/create',
    component: CreateFunctionPage,
    name: 'create-lambda',
  },
  { path: '/lambdas/:uuid', component: LambdaPage, name: 'lambda-details' },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

export default router
