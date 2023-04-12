<template>
	<div class="h-full flex-grow py-8 px-6" data-tauri-drag-region>
		<div class="overflow-scroll h-full flex flex-col">
			<button
				v-for="recent in p.recents"
				@click="invoke_open_project(recent.path)"
				class="
					w-full p-2 rounded bg-black bg-opacity-0 border-black border-opacity-0 border-2 text-left text-sm break-all
					hover:border-opacity-10 hover:bg-opacity-10
					active:bg-opacity-20 active:shadow-inner
				"
			>
				<div class="font-bold select-none">{{ recent.name }}</div>
				<div class="select-none">{{ recent.path }}</div>
			</button>
			<div class="flex-grow" data-tauri-drag-region />
		</div>
	</div>
</template>

<script setup lang="ts">
	let p = defineProps<{
		recents: Array<{ name: string, path: string }>;
	}>();
</script>

<style scoped>
	.hyphens {
		/* todo figure this out lol
		   class is supposed to be on the <button> with the v-for */
		hyphens: auto;
	}
</style>
