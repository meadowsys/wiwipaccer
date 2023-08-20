<template>
	<div class="border border-base-300 rounded-md w-fit" :data-part-of-selectcomponent="id">
		<div :data-part-of-selectcomponent="id">
			<input
				type="text"
				v-model="input_text"
				class="border-none outline-none mx-3 my-1"
				@mousedown="options_shown = !options_shown"
				:data-part-of-selectcomponent="id"
			>
			<ul
				class="p-2 border-t border-t-base-300"
				:class="options_classes"
				:data-part-of-selectcomponent="id"
			>
				<li
					v-for="option in opts_with_stuff"
					class="hover:bg-base-200 active:bg-base-300 rounded-sm select-none cursor-pointer px-1"
					:class="option[1]"
					tabindex="-1"
					@click="set_option(option[0])"
					:data-part-of-selectcomponent="id"
				>
					{{ option[0] }}
				</li>
			</ul>
		</div>
	</div>
</template>

<script setup lang="ts">
	const p = defineProps<{
		options: Array<string>;
		default?: string;
	}>();

	const id = use_id();
	const input_text = ref(p.default || "");

	const options_shown = ref(false);
	const option_set = ref(false);

	const opts_with_stuff = computed(() => p.options.map(option => [
		option,
		option.includes(input_text.value) ? "" : "hidden"
	] as const));
	const options_classes = computed(() =>
		options_shown.value && !opts_with_stuff.value.every(o => o[1] === "hidden")
			? ""
			: "hidden"
	);
	watch(input_text, v => option_set.value = p.options.includes(v));

	function set_option(option: string) {
		input_text.value = option;
		options_shown.value = false;
	}

	function check_option() {
		if (p.options.includes(input_text.value)) {
			option_set.value = true;
		}
	}

	function global_mousedown(e: MouseEvent) {
		let data_attr = (e.target as HTMLElement).getAttribute("data-part-of-selectcomponent");
		if (data_attr === null || data_attr !== id) {
			// element focused is not our element, ie. user clicked outside the element
			options_shown.value = false;
			check_option();
		}
	}

	onMounted(() => {
		document.addEventListener("mousedown", global_mousedown);
	});

	onBeforeUnmount(() => {
		document.removeEventListener("mousedown", global_mousedown);
	});

	type Option =
		| { set: false; }
		| { set: true; option: string; };
	const option = computed<Option>(() => {
		if (!option_set.value) return { set: false };
		return { set: true, option: input_text.value };
	});

	defineExpose({ option });
</script>
