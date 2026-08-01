<script setup lang="ts">
import WambdaLogo from '@/components/WambdaLogo.vue'
import { $api } from '@/composables/useAPI'
import {
  loginFormSchema,
  type LoginErrorResponse,
  type LoginFormState,
  type LoginUserResponse,
  type User,
} from '@/schemas/auth'
import type { FormSubmitEvent } from '@nuxt/ui'
import { reactive, ref } from 'vue'
import { useAuth } from '@/composables/useAuth'
import { useRouter } from 'vue-router'
import type { FetchError } from 'ofetch'
import { useLocalStorage } from '@/composables/useLocalStorage'

const { setAuth } = useAuth()
const userStorage = useLocalStorage<User | null>('user', null, true)
const router = useRouter()

const loginError = ref('')

const state = reactive<LoginFormState>({
  email: '',
  password: '',
})

const onSubmit = async (event: FormSubmitEvent<LoginFormState>) => {
  loginError.value = ''

  try {
    const user = await $api<LoginUserResponse | LoginErrorResponse>('/auth/login', {
      method: 'POST',
      body: {
        email: event.data.email,
        password: event.data.password,
      },
      credentials: 'include'
    })

    userStorage.value = user as LoginUserResponse
    setAuth(true)

    router.replace({ name: 'dashboard' })
  } catch (e) {
    const errData = (e as FetchError).data as LoginErrorResponse
    const globalErr = errData.details?.global
    if (globalErr) {
      loginError.value = globalErr
    }
  }
}
</script>

<template>
  <UMain class="flex flex-col items-center justify-center gap-2 p-4">
    <WambdaLogo />
    <h1 class="text-2xl font-bold">Sign in to Wambda</h1>
    <h2 class="text-sm text-muted">Deploy and run you WASM functions.</h2>
    <UCard variant="outline">
      <UAlert
        v-if="loginError"
        :description="loginError"
        class="mb-2"
        variant="subtle"
        color="error"
        icon="i-lucide-circle-x"
      />
      <UForm
        class="flex flex-col gap-y-2 w-80"
        @submit="onSubmit"
        :schema="loginFormSchema"
        :state="state"
      >
        <UFormField label="Email" name="email">
          <UInput type="email" class="w-full" v-model="state.email" />
        </UFormField>
        <UFormField label="Password" name="password">
          <UInput type="password" class="w-full" v-model="state.password" />
        </UFormField>
        <UButton label="Sign In" type="submit" class="w-full justify-center mt-4" />
      </UForm>
      <USeparator label="Or" class="py-2" />
      <UButton
        disabled
        label="Continue with Github"
        icon="i-lucide-github"
        class="w-full justify-center"
      />
    </UCard>
    <div>Don't have an account? <ULink active :to="{ name: 'register' }">Create one</ULink></div>
  </UMain>
</template>
