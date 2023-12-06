<script lang="ts">
    import {
        Drawer,
        autoModeWatcher,
        getDrawerStore,
        initializeStores
    } from "@skeletonlabs/skeleton"
    import "../app.postcss"
    import { setInitialClassState } from "@skeletonlabs/skeleton"

    import * as m from "$paraglide/messages"
    import { setLanguageTag } from "$paraglide/runtime"
    import { locale } from "$stores/settings"

    import NavBar from "$components/NavBar.svelte"

    // Drawer
    initializeStores()
    const drawerStore = getDrawerStore()

    // Locale
    $: {
        setLanguageTag($locale as (() => "en" | "id") | "en" | "id")
    }

    let menuItems = [
        { label: m.home(), href: "/" },
        { label: m.settings(), href: "/settings" },
        { label: m.about(), href: "/about" }
    ]
</script>

<svelte:head>
    {@html `<script>${autoModeWatcher.toString()} autoModeWatcher();</script>`}
    {@html `<script>(${setInitialClassState.toString()})();</script>`}
</svelte:head>

{#key $locale}
    <!-- Drawer -->
    <Drawer class="w-3/4 h-1/2 sm:hidden">
        <div class="p-4">
            <!-- Menu -->
            <h2 class="font-serif h2">Menu</h2>

            <ul class="p-2 font-serif text-xl list">
                {#each menuItems as { label, href }}
                    <li class="text-lg">
                        <a
                            {href}
                            on:click={drawerStore.close}
                            class="text-lg btn btn-sm w-full hover:variant-filled-primary"
                            >{label}</a
                        >
                    </li>
                {/each}
            </ul>
        </div>
    </Drawer>

    <!-- App -->
    <div>
        <NavBar />
        <slot />
    </div>
{/key}
