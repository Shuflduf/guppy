<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { animate } from "animejs";

  let prompt = $state("");
  let response = $state("");
  let pos = $state({x: -400, y: 40});

  async function submit(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    response = await invoke("prompt_gemini", { prompt });
  }

  onMount(() => {
    animate(
      pos,
      {
        x: 0,
        y: [
          { to: 200, ease: "outQuad" },
          { to: 40, ease: "inQuad" },
        ],
        duration: 799,
      },
    );
  });
</script>

<div class="h-[600px] fixed w-[400px]"></div>
<section class="w-[400px] h-[300px] fixed" style="right: {pos.x}px; bottom: {pos.y}px;">
  <img src="/gup.png" alt="Gup" class="-z-10" />
  <img
    src="/gup.png"
    alt="Gup"
    class="-z-30 relative w-full h-1/2 brightness-0 opacity-50 scale-x-90 -translate-y-full blur-lg"
  />
  <form
    onsubmit={submit}
    class="flex flex-col items-center gap-4 h-full justify-center px-12 absolute z-20 top-0 w-full"
  >
    <input
      placeholder="What do you need?"
      bind:value={prompt}
      class="p-2 px-4 bg-yellow-200/70 rounded-lg outline-none backdrop-blur-xs"
      onsubmit={submit}
    />
    <p class="text-white h-30 break-words overflow-y-auto">{response}</p>
  </form>
</section>
