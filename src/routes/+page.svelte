<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
    import { moveWindow, Position } from "@tauri-apps/plugin-positioner";
    import { onMount } from "svelte";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }

  onMount(() => {
    moveWindow(Position.TrayBottomRight)
  });
</script>

<main class="container">
  <img src="/gup.jpg" alt="Gup" class="fixed -z-10 w-full h-full">
  <form onsubmit={greet}>
    <input placeholder="Enter a name..." bind:value={name} class="p-2 bg-slate-500 rounded-lg"/>
    <button type="submit" class="text-white">Greet</button>
  </form>
  <p class="text-white">{greetMsg}</p>
</main>
