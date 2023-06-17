import { get } from "@/utils/request"
import { BaseResponse } from "./base"

export interface PullReqeust {
    path: string
}

export enum PullMethod {
    Immediate = "Immediate",
    Stream = "Stream",
    Async = "Async",
    Empty = "Empty",
}

export interface PullResponse extends BaseResponse {
    data?: string,
    method: PullMethod,
    size: string,
    file_name: string,
}

export function Pull(path: string) {
    const req: PullReqeust = { path: path }
    return get<PullResponse>({
        url: '/pull',
        data: req
    })
}