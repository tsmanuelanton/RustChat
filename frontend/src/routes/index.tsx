import { Title } from "solid-start";
import { createSignal, Accessor } from "solid-js";
import { MessageType } from "~/types";
import ChatView from "~/components/ChatView";

export default function Home() {
  const [messages, setMessages] = createSignal<MessageType[]>([]);

  const ws = new WebSocket("ws://127.0.0.1:8000/connect-chat");
  ws.onmessage = (event) => {
    setMessages([...messages(), JSON.parse(event.data)]);
    console.log(messages());
  };

  const onSubmit = (event:any): void => {
    event.preventDefault();
    console.log(event.target[0].value);
    if (event.target[0].value) {
      ws.send(event.target[0].value);
    }
  };

  return (
    <main>
      <Title>RustChat</Title>
      <h1>RustChat</h1>
      <ChatView messages={messages()} />
      <p>Escribe algo:</p>
      <form
        onSubmit={onSubmit}
      >
        <input type="text" />
        <button type="submit">Enviar</button>
      </form>
    </main>
  );
}
