<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import * as Select from "$lib/components/ui/select/index.js";
	import * as Card from "$lib/components/ui/card";
	import { Badge } from "$lib/components/ui/badge/index.js";

	const fruits = [
		{ value: "apple", label: "Apple" },
		{ value: "banana", label: "Banana" },
		{ value: "blueberry", label: "Blueberry" },
		{ value: "grapes", label: "Grapes" },
		{ value: "pineapple", label: "Pineapple" },
	];
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

		<div class="w-full flex gap-2 my-3">
			<Select.Root portal={null}>
				<Select.Trigger class="w-full">
					<Select.Value placeholder="Select a port" />
				</Select.Trigger>
				<Select.Content>
					<Select.Group>
						<Select.Label>Fruits</Select.Label>
						{#each fruits as fruit}
							<Select.Item value={fruit.value} label={fruit.label}>{fruit.label}</Select.Item>
						{/each}
					</Select.Group>
				</Select.Content>
				<Select.Input name="favoriteFruit" />
			</Select.Root>
			<Button>Connect</Button>
		</div>
	</div>
</div>
