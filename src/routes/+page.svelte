<script lang="ts">
  import {
    currentProtoFilePath,
    currentSelectedMessageName,
    dependenciesPath,
    messageTypesNames,
  } from "$lib";
  import {
    generateDefaultMessageJson,
    loadProto,
    publishRabbitMessage,
    type RabbitMqParams,
  } from "$lib/api";
  import { Button, Toast } from "flowbite-svelte";
  import { Icon } from "flowbite-svelte-icons";
  import EndpointsConfiguration from "../components/EndpointsConfiguration.svelte";
  import MessageEditorAce from "../components/MessageEditorAce.svelte";
  import MessageSelector from "../components/MessageSelector.svelte";
  import UtillitiesToolbar from "../components/UtillitiesToolbar.svelte";

  let jsonValue: string;
  let publishLoading = false;
  let toastOpen = false;
  $: fileSelectionDisabled = !$dependenciesPath;

  function onDepsDirSelected(e: CustomEvent<any>): void {
    dependenciesPath.set(e.detail);
  }

  function onProtoFileSelected(e: CustomEvent<any>): void {
    currentProtoFilePath.set(e.detail);
    loadProto($dependenciesPath, $currentProtoFilePath).then((res) => {
      console.log("result from backend:", res);
      messageTypesNames.set(res);
    });
  }

  function onMessageSelected(e: CustomEvent<any>): void {
    currentSelectedMessageName.set(e.detail.name);
    generateDefaultMessageJson(
      $dependenciesPath,
      $currentProtoFilePath,
      $currentSelectedMessageName
    ).then((res) => (jsonValue = res));
  }

  async function onPublishRequested(e: CustomEvent<any>): Promise<void> {
    let split = (e.detail.host as string).split(":");
    const rabbitParams: RabbitMqParams = {
      host: split[0],
      port: Number(split[1]),
      password: "guest",
      username: "guest",
      routing_key: "/",
      target_name: e.detail.target,
      is_queue: e.detail.type == "queue",
    };
    publishLoading = true;
    await publishRabbitMessage(
      $dependenciesPath,
      $currentProtoFilePath,
      $currentSelectedMessageName,
      rabbitParams,
      jsonValue
    );
    publishLoading = false;
    toastOpen = true;
  }
</script>

<div class="flex h-screen items-stretch bg-gray-900">
  <div class="flex flex-row grow">
    <!-- selector -->
    <div class="flex flex-col">
      <div class="flex flex-row p-3 h-20 rounded-sm m-1 shadw-sm">
        <UtillitiesToolbar
          disableFileSelection={fileSelectionDisabled}
          on:dirSelected={onDepsDirSelected}
          on:fileSelected={onProtoFileSelected}
        />
      </div>
      <div class="flex flex-row p-3 h-full rounded-sm m-1 shadw-sm">
        <MessageSelector
          messageTypes={$messageTypesNames}
          on:click={onMessageSelected}
        />
      </div>
    </div>

    <!-- editor -->
    <div class="flex flex-col grow">
      <div class="flex flex-row p-3 h-20 rounded-sm m-1 shadw-sm">
        <EndpointsConfiguration
          on:publish={onPublishRequested}
          {publishLoading}
        />
      </div>
      <div
        class="flex flex-row h-screen m-1 rounded-sm shadw-smbg-orange-300 rounded-sm shadw-sm"
      >
        <!-- <MessageEditor value={jsonValue} /> -->
        <MessageEditorAce bind:value={jsonValue} />
      </div>
      <Toast
        color="green"
        bind:open={toastOpen}
        position="bottom-right"
        class=""
      >
        <svelte:fragment slot="icon">
          <Icon name="check-circle-solid" class="w-5 h-5" />
          <span class="sr-only">Check icon</span>
        </svelte:fragment>
        Published message successfully.
      </Toast>
    </div>
  </div>
</div>
