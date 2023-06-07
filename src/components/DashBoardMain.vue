<script setup lang="ts">
import { reactive } from 'vue'
const folders = reactive([
  {
    subtitle: 'Jan 9, 2014',
    title: 'Photos'
  },
  {
    subtitle: 'Jan 17, 2014',
    title: 'Recipes'
  },
  {
    subtitle: 'Jan 28, 2014',
    title: 'Work'
  },
  {
    subtitle: 'Jan 9, 2014',
    title: 'Photos'
  },
  {
    subtitle: 'Jan 17, 2014',
    title: 'Recipes'
  },
  {
    subtitle: 'Jan 28, 2014',
    title: 'Work'
  }
])

const files = reactive([
  {
    color: 'blue',
    icon: 'mdi-clipboard-text',
    subtitle: 'Jan 20, 2014',
    title: 'Vacation itinerary'
  },
  {
    color: 'amber',
    icon: 'mdi-gesture-tap-button',
    subtitle: 'Jan 10, 2014',
    title: 'Kitchen remodel'
  },
  {
    color: 'amber',
    icon: 'mdi-gesture-tap-button',
    subtitle: 'Jan 10, 2014',
    title: 'Kitchen remodel'
  }
])




</script>

<template>
  <v-col>
    <v-card min-height="h-auto" rounded="lg">
      <v-container class="px-14 py-8">
        <v-list height="75dvh" class="overflow-y-auto" lines="two">
          <v-list-subheader inset> Folders </v-list-subheader>

          <v-list-item v-for="folder in folders" :key="folder.title" :title="folder.title" :subtitle="folder.subtitle"
            link rounded="lg">
            <template v-slot:prepend>
              <v-avatar color="grey-lighten-1">
                <v-icon color="white">mdi-folder</v-icon>
              </v-avatar>
            </template>

            <template v-slot:append>
              <v-btn color="grey-lighten-1" icon="mdi-information" variant="text"></v-btn>
            </template>
          </v-list-item>

          <v-divider inset></v-divider>

          <v-list-subheader inset>Files</v-list-subheader>

          <v-list-item v-for="file in files" :key="file.title" :title="file.title" :subtitle="file.subtitle" link
            rounded="lg">
            <template v-slot:prepend>
              <v-avatar :color="file.color">
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
