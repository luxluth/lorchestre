<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { fade } from 'svelte/transition';
	import { _ } from 'svelte-i18n';
	import banner from '$lib/assets/banner-bw.svg?raw';
	import LoaderCircle from 'lucide-svelte/icons/loader-circle';
	import { getAppConfig } from '$lib/config.svelte';
	import { onMount } from 'svelte';

	let { first_run = $bindable() }: { first_run: boolean } = $props();
	const quotes = [
		{
			author: 'Nat King Cole, "Nature Boy"',
			parole: "The greatest thing you'll ever learn is just to love and be loved in return."
		},
		{
			author: 'Kendrick Lamar, "Not like Us"',
			parole: 'Psst, I see dead people'
		},
		{
			author: 'Queen, "Somebody to Love"',
			parole: 'Can anybody find me somebody to love ?'
		}
	];
	let act = $state(1);
	let synched = $state(false);
	let qidx = $state(Math.floor(Math.random() * quotes.length));
	let synchedmsg = $derived(quotes[qidx]);
	let config = getAppConfig();
	const endpoint = config.getDaemonEndpoint();
	let pingIntervalId = $state(-1);

	async function start() {
		pingIntervalId = window.setInterval(() => {
			(async () => {
				let response = null;
				try {
					response = await fetch(`http://${endpoint}/`);
					synched = true;
					clearInterval(pingIntervalId);
				} catch (e) {}
			})();
		}, 2000);
	}

	onMount(() => {});
</script>

<div class="message">
	<div class="icon ns">
		{@html banner}
	</div>

	{#if act === 1}
		<div class="act">
			<h1 class="ns">{$_('app_welcome')}</h1>
			<div class="ns">{$_('welcome_msg')}</div>
		</div>
	{/if}

	{#if act === 2}
		<div class="act" transition:fade={{ duration: 500 }}>
			{#if !synched}
				<h1 class="ns">{$_('indexing_msg')}</h1>
				<div class="notice">
					<div class="icon load">
						<LoaderCircle size={'1em'} />
					</div>
					<p>{$_('notice_msg')}</p>
				</div>
				<div class="ns">
					<p>{synchedmsg.parole} - <b>{synchedmsg.author}</b></p>
				</div>
			{/if}
		</div>
	{/if}
</div>

{#if act === 1}
	<button
		onclick={async () => {
			act = 2;
			await invoke('start_service');
			await start();
		}}
		class="act_btn btn animate">{$_('start_btn')}</button
	>
{/if}
{#if act === 2}
	<button
		class="act_btn btn"
		onclick={async () => {
			await invoke('runned');
			first_run = false;
		}}
		class:inactive={!synched}>{$_('end_button')}</button
	>
{/if}

<style>
	* {
		color: white;
	}

	h1 + div {
		display: flex;
		gap: 0.1em;
	}

	.icon {
		color: white;
	}

	.icon.load {
		animation: rotate 500ms infinite ease-in-out;
	}

	.load {
		width: fit-content;
		height: fit-content;
		display: flex;
		justify-content: center;
		align-items: center;
	}

	@keyframes rotate {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}

	.act_btn {
		background: white;
		position: absolute;
		bottom: 2em;
		right: 2em;
		color: var(--brand-color);
		animation-delay: 0.5s;
	}

	.act_btn.animate {
		animation: fade-in 0.3s ease-in;
	}

	@keyframes fade-in {
		from {
			opacity: 0;
		}

		to {
			opacity: 1;
		}
	}

	.message {
		opacity: 0;
		animation: fade-in 0.3s ease-in forwards;
		animation-delay: 0.1s;
	}
</style>
