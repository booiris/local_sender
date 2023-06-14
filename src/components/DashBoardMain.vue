<script setup lang="ts">
import { ref, onMounted } from 'vue'
import map from 'lodash/map'
import sortBy from 'lodash/sortBy'
import { FileInfo, Ls } from '../api/ls'
import get_file_icon from '../utils/icon/index'

const folders = ref<FileInfo[]>([])

const files = ref<FileInfo[]>([])

onMounted(() => {
    Ls()
        .then((res) => {
            res.dirs = map(res.dirs, function (x: FileInfo) {
                x.icon = 'mdi-folder'
                return x
            })
            res.dirs = sortBy(res.dirs, 'name')
            folders.value = res.dirs

            res.files = map(res.files, function (x: FileInfo) {
                x.icon = get_file_icon(x.name)
                return x
            })
            res.files = sortBy(res.files, 'name')
            files.value = res.files
        })
        .catch((err) => {
            throw err
        })
})
</script>

<template>
    <v-col>
        <v-card min-height="h-auto" rounded="lg">
            <v-container class="px-14 py-8">
                <v-list height="75dvh" class="overflow-y-auto" lines="two">
                    <v-list-subheader inset> Folders </v-list-subheader>

                    <v-list-item
                        v-for="folder in folders"
                        :key="folder.name"
                        :title="folder.name"
                        :subtitle="folder.modified_time"
                        link
                        rounded="lg"
                    >
                        <template v-slot:prepend>
                            <v-avatar color="light-blue-lighten-2">
                                <v-icon color="white"> {{ folder.icon }}</v-icon>
                            </v-avatar>
                        </template>

                        <template v-slot:append>
                            <v-btn
                                color="grey-lighten-1"
                                icon="mdi-information"
                                variant="text"
                            ></v-btn>
                        </template>
                    </v-list-item>

                    <v-divider inset></v-divider>

                    <v-list-subheader inset>Files</v-list-subheader>

                    <v-list-item
                        v-for="file in files"
                        :key="file.name"
                        :title="file.name"
                        :subtitle="file.modified_time"
                        link
                        rounded="lg"
                    >
                        <template v-slot:prepend>
                            <v-avatar color="light-blue-lighten-2">
                                <v-icon color="white">{{ file.icon }}</v-icon>
                            </v-avatar>
                        </template>

                        <template v-slot:append>
                            <v-btn
                                color="grey-lighten-1"
                                icon="mdi-information"
                                variant="text"
                            ></v-btn>
                        </template>
                    </v-list-item>
                </v-list>
            </v-container>
        </v-card>
    </v-col>
</template>
