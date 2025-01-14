<script lang="ts">
	let {
		min = 0,
		max = 1,
		value = $bindable(),
		color = 'var(--fg)',
		thumbColor = 'var(--fg)',
		backgroundColor = 'rgba(100, 100, 100, 0.18)',
		style = 'classic'
	}: {
		min?: number;
		max?: number;
		value: number;
		color?: string;
		thumbColor?: string;
		backgroundColor?: string;
		style?: 'classic' | 'minimal' | 'thick';
	} = $props();

	let sliderElement: HTMLDivElement;
	let sliderWidth: number = $state(0);
	let isDragging = $state(false);

	$effect(() => {
		sliderWidth = sliderElement.clientWidth;
	});

	async function updateValue(clientX: number) {
		const rect = sliderElement.getBoundingClientRect();
		const newValue = (clientX - rect.left) / sliderWidth;
		const clampedValue = Math.min(Math.max(newValue, 0), 1);
		value = clampedValue * max;
	}

	/////// Event Handlers

	async function handleMouseDown(event: MouseEvent) {
		isDragging = true;
		sliderWidth = sliderElement.clientWidth;
		await updateValue(event.clientX);
	}

	async function handleMouseMove(event: MouseEvent) {
		if (isDragging) {
			await updateValue(event.clientX);
		}
	}

	function handleMouseUp() {
		if (isDragging) {
			isDragging = false;
		}
	}

	async function handleTouchStart(event: TouchEvent) {
		isDragging = true;
		sliderWidth = sliderElement.clientWidth;
		await updateValue(event.touches[0].clientX);
	}

	async function handleTouchMove(event: TouchEvent) {
		if (isDragging) {
			await updateValue(event.touches[0].clientX);
		}
	}

	function handleTouchEnd() {
		if (isDragging) {
			isDragging = false;
		}
	}

	function addGlobalEventListeners() {
		window.addEventListener('mousemove', handleMouseMove);
		window.addEventListener('mouseup', handleMouseUp);
		window.addEventListener('touchmove', handleTouchMove);
		window.addEventListener('touchend', handleTouchEnd);
	}

	function removeGlobalEventListeners() {
		window.removeEventListener('mousemove', handleMouseMove);
		window.removeEventListener('mouseup', handleMouseUp);
		window.removeEventListener('touchmove', handleTouchMove);
		window.removeEventListener('touchend', handleTouchEnd);
	}

	$effect(() => {
		addGlobalEventListeners();

		return () => {
			removeGlobalEventListeners();
		};
	});

	function clamp(value: number) {
		if (value < min) return min;
		if (value > max) return max;
		return value;
	}

	function handleKeypress(
		e: KeyboardEvent & {
			currentTarget: EventTarget & HTMLDivElement;
		}
	) {
		const key = e.key.toLowerCase();
		const diff = 0.01 * max;

		if (key === 'arrowleft') {
			value = clamp(value - diff);
		}
		if (key === 'arrowright') {
			value = clamp(value + diff);
		}
	}
</script>

<div
	class="slider"
	class:isDragging
	data-style={style}
	style="--pos: {((clamp(value) * 100) / max).toFixed(
		2
	)}%; --v-color: {color}; --t-color: {thumbColor}; --bg-clr: {backgroundColor};"
	bind:this={sliderElement}
	onmousedown={async (e) => await handleMouseDown(e)}
	ontouchstart={async (e) => await handleTouchStart(e)}
	role="slider"
	tabindex="-1"
	aria-valuenow={Number(value.toFixed(2))}
>
	<div role="progressbar" class="hitbox"></div>
	<div class="handle" role="button" onkeydown={handleKeypress} tabindex="0"></div>
</div>

<style>
	.slider {
		min-width: 7em;
		max-width: 100%;
		height: 0.2em;
		position: relative;
		background-color: var(--bg-clr);
		border-radius: 1em;
		cursor: pointer;
	}

	.slider[data-style='thick'] {
		height: 0.5em;
	}

	.hitbox {
		height: 100%;
		width: var(--pos);
		position: absolute;
		top: 0;
		left: 0;
		background-color: var(--v-color);
		opacity: 0.7;
		border-radius: 1em;
	}

	.handle {
		height: 0.8em;
		width: 0.8em;
		border-radius: 50%;
		border: 1px solid rgba(100, 100, 100, 0.18);
		background-color: var(--t-color);
		box-shadow: rgba(100, 100, 111, 0.2) 0px 7px 29px 0px;
		position: absolute;
		top: calc(100% - 0.4em * 1.3);
		left: calc(var(--pos) - 0.4em);
	}

	.handle:focus {
		opacity: 1;
	}

	.slider[data-style='thick'] .handle {
		height: 1em;
		width: 1em;
		top: calc(100% - 0.5em * 1.5);
		left: calc(var(--pos) - 0.5em);
	}

	.slider[data-style='minimal'] .handle {
		opacity: 0;
		width: 5px;
		height: 0.2em;
		bottom: 0;
		border-radius: 1px;
		left: calc(var(--pos) - 2.5px);
		transition: opacity 0.1s ease-in-out;
	}

	.slider[data-style='minimal'].isDragging:hover .handle {
		opacity: 1;
	}
</style>
