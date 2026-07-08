<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui'
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import FaasmLogo from './FaasmLogo.vue'

const router = useRouter()

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
        <FaasmLogo />
        <div>FAASM</div>
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
  </UHeader>
</template>
