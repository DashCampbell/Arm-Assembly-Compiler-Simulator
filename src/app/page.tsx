"use client"

import Image from "next/image";
import Sidebar from "../components/Sidebar"
import { SourceProvider } from "../context/SourceContext";
import dynamic from "next/dynamic";
import CodeArea from "@/components/CodeArea";
import MemoryArea from "@/components/MemoryArea";
import Toolbar from "@/components/Toolbar";
import { AssemblySourceProvider } from "@/context/AssemblyContext";

const TitleBar = dynamic(() => import('../components/Titlebar'), { ssr: false });

export default function Home() {
  return (
    <div className="wrapper">
      <TitleBar ></TitleBar>
      <AssemblySourceProvider>
        <SourceProvider>
          <Toolbar />
          <div id="editor" className="bg-primary h-screen w-full">
            <Sidebar />
            <CodeArea />
            <MemoryArea />
          </div>
        </SourceProvider>
      </AssemblySourceProvider>
    </div>
  );
}
