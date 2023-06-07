import { ref, computed, App } from 'vue'
import { defineStore } from 'pinia'
import { createPinia } from 'pinia'

export const useCounterStore = defineStore('counter', () => {
    const count = ref(0)
    const doubleCount = computed(() => count.value * 2)
    function increment() {
        count.value++
    }

    return { count, doubleCount, increment }
})

export const store = createPinia()
export function setupStore(app: App) {
    app.use(store)
}
