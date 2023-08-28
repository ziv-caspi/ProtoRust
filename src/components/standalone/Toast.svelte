<script lang="ts">
  import { Toast } from "flowbite-svelte";
  import { Icon } from "flowbite-svelte-icons";

  export let state: {
    shouldOpen: boolean;
    isError: boolean;
    errorMessage: string;
  };

  $: shouldOpenSuccess = state.shouldOpen && !state.isError;
  $: shouldOpenFailure = state.shouldOpen && state.isError;
  $: if (state.shouldOpen) {
    new Promise((r) => setTimeout(r, 2000)).then(
      () => (state = { ...state, shouldOpen: false })
    );
  }

  function closeToast(e: CustomEvent<any>): void {
    state = { ...state, shouldOpen: false };
  }
</script>

<!-- Good Toast -->
<Toast
  open={shouldOpenSuccess}
  on:close={closeToast}
  color="green"
  position="bottom-right"
>
  <svelte:fragment slot="icon">
    <Icon name="check-circle-solid" class="w-5 h-5" />
    <span class="sr-only">Check icon</span>
  </svelte:fragment>
  Published message successfully.
</Toast>

<!-- Bad Toast -->
<Toast
  open={shouldOpenFailure}
  on:close={closeToast}
  color="red"
  position="bottom-right"
>
  <svelte:fragment slot="icon">
    <Icon name="check-circle-solid" class="w-5 h-5" />
    <span class="sr-only">Check icon</span>
  </svelte:fragment>
  Failed to publish. {state.errorMessage}
</Toast>
