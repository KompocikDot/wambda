<script setup lang="ts">
import { $api } from '@/composables/useAPI'
import { useClipboard } from '@vueuse/core'
import { onMounted, onUnmounted, reactive, ref } from 'vue'

const abortController = new AbortController()
const { copy } = useClipboard()

const loading = ref(false)
const show = ref(false)

const state = reactive({
  apiKey: '',
})


const apiKeyLoader = async (method: 'GET' | 'PATCH') => {
  loading.value = true

  try {
    const { apiKey } = await $api('/auth/api-key', {
      method: method,
      signal: abortController.signal,
      credentials: 'include',
    })

    state.apiKey = apiKey
  } catch (e) {
    console.log(e)
  }

  loading.value = false
}

const getApiKey = async () => {
  await apiKeyLoader('GET')
}

const regenerateApiKey = async () => {
  await apiKeyLoader('PATCH')
}

const showKey = () => {
  show.value = !show.value
}


onMounted(() => {
  getApiKey()
})

onUnmounted(() => {
  abortController.abort()
})
</script>

<template>
  <UCard class="w-1/3" variant="subtle">
    <UForm @submit="regenerateApiKey" :state="state" class="flex flex-col gap-y-2">
      <div class="text-highlighted font-bold">API key</div>
      <UFormField label="Current key">
        <UInput
          v-model="state.apiKey"
          :loading="loading"
          :type="show ? 'text' : 'password'"
          :ui="{ trailing: 'pe-1' }"
          class="w-full"
          disabled
        >
          <template #trailing>
            <div class="flex flex-row gap-x-0.5">
              <UButton
                color="neutral"
                variant="link"
                size="sm"
                icon="i-lucide-copy"
                @click="copy(state.apiKey)"
              />
              <UButton
                color="neutral"
                variant="link"
                size="sm"
                :icon="show ? 'i-lucide-eye-off' : 'i-lucide-eye'"
                :aria-label="show ? 'Hide password' : 'Show password'"
                :aria-pressed="show"
                aria-controls="password"
                @click="showKey"
              />
            </div>
          </template>
        </UInput>
      </UFormField>
      <UButton label="Regenerate key" type="submit" class="w-fit" :disabled="loading" />
    </UForm>
  </UCard>
</template>
