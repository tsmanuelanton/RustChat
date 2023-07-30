import { Title } from "solid-start";
import { createSignal } from "solid-js";
import { MessageType } from "~/types";
import ChatView from "~/components/ChatView";

export default function Home() {
  const [messages, setMessages] = createSignal<MessageType[]>([]);
  const [client, setClient] = createSignal<string>();
  let inputMsg : HTMLInputElement;

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
    inputMsg.value = "";
  };

  return (
    <main class="container">
      <Title>RustChat</Title>
      <h1 class="d-flex justify-content-center text-primary-emphasis">
        RustChat
      </h1>
      <p class="fs-5">You're client {client() ?? ""}</p>
      <ChatView messages={messages()} self_client={client() ?? ""} />

      <form onSubmit={onSubmit}>
        <div class="d-flex justify-content-center gap-2">
          <input
            id="input-msg"
            ref={inputMsg}
            type="text"
            class="form-control w-75"
            placeholder="Say something"
          />
          <button class="btn btn-outline-primary btn-lg" type="submit">
            Send
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              fill="currentColor"
              class="bi bi-send-fill mx-2"
              viewBox="0 0 16 16"
            >
              <path d="M15.964.686a.5.5 0 0 0-.65-.65L.767 5.855H.766l-.452.18a.5.5 0 0 0-.082.887l.41.26.001.002 4.995 3.178 3.178 4.995.002.002.26.41a.5.5 0 0 0 .886-.083l6-15Zm-1.833 1.89L6.637 10.07l-.215-.338a.5.5 0 0 0-.154-.154l-.338-.215 7.494-7.494 1.178-.471-.47 1.178Z" />
            </svg>
          </button>
        </div>
      </form>
    </main>
  );
}
