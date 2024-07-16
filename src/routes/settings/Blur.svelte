<script lang="ts">
	import type AppConfig from '$lib/config.svelte';
	import { Checkbox, Label } from 'bits-ui';
	import Check from 'lucide-svelte/icons/check';
	import { _ } from 'svelte-i18n';

	let { appConf }: { appConf: AppConfig } = $props();
</script>

<section class="blur-setter">
	<div class="blur-setting">
		<Checkbox.Root
			id="blur"
			aria-labelledby="setting-desc"
			class="checkbox"
			onCheckedChange={(e) => {
				if (e != 'indeterminate') {
					(async () => {
						await appConf.setBlurTo(e);
					})();
				}
			}}
			checked={appConf.config.global?.enable_blur ?? appConf.defaults.global.enable_blur}
		>
			<Checkbox.Indicator let:isChecked>
				{#if isChecked}
					<Check size={'15px'} />
				{/if}
			</Checkbox.Indicator>
		</Checkbox.Root>

		<Label.Root id="setting-desc" for="blur" class="label">
			{$_('settings_page.section_ui_ux.blur')}
		</Label.Root>
	</div>
</section>

<style>
	section {
		padding-top: 1.5em;
	}

	.blur-setting {
		display: flex;
		align-items: center;
		gap: 0.5em;
	}

	:global([data-checkbox-indicator]) {
		display: flex;
		justify-content: center;
		align-items: center;
	}

	:global([data-checkbox-root]) {
		width: 1.7em;
		height: 1.7em;
		border: 2px solid var(--brand-color);
		border-radius: 6px;
		display: flex;
		justify-content: center;
		align-items: center;
		background: none;
		color: var(--bg);
	}

	:global(.checkbox[aria-checked='true']) {
		background-color: var(--brand-color);
	}
</style>
