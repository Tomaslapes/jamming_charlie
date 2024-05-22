<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { open } from "@tauri-apps/api/dialog";

	let name = "";
	let greetMsg = "";

	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		greetMsg = await invoke("greet", { name })
		// const selected = await open({
		// 	multiple: true,
		// 	filters: [
		// 		{
		// 			name: "Image",
		// 			extensions: ["png", "jpeg"],
		// 		},
		// 	],
		// });
	}
</script>

<div>
	<form class="row" on:submit|preventDefault={greet}>
		<input id="greet-input" placeholder="Enter a name..." bind:value={name} />
		<button type="submit">Greet</button>
	</form>
	<p>{greetMsg}</p>
</div>
