<script setup lang="ts">
import { $api } from '@/composables/useAPI'
import type { FormSubmitEvent } from '@nuxt/ui'
import type { ChangePasswordSubmitInput, ChangePasswordSubmitOutput } from '@/schemas/settings'
import { ChangePasswordSchema } from '@/schemas/settings'
import { reactive, ref } from 'vue'
import { useLocalStorage } from '@/composables/useLocalStorage'
import type { User } from '@/schemas/auth'

const user = useLocalStorage<User>('user', { email: '', fullName: '' }, false)

const loading = ref(false)
const state = reactive<ChangePasswordSubmitInput>({
  currentPassword: '',
  newPassword: '',
})

const changePassword = async (event: FormSubmitEvent<ChangePasswordSubmitOutput>) => {
  loading.value = true

  try {
    await $api('/auth/change-password', {
      credentials: 'include',
      method: 'PATCH',
      body: {
        newPassword: event.data.newPassword,
        currentPassword: event.data.currentPassword,
      },
    })

    user.value.email = ''
    user.value.fullName = ''
  } catch (e) {
    console.error(e)
  }

  loading.value = false
}
</script>

<template>
  <UCard class="w-2/3" variant="subtle">
    <UForm
      class="flex flex-col gap-y-2"
      @submit="changePassword"
      :state="state"
      :schema="ChangePasswordSchema"
    >
      <div class="text-highlighted font-bold">Change password</div>
      <div class="flex flex-row gap-x-2">
        <UFormField class="max-w-1/2" label="Current password" name="currentPassword" required>
          <UInput type="password" v-model="state.currentPassword" placeholder="••••••••" />
        </UFormField>
        <UFormField label="New password" name="newPassword" required>
          <UInput type="password" v-model="state.newPassword" placeholder="••••••••" />
        </UFormField>
      </div>
      <UButton type="submit" label="Update password" variant="outline" class="w-fit" />
    </UForm>
  </UCard>
</template>
