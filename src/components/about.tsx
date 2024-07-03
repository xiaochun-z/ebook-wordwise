import { Modal } from "flowbite-react";
import { useState, useEffect } from "react";
import { faHeart } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { Link } from "react-router-dom";
import { app } from "@tauri-apps/api";

export default function About() {
  const linkClassName =
    "text-blue-700 hover:text-blue-900 dark:text-red-300 dark:hover:text-red-100 hover:underline";
  const [openModal, setOpenModal] = useState(false);
  const [version, setVersion] = useState("");
  useEffect(() => {
    const fetchVersion = async () => {
      const appVersion = await app.getVersion();
      setVersion(appVersion);
    };

    fetchVersion();
  }, []);
  return (
    <>
      <li
        key={10086}
        onClick={() => setOpenModal(true)}
        className="rounded w-9 h-9 mt-3 mb-7 ms-auto me-auto bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 dark:focus:ring-blue-800 dark:hover:text-red-400 hover:text-red-700"
      >
        <Link
          to="#"
          className="flex flex-col items-center justify-center w-9 h-9"
          data-modal-target="default-modal"
          data-modal-toggle="default-modal"
        >
          <FontAwesomeIcon icon={faHeart} className="ml-auto mr-auto w-4 h-4" />
        </Link>
      </li>
      <Modal show={openModal} onClose={() => setOpenModal(false)}>
        <Modal.Header>About Ebook Wordwise {version}</Modal.Header>
        <Modal.Body>
          <div className="space-y-6 text-base leading-relaxed text-gray-500 dark:text-gray-400">
            <p>
              This program is using
              Tauri/React/TailWindCss/Flowbite/FontAwesome/Calibre. Before using
              this program, it is necessary to install{" "}
              <Link
                className={linkClassName}
                to="https://calibre-ebook.com/download"
                target="_blank"
              >
                calibre
              </Link>
              . Gratitude is extended to all the open source communities.
            </p>
            <p>
              If you have questions for using this app, please consider using
              this{" "}
              <Link
                className={linkClassName}
                to="https://github.com/xiaochun-z/ebook-wordwise/wiki"
                target="_blank"
              >
                Wiki Page
              </Link>
              .
            </p>
            <p>
              My family and I currently reside in China and are actively
              searching for new opportunities abroad. If you are interested in
              offering assistance, kindly reach out to me at{" "}
              <Link
                className={linkClassName}
                to="mailto:xiaochun.zh@outlook.com"
              >
                xiaochun.zh@outlook.com
              </Link>
              . Thank you.
            </p>
          </div>
        </Modal.Body>
        <Modal.Footer></Modal.Footer>
      </Modal>
    </>
  );
}
