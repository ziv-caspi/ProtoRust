<script lang="ts">
  import { AceEditor } from "svelte-ace";
  import "brace/mode/json";
  import "brace/theme/chrome";
  import "brace/theme/chaos";
  export let value: string;
</script>

<div class="flex w-full">
  <AceEditor lang="json" theme="chaos" bind:value />
</div>
