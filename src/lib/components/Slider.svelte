<script lang="ts">
	let {
		value,
		color = 'var(--fg)',
		thumbColor = 'var(--fg)',
		backgroundColor = 'rgba(100, 100, 100, 0.18)',
		style = 'classic',
		oninput = (_data: number) => {}
	}: {
		value: number;
		color?: string;
		thumbColor?: string;
		backgroundColor?: string;
		style?: 'classic' | 'minimal' | 'thick';
		oninput?: (data: number) => void;
	} = $props();

	let sliderElement: HTMLDivElement;
	let sliderWidth: number = $state(0);
	let isDragging = $state(false);
	let innerValue = $state(0);

	$effect(() => {
		sliderWidth = sliderElement.clientWidth;
	});

	function updateValue(clientX: number) {
		const rect = sliderElement.getBoundingClientRect();
		const newValue = (clientX - rect.left) / sliderWidth;
		const clampedValue = Math.min(Math.max(newValue, 0), 1);
		innerValue = clampedValue;
	}

	/////// Event Handlers

	function handleMouseDown(event: MouseEvent) {
		isDragging = true;
		sliderWidth = sliderElement.clientWidth;
		updateValue(event.clientX);
	}

	function handleMouseMove(event: MouseEvent) {
		if (isDragging) {
			updateValue(event.clientX);
		}
	}

	function handleMouseUp() {
		if (isDragging) {
			isDragging = false;
			oninput(innerValue);
		}
	}

	function handleTouchStart(event: TouchEvent) {
		isDragging = true;
		sliderWidth = sliderElement.clientWidth;
		updateValue(event.touches[0].clientX);
	}

	function handleTouchMove(event: TouchEvent) {
		if (isDragging) {
			updateValue(event.touches[0].clientX);
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
</script>

<div
	class="slider"
	class:isDragging
	data-style={style}
	style="--pos: {isDragging
		? innerValue * 100
		: value * 100}%; --v-color: {color}; --t-color: {thumbColor}; --bg-clr: {backgroundColor};"
	bind:this={sliderElement}
	onmousedown={handleMouseDown}
	ontouchstart={handleTouchStart}
	role="slider"
	aria-valuenow={isDragging ? innerValue : value}
	tabindex="0"
>
	<div class="hitbox"></div>
	<div class="handle"></div>
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

	.slider[data-style='minimal'] {
		border-radius: 0px;
	}

	.hitbox {
		height: 100%;
		width: var(--pos);
		position: absolute;
		top: 0;
		left: 0;
		background-color: var(--v-color);
		border-radius: 1em;
	}

	.slider[data-style='minimal'] .hitbox {
		border-radius: 0px;
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

	.slider[data-style='thick'] .handle {
		height: 1em;
		width: 1em;
		top: calc(100% - 0.5em * 1.5);
		left: calc(var(--pos) - 0.5em);
	}

	.slider[data-style='minimal'] .handle {
		opacity: 0;
		top: -200%;
		width: 5px;
		border-radius: 1px;
		left: calc(var(--pos) - 2.5px);
		transition: opacity 0.1s ease-in-out;
	}

	.slider[data-style='minimal']:hover .handle {
		opacity: 1;
	}

	.slider[data-style='minimal'].isDragging:hover .handle {
		opacity: 1;
	}
</style>