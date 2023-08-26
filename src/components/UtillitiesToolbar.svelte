<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { Button, Tooltip } from "flowbite-svelte";
  import { createEventDispatcher } from "svelte";

  export let disableFileSelection: boolean;
  const dispatch = createEventDispatcher();

  async function selectDirectory() {
    const selected = (await open({
      directory: true,
    })) as string;
    console.log(selected);
    dispatch("dirSelected", selected);
  }

  async function selectProtoFile() {
    const selected = (await open({
      filters: [
        {
          name: "Proto",
          extensions: ["proto"],
        },
      ],
    })) as string;
    console.log(selected);
    dispatch("fileSelected", selected);
  }
</script>

<Button tooltip class="mr-1" on:click={selectDirectory}>
  <svg
    class="w-3 h-3 text-gray-800 dark:text-white"
    aria-hidden="true"
    xmlns="http://www.w3.org/2000/svg"
    fill="currentColor"
    viewBox="0 0 18 18"
  >
    <path
      d="M18 5H0v11a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V5Zm-7.258-2L9.092.8a2.009 2.009 0 0 0-1.6-.8H2.049a2 2 0 0 0-2 2v1h10.693Z"
    />
  </svg>
</Button>
<Tooltip>load the proto dependencies</Tooltip>

<Button disabled={disableFileSelection} tooltip on:click={selectProtoFile}>
  <svg
    class="w-3 h-3 text-gray-800 dark:text-white"
    aria-hidden="true"
    xmlns="http://www.w3.org/2000/svg"
    fill="currentColor"
    viewBox="0 0 16 20"
  >
    <path
      d="M5 5V.13a2.96 2.96 0 0 0-1.293.749L.879 3.707A2.98 2.98 0 0 0 .13 5H5Z"
    />
    <path
      d="M14.066 0H7v5a2 2 0 0 1-2 2H0v11a1.97 1.97 0 0 0 1.934 2h12.132A1.97 1.97 0 0 0 16 18V2a1.97 1.97 0 0 0-1.934-2Z"
    />
  </svg>
</Button>
<Tooltip>load the proto file</Tooltip>
