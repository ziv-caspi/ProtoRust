<script lang="ts">
  import { protoConfig } from "$lib";
  import { generateDefaultMessageJson, loadProto } from "$lib/api";
  import { createEventDispatcher } from "svelte";
  import MessageSelector from "./standalone/MessageSelector.svelte";
  import UtillitiesToolbar from "./standalone/UtillitiesToolbar.svelte";
  import Toast from "./standalone/Toast.svelte";

  let dispatch = createEventDispatcher();
  let toastState = { shouldOpen: false, isError: false, errorMessage: "" };
  $: selectedDependenciesDir = !$protoConfig?.dependenciesPath;

  function onDepsDirSelected(e: CustomEvent<string>): void {
    protoConfig.update((updater) => {
      updater.dependenciesPath = e.detail;
      return updater;
    });
  }

  async function onProtoFileSelected(e: CustomEvent<string>): Promise<void> {
    protoConfig.update((updater) => {
      updater.protoFilePath = e.detail;
      return updater;
    });
    try {
      let messages = await loadProto(
        $protoConfig?.dependenciesPath,
        $protoConfig?.protoFilePath
      );
      protoConfig.update((updater) => {
        updater.messageTypes = messages;
        return updater;
      });
    } catch (err) {
      toastState = { shouldOpen: true, isError: true, errorMessage: `${err}` };
    }
  }

  async function onMessageSelected(e: CustomEvent<string>): Promise<void> {
    console.log("disaptching click event");
    protoConfig.update((udpater) => {
      udpater.currentSelectedMessage = e.detail;
      return udpater;
    });
    let json = await generateDefaultMessageJson($protoConfig);
    dispatch("messageSelected", json);
  }
</script>

<div class="flex flex-col">
  <div class="flex flex-row p-3 h-20 rounded-sm m-1 shadw-sm">
    <UtillitiesToolbar
      disableFileSelection={selectedDependenciesDir}
      on:dirSelected={onDepsDirSelected}
      on:fileSelected={onProtoFileSelected}
    />
  </div>
  <div class="flex flex-row p-3 h-full rounded-sm m-1 shadw-sm">
    <MessageSelector
      messageTypes={$protoConfig.messageTypes}
      on:click={onMessageSelected}
    />
  </div>
</div>
<Toast state={toastState} />
