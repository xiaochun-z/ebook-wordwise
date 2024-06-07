import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { useState } from "react";


function setTheme(theme: string) {
  if (theme === "dark") {
    document.documentElement.classList.add("dark");
  } else if (theme == "light") {
    document.documentElement.classList.remove("dark");
  }

  const settings = {
    theme: theme,
  };
  invoke("save_settings", { settings: settings }).then((res) => {
    console.log(res);
  });
}

function SettingsPage() {
  const [setting_text, set_setting_text] = useState("");
  async function read_settings() {
    await invoke("read_settings");
  }

  listen("settings_retrived", (event) => {
    //console.log(event.payload);
    set_setting_text(JSON.stringify(event.payload));
  });

  return (
    <div>
      <h1 className="">Settings Page</h1>
      <style>{`
        button {
          margin: 1rem 1rem 0 0;
        }
      `}</style>

      {[
        { text: "Dark Theme", theme: "dark" },
        { text: "Light Theme", theme: "light" },
      ].map(({ theme, text }) => (
        <button
          className="btn px-3 py-1 rounded text-black bg-gray-300 dark:bg-slate-700/75 dark:text-white hover:bg-emerald-700 hover:text-white"
          key={theme}
          onClick={() => {
            setTheme(theme);
          }}
        >
          {text}
        </button>
      ))}
      <button
        onClick={read_settings}
        className="btn px-3 py-1 rounded text-black bg-gray-300 dark:bg-slate-700/75 dark:text-white hover:bg-emerald-700 hover:text-white"
      >
        Read Settings
      </button>
      <p className="text-black dark:text-white">{setting_text}</p>
    </div>
  );
}
export default SettingsPage;