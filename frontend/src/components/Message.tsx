import "./Message.css";
import { MessageType } from "~/types";

export default function Message(props : {message: MessageType, self_client: string}) {
  
  return (

    <div class={`msg ${props.message.client_id != props.self_client ? "other_user" : ""}`} >
        <p class="user-msg">{`Client ${props.message.client_id}`}</p>
        <p class="text-msg"> {`${props.message.text}`}</p>
        <p class="time-msg"> {`${new Date(props.message.created_at.secs_since_epoch * 1000).toLocaleString()}`}</p>
    </div>
  );
}
