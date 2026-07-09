<script setup lang="ts">
import FaasmLogo from '@/components/FaasmLogo.vue'
import { useAuth } from '@/composables/useAuth'
import { $api } from '@/composables/useAPI'
import {
  registerFormSchema,
  type RegisterFormState,
  type RegisterUserResponse,
  type User,
} from '@/schemas/auth'
import type { FormSubmitEvent } from '@nuxt/ui'
import { reactive } from 'vue'
import { useRouter } from 'vue-router'
import { useLocalStorage } from '@/composables/useLocalStorage'

const router = useRouter()
const userStorage = useLocalStorage<User | null>('user', null, true)
const { setAuth } = useAuth()

const state = reactive<RegisterFormState>({
  fullName: '',
  email: '',
  password: '',
  repeatPassword: '',
})

const onSubmit = async (event: FormSubmitEvent<RegisterFormState>) => {
  try {
    const user = await $api<RegisterUserResponse>('/auth/register', {
      method: 'POST',
      body: {
        fullName: event.data.fullName,
        email: event.data.email,
        password: event.data.password,
      },
    })

    userStorage.value = user
    setAuth(true)

    router.replace({ name: 'dashboard' })
  } catch (e) {
    console.error(e)
  }
}
</script>

<template>
  <UMain class="flex flex-col items-center justify-center gap-2 p-4">
    <FaasmLogo />
    <h1 class="text-2xl font-bold">Create your account</h1>
    <h2 class="text-sm text-muted">Start deploying WASM functions now</h2>
    <UCard variant="outline">
      <UForm
        class="flex flex-col gap-y-2 w-80"
        @submit="onSubmit"
        :state="state"
        :schema="registerFormSchema"
        @error="(e) => console.log(e)"
      >
        <UFormField label="Full name" name="fullName" required>
          <UInput type="text" class="w-full" v-model="state.fullName" />
        </UFormField>
        <UFormField label="Email" name="email" required>
          <UInput type="email" class="w-full" v-model="state.email" />
        </UFormField>
        <UFormField label="Password" name="password" required>
          <UInput type="password" class="w-full" v-model="state.password" />
        </UFormField>
        <UFormField label="Confirm password" name="repeatPassword" required>
          <UInput type="password" class="w-full" v-model="state.repeatPassword" />
        </UFormField>

        <UButton label="Create account" type="submit" class="w-full justify-center mt-4" />
      </UForm>
      <USeparator label="Or" class="py-2" />
      <UButton
        disabled
        label="Continue with Github"
        icon="i-lucide-github"
        class="w-full justify-center"
      />
    </UCard>
    <div>Already have an account? <ULink active :to="{ name: 'login' }">Sign in</ULink></div>
  </UMain>
</template>
