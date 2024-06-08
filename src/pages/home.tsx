import { Fragment, useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faFolderOpen,
  faArrowsRotate,
} from "@fortawesome/free-solid-svg-icons";

export default function Home() {
  const [range, setRange] = useState("3");
  const [language, setLanguage] = useState("en");
  const [format, setFormat] = useState("epub");
  const [progress, setProgress] = useState(30);
  const [working, setWorking] = useState(false);

  function start_job() {
    setWorking(!working);
    setProgress(80);
  }
  const supported_languages = [
    { value: "en", text: "English" },
    { value: "cn", text: "Chinese" },
  ];
  const supported_formats = [
    { value: "epub", text: "EPUB" },
    { value: "mobi", text: "EPUB" },
    { value: "pdf", text: "PDF" },
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
                className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full ps-10 p-2.5  dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                placeholder="select your ebook from your computer..."
              />
            </div>
            <button
              type="button"
              className="text-white bg-gradient-to-r from-blue-500 via-blue-600 to-blue-700 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-blue-300 dark:focus:ring-blue-800 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2"
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
            value={range}
            className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700"
            disabled={false}
            onChange={(e) => setRange(e.target.value)}
          />
        </div>
        <div className="flex flex-row space-x-5">
          <label className="inline-flex items-center mb-5 cursor-pointer">
            <input type="checkbox" value="" className="sr-only peer" />
            <div
              className="relative w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4
             peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer
              dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full
               peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px]
                after:bg-white after:border-gray-300 after:border after:rounded-full after:w-5 
                after:h-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"
            ></div>
            <span className="ms-3 text-sm font-medium text-gray-900 dark:text-gray-300">
              Show detailed tips
            </span>
          </label>
          <label className="inline-flex items-center mb-5 cursor-pointer">
            <input type="checkbox" value="" className="sr-only peer" />
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
        <div>
          <button
            type="button"
            onClick={start_job}
            className="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none
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
