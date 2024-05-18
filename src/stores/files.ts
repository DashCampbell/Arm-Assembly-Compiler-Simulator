import { IFile } from "../types/file"
// Ex: {
//   "34sdjwyd3": {
//     "id": "34sdjwyd3",
//     "name": "App.tsx",
//     "kind": "file",
//     "path": "d://path/to/App.tsx",
//   },
//   "872dwehud": {
//     "id": "872dwehud",
//     "name": "components",
//     "kind": "directory",
//     "path": "d://path/to/components",
//   }
// }

interface IEntries {
    [key: string]: IFile
}
const entries: IEntries = {}

export const saveFileObject = (id: string, file: IFile): void => {
    entries[id] = file;
}
export const getFileObject = (id: string): IFile => {
    return entries[id];
}