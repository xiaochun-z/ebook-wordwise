import { faFolderOpen } from "@fortawesome/free-solid-svg-icons";
import { Link } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { executableDir } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/api/shell";

export default function OpenFolder() {
  const openFolderAndSelectFile = async () => {
    try {
      const installationPath = await executableDir();
      console.log(installationPath);
      await open(installationPath);
    } catch (error) {
      console.error("Error selecting file:", error);
    }
  };
  return (
    <>
      <li
        key={10085}
        title="Open Resource Folder"
        onClick={openFolderAndSelectFile}
        className="rounded w-9 h-9 mt-3 mb-7 ms-auto me-auto bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 dark:focus:ring-blue-800 dark:hover:text-blue-50 hover:text-blue-700"
      >
        <Link
          to="#"
          className="flex flex-col items-center justify-center w-9 h-9"
          data-modal-target="default-modal"
          data-modal-toggle="default-modal"
        >
          <FontAwesomeIcon
            icon={faFolderOpen}
            className="ml-auto mr-auto w-4 h-4"
          />
        </Link>
      </li>
    </>
  );
}
