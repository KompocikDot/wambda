<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui'
import { useRoute } from '@nuxt/ui/runtime/vue/stubs/none.js'

const route = useRoute()

const items: NavigationMenuItem[][] = [
  [
    {
      label: 'Overview',
      icon: 'i-lucide-house',
      active: route.path.startsWith('/dashboard'),
    },
    {
      label: 'Functions',
      icon: 'i-lucide-square-function',
      active: route.path.startsWith('/functions'),
    },
    {
      label: 'Runs',
      icon: 'i-lucide-play',
      active: route.path.startsWith('/runs'),
    },
    {
      label: 'Settings',
      icon: 'i-lucide-settings',
      active: route.path.startsWith('/settings'),
    },
  ],
]
</script>

<template>
  <UDashboardSidebar collapsible resizable :ui="{ footer: 'border-t border-default' }">
    <template #header="{ collapsed }">
      <Logo class="pl-1.5 h-5 w-auto shrink-0" />
      <div v-if="!collapsed" class="font-bold">FAASM</div>
    </template>

    <template #default="{ collapsed }">
      <UNavigationMenu :collapsed="collapsed" :items="items[0]" orientation="vertical" />

      <UNavigationMenu
        :collapsed="collapsed"
        :items="items[1]"
        orientation="vertical"
        class="mt-auto"
      />
    </template>

    <template #footer>
      <UUser name="John Doe" description="Software Engineer" />
    </template>
  </UDashboardSidebar>
</template>
