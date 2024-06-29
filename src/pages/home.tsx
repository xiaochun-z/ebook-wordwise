import { Fragment, useEffect, useState, useCallback } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { dialog } from "@tauri-apps/api";
//import { appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import SelectInput from "../components/selectInput";
import Preview from "../components/Preview";
import {
  faFolderOpen,
  faArrowsRotate,
} from "@fortawesome/free-solid-svg-icons";

class WorkMesg {
  class_name: string;
  text: string;
  constructor(className: string, text: string) {
    this.class_name = className;
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
    notify("", "");
    if (window.__TAURI_METADATA__) {
      listen<number>("event-progress", (event) => {
        setProgress(event.payload);
      });
      listen<WorkMesg>("event-workmesg", (event) => {
        setWorkMesg(event.payload);
      });
    }
  }, []);

  // This is your notify function that you want to call
  async function notify(stateName: string, value: any) {
    //console.log(`State ${stateName} has been set to`, value);
    switch (stateName) {
      case "book":
        preview_payload.book = value;
        break;
      case "format":
        preview_payload.format = value;
        break;
      case "language":
        preview_payload.language = value;
        break;
      case "wordwiseStyle":
        preview_payload.wordwise_style = value;
        break;
      case "hintLevel":
        preview_payload.hint_level = value;
        break;
      case "allowLong":
        preview_payload.allow_long = value;
        break;
      case "showPhoneme":
        preview_payload.show_phoneme = value;
        break;
    }

    if (
      (preview_payload.format == "mobi" || preview_payload.format == "azw3") &&
      preview_payload.wordwise_style == 1
    ) {
      setWorkMesg(
        new WorkMesg(
          "text-red-600 dark:text-red-500",
          "Warning: Amazon Kindle probably does not support the `On top` style. "
        )
      );
    } else {
      setWorkMesg(new WorkMesg("text-red-800 dark:text-red-300", ""));
    }
    //console.log(preview_payload);
    await invoke<string>("preview", {
      payload: preview_payload,
      original: default_preview,
    }).then((res) => {
      //console.log(res);
      setPreview(res);
    });
  }

  function useNotifyingState<T>(
    initialValue: T,
    stateName: string
  ): [T, (newValue: T) => void] {
    const [value, setValue] = useState(initialValue);

    const setValueAndNotify = useCallback(
      (newValue: T) => {
        setValue(newValue);
        notify(stateName, newValue);
      },
      [stateName]
    );

    return [value, setValueAndNotify];
  }

  const [book, setbook] = useState("");

  const [format, setFormat] = useNotifyingState("epub", "format");
  const [language, setLanguage] = useNotifyingState("en", "language");
  const [wordwiseStyle, setWordwiseStyle] = useNotifyingState(
    0,
    "wordwiseStyle"
  );
  const [hintLevel, setHintLevel] = useNotifyingState(3, "hintLevel");
  const [allowLong, setAllowLong] = useNotifyingState(false, "allowLong");
  const [showPhoneme, setShowPhoneme] = useNotifyingState(false, "showPhoneme");

  let preview_payload = {
    book: book,
    format: format,
    language: language,
    hint_level: hintLevel,
    allow_long: allowLong,
    show_phoneme: showPhoneme,
    wordwise_style: wordwiseStyle,
  };

  const default_preview: string =
    "<p>This is a sample preview for the converted book as a FYI.</p><p>In a verdant field near the airfield, an unexpected abduction took place, just as a capsule containing a rare type of pepper, crucial to the study of sea power, was being transported.</p>";

  const [preview, setPreview] = useState(default_preview);
  const [progress, setProgress] = useState(0);
  const [working, setWorking] = useState(false);
  const [selecting, setSelecting] = useState(false);
  const [workmesg, setWorkMesg] = useState<WorkMesg>({
    class_name: " ",
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
        hint_level: hintLevel,
        allow_long: allowLong,
        show_phoneme: showPhoneme,
        wordwise_style: wordwiseStyle,
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

  async function select_book_dialog() {
    setSelecting(true);
    try {
      const book_path = await dialog.open({
        directory: false,
        multiple: false,
      });

      if (book_path != null) {
        setbook(book_path.toString());
      }
      setSelecting(false);
    } catch (error) {
      console.error("Error selecting file:", error);
    }
  }

  const supported_languages = [
    { value: "en", text: "English" },
    { value: "cn", text: "中文" },
    { value: "jp", text: "日本語" },
    { value: "ko", text: "한국인" },
    { value: "vi", text: "Tiếng Việt" },
    { value: "ar", text: "عربي" },
    { value: "de", text: "Deutsch" },
    { value: "es", text: "Española" },
    { value: "fr", text: "Français" },
    { value: "hi", text: "टर्की" },
    { value: "pt", text: "Português" },
    { value: "ru", text: "Русский" },
    { value: "th", text: "แบบไทย" },
    { value: "ua", text: "українська" },
  ];
  const supported_formats = [
    { value: "epub", text: "epub" },
    { value: "mobi", text: "mobi" },
    { value: "pdf", text: "pdf" },
    { value: "azw3", text: "azw3" },
    { value: "fb2", text: "fb2" },
    { value: "docx", text: "docx" },
    { value: "rb", text: "rb" },
    { value: "rtf", text: "rtf" },
    { value: "snb", text: "snb" },
    { value: "tcr", text: "tcr" },
  ];
  const supported_styles = [
    { value: 0, text: "Inline" },
    { value: 1, text: "On top" },
  ];

  const select_options = [
    {
      id: "format-select",
      label: "Output Format",
      value: format,
      options: supported_formats,
      onChange: (e: React.ChangeEvent<HTMLSelectElement>) =>
        setFormat(e.target.value),
    },
    {
      id: "language-select",
      label: "Wordwise Language",
      value: language,
      options: supported_languages,
      onChange: (e: React.ChangeEvent<HTMLSelectElement>) =>
        setLanguage(e.target.value),
    },
    {
      id: "style-select",
      label: "Wordwise Style",
      value: wordwiseStyle,
      options: supported_styles,
      onChange: (e: React.ChangeEvent<HTMLSelectElement>) =>
        setWordwiseStyle(parseInt(e.target.value)),
    },
  ];

  return (
    <Fragment>
      <div className="columns-1 w-full px-3 pt-6 space-y-2">
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
        <div className="flex flex-row gap-x-5">
          {select_options.map(({ id, label, value, options, onChange }) => (
            <SelectInput
              id={id}
              key={id}
              label={label}
              value={value}
              options={options}
              onChange={onChange}
            />
          ))}
        </div>
        <div>
          <label
            htmlFor="minmax-range"
            className="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
          >
            Hint Level (Drag to Left means less hints, right side for more
            hints)
          </label>
          <input
            id="minmax-range"
            type="range"
            min="0"
            max="5"
            step="1"
            value={hintLevel}
            className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700"
            disabled={false}
            onChange={(e) => setHintLevel(parseInt(e.target.value))}
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
              Long Definition
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
            <div id="message" className={`line-clamp-2 ${workmesg.class_name}`}>
              {workmesg.text}
            </div>
          </div>
        </div>
        <div className="pt-1">
          <div className="w-full bg-gray-200 rounded-full h-2.5 mb-4 dark:bg-gray-700">
            <div
              className="bg-blue-700 dark:bg-blue-600 h-2.5 rounded-full"
              style={{ width: `${progress}%` }}
            ></div>
          </div>
        </div>
        <div>
          <Preview innerHTML={preview} />
        </div>
      </div>
    </Fragment>
  );
}
