<script setup lang="ts">
import { AppBar, DashBoardSideBar, DashBoardMain } from '@/components'
import { Ls } from '../api/ls'

Ls()
    .then((res) => {
        console.log(res)
    })
    .catch((e) => {
        console.error(e)
    })
</script>

<template>
    <AppBar />

    <v-main class="bg-blue-lighten-3">
        <v-container class="px-10">
            <v-row>
                <DashBoardSideBar />
                <DashBoardMain />
            </v-row>
        </v-container>
    </v-main>
</template>
