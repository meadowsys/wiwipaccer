<template>
	<div class="flex flex-col h-full">
		<div class="h-[26px] bg-white text-center select-none cursor-default flex-shrink-0 flex-grow-0" data-tauri-drag-region>
			{{ title }}
		</div>
		<div class="mx-10 border border-neutral-200" />
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
