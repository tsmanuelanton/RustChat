import "./Message.css";
import { MessageType } from "~/types";

export default function Message({message} : {message: MessageType}) {
  
  return (
    <div class="msg">
        <p class="user-msg">{`Client ${message.client_id}`}</p>
        <p class="text-msg"> {`${message.text}`}</p>
        <p class="time-msg"> {`${new Date(message.created_at.secs_since_epoch * 1000).toLocaleString()}`}</p>
    </div>
  );
}
