<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui'
import { ref } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()

const items = ref<NavigationMenuItem[]>([
  {
    label: 'Lambdas',
    icon: 'i-lucide-clipboard-list',
    to: '/',
    active: route.name === 'main' || route.name === 'lambda-details',
  },
  {
    label: 'Create new',
    icon: 'i-lucide-file-plus',
    to: '/create',
    active: route.path.endsWith('/create'),
  },
])
</script>

<template>
  <UNavigationMenu :items="items" class="w-full justify-center" />
</template>
