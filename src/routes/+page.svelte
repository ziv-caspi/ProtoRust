<script lang="ts">
  import MessageSection from "../components/MessageSection.svelte";
  import ProtoSection from "../components/ProtoSection.svelte";

  let jsonValue: string;
</script>

<div class="flex h-screen items-stretch bg-gray-900">
  <div class="flex flex-row grow">
    <!-- selector -->
    <ProtoSection on:messageSelected={(event) => (jsonValue = event.detail)} />
    <MessageSection bind:jsonValue />
    <!-- editor -->
  </div>
</div>
