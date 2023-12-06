<script></script>

<svelte:head>
    <title>Qwiz - About</title>
</svelte:head>

<div class="flex flex-col justify-center mt-4">
    <!-- Face -->
    <div class="flex justify-center">
        <a href="https://github.com/azzamsa/" class="">
            <img
                class="w-44 transition duration-500 ease-in-out delay-150 hover:scale-125"
                src="face.png"
                alt="Azzam S.A face"
            />
        </a>
    </div>
    <!-- body -->
    <div class="p-6 mx-auto space-y-4 max-w-xl text-lg">
        <p>Hi! 👋</p>
        <p>
            Thanks for using my application. If you enjoy it, feel free to start the <a
                href="https://github.com/azzamsa/learn/">repository</a
            >
        </p>
        <p>
            Should you encounter any issues or have feature requests, please don't hesitate to
            <a href="https://github.com/azzamsa/learn/issues">reach out</a>.
        </p>
        <p>Crafted with 💖 by <a href="https://github.com/azzamsa/">Azzam S.A</a></p>
    </div>
</div>

<style lang="postcss">
    a {
        @apply underline;
    }
</style>
