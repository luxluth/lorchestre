<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { fade } from 'svelte/transition';
	import { _ } from 'svelte-i18n';
	import banner from '$lib/assets/banner-bw.svg?raw';
	import LoaderCircle from 'lucide-svelte/icons/loader-circle';
	import { getAppConfig } from '$lib/config.svelte';
	import { onMount } from 'svelte';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';

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

	function trim(text: string, len = 100) {
		return text.slice(0, len) + (text.length > len ? '...' : '');
	}

	async function start() {
		pingIntervalId = window.setInterval(() => {
			(async () => {
				let response = null;
				try {
					response = await fetch(`http://${endpoint}/`);
					if (response.ok) synched = true;
					clearInterval(pingIntervalId);
				} catch (e) {}
			})();
		}, 2000);
	}

	let msg = $state('');

	async function startListening() {
		return await listen<string>('sync', (e) => {
			msg = e.payload;
		});
	}

	onMount(() => {
		let unlisten: UnlistenFn;
		(async () => {
			unlisten = await startListening();
		})();

		return () => {
			unlisten();
		};
	});
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
			await invoke('start_daemon');
			await start();
		}}
		class="act_btn btn animate">{$_('start_btn')}</button
	>
{/if}
{#if act === 2}
	{#if !synched}
		<div class="notice">
			<div class="icon load">
				<LoaderCircle size={'1em'} />
			</div>
			<p>{msg.length > 0 ? trim(msg) : $_('notice_msg')}</p>
		</div>
	{/if}
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
	h1 + div {
		display: flex;
		gap: 0.1em;
	}

	.icon {
		color: white;
	}

	.notice {
		position: absolute;
		bottom: 2em;
		left: 2em;
		display: flex;
		gap: 0.5em;
		font-family: var(--font-mono);
		font-size: 0.8em;
		font-weight: bold;
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

	.act {
		position: relative;
	}

	.act_btn {
		position: absolute;
		bottom: 2em;
		right: 2em;
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
