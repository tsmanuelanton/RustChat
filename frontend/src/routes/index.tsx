import { Title } from "solid-start";
import { createSignal } from "solid-js";
import { MessageType } from "~/types";
import { NicknameInput } from "~/components/NicknameInput";
import Chat from "~/components/Chat";

export default function Home() {
  const [messages, setMessages] = createSignal<MessageType[]>([]);
  const [client, setClient] = createSignal<string>();
  const [nickname, setNickname] = createSignal<string>();

  const ws = new WebSocket("ws://127.0.0.1:8000/connect-chat");

  ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log("Received data: ", data);
    if (data.handshake) {
      if (data.handshake.welcome) {
        console.log(data.handshake.welcome);
        setClient(data.handshake.client_id);
      }
    } else if (data.message) {
      console.log(data.message);
      setMessages([...messages(), data.message]);
    }
  };

  const onSubmitMessage = (event: any): void => {
    event.preventDefault();
    if (event.target[0].value) ws.send(event.target[0].value);
    event.target[0].value = "";
  };

  const onSubmitNickname = (event: any): void => {
    event.preventDefault();
    const newNickname = event.target[0].value;
    if (newNickname) {
      setNickname(newNickname);
      ws.send(newNickname);
    }
  };

  return (
    <main class="container">
      <Title>RustChat</Title>
      <h1 class="d-flex justify-content-center text-primary-emphasis mb-5">
        RustChat
      </h1>
      {nickname() == undefined ? (
        <NicknameInput onSubmitNickname={onSubmitNickname} />
      ) : (
        <Chat client={client()!} messages={messages()} nickname={nickname()!} onSubmitMessage={onSubmitMessage} />
      )}
    </main>
  );
}
