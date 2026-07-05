<script setup lang="ts">
import SidebarPanel from '@/components/dashboard/SidebarPanel.vue'
import { computed, ref, resolveComponent, h } from 'vue'
import type { TableColumn, TableRow } from '@nuxt/ui'
import { useRouter } from 'vue-router'

const UBadge = resolveComponent('UBadge')
const router = useRouter()

type Function = {
  name: string
  runId: string
  status: string
  statusColor: string
  duration: number | string
  triggered: string
}

const timeFilterOptions = ['Last 24 hours', 'Last 3 days', 'Last 7 days', 'Last month', 'All time']
const functions: Function[] = [
  {
    name: 'image-resize',
    runId: 'run_123312',
    status: 'success',
    statusColor: 'success',
    duration: 1.8,
    triggered: '2 min ago',
  },
  {
    name: 'webhook-relay',
    runId: 'run_2041fc',
    status: 'success',
    statusColor: 'success',
    duration: 1.8,
    triggered: '3 min ago',
  },
  {
    name: 'auth-token-verify',
    runId: 'run_00fac1',
    status: 'error',
    statusColor: 'error',
    duration: 'timeout',
    triggered: '6 min ago',
  },
  {
    name: 'pdf-generate',
    runId: 'run_54ff13',
    status: 'success',
    statusColor: 'success',
    duration: 1.8,
    triggered: '9 min ago',
  },
  {
    name: 'image-resize',
    runId: 'run_00adc1',
    status: 'error',
    statusColor: 'error',
    duration: 'panic',
    triggered: '14 min ago',
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
    accessorKey: 'runId',
    header: 'run id',
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
    accessorKey: 'duration',
    header: 'duration',
    meta: {
      class: {
        td: 'text-highlighted',
      },
    },
    cell: ({ row }) => {
      const latency = row.getValue<number | string>('duration')
      if (typeof latency === 'string') {
        return latency
      }

      if (latency > 1000) {
        return `${(latency / 1000).toFixed(1)}s`
      }
      return `${latency}ms`
    },
  },
  {
    accessorKey: 'triggered',
    header: 'triggered',
  },
]

const globalTableFilter = ref('')
const functionFilter = ref('all')
const timeFilter = ref('Last 24 hours')
const filteredFunctions = computed(() => {
  if (functionFilter.value === 'all') {
    return functions
  }
  return functions.filter((fn) => fn.status === functionFilter.value)
})

const setFnFilter = (value: string) => {
  functionFilter.value = value
}

const onRowClick = (_: Event, row: TableRow<Function>) => {
  router.push({ name: 'singleRun', params: { id: row.getValue<string>('name') } })
}
</script>

<template>
  <UDashboardGroup
    ><SidebarPanel />
    <div class="flex flex-col w-full h-full">
      <UDashboardNavbar title="Runs" icon="i-lucide-play" class="w-full" />
      <UDashboardPanel class="px-6 py-4 gap-y-4">
        <div class="flex gap-x-2">
          <UInput
            icon="i-lucide-search"
            placeholder="Search functions..."
            v-model="globalTableFilter"
          />
          <USelect class="w-36" :items="timeFilterOptions" :default-value="timeFilter" />
          <UButton
            label="All"
            class="rounded-full"
            :variant="functionFilter === 'all' ? 'subtle' : 'ghost'"
            @click="setFnFilter('all')"
          />
          <UButton
            label="Success"
            class="rounded-full"
            :variant="functionFilter === 'success' ? 'subtle' : 'ghost'"
            @click="setFnFilter('success')"
          />
          <UButton
            label="Error"
            class="rounded-full"
            :variant="functionFilter === 'error' ? 'subtle' : 'ghost'"
            @click="setFnFilter('error')"
          />
        </div>
        <UTable
          v-model:global-filter="globalTableFilter"
          :data="filteredFunctions"
          :columns="columns"
          @select="onRowClick"
        />
      </UDashboardPanel>
    </div>
  </UDashboardGroup>
</template>
