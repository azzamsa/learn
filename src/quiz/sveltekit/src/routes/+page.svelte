<script lang="ts">
	import { RadioGroup, RadioItem } from '@skeletonlabs/skeleton';
	import { LightSwitch } from '@skeletonlabs/skeleton';

	let value: number = 0;
</script>

<section class="min-h-screen px-2 pb-6 pt-12 md:px-5 md:pt-20">
	<main class="mx-auto my-10 max-w-5xl rounded-lg p-8 shadow-2xl md:p-12">
		<LightSwitch class="mt-10" />

		<h1 class="h1 mt-10">Hello world!</h1>
		<p class="text-primary-500">Lorem ipsum dolor sit amet consectetur adipisicing elit.</p>

		<section>
			<RadioGroup
				active="variant-filled-secondary"
				hover="hover:variant-soft-primary hover:text-primary"
			>
				<RadioItem bind:group={value} name="justify" value={0}>One</RadioItem>
				<RadioItem bind:group={value} name="justify" value={1}>Two</RadioItem>
				<RadioItem bind:group={value} name="justify" value={2}>Three</RadioItem>
			</RadioGroup>
		</section>
	</main>
</section>
