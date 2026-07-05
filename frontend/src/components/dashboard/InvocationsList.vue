<script setup lang="ts">
export type InvocationStat = {
  fn: string
  value: number
}

defineProps<{
  invocationStats: InvocationStat[]
}>()
</script>

<template>
  <UCard class="flex flex-col" :ui="{ body: 'bg-elevated/50 h-full' }">
    <div class="text-highlighted font-bold">Top functions by invocations</div>
    <div
      class="flex justify-between not-last:border-b pt-2"
      v-for="stat in invocationStats"
      :key="stat.fn"
    >
      <div>{{ stat.fn }}</div>
      <div>{{ stat.value }}</div>
    </div>
    <div v-if="invocationStats.length === 0">No invocations found</div>
  </UCard>
</template>
