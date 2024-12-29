<script lang="ts">
	import creds from '../../CREDITS?raw';
	import { _ } from 'svelte-i18n';

	type Credit = {
		dep: string;
		url: string;
	};

	let cx: Credit[] = creds
		.trim()
		.split('\n')
		.map((line) => {
			const [dep, url] = line.split(' ### ', 2);
			console.log(dep, url);
			return { dep: dep.trim(), url: url.trim() };
		});
</script>

<details>
	<summary>{$_('settings_page.credits')}</summary>
	<div class="pre">
		{#each cx as credit}
			<div class="credit">
				<div class="dep">{credit.dep}</div>
				<a target="_blank" href={credit.url}>{credit.url}</a>
			</div>
		{/each}
	</div>
</details>

<style>
	details {
		user-select: all;
	}

	a {
		color: var(--fg);
		opacity: 0.3;
	}

	a:hover,
	a:focus {
		opacity: 1;
	}

	summary {
		font-family: var(--font-mono);
		margin-top: 2em;
		padding: 1em;
		width: fit-content;
	}

	.dep {
		padding-right: 1;
		display: inline;
	}

	.credit {
		margin-bottom: 0.5em;
	}

	.pre {
		font-family: var(--font-mono);
		font-weight: 400;
		background-color: var(--highlight);
		border-radius: 8px;
		padding: 1em;
	}
</style>
