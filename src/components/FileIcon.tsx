import html from '../assets/html.png';
import css from '../assets/css.png';
import react from '../assets/react.png';
import typescript from '../assets/typescript.png';
import binary from '../assets/binary.png';
import content from '../assets/content.png';
import git from '../assets/git.png';
import image from '../assets/image.png';
import nodejs from '../assets/nodejs.png';
import rust from '../assets/rust.png';
import js from '../assets/js.png';
import { StaticImageData } from 'next/image';

interface Icons {
    [key: string]: StaticImageData
}
const icons: Icons = {
    tsx: react,
    css: css,
    svg: image,
    png: image,
    icns: image,
    ico: image,
    gif: image,
    jpeg: image,
    jpg: image,
    tiff: image,
    bmp: image,
    ts: typescript,
    js: js,
    json: nodejs,
    md: content,
    lock: content,
    gitignore: git,
    html: html,
    rs: rust,
};
interface IFileIconProps {
    name: string;
    size?: 'sm' | 'base'
}

export default function FileIcon({ name, size = 'base' }: IFileIconProps) {
    const lastDotIndex = name.lastIndexOf('.');
    /** File extension, value is 'NONE' if no file extension*/
    const ext = lastDotIndex !== -1 ? name.slice(lastDotIndex + 1).toLowerCase() : 'NONE';
    const cls = size === 'base' ? 'w-4' : 'w-3';

    if (icons[ext]) {
        return <img src={icons[ext].src} alt={name} className={cls} />
    }
    return <img src={binary.src} alt={name} className={cls} />
}