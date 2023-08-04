import { MessageType } from "~/types";
import "bootstrap/dist/css/bootstrap.css";
import ChatView from "./ChatView";

export default function Chat(props: {
  nickname: string;
  client: string;
  onSubmitMessage: (event: any) => void;
  messages: MessageType[];
}) {
  return (
    <>
      <p class="fs-5">You're client {props.nickname}</p>
      <ChatView messages={props.messages} self_client={props.client} />

      <form onSubmit={props.onSubmitMessage}>
        <div class="d-flex justify-content-center gap-2">
          <input
            id="input-msg"
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
    </>
  );
}
