<script setup lang="ts">
const lambdas = [
  { id: 1, name: 'XYZ', language: 'Go', createdAt: '01-12-2025' },
  { id: 2, name: 'ABC', language: 'Python', createdAt: '05-01-2024' },
  { id: 3, name: 'Random ass name', language: 'Javascript', createdAt: '21-01-2026' },
  { id: 4, name: 'i have no other ideas', language: 'C', createdAt: '31-01-2023' },
]
</script>

<template>
  <Navbar />
  <UContainer class="space-y-4">
    <div class="text-4xl">Your lambdas</div>
    <USeparator />
    <UScrollArea
      v-slot="{ item, index }"
      :items="lambdas"
      orientation="vertical"
      virtualize
      class="w-full h-96 outline rounded-sm outline-primary"
    >
      <UPageCard
        :title="item.name"
        :description="item.language"
        :variant="index % 2 === 0 ? 'soft' : 'outline'"
        class="rounded-none"
        :to="{ name: 'lambda-details', params: { uuid: item.id } }"
      />
    </UScrollArea>
  </UContainer>
</template>
