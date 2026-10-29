<script setup lang="ts">
import type { ButtonProps } from '@nuxt/ui'
import TermSrc from '@/assets/term.svg'

const links: ButtonProps[] = [
  {
    label: 'Get started free',
  },
  {
    label: 'Read the docs',
    variant: 'outline',
    color: 'neutral',
  },
]
</script>

<template>
  <NavigationBar />
  <UMain>
    <UContainer>
      <UPageHero
        orientation="horizontal"
        title="Run WebAssembly functions in milliseconds."
        description="Wambda is a serverless platform for WASM functions: fast cold starts, sandboxed and safe by design. Upload a .wasm module and go live in seconds."
        :links="links"
      >
        <template #headline>
          <UBadge label="Built in Rust" class="rounded-full" variant="soft" />
        </template>

        <img :src="TermSrc" alt="Terminal Demo" class="w-full h-auto" />
      </UPageHero>

      <div class="flex">
        <UPageCard
          title="Fast cold starts"
          description="A lightweight Rust runtime boots each WASM module in no time"
          icon="i-lucide-rabbit"
          class="rounded-none rounded-tl-lg"
        />
        <UPageCard
          title="Sandboxed by default"
          description="Every function runs in an isolated WASM sandbox with no host access."
          icon="i-lucide-lock"
          class="rounded-none"
        />
        <UPageCard
          title="Any language, one target"
          description="Compile from Rust, AssemblyScript or C — deploy the same way."
          icon="i-lucide-code"
          class="rounded-none rounded-tr-lg"
        />
      </div>
    </UContainer>
  </UMain>
</template>
