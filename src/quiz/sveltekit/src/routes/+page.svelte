<script lang="ts">
  import { RadioGroup, RadioItem } from "@skeletonlabs/skeleton"
  import { LightSwitch } from "@skeletonlabs/skeleton"

  let value: number = 0
</script>

<section class="px-2 pt-12 pb-6 min-h-screen md:px-5 md:pt-20">
  <main class="p-8 my-10 mx-auto max-w-5xl rounded-lg shadow-2xl md:p-12">
    <LightSwitch class="mt-10" />

    <h1 class="mt-10 h1">Hello world!</h1>
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
