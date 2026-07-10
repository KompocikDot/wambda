<script setup lang="ts">
import { $api } from '@/composables/useAPI'
import { useAuth } from '@/composables/useAuth'
import { useLocalStorage } from '@/composables/useLocalStorage'
import type { User } from '@/schemas/auth'
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const loading = ref(false)
const router = useRouter()
const user = useLocalStorage<User>('user', { email: '', fullName: '' }, false)
const { setAuth } = useAuth()

const deleteAccount = async () => {
  loading.value = true

  try {
    await $api('/user', {
      credentials: 'include',
      method: 'DELETE',
    })

    loading.value = false
    user.value.email = ''
    user.value.fullName = ''
    setAuth(false)

    router.replace({ name: 'main' })
  } catch (e) {
    console.error(e)
  }
}
</script>

<template>
  <UCard
    class="w-2/3 rounded-lg bg-error-50 dark:bg-error-950 ring ring-error-200 dark:ring-error-900"
    variant="subtle"
  >
    <UForm class="flex flex-col" @submit="deleteAccount">
      <h3 class="text-error-700 dark:text-error-400 font-bold">Delete account</h3>
      <div class="text-muted text-sm mt-1">
        Permanently deletes your account, all deployed functions and run history. This cannot be
        undone.
      </div>
      <UButton type="submit" label="Delete account" color="error" class="w-fit mt-4" />
    </UForm>
  </UCard>
</template>
