import { MessageType } from "~/types";
import Message from "./Message";
import { For, Accessor, createEffect, onMount, observable } from "solid-js";
import "bootstrap/dist/css/bootstrap.css";

export default function ChatView(props: {
  messages: Accessor<MessageType[]>;
  selfClient: Accessor<string>;
}) {
  let container: HTMLDivElement;

  onMount(() => {
    createEffect(() => {
      const lastMsg = props.messages()[props.messages().length - 1];
      if (lastMsg?.client_id == props.selfClient())
        container.scrollTop = container.scrollHeight - container.clientHeight;
    });
  });

  return (
    <div
      ref={container}
      class="border border-light-subtle border-2 rounded m-2 p-2 shadow-sm overflow-auto"
      style="height: 500px;"
    >
      <For each={props.messages()}>
        {(message: MessageType) => (
          <Message message={message} self_client={props.selfClient()} />
        )}
      </For>
    </div>
  );
}
