import { MessageType } from "~/types";
import Message from "./Message";
import { For } from "solid-js";
import 'bootstrap/dist/css/bootstrap.css';

export default function ChatView(props: {
  messages: MessageType[];
  self_client: string;
}) {
  return (
    <div class="border border-light-subtle border-2 rounded m-2 p-2 shadow-sm overflow-auto" style="height: 500px;">
      <For each={props.messages}>
        {(message: MessageType) => (
          <Message message={message} self_client={props.self_client} />
        )}
      </For>
    </div>
  );
}
