const fileIconMap: { [extension: string]: string } = {
    "doc": "file-word-box",
    "json": "code-json",
    "toml": "file-code",
    "yaml": "file-code",
    "yml": "file-code",

    "md": "language-markdown", // may deprecated
    "html": "language-html5", // may deprecated
    "rs": "language-rust", // may deprecated
    "cpp": "language-cpp", // may deprecated
    "c++": "language-cpp", // may deprecated
    "c": "language-c", // may deprecated
    "py": "language-python", // may deprecated
    "go": "language-go", // may deprecated
    "js": "language-javascript", // may deprecated
    "css": "language-css3", // may deprecated
    "sh": "language-bash", // may deprecated
    "bash": "language-bash", // may deprecated
    "bat": "language-bash", // may deprecated
    "ts": "language-typescript", // may deprecated
    "tsx": "language-typescript", // may deprecated
    "java": "language-java", // may deprecated
    "haskell": "language-haskell", // may deprecated
}


export default function get_file_icon(file_name: string): string {
    const extension = get_file_extension(file_name)
    const icon = fileIconMap[extension] || "file-document"
    return "mdi-" + icon
}

function get_file_extension(file_name: string): string {
    return file_name.split('.').pop() || ''
}