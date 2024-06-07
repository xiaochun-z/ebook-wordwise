import { Fragment } from "react";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
let setting_loaded: boolean = false;

export default function Home() {
  function setTheme(theme: string) {
    if (theme === "dark") {
      document.documentElement.classList.add("dark");
    } else if (theme == "light") {
      document.documentElement.classList.remove("dark");
    }
  }

  const loadSettings = async () => {
    await invoke("read_settings");
  };
  useEffect(() => {
    if (!setting_loaded) {
      loadSettings();
      setting_loaded = true;
    }
  }, []);

  listen<{ theme: string }>("settings_retrived", (event) => {
    console.log(event.payload);
    if (event.payload) {
      if (event.payload) {
        setTheme(event.payload.theme);
      }
    }
  });

  return (
    <Fragment>
      <h1>Home Page</h1>
    </Fragment>
  );
}
