import type {WorkerMessageType} from "./WorkerMessage";
import {WorkerMessage} from "./WorkerMessage";
import init, { run } from 'json_typegen_wasm'

let largeFileInput: string;
const postMsg: (data: WorkerMessageType) => void = postMessage

init().then(() => {
  postMsg({
    type: WorkerMessage.WASM_READY
  });
});

onmessage = messageEvent => {
  const message: WorkerMessageType = messageEvent.data;

  if (message.type === WorkerMessage.CODEGEN) {
    const input = largeFileInput || message.input;
    const result = run(message.typename, input, JSON.stringify(message.options));
    postMsg({
      type: WorkerMessage.CODEGEN_COMPLETE,
      result,
      typename: message.typename,
      options: message.options,
    });
  } else if (message.type === WorkerMessage.LOAD_FILE) {
    const reader = new FileReader();
    reader.onload = (fileEvent) => {
      largeFileInput = fileEvent.target.result as string;
      postMsg({
        type: WorkerMessage.LOAD_FILE_COMPLETE,
      });
    }
    reader.readAsText(message.file);
  } else if (message.type === WorkerMessage.CLEAR_FILE) {
    largeFileInput = undefined;
  } else {
    console.warn("Unknown message to worker", messageEvent);
  }
};
