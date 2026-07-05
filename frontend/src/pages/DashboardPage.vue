<script setup lang="ts">
import type { InvocationStat } from '@/components/dashboard/InvocationsList.vue'
import InvocationsList from '@/components/dashboard/InvocationsList.vue'
import SidebarPanel from '@/components/dashboard/SidebarPanel.vue'
import OverviewStats from '@/components/dashboard/OverviewStats.vue'
import type { OverviewStat } from '@/components/dashboard/OverviewStats.vue'
import type { InvocationGraphPoint } from '@/components/dashboard/InvocationsGraph.vue'

const overviewStats: OverviewStat[] = [
  {
    header: 'Total invocations (24h)',
    value: 213.7,
    summary: '8.2% vs yesterday',
    summaryIcon: 'i-lucide-arrow-up',
    summaryColor: 'text-success',
  },
  {
    header: 'Avg latency',
    value: 3.4,
    valueUnit: 'ms',
    summary: '0.6ms vs yesterday',
    summaryIcon: 'i-lucide-arrow-down',
    summaryColor: 'text-success',
  },
  {
    header: 'Error rate',
    value: 0.12,
    valueUnit: '%',
    summary: '0.03% vs yesterday',
    summaryIcon: 'i-lucide-arrow-up',
    summaryColor: 'text-warning',
  },
  {
    header: 'Active functions',
    value: '11/14',
    summary: '3 disabled',
    summaryColor: 'text-muted',
  },
]

const invocationStats: InvocationStat[] = [
  {
    fn: 'webhook-relay',
    value: 301_004,
  },
  {
    fn: 'image-resize',
    value: 128_402,
  },
  {
    fn: 'auth-token-verify',
    value: 88_730,
  },
  {
    fn: 'pdf-generate',
    value: 42_918,
  },
]

const invocationTimeline: InvocationGraphPoint[] = [
  { date: new Date('2026-01-01'), totalInvocations: 0 },
  { date: new Date('2026-01-02'), totalInvocations: 5 },
  { date: new Date('2026-01-03'), totalInvocations: 59 },
  { date: new Date('2026-01-04'), totalInvocations: 124 },
  { date: new Date('2026-01-31'), totalInvocations: 180 },
  { date: new Date('2026-02-04'), totalInvocations: 210 },
  { date: new Date('2026-02-16'), totalInvocations: 300 },
]
</script>

<template>
  <UDashboardGroup>
    <SidebarPanel />
    <div class="flex flex-col w-full h-full">
      <UDashboardNavbar title="Overview" icon="i-lucide-house" class="w-full" />
      <UDashboardPanel class="px-6 py-4 gap-y-4">
        <OverviewStats :overview-stats="overviewStats" />
        <div class="flex gap-x-4">
          <div class="w-3/5 bg-elevated/50 ring ring-default rounded-lg p-4">
            <div class="text-highlighted font-bold">Invocations over time</div>
            <InvocationsGraph :usage-data="invocationTimeline" />
          </div>
          <InvocationsList class="w-2/5" :invocation-stats="invocationStats" />
        </div>
      </UDashboardPanel>
    </div>
  </UDashboardGroup>
</template>
