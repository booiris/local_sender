import axios, { type AxiosResponse } from 'axios'

const service = axios.create({
    baseURL: import.meta.env.VITE_APP_API_BASE_URL
})

service.interceptors.response.use(
    (response: AxiosResponse): AxiosResponse => {
        if (response.status === 200) return response

        throw new Error(response.status.toString())
    },
    (error) => {
        return Promise.reject(error)
    }
)

export default service
