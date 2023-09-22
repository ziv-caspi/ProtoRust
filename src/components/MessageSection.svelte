<script lang="ts">
  import EndpointsConfiguration from "./standalone/EndpointsConfiguration.svelte";
  import MessageEditorAce from "./standalone/MessageEditorAce.svelte";
  import { protoConfig, type EndpointConfig } from "$lib";
  import {
    publishRabbitMessage as startPublishRabbitMessage,
    type RabbitMqParams,
    cancelPublishing,
  } from "$lib/api";
  import Toast from "./standalone/Toast.svelte";
  import { appWindow } from "@tauri-apps/api/window";

  export let jsonValue: string;
  let publishLoading = false;
  let toastState = { shouldOpen: false, isError: false, errorMessage: "" };
  appWindow.listen("publish_end", (event) => {
    console.log(event.event, event.payload);

    const result: any = event.payload;
    if (result.hasOwnProperty("Ok")) {
      let ok = event.payload as { Ok: number };
      toastState = {
        ...toastState,
        isError: false,
        shouldOpen: true,
      };
    } else if (result.hasOwnProperty("Err")) {
      let err = event.payload as { Err: string };
      toastState = {
        ...toastState,
        isError: true,
        shouldOpen: true,
        errorMessage: `${err.Err}`,
      };
    }

    publishLoading = false;
  });

  async function onPublishRequested(
    e: CustomEvent<EndpointConfig>
  ): Promise<void> {
    const rabbitParams: RabbitMqParams = {
      host: e.detail.host,
      port: e.detail.port,
      password: e.detail.password,
      username: e.detail.username,
      routing_key: e.detail.routingKey,
      target_name: e.detail.target,
      is_queue: e.detail.type == "queue",
      quantity: e.detail.loop ? -1 : e.detail.quantity,
      speed: e.detail.speed,
    };
    publishLoading = true;
    try {
      await startPublishRabbitMessage($protoConfig, rabbitParams, jsonValue);
      // toastState = { ...toastState, isError: false, shouldOpen: true };
    } catch (err) {
      console.error(err);
      toastState = { errorMessage: `${err}`, isError: true, shouldOpen: true };
      publishLoading = false;
    }
  }

  async function onCancelRequested() {
    try {
      await cancelPublishing();
    } catch (err) {
      toastState = {
        errorMessage: `could not cancel: ${err}`,
        isError: true,
        shouldOpen: true,
      };
    }
  }
</script>

<div class="flex flex-col grow">
  <div class="flex flex-row p-3 h-20 rounded-sm m-1 shadw-sm">
    <EndpointsConfiguration
      on:publish={onPublishRequested}
      on:cancel={onCancelRequested}
      {publishLoading}
    />
  </div>
  <div
    class="flex flex-row h-screen m-1 rounded-sm shadw-smbg-orange-300 rounded-sm shadw-sm"
  >
    <!-- <MessageEditor value={jsonValue} /> -->
    <MessageEditorAce bind:value={jsonValue} />
  </div>
  <Toast state={toastState} />
</div>
