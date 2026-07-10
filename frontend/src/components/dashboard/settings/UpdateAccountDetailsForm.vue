<script setup lang="ts">
import { $api } from '@/composables/useAPI'
import { useLocalStorage } from '@/composables/useLocalStorage'
import type { User } from '@/schemas/auth'
import {
  UpdateAccountSchema,
  type UpdateAccountSubmitInput,
  type UpdateAccountSubmitOutput,
} from '@/schemas/settings'
import type { FormSubmitEvent } from '@nuxt/ui'
import { reactive, ref } from 'vue'

const user = useLocalStorage<User>('user', { email: '', fullName: '' }, false)

const loading = ref(false)
const state = reactive<UpdateAccountSubmitInput>({
  email: user.value.email,
  fullName: user.value.fullName,
})

const updateAccountDetails = async (event: FormSubmitEvent<UpdateAccountSubmitOutput>) => {
  loading.value = true

  try {
    await $api('/user', {
      credentials: 'include',
      method: 'PATCH',
      body: {
        email: event.data.email,
        fullName: event.data.fullName,
      },
    })

    user.value.email = event.data.email
    user.value.fullName = event.data.fullName
  } catch (e) {
    console.error(e)
  }

  loading.value = true
}
</script>

<template>
  <UCard class="w-2/3" variant="subtle">
    <UForm
      class="flex flex-col gap-y-2"
      @submit="updateAccountDetails"
      :schema="UpdateAccountSchema"
      :state="state"
    >
      <div class="text-highlighted font-bold">Account details</div>
      <div class="flex flex-row gap-x-2">
        <UFormField label="Full name" name="fullName" required>
          <UInput v-model="state.fullName" />
        </UFormField>
        <UFormField label="Email" name="email" required>
          <UInput v-model="state.email" />
        </UFormField>
      </div>
      <UButton type="submit" label="Save changes" class="w-fit" />
    </UForm>
  </UCard>
</template>
