import type { AxiosProgressEvent, AxiosResponse, GenericAbortSignal, AxiosError } from 'axios'
import service from './axios'
import { BaseResponse } from '@/api/base'

enum Method {
    Get,
    Post
}

interface HttpOption {
    url: string
    data?: any
    method?: Method
    headers?: any
    onDownloadProgress?: (progressEvent: AxiosProgressEvent) => void
    signal?: GenericAbortSignal
    beforeRequest?: () => void
    afterRequest?: () => void
}


// let onProgressId = 0

function http<T extends BaseResponse>({
    url,
    data,
    method,
    headers,
    onDownloadProgress,
    signal,
    beforeRequest,
    afterRequest
}: HttpOption) {
    const successHandler = (res: AxiosResponse<T>) => {
        if (res.status === 200 && res.data.base.code === 0) return res.data

        return Promise.reject(res.data)
    }

    const failHandler = (error: AxiosError) => {
        afterRequest?.()
        // const msg = isTauri ? error : error?.message
        // throw new Error((msg as string) || 'Error')
        const msg = "error code: " + error.code + " " + error.message
        throw new Error((msg as string) || 'Error')
    }

    beforeRequest?.()

    method = method || Method.Get

    const params = Object.assign(typeof data === 'function' ? data() : data ?? {}, {})

    //   if (isTauri) {
    //     let on_progress_id: string | undefined
    //     if (onDownloadProgress) {
    //       on_progress_id = String(onProgressId)
    //       onProgressId += 1
    //       if (!window.on_progress) window.on_progress = {}

    //       window.on_progress[on_progress_id] = onDownloadProgress
    //     } else {
    //       on_progress_id = undefined
    //     }
    //     return call(url, params, on_progress_id).then(
    //       (arg) => {
    //         if (on_progress_id) delete window.on_progress[on_progress_id]
    //         return successHandler(arg)
    //       },
    //       (arg) => {
    //         if (on_progress_id) delete window.on_progress[on_progress_id]
    //         return failHandler(arg)
    //       }
    //     )
    //   }


    return method === Method.Get
        ? service.get(url, { params, signal, onDownloadProgress, timeout: 1000 }).then(successHandler, failHandler)
        : service
            .post(url, params, { headers, signal, onDownloadProgress, timeout: 1000 })
            .then(successHandler, failHandler)
}

export function post<T extends BaseResponse>({
    url,
    data,
    method = Method.Post,
    headers,
    onDownloadProgress,
    signal,
    beforeRequest,
    afterRequest
}: HttpOption): Promise<T> {
    return http<T>({
        url,
        method,
        data,
        headers,
        onDownloadProgress,
        signal,
        beforeRequest,
        afterRequest
    })
}

export function get<T extends BaseResponse>({
    url,
    data,
    method = Method.Get,
    headers,
    onDownloadProgress,
    signal,
    beforeRequest,
    afterRequest
}: HttpOption): Promise<T> {
    return http<T>({
        url,
        method,
        data,
        headers,
        onDownloadProgress,
        signal,
        beforeRequest,
        afterRequest
    })
}
