<script setup lang="ts">
import { AppBar, DashBoardSideBar, DashBoardMain } from '@/components'
</script>

<template>
  <AppBar></AppBar>

  <v-main class="bg-blue-lighten-3">
    <v-container class="px-16">
      <v-row>
        <DashBoardSideBar></DashBoardSideBar>
        <DashBoardMain></DashBoardMain>
      </v-row>
    </v-container>
  </v-main>
</template>
