<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui'
import { ref } from 'vue'
import WambdaLogo from './WambdaLogo.vue'

const items = ref<NavigationMenuItem[]>([
  {
    label: 'Product',
  },
  {
    label: 'Docs',
  },
  {
    label: 'Changelog',
  },
])
</script>

<template>
  <UHeader>
    <template #title>
      <div class="flex gap-x-1 items-center">
        <WambdaLogo />
        <div>Wambda </div>
      </div>
    </template>

    <UNavigationMenu :items="items" />

    <template #right>
      <div class="flex gap-x-1">
        <UButton variant="ghost" :to="{ name: 'login' }">Sign In</UButton>
        <UButton :to="{ name: 'register' }">Get started</UButton>
        <UColorModeButton />
      </div>
    </template>

    <template #body>
      <UNavigationMenu orientation="vertical" :items="items" />
    </template>
  </UHeader>
</template>
