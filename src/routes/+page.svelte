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
  <img src="/gup.png" alt="Gup" class="fixed -z-20 w-full h-1/2 bottom-0 brightness-0 opacity-30 scale-x-90 blur-lg" />
  <form data-tauri-drag-region onsubmit={submit} class="flex flex-col items-center gap-4 h-screen justify-center px-16">
    <input
      placeholder="What do you need?"
      bind:value={prompt}
      class="p-2 bg-yellow-200 rounded-lg outline-none"
      onsubmit={submit}
    />
    <p class="text-white h-30 break-words max-w-3xs overflow-y-auto">{response}</p>
  </form>
</main>
