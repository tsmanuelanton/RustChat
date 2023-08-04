export const NicknameInput = (props: {
  onSubmitNickname: (e: any) => void;
}) => {
  return (
    <>
      <div class="d-flex flex-column justify-content-center">
        <p class="d-flex justify-content-center">What's your nickname?</p>
        <form onSubmit={props.onSubmitNickname}>
          <div class="d-flex justify-content-center gap-2">
            <input
              class="form-control d-flex justify-content-center w-25"
              type="text"
              placeholder="Set nickname"
            />
            <button class="btn btn-outline-primary btn-lg" type="submit">
              Save
            </button>
          </div>
        </form>
      </div>
    </>
  );
};
