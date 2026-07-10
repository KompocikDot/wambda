<script setup lang="ts">
import SidebarPanel from '@/components/dashboard/SidebarPanel.vue'
import { $api } from '@/composables/useAPI'
import type { TableColumn, TableRow } from '@nuxt/ui'
import { computed, h, onMounted, ref, resolveComponent } from 'vue'
import { useRouter } from 'vue-router'

type Function = {
  uuid: string
  name: string
  language: string
  status: 'active' | 'degraded' | 'disabled'
  avgLatencyMs: number | null
  invocations: number
}

const statusToColour = {
  active: 'success',
  degraded: 'muted',
  disabled: 'warning',
}

const UBadge = resolveComponent('UBadge')
const router = useRouter()

const loading = ref<boolean>(false)
const functions = ref<Function[]>([])

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
        color: statusToColour[row.original.status] ?? 'primary',
        class: 'rounded-full',
      })
    },
  },
  {
    accessorKey: 'avgLatency',
    header: 'avg latency',
    cell: ({ row }) => {
      const latency = row.getValue<number | null>('avgLatency')
      if (latency === null) {
        return '-'
      }

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
    return functions.value
  }
  return functions.value.filter((fn) => fn.status === functionFilter.value)
})

const setFnFilter = (value: string) => {
  functionFilter.value = value
}

const onRowClick = (_: Event, row: TableRow<Function>) => {
  router.push({
    name: 'singleFunction',
    params: { uuid: row.original.uuid, name: row.getValue<string>('name') },
  })
}

const loadFunctions = async () => {
  loading.value = true

  try {
    functions.value = await $api('/functions', { credentials: 'include' })
  } catch (e) {
    console.error(e)
  }

  loading.value = false
}

onMounted(() => {
  loadFunctions()
})
</script>

<template>
  <UDashboardGroup>
    <SidebarPanel />
    <div class="flex flex-col w-full h-full">
      <UDashboardNavbar title="Functions" icon="i-lucide-square-function">
        <template #right>
          <UButton
            :to="{ name: 'singleFunction', params: { uuid: 'new', name: 'new' } }"
            icon="i-lucide-plus"
            label="New function"
          />
        </template>
      </UDashboardNavbar>
      <UDashboardPanel class="px-6 py-4 gap-y-4 flex flex-col min-h-0 h-full w-full">
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
          v-if="functions.length === 0 && !loading"
          icon="i-lucide-file"
          title="No functions found"
          description="It looks like you haven't added any functions. Create one to get started."
          :actions="[
            {
              icon: 'i-lucide-plus',
              label: 'Create new',
              to: { name: 'singleFunction', params: { uuid: 'new', name: 'new' } },
            },
            {
              icon: 'i-lucide-refresh-cw',
              label: 'Refresh',
              color: 'neutral',
              variant: 'subtle',
              onClick: loadFunctions,
            },
          ]"
        />
        <UTable
          v-else
          v-model:global-filter="globalTableFilter"
          sticky
          :data="filteredFunctions"
          :loading="loading"
          :columns="columns"
          @select="onRowClick"
          class="h-full"
        />
      </UDashboardPanel>
    </div>
  </UDashboardGroup>
</template>
