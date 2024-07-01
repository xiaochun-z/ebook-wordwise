import Menu from "./components/menu";
import { Routes, Route } from "react-router-dom";
import Home from "./pages/home";
import Settings from "./pages/settings";
import "./App.css";
import { appWindow } from "@tauri-apps/api/window";
import { useEffect } from "react";

function App() {
  useEffect(() => {
    appWindow.theme().then((theme) => {
      if (theme === "dark") {
        document.documentElement.classList.add("dark");
      } else {
        document.documentElement.classList.remove("dark");
      }
    });
  }, []);
  return (
    <div className="flex min-h-screen flex-row antialiased text-slate-500 dark:text-slate-400 bg-gray-100 dark:bg-slate-900">
      <nav className="flex flex-col min-h-screen w-14 items-center pt-0 pb-10 border-r-2 dark:border-slate-600">
        <Menu />
      </nav>
      <main className="flex min-h-screen px-4 py-2 container">
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
