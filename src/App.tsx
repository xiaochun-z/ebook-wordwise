import Menu from "./components/menu";
import { Routes, Route } from "react-router-dom";
import Home from "./pages/home";
import Settings from "./pages/settings";
import "./App.css";
import { appWindow } from "@tauri-apps/api/window";
import { useState, useEffect } from "react";

function App() {
  const darkModeMediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  const [theme, setTheme] = useState(false);

  darkModeMediaQuery.addEventListener('change', (event) => {
    if (event.matches) {
      setDarkMode(true);
    } else {
      setDarkMode(false);
    }
  });

  useEffect(() => {
    appWindow.theme().then(t => {
      setDarkMode(t === "dark");
    });

  }, []);

  function setDarkMode(isDark: boolean) {
    if (isDark) {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
    setTheme(isDark);
  }
  return (
    <div className="flex min-h-screen flex-row antialiased text-slate-500 dark:text-slate-400 bg-gray-100 dark:bg-slate-900">
      <nav className="flex flex-col min-h-screen w-14 items-center pt-0 pb-10 border-r-2 menu-bg dark:menu-border dark:menu-bg">
        <Menu darkTheme={theme} setDarkTheme={setDarkMode} />
      </nav>
      <main className="flex min-h-screen px-4 py-2 container bg dark:bg">
        <Routes>
          <Route index path="/" element={<Home />} />
          <Route path="/settings" element={<Settings />} />
          <Route path="*" element={<Home />} />
        </Routes>
      </main>
    </div>
  );
}

export default App;
