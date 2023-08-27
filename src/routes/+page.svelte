<script lang="ts">
  import {
    currentProtoFilePath,
    currentSelectedMessageName,
    dependenciesPath,
    messageTypesNames,
    type EndpointConfig,
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
  let successToastOpen = false;
  let failToastOpen = false;
  let failMessage = "";
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

  async function onPublishRequested(
    e: CustomEvent<EndpointConfig>
  ): Promise<void> {
    let split = (e.detail.host as string).split(":");
    const rabbitParams: RabbitMqParams = {
      host: split[0],
      port: Number(split[1]),
      password: e.detail.password,
      username: e.detail.username,
      routing_key: e.detail.routingKey,
      target_name: e.detail.target,
      is_queue: e.detail.type == "queue",
    };
    publishLoading = true;
    try {
      await publishRabbitMessage(
        $dependenciesPath,
        $currentProtoFilePath,
        $currentSelectedMessageName,
        rabbitParams,
        jsonValue
      );
      successToastOpen = true;
    } catch (err) {
      console.error(err);
      failMessage = `${err}`;
      failToastOpen = true;
    }
    publishLoading = false;
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
        bind:open={successToastOpen}
        position="bottom-right"
        class=""
      >
        <svelte:fragment slot="icon">
          <Icon name="check-circle-solid" class="w-5 h-5" />
          <span class="sr-only">Check icon</span>
        </svelte:fragment>
        Published message successfully.
      </Toast>

      <Toast
        color="red"
        bind:open={failToastOpen}
        position="bottom-right"
        class=""
      >
        <svelte:fragment slot="icon">
          <Icon name="check-circle-solid" class="w-5 h-5" />
          <span class="sr-only">Check icon</span>
        </svelte:fragment>
        Failed to publish. {failMessage}
      </Toast>
    </div>
  </div>
</div>
