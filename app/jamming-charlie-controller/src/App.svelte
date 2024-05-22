<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import * as Select from "$lib/components/ui/select/index.js";
	import * as Card from "$lib/components/ui/card";
	import { Badge } from "$lib/components/ui/badge/index.js";
	import { invoke } from "@tauri-apps/api";
	import { onDestroy, onMount } from "svelte";

	async function get_serial_ports() {
		try {
			let res = await invoke("get_serial_ports");
			console.log(res)
			return res.map((port) => {
				return { value: port.port_name, label: `${port.port_name} - ${port.port_type}` };
			});
		} catch (err) {
			console.error(err);
			return [];
		}
	}

	let ports = get_serial_ports();
	let device_status = null;

	let selected_port = null;
	$: connect_disabled = selected_port == null;
	$: console.log(selected_port);

	const intervalId = setInterval(async () => {
		device_status = await invoke("get_device_state");
	}, 200); // Interval in milliseconds

	$: console.log(ports);

	onDestroy(() => {
		clearInterval(intervalId);
	});

	// onMount(async () => {
	// 	get_serial_ports();
	// });

	async function handleConnect() {
		try {
			let res = await invoke("connect", { portName: selected_port });
			console.log(res)
		} catch (err) {
			console.error(err);
			// return [];
		}
	}


</script>

<div class="w-screen items-center flex flex-col p-10">
	<div class="max-w-4xl w-full">
		<h1 class="text-2xl font-bold">Robotic Ukulele Controller</h1>

		<Card.Root class="my-5">
			<Card.Header>
				<Card.Title>
					<div>
						Status
						<Badge variant="destructive">Offline</Badge>
					</div>
				</Card.Title>
				<Card.Description>Card Description</Card.Description>
			</Card.Header>
			<Card.Content>
				<p></p>
			</Card.Content>
			<Card.Footer>
				<p>Card Footer</p>
			</Card.Footer>
		</Card.Root>
		{#await ports}
			Loading...
		{:then ports}
			<div class="w-full flex gap-2 my-3">
				<Select.Root
					portal={null}
					selected={selected_port}
					onSelectedChange={(v) => {
						selected_port = v.value;
					}}
				>
					<Select.Trigger class="w-full">
						<Select.Value placeholder="Select a port" />
					</Select.Trigger>
					<Select.Content>
						<Select.Group>
							<Select.Label>Ports</Select.Label>
							{#each ports as port}
								<Select.Item value={port.value} label={port.label}>{port.label}</Select.Item>
							{/each}
						</Select.Group>
					</Select.Content>
					<Select.Input name="favoriteFruit" />
				</Select.Root>
				<Button disabled={connect_disabled} on:click={handleConnect}>Connect</Button>
				<Button on:click={()=>{
					ports = get_serial_ports();
				}}>Refresh</Button>
			</div>
		{/await}
	</div>
</div>
