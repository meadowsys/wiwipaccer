<template>
	<div>
		you have selected version <code>{{ selectedVersion }}</code> in build mode
	</div>
</template>

<script setup lang="ts">
	defineProps<{
		selectedVersion: string;
	}>();
</script>
