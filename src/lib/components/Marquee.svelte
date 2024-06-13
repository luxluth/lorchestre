<script lang="ts">
	import { onMount, type Snippet } from 'svelte';
	import MarqueeState from './marquee.svelte';

	let { children, width = '14vw' }: { children: Snippet; width: string } = $props();

	let marquee: HTMLDivElement;
	let content: HTMLDivElement;

	let state = new MarqueeState();

	onMount(() => {
		const resizeObserver = new ResizeObserver(() => {
			state.updateOverflow(content, marquee);
		});

		resizeObserver.observe(marquee);
		resizeObserver.observe(content);
		const mutationObserver = new MutationObserver((mutations) => {
			mutations.forEach((mutation) => {
				if (mutation.type === 'attributes') {
					state.updateOverflow(content, marquee);
				}
			});
		});

		mutationObserver.observe(content, {
			attributes: true,
			childList: true,
			subtree: true
		});

		state.updateOverflow(content, marquee);
		return () => {
			resizeObserver.unobserve(marquee);
			resizeObserver.unobserve(content);
			mutationObserver.disconnect();
		};
	});
</script>

<div
	class="marquee"
	class:overflow={state.overflow}
	class:is_animating={state.animating}
	style="width: {width}; --marquee-scroll-width: {state.marqueeScrollWidth}"
	bind:this={marquee}
>
	<div
		data-width={state.marqueeScrollWidth}
		class="content"
		class:overflow={state.overflow}
		class:animating={state.animating}
		bind:this={content}
		onanimationend={() => state.reset()}
	>
		{@render children()}
		<div class="clone" class:overflow={state.overflow}>
			{@render children()}
		</div>
	</div>
</div>

<style>
	.clone:not(.overflow) {
		display: none;
	}

	.marquee {
		overflow: hidden;
		text-wrap: nowrap;
	}

	.marquee:not(.overflow) {
		display: flex;
		justify-content: center;
	}

	.marquee.overflow {
		mask: linear-gradient(270deg, transparent 0, black 15px);
	}

	.marquee.overflow.is_animating {
		mask: linear-gradient(
			90deg,
			transparent 0,
			black 30px,
			black calc(100% - 30px),
			transparent 100%
		);
	}

	.content {
		display: grid;
		width: fit-content;
		grid-template-columns: auto 1fr;
		white-space: nowrap;
		letter-spacing: -0.05vw;
		gap: 2em;
	}

	.animating.overflow {
		animation-name: marquee;
		animation-duration: calc(var(--marquee-scroll-width) / 25 * 1.25s);
		animation-timing-function: linear;
		animation-delay: 0s;
		animation-iteration-count: 1;
		animation-fill-mode: forwards;
		will-change: transform;
	}

	@keyframes marquee {
		0% {
			transform: translateX(0);
		}
		100% {
			transform: translateX(calc(var(--marquee-scroll-width) * -1px / 2));
		}
	}
</style>
