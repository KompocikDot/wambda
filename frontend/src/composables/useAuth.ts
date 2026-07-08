import { ref } from 'vue'

const isAuthenticated = ref(false)

export const useAuth = () => {
  const setAuth = (status: boolean) => {
    isAuthenticated.value = status
  }

  return { isAuthenticated, setAuth }
}
