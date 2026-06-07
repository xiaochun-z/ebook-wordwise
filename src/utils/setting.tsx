import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

// define an application class/struct for the settings
// this will be used to store the settings
export class AppSetting {
  theme: string;
  constructor(theme: string) {
    this.theme = theme;
  }
}

export async function GetSettings(): Promise<AppSetting> {
  return new Promise(async (resolve, reject) => {
    if ('__TAURI_INTERNALS__' in window) {
      listen<AppSetting>("settings_retrived", (event) => {
        resolve(event.payload);
      });
      await invoke("read_settings").catch((e) => {
        reject(e);
      });
    }
  });
}

export async function SaveSettings(settings: AppSetting): Promise<void> {
  return new Promise(async (resolve, reject) => {
    if ('__TAURI_INTERNALS__' in window) {
      await invoke("save_settings", { settings: settings }).catch((e) => {
        reject(e);
      });
      resolve();
    }
  });
}
