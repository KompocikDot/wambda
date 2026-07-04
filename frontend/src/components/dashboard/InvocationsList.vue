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
  <div class="bg-elevated/50 ring ring-default rounded-lg p-4">
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
  </div>
</template>
