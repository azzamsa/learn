<script lang="ts">
    import { AppRail, AppRailTile } from "@skeletonlabs/skeleton"
    import { setModeCurrent } from "@skeletonlabs/skeleton"

    import { difficulty, numberOfQuestions, locale } from "$stores/settings"
    import QuizIcon from "virtual:icons/fluent/quiz-new-48-regular"
    import LabsIcon from "virtual:icons/fluent/beaker-settings-20-regular"

    import * as m from "$paraglide/messages"

    function updateDifficulty(value: string) {
        $difficulty = value
    }
    function updateNumberOfQuestions(value: string) {
        $numberOfQuestions = value
    }
    function updateLocale(value: string) {
        $locale = value
    }

    let themeKey = true // light
    let theme = "Light" // light
    function updateTheme() {
        if (themeKey == true) {
            setModeCurrent(false)
            themeKey = false
            theme = "Dark"
        } else {
            themeKey = true
            theme = "Light"
            setModeCurrent(true)
        }
    }

    let currentTile: number = 0
</script>

<svelte:head>
    <title>Qwiz - Settings</title>
</svelte:head>

<section class="flex justify-center items-center py-20 px-4 space-y-8 font-sans sm:px-20">
    <!-- Body -->
    <div class="flex h-80 rounded-xl card bg-surface-50-900-token w-full sm:w-3/4 max-w-2xl">
        <!-- App Rails -->
        <div class="">
            <AppRail class="rounded-l-xl">
                <AppRailTile bind:group={currentTile} name="tile-1" value={0} title="tile-1">
                    <svelte:fragment slot="lead">
                        <div class="flex justify-center py-1">
                            <QuizIcon class="w-8 h-8" />
                        </div>
                    </svelte:fragment>
                    <span>Quiz</span>
                </AppRailTile>
                <AppRailTile bind:group={currentTile} name="tile-2" value={1} title="tile-2">
                    <svelte:fragment slot="lead">
                        <div class="flex justify-center py-1">
                            <LabsIcon class="w-8 h-8" />
                        </div>
                    </svelte:fragment>
                    <span>App</span>
                </AppRailTile>
            </AppRail>
        </div>
        <!-- Content -->
        <div class="m-auto">
            <!-- Quiz -->
            {#if currentTile === 0}
                <div class="flex flex-col py-8 space-y-3">
                    <!-- Difficulty Levels -->
                    <p>Difficulty Levels</p>
                    <select
                        bind:value={$difficulty}
                        on:change={(e) => updateDifficulty(e.currentTarget.value)}
                        class="text-center select"
                    >
                        <option value="easy">Easy</option>
                        <option value="medium">Medium</option>
                        <option value="hard">Hard</option>
                    </select>
                    <!-- Number of Questions -->
                    <p>Number of Questions</p>
                    <select
                        bind:value={$numberOfQuestions}
                        on:change={(e) => updateNumberOfQuestions(e.currentTarget.value)}
                        class="text-center select"
                    >
                        <option value="10">10</option>
                        <option value="20">20</option>
                        <option value="30">30</option>
                    </select>
                </div>
            {/if}
            {#if currentTile === 1}
                <div class="flex flex-col py-8 space-y-3">
                    <!-- Locale -->
                    <p>{m.language()}</p>
                    <select
                        bind:value={$locale}
                        on:change={(e) => updateLocale(e.currentTarget.value)}
                        class="text-center select"
                    >
                        <option value="id">Indonesia</option>
                        <option value="en">English</option>
                    </select>
                    <!-- Theme -->
                    <p>{m.theme()}</p>
                    <select bind:value={theme} on:change={updateTheme} class="text-center select">
                        <option>Dark</option>
                        <option>Light</option>
                    </select>
                </div>
            {/if}
        </div>
    </div>
</section>
