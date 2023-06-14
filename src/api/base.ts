export interface BaseResponseInfo {
    code: number
    msg: string
}
export interface BaseResponse {
    base: BaseResponseInfo
}

export interface ErrorResponse {
    base: BaseResponse
}
