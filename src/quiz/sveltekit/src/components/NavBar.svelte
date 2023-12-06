<script lang="ts">
    import { AppBar, getDrawerStore } from "@skeletonlabs/skeleton"

    import * as m from "$paraglide/messages"

    import Hamburger from "$root/components/icons/Hamburger.svelte"
    import AccountsIcon from "virtual:icons/material-symbols/manage-accounts"

    // Drawer
    const drawerStore = getDrawerStore()
    function toggleDrawer() {
        if ($drawerStore.open === true) {
            drawerStore.close()
        } else {
            drawerStore.open()
        }
    }
</script>

<AppBar
    class="font-serif"
    gridColumns="grid-cols-3"
    slotDefault="place-self-center"
    slotTrail="place-content-end"
>
    <svelte:fragment slot="lead">
        <a href="/" class="flex gap-2 items-center">
            <img src="logo.png" alt="logo" class="w-9" />
            <span class="hidden text-xl font-bold sm:block">Qwiz</span>
        </a>
    </svelte:fragment>

    <nav class="hidden gap-8 font-bold sm:flex">
        <a href="/" class="text-xl btn hover:variant-filled-primary">{m.home()}</a>
        <a href="/about" class="text-xl btn hover:variant-filled-primary">{m.about()}</a>
    </nav>

    <svelte:fragment slot="trail">
        <!-- big screen  -->
        <div class="hidden items-center space-x-2 sm:inline-flex">
            <a href="/settings" class="btn rounded-full btn-icon hover:variant-filled-primary">
                <AccountsIcon class="w-7 h-7" />
            </a>
        </div>
        <!-- small screen  -->
        <div class="sm:hidden">
            <button on:click={toggleDrawer} class="btn btn-sm">
                <div class="w-8 h-8">
                    <Hamburger />
                </div>
            </button>
        </div>
        <!-- small screen  -->
    </svelte:fragment>
</AppBar>
