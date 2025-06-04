<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let prompt = $state("");
  let response = $state("");

  async function submit(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    response = await invoke("greet", { name: prompt });
  }

</script>

<main class="container">
  <img src="/gup.png" alt="Gup" class="fixed -z-10 w-full h-full" />
  <form data-tauri-drag-region onsubmit={submit} class="flex flex-col items-center gap-4 h-screen justify-center px-16">
    <input
      placeholder="What do you need?"
      bind:value={prompt}
      class="p-2 bg-slate-500 rounded-lg"
      onsubmit={submit}
    />
    <p class="text-white">{response}</p>
  </form>
</main>
