import { MessageType } from "~/types";
import "./ChatView.css";
import Message from "./Message";
import { For } from "solid-js";

export default function ChatView(props: { messages: MessageType[] }) {
  return (
    <div class="container">
      <For each={props.messages}>
        {(message: MessageType) => <Message message={message} />}
      </For>
    </div>
  );
}
