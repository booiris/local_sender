import { get } from '@/utils/request'
import { BaseResponse } from './base'

export interface LsReqeust {
    path: string
}

export interface FileInfo {
    name: string,
    size: string,
    modified_time: string,
    icon: string,
}

export interface LsResponse extends BaseResponse {
    files: FileInfo[]
    dirs: FileInfo[]
}

export function Ls() {
    const req: LsReqeust = { path: './' }
    return get<LsResponse>({
        url: '/ls',
        data: req
    })
}
