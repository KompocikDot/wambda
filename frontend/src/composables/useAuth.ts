import { useLocalStorage } from './useLocalStorage'

const isAuthenticated = useLocalStorage<boolean>("isAuthenticated", false)

export const useAuth = () => {
  const setAuth = (status: boolean) => {
    isAuthenticated.value = status
  }

  return { isAuthenticated, setAuth }
}
