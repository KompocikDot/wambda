<script setup lang="ts">
import { AnsiUp } from 'ansi_up'
import { computed } from 'vue'

const props = defineProps<{ rawIO: string }>()

const ansiUp = new AnsiUp()
const htmlIO = computed(() => ansiUp.ansi_to_html(props.rawIO))
</script>

<template>
  <div class="p-4 w-full h-full min-w-0 min-h-0 flex flex-col">
    <UCard
      class="flex-1 flex flex-col min-h-0 min-w-0"
      variant="outline"
      :ui="{ body: 'flex-1 min-h-0 min-w-0 overflow-auto' }"
    >
      <pre v-html="htmlIO" />
    </UCard>
  </div>
</template>
