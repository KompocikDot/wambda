<script setup lang="ts">
import FaasmLogo from '@/components/FaasmLogo.vue'
</script>

<template>
  <UMain class="flex flex-col items-center justify-center gap-2 p-4">
    <FaasmLogo />
    <h1 class="text-2xl font-bold">Sign in to FAASM</h1>
    <h2 class="text-sm text-muted">Deploy and run you WASM functions.</h2>
    <UCard variant="outline">
      <UForm class="flex flex-col gap-y-2 w-80">
        <UFormField label="Email">
          <UInput type="email" class="w-full" />
        </UFormField>
        <UFormField label="Password">
          <UInput type="password" class="w-full" />
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
