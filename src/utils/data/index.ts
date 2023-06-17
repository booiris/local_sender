export function base64StringToUint8Array(base64String: string): Uint8Array {
    const byteCharacters = atob(base64String)
    const bytes = new Uint8Array(byteCharacters.length);
    for (let i = 0; i < byteCharacters.length; i++) {
        bytes[i] = byteCharacters.charCodeAt(i);
    }
    return bytes;
}