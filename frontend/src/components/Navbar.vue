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
    active: route.path.endsWith('/'),
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
