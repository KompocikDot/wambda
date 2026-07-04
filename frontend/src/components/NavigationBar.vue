<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui'
import { ref } from 'vue'

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
        <Logo />
        <div>FAASM</div>
      </div>
    </template>

    <UNavigationMenu :items="items" />

    <template #right>
      <div class="flex gap-x-1">
        <UButton variant="ghost">Sign In</UButton>
        <UButton>Get started</UButton>
        <UColorModeButton />
      </div>
    </template>
  </UHeader>
</template>
