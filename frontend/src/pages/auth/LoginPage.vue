<script setup lang="ts">
import FaasmLogo from '@/components/FaasmLogo.vue'
import { $api } from '@/composables/useAPI'
import { loginFormSchema, type LoginFormState } from '@/schemas.ts/auth'
import type { FormSubmitEvent } from '@nuxt/ui'
import { reactive } from 'vue'
import { useAuth } from '@/composables/useAuth'
import { useRouter } from 'vue-router'

const { setAuth } = useAuth()
const router = useRouter()

const state = reactive<LoginFormState>({
  email: '',
  password: '',
})

const onSubmit = async (event: FormSubmitEvent<LoginFormState>) => {
  try {
    await $api('/auth/login', {
      method: 'POST',
      body: {
        email: event.data.email,
        password: event.data.password,
      },
    })

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
    <h1 class="text-2xl font-bold">Sign in to FAASM</h1>
    <h2 class="text-sm text-muted">Deploy and run you WASM functions.</h2>
    <UCard variant="outline">
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
