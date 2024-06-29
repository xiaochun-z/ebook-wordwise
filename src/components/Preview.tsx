import { platform } from "@tauri-apps/api/os";
import { useEffect, useState } from "react";
import "./preview.css";

export default function Preview({ innerHTML }: PreviewProps) {
  const [maxHeight, setMaxHeight] = useState(215);
  useEffect(() => {
    const setOSBasedMaxHeight = async () => {
      const currentPlatform = await platform();
      if (currentPlatform == "win32") {
        setMaxHeight(215);
      } else if (currentPlatform == "darwin") {
        setMaxHeight(190);
      }
    };
    setOSBasedMaxHeight();
  }, []);

  return (
    <div
      style={{ height: maxHeight + "px", overflow: "hidden" }}
      className="preview overflow-ellipsis bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 p-4 font-normal text-gray-700 dark:text-gray-400"
    >
      <div dangerouslySetInnerHTML={{ __html: innerHTML }}></div>
    </div>
  );
}

export interface PreviewProps {
  innerHTML: string;
}
