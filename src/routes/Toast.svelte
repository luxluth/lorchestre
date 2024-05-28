<script lang="ts">
	import { LoaderCircle, CircleX } from 'lucide-svelte';
	import { ToastKind, type ToastEvent } from '$lib/type';
	import { toast } from '$lib/events';

	const DEFAULT_TIMEOUT = 3 * 1000;

	let active = $state(false);

	let title = $state('');
	let message = $state('');

	let kind = $state<ToastKind>(ToastKind.Message);

	async function handleShow(e: CustomEvent<ToastEvent>) {
		let newData = e.detail;
		active = true;
		kind = newData.kind;
		if (newData.kind !== ToastKind.Close) {
			title = newData.title;
			message = newData.message;
			if (newData.kind !== ToastKind.Loading) {
				if (newData.timeout > 0) {
					setTimeout(() => {
						active = false;
					}, newData.timeout);
				} else {
					setTimeout(() => {
						active = false;
					}, DEFAULT_TIMEOUT);
				}
			}
		}
	}

	function handleClose() {
		active = false;
		title = '';
		message = '';
	}

	$effect(() => {
		console.log('toast mounted');
		//@ts-ignore
		document.addEventListener(toast.SHOW_EV, handleShow);
		document.addEventListener(toast.CLOSE_EV, handleClose);

		return () => {
			//@ts-ignore
			document.removeEventListener(toast.SHOW_EV, handleShow);
			document.removeEventListener(toast.CLOSE_EV, handleClose);
		};
	});
</script>

<div class="__toast" class:active>
	<div class="content">
		<h5>{title}</h5>
		<div class="msg">
			{#if kind === ToastKind.Loading}
				<div class="icon loading">
					<LoaderCircle size={'1em'} />
				</div>
			{/if}
			{#if kind === ToastKind.Error}
				<div class="icon error">
					<CircleX size={'1em'} />
				</div>
			{/if}
			<p>{message}</p>
		</div>
	</div>
	<!-- <div class="close"> -->
	<!-- 	<button -->
	<!-- 		onclick={() => { -->
	<!-- 			active = false; -->
	<!-- 		}} -->
	<!-- 	> -->
	<!-- 		<X /> -->
	<!-- 	</button> -->
	<!-- </div> -->
</div>

<style>
	.__toast {
		background: var(--bg);
		border: 1px solid #333;
		min-height: 2.5em;
		max-width: 40em;
		min-width: 20em;
		border-radius: 4px;
		position: fixed;
		bottom: 2em;
		right: 1.5em;
		display: flex;
		justify-content: center;
		align-items: center;
		padding: 0.5em;
		gap: 0.5em;
		transform: translateY(200%);
		transition: all 0.2s ease-in-out;
	}

	.__toast.active {
		transform: translateY(0);
	}

	.__toast .content {
		flex-grow: 1;
	}

	.__toast .content .msg {
		font-size: 0.9em;
		display: flex;
		gap: 0.5em;
		width: 100%;
	}

	.__toast .content .msg .icon {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.__toast .content .msg .icon.loading :global(svg) {
		animation: rotate 1s linear infinite;
	}

	.__toast .content .msg .icon.error {
		color: red;
	}

	/* .__toast .close { */
	/* 	display: flex; */
	/* 	flex-direction: column; */
	/* 	align-items: center; */
	/* 	justify-content: flex-start; */
	/* } */
	/**/
	/* .__toast .close button { */
	/* 	background: none; */
	/* 	border: none; */
	/* 	color: var(--fg); */
	/* 	opacity: 0.5; */
	/* 	transition: all 0.1s ease-in-out; */
	/* 	cursor: pointer; */
	/* } */
	/**/
	/* .__toast .close button:hover { */
	/* 	opacity: 1; */
	/* } */
	/**/
	/* .__toast .close button:active { */
	/* 	transform: scale(0.95); */
	/* } */

	@keyframes rotate {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}
</style>
