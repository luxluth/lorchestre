<script lang="ts">
	import type Manager from '$lib/manager.svelte';
	import type MediaState from '$lib/media.svelte';
	import type { Track } from '$lib/type';
	import { tweened } from 'svelte/motion';
	import { cubicOut, quartInOut } from 'svelte/easing';

	import { formatTime, getCoverUri, setTitle } from '$lib/utils';
	import { getContext } from 'svelte';
	import { _ } from 'svelte-i18n';
	import Play from 'lucide-svelte/icons/play';
	import type AppConfig from '$lib/config.svelte';

	let manager = getContext<Manager>('manager');
	let media = getContext<MediaState>('media');
	let config = getContext<AppConfig>('appconf');
	let songLenght = tweened(0, {
		duration: 700,
		easing: quartInOut
	});

	$effect(() => {
		$songLenght = media.getSongs().length;
	});

	$effect(() => {
		getLatestSong();
	});

	function sortTracksByDate(tracks: Track[]): Track[] {
		return tracks.slice().sort((a, b) => {
			const dateA = a.created_at.secs_since_epoch * 1e9 + a.created_at.nanos_since_epoch;
			const dateB = b.created_at.secs_since_epoch * 1e9 + b.created_at.nanos_since_epoch;
			return dateB - dateA;
		});
	}

	let song = $state<Track>();

	function getLatestSong() {
		song = sortTracksByDate(media.getSongs())[0];
	}

	$effect(() => {
		setTitle("L'orchestre");
	});
</script>

<p class="ns">{$_('stats_page.total')}</p>
<h1 class="song_count ns">
	{$songLenght.toFixed(0)}
	<span>{$songLenght > 1 ? $_('stats_page.songs') : $_('stats_page.song')}</span>
</h1>
<hr />

{#if song}
	<p class="ns">{$_('stats_page.latest_add')}</p>
	<div
		class="recent_added glass"
		style="--r: {song.color?.r ?? 0}; --g: {song.color?.g ?? 0}; --b: {song.color?.b ?? 0};"
	>
		<div
			class="cover"
			style="background-image: url({getCoverUri(song.album_id, song.cover_ext, config)});"
		></div>
		<div class="infos ns">
			<div class="bitrate ns">{song.bitrate - 1}Kb/s</div>
			<div class="title">{song.title}</div>
			<div class="artist">{song.album_artist ?? song.artists[0]}</div>
			<div class="dur">{formatTime(song.duration)}</div>
			<button
				class="play"
				onclick={async () => {
					await manager.play(song as Track);
				}}
			>
				<Play size={'1.5em'} fill={'var(--bg)'} />
			</button>
		</div>
	</div>
{/if}

<style>
	.song_count {
		font-size: clamp(2.5rem, 1.8333rem + 5.3333vw, 7.5rem);
		font-family: var(--font-fantasy);
	}

	hr {
		color: var(--highlight);
		margin-block: 2em;
	}

	.recent_added {
		margin-top: 1em;
		height: 20em;
		width: 100%;
		background-color: rgba(var(--r), var(--g), var(--b), 0.2);
		border: 1px solid var(--highlight);
		border-radius: 10px;
		display: flex;
		align-items: center;
		padding-inline: 1em;
		position: relative;
		gap: 1em;
	}

	.recent_added .cover {
		height: 18em;
		width: 18em;
		aspect-ratio: 1/1;
		background-color: var(--clr);
		border-radius: 8px;
		margin-bottom: 0.3em;
		background-size: cover;

		box-shadow: rgba(17, 12, 46, 0.15) 0px 48px 100px 0px;
	}

	.infos {
		display: flex;
		flex-direction: column;
		align-self: flex-start;
		padding-block: 1em;
		flex-grow: 1;
		height: 100%;
		position: relative;
	}

	.bitrate {
		position: absolute;
		bottom: 1em;
		right: 0;
		color: var(--bg);
		background: var(--fg);
		width: fit-content;
		padding: 0.2em;
		font-weight: bold;
		border-radius: 4px;
	}

	.infos .title {
		font-size: clamp(0.9375rem, 0.7292rem + 1.6667vw, 2.5rem);
		font-weight: bold;
	}

	.infos .artist,
	.infos .dur {
		font-size: clamp(0.9375rem, 0.7292rem + 1.6667vw, 2.5rem);
	}

	.infos .dur {
		opacity: 0.5;
		margin-top: 0.5em;
	}

	.infos .play {
		position: absolute;
		bottom: 1em;
		left: 0;
		padding: 1em;
		background: var(--fg);
		color: var(--bg);
		border: 0px;
		display: flex;
		justify-content: center;
		align-items: center;
		border-radius: 50%;
		cursor: pointer;
	}

	button.play:active {
		transform: scale(0.98);
	}
</style>
