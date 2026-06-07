import { Modal, ModalBody, Button } from "flowbite-react";
import { useState, useEffect } from "react";
import { faHeart, faBookOpen } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { Link } from "react-router-dom";
import { app } from "@tauri-apps/api";

export default function About() {
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
        className="rounded w-9 h-9 mt-3 mb-7 ms-auto me-auto bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 dark:focus:ring-blue-800 dark:hover:text-red-400 hover:text-red-700 cursor-pointer flex items-center justify-center transition-colors duration-200 shadow-sm"
      >
        <FontAwesomeIcon icon={faHeart} className="w-4 h-4" />
      </li>
      
      <Modal show={openModal} onClose={() => setOpenModal(false)} dismissible size="sm" popup>
        <ModalBody className="p-6">
          <div className="text-center">
            <FontAwesomeIcon icon={faHeart} className="mx-auto mb-4 h-14 w-14 text-red-500 dark:text-red-400" />
            <h3 className="mb-2 text-2xl font-bold text-gray-900 dark:text-white">
              Ebook Wordwise <span className="text-blue-600 dark:text-blue-400">v{version}</span>
            </h3>
            
            <div className="mb-6 text-sm font-normal text-gray-500 dark:text-gray-400 space-y-4">
              <p>
                Powered by Tauri, React, Tailwind CSS, Flowbite, FontAwesome, and Calibre.
              </p>
              <div className="p-3 bg-blue-50 dark:bg-gray-800 rounded-lg text-blue-800 dark:text-blue-300 border border-blue-100 dark:border-gray-700">
                <span className="font-semibold">Requirement: </span>
                Please install <Link to="https://calibre-ebook.com/download" target="_blank" className="font-bold underline hover:text-blue-600 dark:hover:text-blue-400 transition-colors">Calibre</Link> before using this program.
              </div>
              
              <div className="flex flex-col gap-3 mt-6">
                <Link
                  to="https://github.com/xiaochun-z/ebook-wordwise/wiki"
                  target="_blank"
                  className="flex items-center justify-center p-2.5 rounded-lg bg-gray-100 hover:bg-gray-200 dark:bg-gray-700 dark:hover:bg-gray-600 text-gray-900 dark:text-white transition-all font-medium"
                >
                  <FontAwesomeIcon icon={faBookOpen} className="mr-2" />
                  Documentation & Wiki
                </Link>
              </div>
            </div>
            <div className="flex justify-center gap-4">
              <Button color="blue" onClick={() => setOpenModal(false)} className="w-auto px-4">
                Awesome, Close!
              </Button>
            </div>
          </div>
        </ModalBody>
      </Modal>
    </>
  );
}
