<script setup lang="ts">
defineProps<{
  title: string
  confirmColor: "success" | "error" | "primary"
}>()

const emit = defineEmits<{ close: [boolean] }>()
</script>

<template>
  <UModal
    :close="{ onClick: () => emit('close', false) }"
    :title="title"
  >
    <template #footer>
      <div class="flex gap-2">
        <UButton color="neutral" label="Abort" @click="emit('close', false)" />
        <UButton :color="confirmColor" label="Confirm" @click="emit('close', true)" />
      </div>
    </template>
  </UModal>
</template>
