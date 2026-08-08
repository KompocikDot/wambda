<script setup lang="ts">
import InvocationsList, { type InvocationStat } from '@/components/dashboard/InvocationsList.vue'
import SidebarPanel from '@/components/dashboard/SidebarPanel.vue'
import OverviewStats from '@/components/dashboard/OverviewStats.vue'
import type { OverviewStat } from '@/components/dashboard/OverviewStats.vue'
import type { InvocationGraphPoint } from '@/components/dashboard/InvocationsGraph.vue'
import { computed, onMounted, onUnmounted, reactive, ref } from 'vue'
import { $api } from '@/composables/useAPI'

const abortController = new AbortController()

const loadingStats = ref(true)
const stats = reactive<{
  invocations: InvocationStat[]
  invocationTimeline: InvocationGraphPoint[]
  metrics: {
    avgLatencyMs: number
    errorRateChangePct: number
    errorRatePct: number
    invocationsChangePct: number
    latencyChangeMs: number
    totalInvocations: number
  }
}>({
  invocations: [],
  invocationTimeline: [],
  metrics: {
    avgLatencyMs: 0,
    errorRateChangePct: 0,
    errorRatePct: 0,
    invocationsChangePct: 0,
    latencyChangeMs: 0,
    totalInvocations: 0,
  },
})

const overviewStats: OverviewStat[] = computed(() => {
  return [
    {
      header: 'Total invocations (24h)',
      value: stats.metrics.totalInvocations,
      summary: `${stats.metrics.invocationsChangePct}% vs yesterday`,
      summaryIcon:
        stats.metrics.invocationsChangePct > 0 ? 'i-lucide-arrow-up' : 'i-lucide-arrow-down',
      summaryColor: stats.metrics.invocationsChangePct > 0 ? 'text-success' : 'text-error',
    },
    {
      header: 'Avg latency',
      value: stats.metrics.avgLatencyMs,
      valueUnit: 'ms',
      summary: `${stats.metrics.latencyChangeMs} ms vs yesterday`,
      summaryIcon: stats.metrics.latencyChangeMs > 0 ? 'i-lucide-arrow-up' : 'i-lucide-arrow-down',
      summaryColor: stats.metrics.latencyChangeMs > 0 ? 'text-error' : 'text-success',
    },
    {
      header: 'Error rate',
      value: stats.metrics.errorRatePct,
      valueUnit: '%',
      summary: `${stats.metrics.errorRateChangePct}% vs yesterday`,
      summaryIcon: stats.metrics.errorRateChangePct > 0 ? 'i-lucide-arrow-up' : 'i-lucide-arrow-down',
      summaryColor: stats.metrics.errorRateChangePct > 0 ? 'text-error' : 'text-success',
    },
    {
      header: 'Active functions',
      value: '11/14',
      summary: '3 disabled',
      summaryColor: 'text-muted',
    },
  ]
})

const fetchStats = async () => {
  loadingStats.value = true

  try {
    const statsResp = await $api('/stats', {
      credentials: 'include',
      signal: abortController.signal,
    })

    stats.invocations = statsResp.invocations
    stats.invocationTimeline = statsResp.usageOverTime
    stats.metrics = statsResp.metrics
  } catch (e) {
    console.error(e)
  }

  loadingStats.value = false
}

onMounted(() => {
  fetchStats()
})
onUnmounted(() => {
  abortController.abort()
})
</script>

<template>
  <UDashboardGroup>
    <SidebarPanel />
    <div class="flex flex-col w-full h-full">
      <UDashboardNavbar title="Overview" icon="i-lucide-house" class="w-full" />
      <UDashboardPanel class="flex-1 min-h-0 overflow-y-auto px-6 py-4 gap-y-4">
        <OverviewStats :overview-stats="overviewStats" :loading="loadingStats" />
        <div class="flex gap-4 flex-col md:flex-row">
          <UCard class="w-full md:w-3/5 h-full">
            <div class="text-highlighted font-bold">Invocations over time</div>
            <InvocationsGraph :loading="loadingStats" :usage-data="stats.invocationTimeline" />
          </UCard>
          <InvocationsList :loading="loadingStats" class="w-full md:w-2/5" :invocations="stats.invocations" />
        </div>
      </UDashboardPanel>
    </div>
  </UDashboardGroup>
</template>
