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
}>()
</script>

<template>
  <div class="flex gap-x-4 w-full">
    <div
      class="bg-elevated/50 ring ring-default rounded-lg p-4 w-full"
      v-for="stat in overviewStats"
      :key="stat.header"
    >
      <div class="text-muted text-sm">{{ stat.header }}</div>
      <div class="text-3xl font-bold text-highlighted flex">
        <div>{{ stat.value.toLocaleString() }}</div>
        <div v-if="stat.valueUnit !== undefined">{{ stat.valueUnit }}</div>
      </div>
      <div class="text-sm flex items-center" :class="stat.summaryColor">
        <UIcon v-if="stat.summaryIcon !== undefined" :name="stat.summaryIcon" />
        <div>{{ stat.summary }}</div>
      </div>
    </div>
  </div>
</template>
