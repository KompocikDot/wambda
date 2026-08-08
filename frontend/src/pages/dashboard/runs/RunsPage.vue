<script setup lang="ts">
import SidebarPanel from '@/components/dashboard/SidebarPanel.vue'
import { computed, ref, resolveComponent, h, onMounted, onUnmounted } from 'vue'
import type { TableColumn, TableRow } from '@nuxt/ui'
import { useRouter } from 'vue-router'
import { $api } from '@/composables/useAPI'

const UBadge = resolveComponent('UBadge')
const router = useRouter()

const abortController = new AbortController()
const timeFilterOptions = [
  'Last 24 hours',
  'Last 3 days',
  'Last 7 days',
  'Last 30 days',
  'All time',
]

const runs = ref<Run[]>([])
const loadingRuns = ref(false)

type Run = {
  functionName: string
  runId: string
  status: string
  triggeredAt: number
  startedAt: number
  finishedAt: number
}

const columns: TableColumn<Run>[] = [
  {
    accessorKey: 'functionName',
    header: 'Function name',
    meta: {
      class: {
        td: 'text-highlighted font-bold',
      },
    },
  },
  {
    accessorKey: 'runId',
    header: 'Run ID',
  },
  {
    accessorKey: 'status',
    header: 'Status',
    cell: ({ row }) => {
      const label = row.getValue<string>('status')
      const capitalizedLabel = label[0]?.toUpperCase() + label.slice(1)
      const color = label === 'success' ? 'success' : 'error'

      return h(UBadge, {
        variant: 'subtle',
        label: capitalizedLabel,
        class: 'rounded-full',
        color,
      })
    },
  },
  {
    header: 'Duration',
    meta: {
      class: {
        td: 'text-highlighted',
      },
    },
    cell: ({ row }) => {
      const startedAt = row.original.startedAt
      const finishedAt = row.original.finishedAt
      const latency = finishedAt - startedAt

      if (latency > 60000) {
        return `${(latency / 60000).toFixed(1)}m ${(latency / 1000).toFixed(1)}s`
      }

      if (latency > 1000) {
        return `${(latency / 1000).toFixed(1)}s`
      }
      return `${latency}ms`
    },
  },
  {
    accessorKey: 'triggeredAt',
    header: 'Triggered at',
    cell: ({ row }) => {
      const timestamp = row.getValue<number>('triggeredAt')
      const date = new Date(timestamp)
      return `${date.toLocaleTimeString()} ${date.toLocaleDateString()}`
    },
  },
]

const globalTableFilter = ref('')
const statusFilter = ref('all')
const timeFilter = ref('Last 24 hours')
const filteredRuns = computed(() => {
  let statusFiltered: Run[]
  if (statusFilter.value === 'all') {
    statusFiltered = runs.value
  } else {
    statusFiltered = runs.value.filter((run) => run.status == statusFilter.value)
  }

  // const semiFiltered

  // ['Last 24 hours', 'Last 3 days', 'Last 7 days', 'Last 30 days', 'All time']
  const unixTimestampNow = new Date().getTime()
  switch (timeFilter.value) {
    case 'Last 24 hours':
      return statusFiltered.filter((run) => unixTimestampNow - run.triggeredAt < 86400000)
    case 'Last 3 days':
      return statusFiltered.filter((run) => unixTimestampNow - run.triggeredAt < 259200000)
    case 'Last 7 days':
      return statusFiltered.filter((run) => unixTimestampNow - run.triggeredAt < 604800000)
    case 'Last 30 days':
      return statusFiltered.filter((run) => unixTimestampNow - run.triggeredAt < 2592000000)
    default:
      return statusFiltered
  }
})

const setStatusFilter = (value: string) => {
  statusFilter.value = value
}

const onRowClick = (_: Event, row: TableRow<Run>) => {
  router.push({
    name: 'singleRun',
    params: { name: row.getValue<string>('functionName'), uuid: row.getValue<string>('runId') },
  })
}

const loadRuns = async () => {
  loadingRuns.value = true

  try {
    runs.value = await $api(`/runs`, { credentials: 'include', signal: abortController.signal })
  } catch (e) {
    console.error(e)
  }

  loadingRuns.value = false
}

onMounted(() => {
  loadRuns()
})

onUnmounted(() => {
  abortController.abort()
})
</script>

<template>
  <UDashboardGroup
    ><SidebarPanel />
    <div class="flex flex-col w-full h-full">
      <UDashboardNavbar title="Runs" icon="i-lucide-play" class="w-full" />
      <UDashboardPanel class="px-6 py-4 gap-y-4 flex flex-col min-h-0 h-full w-full">
        <div class="flex gap-x-2">
          <UInput
            icon="i-lucide-search"
            placeholder="Search functions..."
            v-model="globalTableFilter"
          />
          <USelect
            v-model="timeFilter"
            class="w-36"
            :items="timeFilterOptions"
            :default-value="timeFilter"
          />
          <UButton
            label="All"
            class="rounded-full"
            :variant="statusFilter === 'all' ? 'subtle' : 'ghost'"
            @click="setStatusFilter('all')"
          />
          <UButton
            label="Success"
            class="rounded-full"
            :variant="statusFilter === 'success' ? 'subtle' : 'ghost'"
            @click="setStatusFilter('success')"
          />
          <UButton
            label="Error"
            class="rounded-full"
            :variant="statusFilter === 'error' ? 'subtle' : 'ghost'"
            @click="setStatusFilter('error')"
          />
        </div>
        <UTable
          v-model:global-filter="globalTableFilter"
          sticky
          :data="filteredRuns"
          :loading="loadingRuns"
          :columns="columns"
          @select="onRowClick"
        />
      </UDashboardPanel>
    </div>
  </UDashboardGroup>
</template>
