<script lang="ts">
	import '../app.postcss';
	import { autoModeWatcher } from '@skeletonlabs/skeleton';
</script>

<svelte:head>
	{@html `<script>${autoModeWatcher.toString()} autoModeWatcher();</script>`}
</svelte:head>

<slot />
