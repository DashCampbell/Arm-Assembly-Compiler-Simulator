'use client'

import { useState } from "react";
import { appWindow } from "@tauri-apps/api/window";
import { Icon } from '@iconify/react';

export default function Titlebar() {
    const [isScaleup, setScaleup] = useState(false);
    // .minimize() - to minimize the window
    const onMinimize = () => appWindow.minimize();
    const onScaleup = () => {
        // .toggleMaximize() - to swap the window between maximize and minimum
        appWindow.toggleMaximize();
        setScaleup(true);
    }
    const onScaledown = () => {
        appWindow.toggleMaximize();
        setScaleup(false);
    }
    // .close() - to close the window
    const onClose = () => appWindow.close();

    return (
        <div id="titlebar" data-tauri-drag-region>
            <div className="flex items-center gap-1 5 pl-2">
                <img src="/tauri.svg" style={{ width: 10 }} alt="" />
                <span className="text-xs uppercase"></span>
            </div>
            <div className="titlebar-actions">
                <i className="titlebar-icon" onClick={onMinimize}><Icon icon="ri:subtract-line" /></i>
                {isScaleup ? <i className="titlebar-icon" onClick={onScaledown}><Icon icon="mage:scale-down" /></i> :
                    <i className="titlebar-icon" onClick={onScaleup}><Icon icon="mage:scale-up" /></i>}
                <i id="ttb-close" className="titlebar-icon" onClick={onClose}><Icon icon="ri:close-fill" /></i>
            </div>
        </div>
    );
}