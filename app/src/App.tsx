import "./App.css";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [name, setName] = useState("");
  const [greeting, setGreeting] = useState("");

  const handleGreet = async () => {
    // Evitar errores cuando se corra solo con Vite
    if (typeof window.__TAURI_INTERNALS__ === "undefined") {
      setGreeting("Invoke only available when running inside Tauri.");
      return;
    }
    try {
      const response = await invoke("greet", { name });
      setGreeting(response as string);
    } catch (error) {
      console.error("invoke error:", error);
      setGreeting("Error calling Rust command");
    }
  };

  return (
    <main className="container">
      <div>
        <input
          value={name}
          onChange={(e) => setName((e.target as HTMLInputElement).value)}
        />
        <button onClick={handleGreet}>Greet</button>
      </div>
      {greeting && <div>{greeting}</div>}
    </main>
  );
}

export default App;
