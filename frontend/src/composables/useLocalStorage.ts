import { onMounted, onUnmounted, ref, watch, type Ref } from 'vue'

export const useLocalStorage = <T>(key: string, defaultValue: T, deep = false): Ref<T> => {
  let initialValue

  try {
    initialValue = JSON.parse(localStorage.getItem(key) || 'null') || defaultValue
  } catch {
    initialValue = defaultValue
  }

  const item = ref(initialValue)

  watch(
    item,
    (value) => {
      if (!value) localStorage.removeItem(key)
      else localStorage.setItem(key, JSON.stringify(value))
    },
    { deep },
  )

  const handleStorageChange = (event: StorageEvent) => {
    if (event.key === key) {
      try {
        item.value = JSON.parse(event.newValue || 'null')
      } catch {
        console.log('ERROR parsing local storage value')
        item.value = defaultValue
      }
    }
  }

  onMounted(() => {
    window.addEventListener('storage', handleStorageChange)
  })

  onUnmounted(() => {
    window.removeEventListener('storage', handleStorageChange)
  })

  return item
}
