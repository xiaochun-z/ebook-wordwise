import { platform } from "@tauri-apps/api/os";
import { useEffect } from "react";

export default function Preview({ innerHTML }: PreviewProps) {
  let maxHeight = 215;
  let clamp = "line-clamp-8";
  useEffect(() => {
    const setOSBasedMaxHeight = async () => {
      const currentPlatform = await platform();
      if (currentPlatform === "win32") {
        maxHeight = 215;
      } else if (currentPlatform == "darwin") {
        maxHeight = 180;
        clamp = "line-clamp-6";
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
