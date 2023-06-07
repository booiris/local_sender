export interface BaseResponse {
    code: number
    msg: string
}

export interface ErrorResponse {
    base: BaseResponse
}
