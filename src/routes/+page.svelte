<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { animate } from "animejs";
  import { moveWindow, Position } from "@tauri-apps/plugin-positioner";
    import { marked } from "marked";

  let prompt = $state("");
  let response = $state("");
  let responseHTML = $state("");
  let pos = $state({ x: -400, y: 60, h: 1.0, w: 1.0, r: 0.0 });

  async function submit(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    response = await invoke("prompt_gemini", { prompt });
    responseHTML = await marked.parse(response);
  }

  onMount(() => {
    reset();
  });

  function reset() {
    moveWindow(Position.BottomRight);
    animate(pos, {
      x: [
        { to: -400, duration: 0 },
        { to: 0, ease: "outCubic", duration: 800 },
      ],
      y: [
        { to: 200, ease: "outQuad", duration: 400 },
        { to: 60, ease: "inQuad", duration: 400 },
      ],
      h: [
        { to: 1.4, duration: 0 },
        { to: 1.0, ease: "outQuad", duration: 600 },
      ],
      w: [
        { to: 0.6, duration: 0 },
        { to: 1.0, ease: "outQuad", duration: 600 },
      ],
      r: [
        { to: -20, duration: 0 },
        { to: 0, ease: "inQuad", duration: 600 },
      ],
      onUpdate: () => {
        console.log("Position updated:", pos);
      }
    });
  }
</script>

<div class="h-[600px] fixed w-[400px] border"></div>
<div class="fixed h-[250px] w-full bg-yellow-200/70 p-4 rounded-lg prose overflow-y-auto scrollbar-hidden backdrop-blur-md">{@html responseHTML}</div>
<section
  class="fixed h-[300px] w-[400px]"
  style="right: {pos.x}px; bottom: {pos.y}px;"
>
  <div class="relative left-10 top-10 z-30 text-white flex flex-row gap-4">
    <h1 data-tauri-drag-region>M</h1>
    <button onclick={reset}>R</button>
  </div>
  <img src="/gup.png" alt="Gup" class="-z-10" style="scale: {pos.w} {pos.h}; rotate: {pos.r}deg"/>
  <img
    src="/gup.png"
    alt="Gup"
    class="-z-30 relative w-full h-1/2 brightness-0 opacity-50 scale-x-90 -translate-y-full blur-lg"
  />
  <form
    onsubmit={submit}
    class="flex flex-col items-center gap-4 h-full justify-center px-12 absolute z-20 top-12 w-full"
  >
    <input
      placeholder="What do you need?"
      bind:value={prompt}
      class="p-2 px-4 bg-yellow-200/70 rounded-lg outline-none backdrop-blur-xs"
      onsubmit={submit}
    />
    <!-- <p class="text-white h-30 break-words overflow-y-auto">{response}</p> -->
  </form>
</section>

<style>
  .scrollbar-hidden {
    scrollbar-width: none;
    -ms-overflow-style: none;
  }
  .scrollbar-hidden::-webkit-scrollbar {
    display: none;
  }
</style>
