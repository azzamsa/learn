<script lang="ts">
  import Footer from "$lib/FooterItem.svelte"
  import Banner from "$lib/BannerItem.svelte"
  import Dashboard from "$lib/DashboardItem.svelte"

  import { Client, setContextClient, cacheExchange, fetchExchange } from "@urql/svelte"

  const client = new Client({
    url: "http://localhost:7000/graphql",
    // CORS doesn't work without this in SvelteKit
    fetchOptions: () => ({
      headers: {
        "Content-Type": "application/json"
      }
    }),
    exchanges: [cacheExchange, fetchExchange]
  })

  setContextClient(client)
</script>

<svelte:head>
  <title>Caesar</title>
</svelte:head>

<section class="bg-main min-h-screen px-2 pb-6 pt-12 md:px-5 md:pt-20">
  <header class="mx-auto max-w-lg">
    <h1 class="text-center text-5xl font-bold text-white">Caesar</h1>
  </header>

  <main class="mx-auto my-10 max-w-5xl rounded-lg bg-white p-8 shadow-2xl md:p-12">
    <Banner />
    <Dashboard />
  </main>

  <Footer />
</section>

<style>
  .bg-main {
    background-color: #e4af79;
    background-image: linear-gradient(315deg, #e4af79 0%, #df9c41 74%);
  }
</style>
