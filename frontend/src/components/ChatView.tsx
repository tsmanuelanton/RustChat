import { MessageType } from "~/types";
import "./ChatView.css";
import Message from "./Message";
import { For } from "solid-js";

export default function ChatView(props: {
  messages: MessageType[];
  self_client: string;
}) {
  return (
    <div class="container">
      <For each={props.messages}>
        {(message: MessageType) => (
          <Message message={message} self_client={props.self_client} />
        )}
      </For>
    </div>
  );
}
