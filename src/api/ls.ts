import { get } from '@/utils/request'
import { BaseResponse } from './base'

export interface LsReqeust {
    path: string
}

export interface LsResponse {
    files: string[]
    base: BaseResponse
}

export function Ls<T extends BaseResponse>() {
    const req: LsReqeust = { path: './' }
    return get<T>({
        url: '/ls',
        data: req
    })
}
