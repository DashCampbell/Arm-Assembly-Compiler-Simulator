"use client"

import Image from "next/image";
import Sidebar from "../components/Sidebar"
import { SourceProvider } from "../context/SourceContext";
import dynamic from "next/dynamic";
import CodeArea from "@/components/CodeArea";
import MemoryArea from "@/components/MemoryArea";
import Toolbar from "@/components/Toolbar";
import 'codemirror/lib/codemirror.css';

const TitleBar = dynamic(() => import('../components/Titlebar'), { ssr: false });

export default function Home() {
  return (
    <div className="wrapper">
      <TitleBar ></TitleBar>
      <SourceProvider>
        <Toolbar />
        <div id="editor" className="bg-primary h-screen">
          <Sidebar />
          <CodeArea />
          <MemoryArea />
        </div>
      </SourceProvider>
    </div>
  );
}
