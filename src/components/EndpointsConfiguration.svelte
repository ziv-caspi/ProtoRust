<script lang="ts">
  import type { EndpointConfig } from "$lib";
  import { Button, Input, Select, Modal, Toggle, Label } from "flowbite-svelte";
  import { createEventDispatcher } from "svelte";

  export let publishLoading: boolean;
  const dispatch = createEventDispatcher();
  let modalOpen = false;
  let username = "admin";
  let password = "admin";
  let host: string;
  let loop: boolean = false;
  let rate = 1;
  let routingKey = "/";
  let target: string;
  let type: string;
  const options = [
    { name: "Echange", value: "exchange" },
    { name: "Queue", value: "queue" },
  ];

  function dispatchResult() {
    let config: EndpointConfig = {
      host,
      target,
      type,
      loop,
      routingKey,
      ratePerSec: rate,
      username,
      password,
    };
    dispatch("publish", config);
  }
</script>

<div class="flex flex-row grow">
  <Input bind:value={host} class="m-1" id="host" placeholder="localhost:5672" />
  <Select class="m-1" items={options} bind:value={type} />
  <Input bind:value={target} class="m-1" id="target" placeholder="MY_QUEUE" />
  <Button class="m-1" color="light" on:click={() => (modalOpen = true)}>
    More
  </Button>
  <Button class="m-1" on:click={dispatchResult}>
    {#if publishLoading}
      Loading...
    {:else}
      Publish
    {/if}
  </Button>

  <Modal title="Advanced" bind:open={modalOpen} autoclose outsideclose>
    <div class="flex flex-row justify-between">
      <div class="flex flex-col">
        <Label>Loop</Label>
        <Toggle bind:value={loop} />
        <Label>Messages/Second</Label>
        <Input type="number" bind:value={routingKey} />
      </div>
      <div class="flex flex-col">
        <Label>Username</Label>
        <Input type="text" bind:value={username} />
        <Label>Password</Label>
        <Input type="text" bind:value={password} />
        <Label>Routing Key</Label>
        <Input type="text" bind:value={routingKey} />
      </div>
    </div>
  </Modal>
</div>
