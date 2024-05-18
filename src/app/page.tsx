"use client"

import Image from "next/image";
import Sidebar from "../components/Sidebar"
import { SourceProvider } from "../context/SourceContext";
import dynamic from "next/dynamic";
import CodeArea from "@/components/CodeArea";

const TitleBar = dynamic(() => import('../components/Titlebar'), { ssr: false });

export default function Home() {
  return (
    <div className="wrapper">
      <TitleBar ></TitleBar>
      <div id="editor" className="bg-primary">
        <SourceProvider>
          <Sidebar />
          <CodeArea />
        </SourceProvider>
      </div>
    </div>
  );
}
