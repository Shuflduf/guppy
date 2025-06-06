<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { moveSmoothly, moveWindowBy } from "../lib/window";
  import { onMount } from "svelte";
  import { PhysicalPosition } from "@tauri-apps/api/dpi";

  let prompt = $state("");
  let response = $state("");

  async function submit(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    response = await invoke("prompt_gemini", { prompt });
  }

  onMount(async () => {
    // moveSmoothly();
    // setInterval(async () => {
    //   await moveWindowBy(new PhysicalPosition(1, 0));
    // }, 1);
    // for (let frame = 0; frame < 100; frame++) {
    //   await moveWindowBy(new PhysicalPosition(0, (frame - 50)/2));
    //   await new Promise((resolve) => setTimeout(resolve, 1));
    // }
  });
</script>

<main class="container">
  <img
    data-tauri-drag-region
    src="/gup.png"
    alt="Gup"
    class="fixed -z-10 w-full h-full"
  />
  <img
    src="/gup.png"
    alt="Gup"
    class="fixed -z-20 w-full h-1/2 bottom-0 brightness-0 opacity-30 scale-x-90 blur-lg"
  />
  <form
    data-tauri-drag-region
    onsubmit={submit}
    class="flex flex-col items-center gap-4 h-full justify-center px-4 fixed top-0 left-0 right-0 mx-12"
  >
    <input
      placeholder="What do you need?"
      bind:value={prompt}
      class="p-2 px-4 bg-yellow-200/70 rounded-lg outline-none backdrop-blur-xs"
      onsubmit={submit}
    />
    <p class="text-white h-30 break-words overflow-y-auto">{response}</p>
  </form>
</main>
