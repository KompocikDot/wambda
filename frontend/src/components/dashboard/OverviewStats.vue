<script setup lang="ts">
export type OverviewStat = {
  header: string
  value: number | string
  valueUnit?: string
  summary: string
  summaryIcon?: string
  summaryColor: string
}

defineProps<{
  overviewStats: OverviewStat[]
  loading: boolean
}>()
</script>

<template>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 w-full">
    <UCard v-for="stat in overviewStats" :key="stat.header">
      <div class="text-muted text-sm">{{ stat.header }}</div>
      <USkeleton v-if="loading" class="h-8 w-full" />
      <div v-else class="text-3xl font-bold text-highlighted flex">
        <div>{{ stat.value.toLocaleString() }}</div>
        <div v-if="stat.valueUnit !== undefined">{{ stat.valueUnit }}</div>
      </div>
      <USkeleton v-if="loading" class="h-8 w-full" />
      <div v-else class="text-sm flex items-center" :class="stat.summaryColor">
        <UIcon v-if="stat.summaryIcon !== undefined" :name="stat.summaryIcon" />
        <div>{{ stat.summary }}</div>
      </div>
    </UCard>
  </div>
</template>
