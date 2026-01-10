<script lang="ts" setup>
import hljs from 'highlight.js'
import CodeEditor from 'simple-code-editor/CodeEditor.vue'
import { ref, watch, reactive } from 'vue'

const languagePicks = [
  { label: 'javascript', value: ['javascript', 'JS'] },
  { label: 'go', value: ['go', 'GO'] },
  { label: 'python', value: ['python', 'PY'] },
  { label: 'rust', value: ['rust', 'RS'] },
  { label: 'c', value: ['c', 'C'] },
]

const formState = reactive({
  code: '',
})

const language = ref(languagePicks[0].value)

const createLambdaFunction = async (event) => {
  console.log(event)
}
</script>

<template>
  <UForm @submit="createLambdaFunction" :state="formState">
    <div class="flex justify-between">
      <UFormField label="Code language">
        <USelectMenu :items="languagePicks" v-model="language" class="w-full" valueKey="value" />
      </UFormField>
      <UButton type="submit" label="Create function" :disabled="formState.code.length === 0" />
    </div>
    <CodeEditor
      :key="language[0]"
      v-model="formState.code"
      :highlight="hljs"
      :languages="[language]"
      :line-nums="true"
      theme="atom-one-dark"
      width="100%"
      height="300px"
    />
  </UForm>
</template>
