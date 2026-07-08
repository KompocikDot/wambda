<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()

const redirect = () => {
  router.push({ name: 'main' })
}
</script>

<template>
  <UError
    :clear="{ onClick: redirect }"
    :error="{
      statusCode: 404,
      statusMessage: 'Page not found',
      message: 'The page you are looking for does not exist.',
    }"
  />
</template>
