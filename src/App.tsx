import Menu from "./components/menu";
import { Routes, Route } from "react-router-dom";
import Home from "./pages/home";
import Settings from "./pages/settings";
// import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
// import { useEffect } from "react";
// import { GetSettings } from "./utils/setting";

// let setting_loaded: boolean = false;

function App() {
  // const [greetMsg, setGreetMsg] = useState("");
  // const [name, setName] = useState("");

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name }));
  // }
  // function setTheme(theme: string) {
  //   if (theme === "dark") {
  //     document.documentElement.classList.add("dark");
  //   } else if (theme == "light") {
  //     document.documentElement.classList.remove("dark");
  //   }
  // }

  // useEffect(() => {
  //   if (!setting_loaded) {
  //     GetSettings().then((res) => {
  //       setTheme(res.theme);
  //       console.log(res);
  //     });
  //     setting_loaded = true;
  //   }
  // }, []);

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
