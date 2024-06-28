import { Modal } from "flowbite-react";
import { useState } from "react";
import { faHeart } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { Link } from "react-router-dom";

export default function About() {
    const [openModal, setOpenModal] = useState(false);

    return (
        <>
            <li key={10086} onClick={() => setOpenModal(true)} className="rounded w-9 h-9 mt-3 mb-7 ms-auto me-auto bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 dark:focus:ring-blue-800 dark:hover:text-red-400 hover:text-red-700">
                <Link
                    to="#"
                    className="flex flex-col items-center justify-center w-9 h-9"
                    data-modal-target="default-modal" data-modal-toggle="default-modal"
                >
                    <FontAwesomeIcon
                        icon={faHeart}
                        className="ml-auto mr-auto w-4 h-4"
                    />
                </Link>
            </li>
            <Modal show={openModal} onClose={() => setOpenModal(false)}>
                <Modal.Header>About Ebook Wordwise</Modal.Header>
                <Modal.Body>
                    <div className="space-y-6">
                        <p className="text-base leading-relaxed text-gray-500 dark:text-gray-400">
                            This program is using Tauri/React/TailWindCss/Flowbite/FontAwesome/Calibre. Before using this program, it is necessary to install <Link className="text-blue-700 hover:text-blue-900 dark:text-red-300 dark:hover:text-red-100 hover:underline" to="https://calibre-ebook.com/download" target="_blank">calibre</Link>. Gratitude is extended to all the open source communities.
                        </p>
                        <p className="text-base leading-relaxed text-gray-500 dark:text-gray-400">
                            If you have questions for using this app, please consider using this <Link className="text-blue-700 hover:text-blue-900 dark:text-red-300 dark:hover:text-red-100 hover:underline" to="https://github.com/xiaochun-z/rust-wordwise/wiki" target="_blank">Wiki Page</Link>.
                        </p>
                        <p className="text-base leading-relaxed text-gray-500 dark:text-gray-400">My family and I currently reside in China and are actively searching for new opportunities abroad. If you are interested in offering assistance, kindly reach out to me at <Link className="text-blue-700 hover:text-blue-900 dark:text-red-300 dark:hover:text-red-100 hover:underline" to="mailto:xiaochun.zh@outlook.com">xiaochun.zh@outlook.com</Link>. Thank you.</p>
                    </div>
                </Modal.Body>
                <Modal.Footer>
                </Modal.Footer>
            </Modal>
        </>
    );
}
