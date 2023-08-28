<script lang="ts">
  import EndpointsConfiguration from "./standalone/EndpointsConfiguration.svelte";
  import MessageEditorAce from "./standalone/MessageEditorAce.svelte";
  import { protoConfig, type EndpointConfig } from "$lib";
  import { publishRabbitMessage, type RabbitMqParams } from "$lib/api";
  import Toast from "./standalone/Toast.svelte";

  export let jsonValue: string;
  let publishLoading = false;
  let toastState = { shouldOpen: false, isError: false, errorMessage: "" };

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
    };
    publishLoading = true;
    try {
      await publishRabbitMessage($protoConfig, rabbitParams, jsonValue);
      toastState = { ...toastState, isError: false, shouldOpen: true };
    } catch (err) {
      console.error(err);
      toastState = { errorMessage: `${err}`, isError: true, shouldOpen: true };
    }
    publishLoading = false;
  }
</script>

<div class="flex flex-col grow">
  <div class="flex flex-row p-3 h-20 rounded-sm m-1 shadw-sm">
    <EndpointsConfiguration on:publish={onPublishRequested} {publishLoading} />
  </div>
  <div
    class="flex flex-row h-screen m-1 rounded-sm shadw-smbg-orange-300 rounded-sm shadw-sm"
  >
    <!-- <MessageEditor value={jsonValue} /> -->
    <MessageEditorAce bind:value={jsonValue} />
  </div>
  <Toast state={toastState} />
</div>