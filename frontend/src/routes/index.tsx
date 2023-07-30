import { Title } from "solid-start";
import { createSignal } from "solid-js";
import { MessageType } from "~/types";
import ChatView from "~/components/ChatView";

export default function Home() {
  const [messages, setMessages] = createSignal<MessageType[]>([]);
  const [client, setClient] = createSignal<string>();

  const ws = new WebSocket("ws://127.0.0.1:8000/connect-chat");
  ws.onmessage = (event) => {
    if (!client()) {
      const info = JSON.parse(event.data);
      if (info.welcome) {
        console.log(info.welcome);
        setClient(info.client_id);
      } else {
        console.error(
          "Error al conectar, falta el mensaje de bienvenida del servidor"
        );
      }
    } else {
      setMessages([...messages(), JSON.parse(event.data)]);
    }
  };

  const onSubmit = (event: any): void => {
    event.preventDefault();
    if (event.target[0].value) ws.send(event.target[0].value);
  };

  return (
    <main>
      <Title>RustChat</Title>
      <h1>RustChat</h1>
      <p>You're client {client() ?? ""}</p>
      <ChatView messages={messages()} self_client={client() ?? ""} />
      <p>Escribe algo:</p>
      <form onSubmit={onSubmit}>
        <input type="text" />
        <button type="submit">Enviar</button>
      </form>
    </main>
  );
}
