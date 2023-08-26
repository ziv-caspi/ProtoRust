<script lang="ts">
  import { Button, Input, Select } from "flowbite-svelte";
  import { createEventDispatcher } from "svelte";

  export let publishLoading: boolean;
  const dispatch = createEventDispatcher();
  let host: string;
  let target: string;
  let type: string;
  const options = [
    { name: "Echange", value: "exchange" },
    { name: "Queue", value: "queue" },
  ];
</script>

<div class="flex flex-row grow">
  <Input bind:value={host} class="m-1" id="host" placeholder="localhost:5672" />
  <Select class="m-1" items={options} bind:value={type} />
  <Input bind:value={target} class="m-1" id="target" placeholder="MY_QUEUE" />
  <Button
    class="m-1"
    on:click={() => dispatch("publish", { host, type, target })}
  >
    {#if publishLoading}
      Loading...
    {:else}
      Publish
    {/if}
  </Button>
</div>
