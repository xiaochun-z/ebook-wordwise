import { platform } from "@tauri-apps/api/os";
import { useEffect, useState } from "react";

export default function Preview({ innerHTML }: PreviewProps) {
  const [maxHeight, setMaxHeight] = useState(215);
  const [clamp, setClamp] = useState("line-clamp-8");
  useEffect(() => {
    const setOSBasedMaxHeight = async () => {
      const currentPlatform = await platform();
      if (currentPlatform == "win32") {
        setMaxHeight(215);
      } else if (currentPlatform == "darwin") {
        setMaxHeight(180);
        setClamp("line-clamp-6");
      }
    };
    setOSBasedMaxHeight();
  }, []);

  return (
    <div
      dangerouslySetInnerHTML={{ __html: innerHTML }}
      style={{ maxHeight: maxHeight + "px", overflow: "hidden" }}
      id="preview"
      className={`overflow-ellipsis bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 p-4 font-normal text-gray-700 dark:text-gray-400 ${clamp}`}
    ></div>
  );
}

export interface PreviewProps {
  innerHTML: string;
}
