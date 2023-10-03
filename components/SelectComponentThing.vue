<template>
	<div class="w-fit text-xs z-50" :data-part-of-selectcomponent="id">
		<div :data-part-of-selectcomponent="id" :class="shadow" class="rounded-lg shadow-base-200">
			<input
				type="text"
				ref="input"
				v-model="input_text"
				class="outline-none border rounded-lg border-base-300 px-3 hover:border-primary focus:border-primary cursor-pointer focus:cursor-text text-xs text-center"
				style="box-sizing: border-box; font-weight: bold"
				:style="{ width }"
				:class="input_classes"
				@mousedown="input_mousedown"
				@focusin="input_focusin"
				:data-part-of-selectcomponent="id"
			>
			<ul
				class="p-1 border-b border-x border-b-primary border-primary rounded-b-lg overflow-scroll max-h-64 bg-base-100"
				:class="options_classes"
				:data-part-of-selectcomponent="id"
			>
				<li
					v-for="option in opts_with_stuff"
					class="hover:bg-base-200 active:bg-base-300 rounded-md select-none cursor-pointer px-2 py-1 bg-base-100"
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
		options: ReadonlyArray<string>;
		default?: string;
		width: string;
		modelValue: string;
	}>();

	const emit = defineEmits<{
		(e: "update:modelValue", val: string): void;
	}>();

	const input = ref<HTMLInputElement>();

	const id = use_id();
	const input_text = ref(p.default || "");

	const options_shown = ref(false);
	const option_set = ref(false);
	const override_option_hiding = ref(false);

	const last_good_value = ref(p.default || "");
	watch([option_set, input_text], () => {
		if (option_set.value) {
			last_good_value.value = input_text.value;
		}
	});
	watch(last_good_value, val => emit("update:modelValue", val));

	const opts_with_stuff = computed<Array<readonly [option: string, hidden: "" | "hidden"]>>(
		() => p.options.map(option => [
			option,
			search(option, input_text.value) || override_option_hiding.value ? "" : "hidden"
		] as const)
	);
	const is_options_shown = computed(() => options_shown.value && !opts_with_stuff.value.every(o => o[1] === "hidden"));
	const input_classes = computed(() => is_options_shown.value ? ["rounded-b-none", "border-primary", ""] : ["border-base-300"]);
	const options_classes = computed(() => is_options_shown.value ? [] : ["hidden"]);
	const shadow = computed(() => is_options_shown.value ? ["shadow-lg"] : []);

	watch([input_text, p], ([v]) => {
		override_option_hiding.value = false;
		option_set.value = p.options.includes(v);
		if (!option_set.value) options_shown.value = true;
	});

	watch(p, v => {
		if (!v.options.includes(input_text.value) && v.default) {
			input_text.value = v.default;
		}
	});

	function set_option(option: string) {
		input_text.value = option;
		options_shown.value = false;
	}

	function check_option() {
		option_set.value = p.options.includes(input_text.value);
	}

	function input_mousedown() {
		if (options_shown.value && opts_with_stuff.value.every(o => o[1] === "hidden")) {
			// if its shown but effectively hidden because there's no results
			override_option_hiding.value = true;
		} else {
			options_shown.value = !options_shown.value;
			override_option_hiding.value = true;
		}
	}

	function input_focusin() {
		input.value!.select();
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
		if (p.default) emit("update:modelValue", p.default);
	});

	onBeforeUnmount(() => {
		document.removeEventListener("mousedown", global_mousedown);
	});

	function search(option: string, input_text: string) {
		let input_text_tokens = input_text.split(" ")
			.map(s => s.trim())
			.filter(s => s.length > 0);
		let option_tokens = option.split(" ")
			.map(s => s.trim())
			.filter(s => s.length > 0);

		return input_text_tokens.every(t => !option_tokens.every(o => !o.includes(t)));
	}
</script>
