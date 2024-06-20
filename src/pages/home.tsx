import { Fragment, useEffect, useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
//import { appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import {
  faFolderOpen,
  faArrowsRotate,
} from "@fortawesome/free-solid-svg-icons";

class WorkMesg {
  className: string;
  text: string;
  constructor(className: string, text: string) {
    this.className = className;
    this.text = text;
  }
}

export default function Home() {
  async function check_ebook_convert() {
    await invoke<boolean>("check_ebook_convert").then((result) => {
      if (result) {
        setWorkMesg(
          new WorkMesg(
            "text-green-800 dark:text-green-300",
            "Calibre detected, you're good to go!"
          )
        );
      } else {
        setWorkMesg(
          new WorkMesg(
            "text-red-800 dark:text-red-300",
            "Calibre is not detected, please install calibre and add calibre to your PATH."
          )
        );
      }
    });
  }
  

  useEffect(() => {
    check_ebook_convert();
    if (window.__TAURI_METADATA__) {
      listen<number>("event-progress", (event) => {
        setProgress(event.payload);
      });
    }
  }, []);
  const [book, setbook] = useState("");
  const [format, setFormat] = useState("epub");
  const [language, setLanguage] = useState("en");
  const [hintLevel, setHintLevel] = useState("3");
  const [allowLong, setAllowLong] = useState(false);
  const [showPhoneme, setShowPhoneme] = useState(false);
  const [progress, setProgress] = useState(0);
  const [working, setWorking] = useState(false);
  const [selecting, setSelecting] = useState(false);
  const [workmesg, setWorkMesg] = useState<WorkMesg>({
    className: " ",
    text: "",
  });

  async function start_job() {
    setWorkMesg(new WorkMesg(" ", ""));
    setWorking(true);
    await invoke<string>("start_job", {
      payload: {
        book: book,
        format: format,
        language: language,
        hint_level: parseInt(hintLevel),
        allow_long: Boolean(allowLong),
        show_phoneme: Boolean(showPhoneme),
      },
    })
      .then((result) => {
        setWorkMesg(new WorkMesg("text-blue-800 dark:text-blue-300", result));
      })
      .catch((error) => {
        setWorkMesg(new WorkMesg("text-red-800 dark:text-red-300", error));
      });

    setWorking(false);
  }

  async function select_book_dialog(): Promise<string> {
    setSelecting(true);
    return new Promise(async (resolve, reject) => {
      if (window.__TAURI_METADATA__) {
        try {
          const book_path: string = await invoke<string>("open_file_dialog", {
            initialPath: book,
          });
          setbook(book_path);
          resolve(book_path);
          setSelecting(false);
        } catch (e) {
          reject(e);
          setSelecting(false);
        }
      }
    });
  }

  const supported_languages = [
    { value: "en", text: "English" },
    { value: "cn", text: "Chinese" },
  ];
  const supported_formats = [
    { value: "epub", text: "epub" },
    { value: "mobi", text: "mobi" },
    { value: "pdf", text: "pdf" },
  ];
  return (
    <Fragment>
      <div className="columns-1 w-full p-10 space-y-4">
        <div>
          <label
            htmlFor="book-location-icon"
            className="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
          >
            Your Book
          </label>
          <div className="flex flex-row space-x-2 justify-between">
            <div className="relative flex-1">
              <div className="absolute inset-y-0 start-0 flex items-center ps-3.5 pointer-events-none">
                <FontAwesomeIcon
                  icon={faFolderOpen}
                  className="w-4 h-4 text-gray-500 dark:text-gray-400"
                />
              </div>
              <input
                type="text"
                id="book-location-icon"
                value={book}
                onChange={(e) => setbook(e.target.value)}
                className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full ps-10 p-2.5  dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                placeholder="select your ebook from your computer..."
              />
            </div>
            <button
              type="button"
              onClick={select_book_dialog}
              disabled={working || selecting}
              className="disabled:opacity-50 disabled:cursor-not-allowed text-white bg-gradient-to-r from-blue-500 via-blue-600 to-blue-700 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-blue-300 dark:focus:ring-blue-800 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2"
            >
              Browse...
            </button>
          </div>
        </div>
        <div>
          <label
            htmlFor="format-select"
            className="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
          >
            Select Output Format
          </label>
          <select
            id="format-select"
            value={format}
            onChange={(e) => setFormat(e.target.value)}
            className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg
             focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700
              dark:border-gray-600 dark:placeholder-gray-400 dark:text-white
               dark:focus:ring-blue-500 dark:focus:border-blue-500"
          >
            {supported_formats.map(({ value, text }) => (
              <option key={value} value={value}>
                {text}
              </option>
            ))}
          </select>
        </div>
        <div>
          <label
            htmlFor="language-select"
            className="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
          >
            Select Language
          </label>
          <select
            id="language-select"
            value={language}
            onChange={(e) => setLanguage(e.target.value)}
            className="bg-gray-50 border border-gray-300 text-gray-900 text-sm 
            rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5
             dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400
              dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
          >
            {supported_languages.map(({ value, text }) => (
              <option key={value} value={value}>
                {text}
              </option>
            ))}
          </select>
        </div>
        <div>
          <label
            htmlFor="minmax-range"
            className="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
          >
            Hint Level
          </label>
          <input
            id="minmax-range"
            type="range"
            min="1"
            max="5"
            step="1"
            value={hintLevel}
            className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700"
            disabled={false}
            onChange={(e) => setHintLevel(e.target.value)}
          />
        </div>
        <div className="flex flex-row space-x-5">
          <label className="inline-flex items-center mb-5 cursor-pointer">
            <input
              type="checkbox"
              value=""
              checked={allowLong}
              onChange={(_) => setAllowLong(!allowLong)}
              className="sr-only peer"
            />
            <div
              className="relative w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4
             peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer
              dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full
               peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px]
                after:bg-white after:border-gray-300 after:border after:rounded-full after:w-5 
                after:h-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"
            ></div>
            <span className="ms-3 text-sm font-medium text-gray-900 dark:text-gray-300">
              Allow Long Description
            </span>
          </label>
          <label className="inline-flex items-center mb-5 cursor-pointer">
            <input
              type="checkbox"
              value=""
              className="sr-only peer"
              checked={showPhoneme}
              onChange={(_) => setShowPhoneme(!showPhoneme)}
            />
            <div
              className="relative w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4
             peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 
             peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full
              peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px]
               after:bg-white after:border-gray-300 after:border after:rounded-full after:w-5 after:h-5 
               after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"
            ></div>
            <span className="ms-3 text-sm font-medium text-gray-900 dark:text-gray-300">
              Show Phoneme
            </span>
          </label>
        </div>
        <div className="flex flex-row space-x-5">
          <button
            type="button"
            onClick={start_job}
            disabled={working || selecting}
            className="disabled:opacity-50 text-white bg-blue-700 hover:bg-blue-800 focus:ring-1 focus:outline-none
             focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex 
             items-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
          >
            {
              <FontAwesomeIcon
                icon={faArrowsRotate}
                className="w-4 h-4 mr-2 animate-spin"
                style={{ animationPlayState: working ? "running" : "paused" }}
              />
            }
            Process
          </button>
          <div className="flex items-center">
            <div id="message" className={`line-clamp-2 ${workmesg.className}`}>
              {workmesg.text}
            </div>
          </div>
        </div>
        <div className="pt-5">
          <div className="w-full bg-gray-200 rounded-full h-2.5 mb-4 dark:bg-gray-700">
            <div
              className="bg-blue-700 dark:bg-blue-600 h-2.5 rounded-full"
              style={{ width: `${progress}%` }}
            ></div>
          </div>
        </div>
      </div>
    </Fragment>
  );
}
