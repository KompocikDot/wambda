<script setup lang="ts">
import type { DropdownMenuItem, NavigationMenuItem } from '@nuxt/ui'
import { computed, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useColorMode } from '@vueuse/core'
import WambdaLogo from '../WambdaLogo.vue'
import { useAuth } from '@/composables/useAuth'
import { useLocalStorage } from '@/composables/useLocalStorage'
import type { User } from '@/schemas/auth'
import { $api } from '@/composables/useAPI'

const route = useRoute()
const router = useRouter()
const colorMode = useColorMode()
const userStorage = useLocalStorage<User | null>('user', null, true)
const { setAuth } = useAuth()
const abortController = new AbortController();

const logout = async () => {
  await $api("/auth/logout", {method: "POST", credentials: 'include', signal: abortController.signal});
  userStorage.value = null
  setAuth(false)
  router.push({ name: 'main' })
}

const items = computed<NavigationMenuItem[][]>(() => [
  [
    {
      label: 'Overview',
      icon: 'i-lucide-house',
      active: route.path.startsWith('/dashboard'),
      to: '/dashboard',
    },
    {
      label: 'Functions',
      icon: 'i-lucide-square-function',
      active: route.path.startsWith('/functions'),
      to: '/functions',
    },
    {
      label: 'Runs',
      icon: 'i-lucide-play',
      active: route.path.startsWith('/runs'),
      to: '/runs',
    },
    {
      label: 'Settings',
      icon: 'i-lucide-settings',
      active: route.path.startsWith('/settings'),
      to: '/settings',
    },
  ],
])

const userItems = computed<DropdownMenuItem[][]>(() => [
  [
    {
      label: 'Appearance',
      icon: 'i-lucide-sun-moon',
      children: [
        {
          label: 'Light',
          icon: 'i-lucide-sun',
          type: 'checkbox',
          checked: colorMode.value === 'light',
          onUpdateChecked(checked: boolean) {
            if (checked) {
              colorMode.value = 'light'
            }
          },
          onSelect(e: Event) {
            e.preventDefault()
          },
        },
        {
          label: 'Dark',
          icon: 'i-lucide-moon',
          type: 'checkbox',

          checked: colorMode.value === 'dark',
          onUpdateChecked(checked: boolean) {
            if (checked) {
              colorMode.value = 'dark'
            }
          },
          onSelect(e: Event) {
            e.preventDefault()
          },
        },
      ],
    },
  ],
  [
    {
      label: 'Log out',
      icon: 'i-lucide-log-out',
      onSelect: logout,
    },
  ],
])

onUnmounted(() => {
  abortController.abort()
})
</script>

<template>
  <UDashboardSidebar
    collapsible
    resizable
    :ui="{ footer: 'border-t border-default', root: 'bg-muted' }"
  >
    <template #header="{ collapsed }">
      <WambdaLogo class="pl-1.5 h-5 w-auto shrink-0" />
      <div v-if="!collapsed" class="font-bold">Wambda</div>
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
      <UDropdownMenu
        :items="userItems"
        :content="{ align: 'center', collisionPadding: 12 }"
        :ui="{ content: 'w-(--reka-dropdown-menu-trigger-width) min-w-48' }"
      >
        <template #colorpicker></template>
        <UUser
          class="w-full"
          :name="userStorage?.fullName"
          :description="userStorage?.email ?? 'UNKNOWN'"
        />
      </UDropdownMenu>
    </template>
  </UDashboardSidebar>
</template>
