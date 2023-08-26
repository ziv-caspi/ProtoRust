<script lang="ts">
  import { Listgroup } from "flowbite-svelte";
  import { createEventDispatcher } from "svelte";

  export let messageTypes: string[];
  const dispatch = createEventDispatcher();

  function onMessageClicked(e: CustomEvent<any>): void {
    console.log("message clicked");
    dispatch("click", {
      name: e.detail,
    });
  }
</script>

{#if messageTypes}
  <Listgroup
    class="w-40 h-min"
    active
    items={messageTypes}
    let:item
    on:click={onMessageClicked}
  >
    {item}
  </Listgroup>
{:else}
  <p class="text-primary-50">no protos loaded</p>
{/if}
