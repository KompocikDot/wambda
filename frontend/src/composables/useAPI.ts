import { ofetch } from 'ofetch'
import { useAuth } from '@/composables/useAuth'
import router from '@/router'

export const $api = ofetch.create({
  baseURL: import.meta.env.VITE_API_BASE_URL || '',
  async onResponseError({ response }) {
    if (response.status === 401) {
      const { setAuth } = useAuth()
      setAuth(false)
      router.push({ name: 'login' })
    }
  },
})
