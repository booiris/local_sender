<script setup lang="ts">
import { ref, onMounted } from 'vue'
import map from 'lodash/map'
import sortBy from 'lodash/sortBy'
import { FileInfo, Ls } from '../api/ls'
import get_file_icon from '../utils/icon/index'
import { base64StringToUint8Array } from '../utils/data/index';
import { Pull } from '@/api/pull'
import { PullMethod } from '../api/pull';
import { useFileSystemAccess } from '@vueuse/core'


const folders = ref<FileInfo[]>([])

const files = ref<FileInfo[]>([])

const now_path = ref<string>('./')

function click_back() {
    if (now_path.value == './') {
        return
    }
    now_path.value = now_path.value.replace(/\/[^/]+\/$/, '/')
    get_folders_and_files()
}

function click_dir(path: string) {
    if (path == "...") {
        return
    }
    now_path.value += path + '/'
    get_folders_and_files()
}

async function onCreate(file_name: string, data: string) {
    let byte_data = base64StringToUint8Array(data)
    const file_system = useFileSystemAccess({
        types: [{
            description: 'file',
            accept: {
                'file/*': ['.' + file_name.split('.').pop() as string],
            },
        }],
        excludeAcceptAllOption: false,
    })
    file_system.data.value = byte_data
    await file_system.saveAs({
        'suggestedName': file_name
    })
}

function click_file(path: string) {
    Pull(path)
        .then((res) => {
            switch (res.method) {
                case PullMethod.Immediate:
                    if (res.data) {
                        onCreate(res.file_name, res.data)
                    }
                    break
                default:
                    console.log(res)
            }
        })
        .catch((err) => {
            throw err
        })
}

function get_folders_and_files() {
    Ls(now_path.value)
        .then((res) => {
            res.dirs = map(res.dirs, function (x: FileInfo) {
                x.icon = 'mdi-folder'
                return x
            })
            res.dirs = sortBy(res.dirs, 'name')
            if (res.dirs.length == 0) {
                res.dirs.push({
                    name: '...',
                    modified_time: '',
                    icon: 'mdi-cancel',
                    size: ''
                })
            }
            folders.value = res.dirs

            res.files = map(res.files, function (x: FileInfo) {
                x.icon = get_file_icon(x.name)
                return x
            })
            res.files = sortBy(res.files, 'name')
            if (res.files.length == 0) {
                res.files.push({
                    name: '...',
                    modified_time: '',
                    icon: 'mdi-cancel',
                    size: ''
                })
            }
            files.value = res.files
        })
        .catch((err) => {
            throw err
        })
}

onMounted(() => {
    get_folders_and_files()
})
</script>

<template>
    <v-col>
        <v-card min-height="h-auto" rounded="lg">
            <v-container class="px-14 py-8">
                <v-list height="75dvh" class="overflow-y-auto" lines="two">
                    <v-list-header>

                        <v-btn variant="text" color="black" icon="mdi-arrow-left-circle" size="large"
                            @click="click_back"></v-btn>

                        {{ now_path }}
                    </v-list-header>

                    <v-list-subheader inset> Folders </v-list-subheader>

                    <v-list-item v-for="folder in folders" :key="folder.name" :title="folder.name"
                        :subtitle="folder.modified_time" link rounded="lg" @click="click_dir(folder.name)">
                        <template v-slot:prepend>
                            <v-avatar color="light-blue-lighten-2">
                                <v-icon color="white"> {{ folder.icon }}</v-icon>
                            </v-avatar>
                        </template>

                        <template v-slot:append>
                            <v-btn color="grey-lighten-1" icon="mdi-information" variant="text"></v-btn>
                        </template>
                    </v-list-item>

                    <v-divider inset></v-divider>

                    <v-list-subheader inset>Files</v-list-subheader>

                    <v-list-item v-for="file in files" :key="file.name" :title="file.name" :subtitle="file.modified_time"
                        @click="click_file(now_path + file.name)" link rounded="lg">
                        <template v-slot:prepend>
                            <v-avatar color="light-blue-lighten-2">
                                <v-icon color="white">{{ file.icon }}</v-icon>
                            </v-avatar>
                        </template>

                        <template v-slot:append>
                            <v-btn color="grey-lighten-1" icon="mdi-information" variant="text"></v-btn>
                        </template>
                    </v-list-item>
                </v-list>
            </v-container>
        </v-card>
    </v-col>
</template>
