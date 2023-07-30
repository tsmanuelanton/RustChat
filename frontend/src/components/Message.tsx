import { MessageType } from "~/types";

export default function Message(props: {
  message: MessageType;
  self_client: string;
}) {
  const mine_msg = props.message.client_id == props.self_client;

  return (
    <div class={`d-flex m-2 ${mine_msg ? "justify-content-end" : ""}`}>
      <div class="card" style={"max-width: 75%"}>
        <div class="card-body justify-content-end">
          <h5 class="card-title text-body-secondary">{`Client ${props.message.client_id}`}</h5>
          <p class="card-text"> {`${props.message.text}`}</p>
          <p
            class={`text-body-secondary d-flex ${
              mine_msg ? "justify-content-end" : ""
            }`}
          >
            {`${new Date(
              props.message.created_at.secs_since_epoch * 1000
            ).toLocaleString()}`}
          </p>
        </div>
      </div>
    </div>
  );
}
