<template>
	<div class="flex flex-col h-full">
		<div class="h-[6rem] bg-white text-center select-none cursor-default" data-tauri-drag-region>
			{{ title }}
		</div>
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
