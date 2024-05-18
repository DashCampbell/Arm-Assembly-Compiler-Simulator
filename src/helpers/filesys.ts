import { nanoid } from "nanoid"
import { invoke } from "@tauri-apps/api/tauri"
import { IFile } from "../types/file"
import { saveFileObject } from "../stores/files" // we'll defines this file below
import { resolve } from "path"

export const readFile = (filePath: string): Promise<string> => {
    return new Promise((resolve, reject) => {
        invoke("get_file_content", { filePath }).then((message: unknown) => {
            resolve(message as string);
        }).catch(error => reject(error))
    });
}

export const writeFile = (filePath: string, content: string): Promise<string> => {
    return new Promise((resolve, reject) => {
        invoke("write_file", { filePath, content }).then((message: unknown) => {
            if (message === 'OK') {
                resolve(message as string)
            } else {
                reject('WRITE ERROR')
            }
        })
    });
}

export const readDirectory = (folderPath: string): Promise<[IFile[], string]> => {
    return new Promise((resolve, reject) => {
        invoke<{files: string, folder: string}>("open_folder", { folderPath }).then(res => {
            const mess = res.files as string;
            const files = JSON.parse(mess.replaceAll('\\', '/').replaceAll('//', '/'));
            const entries: IFile[] = [];
            const folders: IFile[] = [];

            if (!files || !files.length) {
                resolve([entries, res.folder]);
                return;
            }
            for (let i = 0; i < files.length; ++i) {
                const file = files[i];
                const id = nanoid();
                const entry: IFile = {
                    id,
                    kind: file.kind,
                    name: file.name,
                    path: file.path
                };
                if (file.kind === 'file') {
                    entries.push(entry);
                } else {
                    folders.push(entry);
                }

                saveFileObject(id, entry);
            }
            resolve([[...folders, ...entries], res.folder]);
        })
    });
}