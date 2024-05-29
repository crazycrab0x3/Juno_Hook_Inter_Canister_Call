<script setup lang="ts">
import { signIn , NFIDProvider , InternetIdentityProvider } from '@junobuild/core'
import Button from '@/components/Button.vue'

const login = async () => {
  await signIn({
    provider: new InternetIdentityProvider({
      domain: "ic0.app"
    })
  });
}
</script>

<template>
  <Button @click="login">Sign in</Button>
</template>
