<template>
	<div class="h-full flex flex-row">
		<div
			class="bg-opacity-100 bg-white py-10 px-14 flex flex-col h-full"
			data-tauri-drag-region
		>
			<div class="cursor-default text-4xl pb-2 px-2" data-tauri-drag-region>wiwipaccer</div>
			<div class="cursor-default text-neutral-500 px-2" data-tauri-drag-region>Pack Builder&trade;</div>

			<div class="flex-grow" data-tauri-drag-region />

			<welcome-header-button>
				New Project
			</welcome-header-button>
			<welcome-header-button>
				Open Project
			</welcome-header-button>
			<div class="flex flex-col">
			</div>
		</div>

		<welcome-project-history :recents="recents" />

		<!-- <div class="py-8 px-12">
			<table class="w-full">
				<welcome-recents-entry-button v-for="item in recent_stuff" :item="item" />
			</table>
		</div> -->
	</div>
</template>

<script setup lang="ts">
	let recents: Array<{ name: string, path: string }> = [
		{ name: "recent project", path: "/a/path/to/recent-project" },
		{ name: "another recent asdfghgfdsasdfghgfdsasdfsadfghjgfdsaproject", path: "/a/path/to/anotherrecentprojectanotherrecentprojectanotherrecentprojectanotherrecentprojectanotherrecentprojectanotherrecentprojectanotherrecentprojectanotherrecentprojectanotherrecentproject"}
	];
</script>
