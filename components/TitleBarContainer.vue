<template>
	<div class="flex flex-col h-full">
		<div class="h-[26px] bg-white select-none cursor-default flex-shrink-0 flex-grow-0 flex flex-row">
			<div class="flex-grow" data-tauri-drag-region />
			<div data-tauri-drag-region>
				{{ title }}
			</div>
			<div class="flex-grow basis-0" data-tauri-drag-region>
				<slot name="right" />
			</div>
		</div>
		<div class="border border-neutral-200" />
		<div class="flex-grow overflow-scroll">
			<slot />
		</div>
	</div>
</template>

<script setup lang="ts">
	defineProps<{
		title: string;
	}>();
</script>
