<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { animate } from "animejs";
  import { moveWindow, Position } from "@tauri-apps/plugin-positioner";
  import { marked } from "marked";

  let prompt = $state("");
  let responseHTML = $state("");
  const startPos = { x: 0, y: 60, h: 1.0, w: 1.0, r: 0.0 };
  let pos = $state(startPos);
  let chatHistory: { role: string; content: string }[] = $state([]);
  const keepHistory = $state(false);

  async function submit(event: Event) {
    event.preventDefault();
    if (!keepHistory) {
      responseHTML = "";
    }
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    let jumping = animate(pos, {
      y: [
        { to: 100, ease: "outQuad", duration: 400 },
        { to: 60, ease: "inQuad", duration: 400 },
      ],
      h: [
        { to: 1.05, duration: 50 },
        { to: 1.0, ease: "inOutQuad", duration: 600 },
      ],
      w: [
        { to: 0.9, duration: 50 },
        { to: 1.0, ease: "inOutQuad", duration: 600 },
      ],
      loop: true,
    });
    chatHistory.push({ role: "user", content: prompt });
    let response: string = await invoke("prompt_gemini", {
      chatHistory: JSON.stringify(chatHistory),
    });
    chatHistory.push({ role: "model", content: response });
    prompt = "";
    responseHTML += await marked.parse(response);
    responseHTML += "<hr>";
    jumping.cancel();
    pos = startPos;
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
    });
  }

  function resetAI() {
    chatHistory = [];
    responseHTML = "";
  }
</script>

<div class="fixed h-[600px] w-[400px] border"></div>
{#if responseHTML}
  <div
    class="prose scrollbar-hidden fixed h-[250px] w-full overflow-y-auto rounded-lg bg-yellow-200/70 p-4 backdrop-blur-md"
  >
    {@html responseHTML}
  </div>
  <button onclick={resetAI} class="fixed top-4 right-4">X</button>
{/if}
<section
  class="fixed h-[300px] w-[400px]"
  style="right: {pos.x}px; bottom: {pos.y}px;"
>
  <div class="relative top-10 left-10 z-30 flex flex-row gap-4 text-white">
    <h1 data-tauri-drag-region>M</h1>
    <button onclick={reset}>R</button>
  </div>
  <img
    src="/gup.png"
    alt="Gup"
    class="-z-10"
    style="scale: {pos.w} {pos.h}; rotate: {pos.r}deg"
  />
  <img
    src="/gup.png"
    alt="Gup"
    class="relative -z-30 h-1/2 w-full -translate-y-full scale-x-90 opacity-50 blur-lg brightness-0"
  />
  <form
    onsubmit={submit}
    class="absolute top-12 z-20 flex h-full w-full flex-col items-center justify-center gap-4 px-12"
  >
    <input
      placeholder="What do you need?"
      bind:value={prompt}
      class="rounded-lg bg-yellow-200/70 p-2 px-4 backdrop-blur-xs outline-none"
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
