<script setup lang="ts">
import { $api } from '@/composables/useAPI'
import {
  CreateUpdateFunctionSchema,
  type CreateUpdateFunctionInput,
  type CreateUpdateFunctionOutput,
  type EnvVarInput,
  type EnvVarOutput,
} from '@/schemas/function'
import type { FormSubmitEvent } from '@nuxt/ui'
import { computed, onMounted, reactive, useTemplateRef } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()
const form = useTemplateRef<HTMLFormElement>('form')
const loading = defineModel<boolean>('loading', { default: false })
const isNewForm = computed(() => route.params.uuid === 'new' && route.params.name === 'new')

const formState = reactive<CreateUpdateFunctionInput>({
  name: '',
  memoryLimit: 512,
  timeout: 300,
  envs: [],
  wasm: undefined as unknown as File,
})

const newEnvVariable = reactive<EnvVarInput>({
  key: '',
  value: '',
})

const addVariable = () => {
  if (!newEnvVariable.key.trim()) {
    return
  }
  formState.envs.push({
    key: newEnvVariable.key,
    value: newEnvVariable.value,
  })
  newEnvVariable.key = ''
  newEnvVariable.value = ''
}

const removeVariable = (index: number) => {
  formState.envs.splice(index, 1)
}

const clearFirstField = () => {
  newEnvVariable.key = ''
  newEnvVariable.value = ''
}

const stringifyEnvVariables = (envs: EnvVarOutput[]): string => {
  let stringifiedEnvs = ''
  for (const env of envs) {
    stringifiedEnvs += atob(env.key)
    stringifiedEnvs += ':'
    stringifiedEnvs += atob(env.value)
    stringifiedEnvs += ','
  }

  return stringifiedEnvs
}

const onSubmit = async (event: FormSubmitEvent<CreateUpdateFunctionOutput>) => {
  const method = isNewForm.value ? 'POST' : 'PATCH'
  const endpoint = isNewForm.value ? '/functions' : `/functions/${route.params.uuid}`

  const payload = new FormData()
  payload.append('name', event.data.name)
  payload.append('wasm', event.data.wasm)
  payload.append('memoryLimit', event.data.memoryLimit.toString())
  payload.append('timeout', event.data.timeout.toString())
  payload.append('envsStringified', stringifyEnvVariables(event.data.envs))

  try {
    await $api(endpoint, {
      credentials: 'include',
      method,
      body: payload,
    })
  } catch (e) {
    console.error(e)
  }
}

const onLoad = async () => {
  loading.value = true

  try {
    const functionData = await $api(`/functions/${route.params.uuid}`, { credentials: 'include' })
    formState.name = functionData.name
    formState.memoryLimit = functionData.memoryLimit
    formState.timeout = functionData.timeout
    formState.envs = functionData.envs
  } catch (e) {
    console.error(e)
  }

  loading.value = false
}

defineExpose({
  submit: () => form.value?.submit(),
})

onMounted(() => {
  if (!isNewForm.value) {
    onLoad()
  }
})
</script>

<template>
  <UForm
    ref="form"
    class="flex gap-x-4"
    :state="formState"
    @submit="onSubmit"
    :schema="CreateUpdateFunctionSchema"
  >
    <div class="flex flex-col w-2/3 gap-y-2">
      <UFormField label="Function name" class="w-full" name="name" required>
        <UInput
          class="w-full"
          placeholder="New, unique function name"
          :loading="loading"
          :disabled="loading"
          v-model="formState.name"
        />
      </UFormField>
      <div class="flex gap-x-2">
        <UFormField label="Memory limit (Mb)" class="w-1/2" name="memoryLimit" required>
          <UInput
            class="w-full"
            type="number"
            step="1"
            min="1"
            max="2048"
            placeholder="0"
            :loading="loading"
            :disabled="loading"
            v-model="formState.memoryLimit"
          />
        </UFormField>
        <UFormField label="Timeout (s)" class="w-1/2" name="timeout" required>
          <UInput
            class="w-full"
            type="number"
            step="1"
            min="0"
            placeholder="0"
            :loading="loading"
            :disabled="loading"
            v-model="formState.timeout"
          />
        </UFormField>
      </div>
      <div class="flex flex-col gap-2">
        <UFormField label="Environment variables">
          <div class="flex flex-col gap-y-2">
            <div
              class="flex items-center gap-x-2"
              v-for="(variable, index) in formState.envs"
              :key="index"
            >
              <UInput
                class="w-full"
                :loading="loading"
                :disabled="loading"
                v-model="variable.key"
              />
              <UInput
                class="w-full"
                :loading="loading"
                :disabled="loading"
                v-model="variable.value"
              />
              <UButton icon="i-lucide-x" variant="outline" @click="removeVariable(index)" />
            </div>
            <div class="flex items-center gap-x-2">
              <UInput
                class="w-full"
                placeholder="Key"
                :disabled="loading"
                :loading="loading"
                v-model="newEnvVariable.key"
              />
              <UInput
                class="w-full"
                placeholder="Value"
                :disabled="loading"
                :loading="loading"
                v-model="newEnvVariable.value"
              />
              <UButton
                icon="i-lucide-x"
                variant="outline"
                :disabled="
                  loading ||
                  formState.envs.length !== 0 ||
                  (newEnvVariable.key === '' && newEnvVariable.value === '')
                "
                @click="clearFirstField"
              />
            </div>
          </div>
        </UFormField>
        <UButton
          label="Add variable"
          class="w-fit"
          @click="addVariable"
          :disabled="loading || newEnvVariable.key.trim() === ''"
          icon="i-lucide-plus"
          variant="outline"
        />
      </div>
    </div>
    <div class="w-1/3">
      <UFormField label="Module (.wasm)" name="wasm" required>
        <UFileUpload
          accept="application/wasm"
          highlight
          label="Drag & drop your .wasm file"
          description="or click to browse"
          layout="list"
          :disabled="loading"
          :loading="loading"
          v-model="formState.wasm"
        />
      </UFormField>
    </div>
  </UForm>
</template>
