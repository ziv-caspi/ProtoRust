<script lang="ts">
  import { Listgroup } from "flowbite-svelte";
  import { createEventDispatcher } from "svelte";

  export let messageTypes: string[];
  const dispatch = createEventDispatcher();

  function onMessageClicked(e: CustomEvent<any>): void {
    dispatch("click", e.detail);
  }
</script>

{#if messageTypes && messageTypes.length > 0}
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
  <p class="text-primary-50">no properly loaded protos</p>
{/if}
