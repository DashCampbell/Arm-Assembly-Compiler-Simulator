import { useSource } from "@/context/SourceContext";
import { IFile } from "@/types/file";
import FileIcon from "./FileIcon";
import { Icon } from "@iconify/react/dist/iconify.js";

interface Props {
    file: IFile;
    active: string;
    id: string;
    save: boolean;
}

export default function Tab({ file, active, id, save }: Props) {
    const { opened, setSelect, delOpenedFile } = useSource();

    const onSelectItem = (id: string) => {
        setSelect(id);
    };
    const close = (ev: React.MouseEvent<Element>, id: string) => {
        ev.stopPropagation();
        delOpenedFile(id);
    }
    return (
        <div onClick={() => onSelectItem(file.id)} className={`tab-item shrink-0 px-3 py-1.5 text-gray-500 cursor-pointer hover:text-gray-400 flex items-center gap-2 ${active}`} key={id}>
            <FileIcon name={file.name} size="sm" />
            <span>{file.name}</span>
            <i onClick={(ev) => close(ev, id)} className={file.id + " hover:text-red-400"}>
                <Icon icon={save ? "entypo:dot-single" : "ri:close-fill"} />
            </i>
        </div>
    )
}