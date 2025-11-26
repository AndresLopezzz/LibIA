import "./App.css";
import { useState } from "react";

type ChatMessage = {
  id: number;
  role: "user" | "assistant" | "system";
  content: string;
};

function App() {
  const [messages, setMessages] = useState<ChatMessage[]>([
    {
      id: 1,
      role: "system",
      content:
        "Bienvenido a tu asistente de PDFs con IA local. Esta UI es solo una base, todavía no está conectada al backend.",
    },
    {
      id: 2,
      role: "assistant",
      content:
        "Cuando esté listo, aquí verás respuestas basadas en tus documentos.",
    },
  ]);

  const [input, setInput] = useState("");

  const handleSend = (event: React.FormEvent) => {
    event.preventDefault();
    const trimmed = input.trim();
    if (!trimmed) return;

    // Por ahora solo agregamos el mensaje del usuario a la lista.
    setMessages((prev) => [
      ...prev,
      {
        id: prev.length + 1,
        role: "user",
        content: trimmed,
      },
    ]);
    setInput("");
  };

  return (
    <div className="app-root">
      <div className="chat-wrapper">
        <header className="chat-header">
          <div className="chat-header-title">LibAI · Chat de PDFs</div>
          <div className="chat-header-subtitle">
            Base de UI (frontend) · Aún sin lógica real
          </div>
        </header>

        <main className="chat-main">
          <div className="chat-messages">
            {messages.map((message) => (
              <div
                key={message.id}
                className={`chat-message chat-message-${message.role}`}
              >
                <div className="chat-message-bubble">
                  {message.role !== "user" && (
                    <div className="chat-message-role">
                      {message.role === "assistant" ? "Asistente" : "Sistema"}
                    </div>
                  )}
                  <div className="chat-message-content">{message.content}</div>
                </div>
              </div>
            ))}
          </div>
        </main>

        <footer className="chat-footer">
          <form className="chat-input-form" onSubmit={handleSend}>
            <textarea
              className="chat-input"
              value={input}
              onChange={(e) => setInput(e.target.value)}
              placeholder="Haz una pregunta sobre tus PDFs... (por ahora solo se queda en la UI)"
              rows={2}
            />
            <button
              type="submit"
              className="chat-send-button"
              disabled={!input.trim()}
            >
              Enviar
            </button>
          </form>
          <div className="chat-footer-hint">
            Más adelante aquí conectaremos Tauri + Rust + BD + IA. Por ahora es
            solo layout.
          </div>
        </footer>
      </div>
    </div>
  );
}

export default App;
