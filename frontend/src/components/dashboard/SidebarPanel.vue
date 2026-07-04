<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui'

const items: NavigationMenuItem[][] = [
  [
    {
      label: 'Overview',
      icon: 'i-lucide-house',
    },
    {
      label: 'Functions',
      icon: 'i-lucide-square-function',
    },
    {
      label: 'Runs',
      icon: 'i-lucide-play',
    },
    {
      label: 'Settings',
      icon: 'i-lucide-settings',
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

    <template #footer="{ collapsed }">
      <UButton
        :avatar="{
          src: 'https://github.com/benjamincanac.png',
          loading: 'lazy' as const,
        }"
        :label="collapsed ? undefined : 'Benjamin'"
        color="neutral"
        variant="ghost"
        class="w-full"
        :block="collapsed"
      />
    </template>
  </UDashboardSidebar>
</template>
