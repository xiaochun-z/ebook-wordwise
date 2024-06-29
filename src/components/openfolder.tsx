import { faFolderOpen } from "@fortawesome/free-solid-svg-icons";
import { Link } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { invoke } from "@tauri-apps/api/tauri";

export default function OpenFolder() {
  const openFolderAndSelectFile = async () => {
    await invoke<boolean>("open_directory").catch((error) => {
      console.log(error);
    });
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
