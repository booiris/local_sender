<script setup lang="ts">
import { reactive } from 'vue'
const data = reactive([
  {
    text: '192.168.31.1'
  },
  {
    text: '192.168.31.177'
  },
  {
    text: '192.168.31.3'
  }
])
</script>

<template>
  <v-col cols="auto">
    <v-card class="mx-auto pa-2" max-width="250">
      <v-list rounded="shaped">
        <v-list-header> Reciver </v-list-header>
        <v-list-item v-for="(item, i) in data" :key="i" :value="item" rounded="lg">
          <v-list-item-title>{{ item.text }}</v-list-item-title>
        </v-list-item>

        <v-divider class="my-2"></v-divider>

        <v-list-item link color="grey-lighten-4">
          <v-list-item-title> History </v-list-item-title>
        </v-list-item>

        <v-divider class="my-2"></v-divider>

        <v-list-header> Sender </v-list-header>
        <v-list-item v-for="(item, i) in data" :key="i" :value="item" rounded="lg">
          <v-list-item-title>{{ item.text }}</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-card>
  </v-col>
</template>
