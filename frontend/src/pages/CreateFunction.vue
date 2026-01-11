<script lang="ts" setup>
import hljs from 'highlight.js'
import CodeEditor from 'simple-code-editor/CodeEditor.vue'
import { ref, watch, reactive } from 'vue'

const languagePicks = [
  { label: 'Javascript', value: ['javascript', 'Javascript'] },
  { label: 'Go', value: ['go', 'Golang'] },
  { label: 'Python', value: ['python', 'Python'] },
  { label: 'Rust', value: ['rust', 'Rust'] },
  { label: 'C', value: ['c', 'C'] },
]

const formState = reactive({
  code: '',
  language: languagePicks[0].value,
  name: '',
})

const createLambdaFunction = async (event) => {
  console.log(event)
}
</script>

<template>
  <Navbar />
  <UContainer>
    <div class="text-4xl">Create lambda function</div>

    <USeparator class="py-4" />

    <UForm @submit="createLambdaFunction" :state="formState" class="space-y-4">
      <div class="flex justify-between">
        <div class="flex space-x-2">
          <UFormField label="Name" class="w-1/2" required>
            <UInput v-model="formState.name" class="w-full" valueKey="value" />
          </UFormField>
          <UFormField label="Programming language" class="w-1/2" required>
            <USelectMenu
              :items="languagePicks"
              v-model="formState.language"
              class="w-full"
              valueKey="value"
            />
          </UFormField>
        </div>
        <UButton
          class="self-end"
          type="submit"
          label="Create"
          :disabled="formState.code.length === 0"
          variant="soft"
        />
      </div>
      <UFormField label="Code" required>
        <CodeEditor
          :key="formState.language[0]"
          v-model="formState.code"
          :highlight="hljs"
          :languages="[formState.language]"
          :line-nums="true"
          theme="atom-one-dark"
          width="100%"
          height="500px"
        />
      </UFormField>
    </UForm>
  </UContainer>
</template>
