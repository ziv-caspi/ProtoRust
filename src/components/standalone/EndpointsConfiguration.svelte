<script lang="ts">
  import type { EndpointConfig } from "$lib";
  import { Button, Input, Select, Modal, Toggle, Label } from "flowbite-svelte";
  import { createEventDispatcher } from "svelte";

  export let publishLoading: boolean;
  const dispatch = createEventDispatcher();
  let modalOpen = false;
  let username = "admin";
  let password = "admin";
  let host: string = "localhost:5672";
  let loop: boolean = false;
  let rate = 1;
  let routingKey = "/";
  let target: string = "MY_QUEUE";
  let type: string = "queue";
  let speed: number = 10;
  let quantity: number = 1;

  const options = [
    { name: "Echange", value: "exchange" },
    { name: "Queue", value: "queue" },
  ];

  function dispatchResult() {
    const splitHost = host.split(":");

    let config: EndpointConfig = {
      host: splitHost[0],
      port: Number(splitHost[1]),
      target,
      type,
      loop,
      routingKey,
      ratePerSec: rate,
      username,
      password,
      speed,
      quantity,
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
      <!-- <div class="flex flex-col">
        <Label>Loop</Label>
        <Toggle bind:value={loop} />
        <Label>Messages/Second</Label>
        <Input type="number" bind:value={routingKey} />
      </div> -->
      <div class="flex flex-col">
        <Label>Username</Label>
        <Input type="text" bind:value={username} />
        <Label>Password</Label>
        <Input type="text" bind:value={password} />
        <Label>Routing Key</Label>
        <Input type="text" bind:value={routingKey} />
      </div>
      <div class="flex flex-col">
        <Label>Messages To Publish</Label>
        <Input type="text" bind:value={quantity} />
        <Label>Messages\Second</Label>
        <Input type="text" bind:value={speed} />
      </div>
    </div>
  </Modal>
</div>
