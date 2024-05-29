<script lang="ts">
	import { convertFileSrc, invoke } from '@tauri-apps/api/core';
	import { type Album, type Media } from '$lib/type';
	// import { getContext, hasContext, setContext } from 'svelte';
	// import createMediaState from '$lib/media.svelte';

	type Albums = { albums: Album[] };

	async function getAlbums(): Promise<Albums> {
		const nctx = await invoke<Media>('index');
		return { albums: nctx.albums };
	}
</script>

<h1 class="__page_title ns">Albums</h1>

<div class="__medias">
	{#await getAlbums()}
		<div class="shell"></div>
	{:then media}
		{#each media.albums as { tracks, artist, name, id }}
			<a class="__audio" data-id={id} href="album/{id}">
				{#if tracks[0].cover}
					<div
						class="cover"
						style="--clr: {tracks[0].color
							? `rgb(${tracks[0].color.r}, ${tracks[0].color.g}, ${tracks[0].color.b})`
							: 'rgb(255, 255, 255)'}; background-image: url('{convertFileSrc(tracks[0].cover)}');"
					></div>
				{/if}
				<p class="title ns">{name}</p>
				<p class="artist ns">{artist}</p>
			</a>
		{/each}
	{/await}
</div>

<style>
	.__page_title {
		font-weight: 800;
		font-size: 1.5em;
		padding-bottom: 2em;
		text-align: center;
	}

	.__medias {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(15em, 1fr));
		column-gap: 1em;
		row-gap: 3em;
	}

	.__audio {
		width: 15em;
		text-decoration: none;
		color: var(--fg);
		cursor: pointer;
	}

	.__audio .cover {
		width: 100%;
		position: relative;
	}

	.__audio:active {
		transform: scale(0.95);
	}

	/* .__audio .cover:hover button { */
	/* 	opacity: 1; */
	/* } */
	/**/
	/* .__audio .cover button { */
	/* 	position: absolute; */
	/* 	bottom: 50%; */
	/* 	left: 50%; */
	/* 	transform: translate(-50%, 50%); */
	/* 	background-color: var(--fg); */
	/* 	border: none; */
	/* 	display: flex; */
	/* 	align-items: center; */
	/* 	justify-content: center; */
	/* 	padding: 2em; */
	/* 	border-radius: 50%; */
	/* 	opacity: 0; */
	/* 	transition: all 0.15s ease-in-out; */
	/* 	cursor: pointer; */
	/* } */
	/**/
	/* .__audio .cover button:active { */
	/* 	transform: translate(-50%, 50%) scale(0.9); */
	/* 	opacity: 0.5; */
	/* } */

	.__audio .cover {
		aspect-ratio: 1/1;
		background-color: var(--clr);
		border-radius: 10px;
		margin-bottom: 0.3em;
		background-size: cover;
	}

	.__audio .artist {
		opacity: 0.5;
		padding-top: 0.2em;
	}

	.__audio .title {
		font-weight: bold;
	}
</style>
