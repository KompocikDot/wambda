<script setup lang="ts">
import SidebarPanel from '@/components/dashboard/SidebarPanel.vue'
import type { TableColumn } from '@nuxt/ui'
import { computed, h, ref, resolveComponent } from 'vue'

const UBadge = resolveComponent('UBadge')

type Function = {
  name: string
  language: string
  status: string
  statusColor: string
  avgLatencyMs: number
  invocations: number
}

const functions: Function[] = [
  {
    name: 'image-resize',
    language: 'rust',
    status: 'active',
    statusColor: 'success',
    avgLatencyMs: 1.8,
    invocations: 128_402,
  },
  {
    name: 'pdf-generate',
    language: 'rust',
    status: 'active',
    statusColor: 'success',
    avgLatencyMs: 6.2,
    invocations: 42_918,
  },
  {
    name: 'webhook-relay',
    language: 'assemblyScript',
    status: 'active',
    statusColor: 'success',
    avgLatencyMs: 0.9,
    invocations: 301_004,
  },
  {
    name: 'auth-token-verify',
    language: 'rust',
    status: 'degraded',
    statusColor: 'warning',
    avgLatencyMs: 1400.1,
    invocations: 88_730,
  },
  {
    name: 'csv-transform',
    language: 'c',
    status: 'disabled',
    statusColor: 'neutral',
    avgLatencyMs: 3.4,
    invocations: 0,
  },
]
const columns: TableColumn<Function>[] = [
  {
    accessorKey: 'name',
    header: 'name',
    meta: {
      class: {
        td: 'text-highlighted font-bold',
      },
    },
  },
  {
    accessorKey: 'language',
    header: 'language',
    cell: ({ row }) => {
      const label = row.getValue<string>('language')
      const capitalizedLabel = label[0]?.toUpperCase() + label.slice(1)
      return capitalizedLabel
    },
  },
  {
    accessorKey: 'status',
    header: 'status',
    cell: ({ row }) => {
      const label = row.getValue<string>('status')
      const capitalizedLabel = label[0]?.toUpperCase() + label.slice(1)
      return h(UBadge, {
        variant: 'subtle',
        label: capitalizedLabel,
        color: row.original.statusColor,
        class: 'rounded-full',
      })
    },
  },
  {
    accessorKey: 'avgLatencyMs',
    header: 'avg latency',
    cell: ({ row }) => {
      const latency = row.getValue<number>('avgLatencyMs')
      if (latency > 1000) {
        return `${(latency / 1000).toFixed(1)}s`
      }
      return `${latency}ms`
    },
  },
  {
    accessorKey: 'invocations',
    header: 'invocations (24h)',
    cell: ({ row }) => row.getValue<number>('invocations').toLocaleString(),
  },
]

const functionFilter = ref('all')
const globalTableFilter = ref('')

const filteredFunctions = computed(() => {
  if (functionFilter.value === 'all') {
    return functions
  }
  return functions.filter((fn) => fn.status === functionFilter.value)
})

const setFnFilter = (value: string) => {
  functionFilter.value = value
}
</script>

<template>
  <UDashboardGroup>
    <SidebarPanel />
    <div class="flex flex-col w-full h-full">
      <UDashboardNavbar title="Functions" icon="i-lucide-square-function">
        <template #right>
          <UButton to="/functions/new" icon="i-lucide-plus" label="New function" />
        </template>
      </UDashboardNavbar>
      <UDashboardPanel class="px-6 py-4 gap-y-4">
        <div class="flex gap-x-2">
          <UInput
            icon="i-lucide-search"
            placeholder="Search functions..."
            v-model="globalTableFilter"
          />
          <UButton
            label="All"
            class="rounded-full"
            :variant="functionFilter === 'all' ? 'subtle' : 'ghost'"
            @click="setFnFilter('all')"
          />
          <UButton
            label="Active"
            class="rounded-full"
            :variant="functionFilter === 'active' ? 'subtle' : 'ghost'"
            @click="setFnFilter('active')"
          />
          <UButton
            label="Degraded"
            class="rounded-full"
            :variant="functionFilter === 'degraded' ? 'subtle' : 'ghost'"
            @click="setFnFilter('degraded')"
          />
          <UButton
            label="Disabled"
            class="rounded-full"
            :variant="functionFilter === 'disabled' ? 'subtle' : 'ghost'"
            @click="setFnFilter('disabled')"
          />
        </div>
        <UEmpty
          v-if="functions.length === 0"
          icon="i-lucide-file"
          title="No functions found"
          description="It looks like you haven't added any functions. Create one to get started."
          :actions="[
            {
              icon: 'i-lucide-plus',
              label: 'Create new',
            },
            {
              icon: 'i-lucide-refresh-cw',
              label: 'Refresh',
              color: 'neutral',
              variant: 'subtle',
            },
          ]"
        />
        <UTable
          v-else
          v-model:global-filter="globalTableFilter"
          :data="filteredFunctions"
          :columns="columns"
        />
      </UDashboardPanel>
    </div>
  </UDashboardGroup>
</template>
