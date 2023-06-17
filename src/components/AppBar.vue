<script setup lang="ts">
const links = ['Dashboard', 'Setting']
</script>

<template>
  <v-app-bar flat>
    <v-container class="fill-height d-flex align-center">
      <v-btn v-for="link in links" :key="link" variant="text">
        {{ link }}
      </v-btn>

      <v-spacer></v-spacer>
    </v-container>
  </v-app-bar>
</template>
