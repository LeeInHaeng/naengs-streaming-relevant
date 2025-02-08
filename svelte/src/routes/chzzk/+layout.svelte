<script lang="ts">
    import { setToggleMenuEvent } from "$lib/chzzk/layout";
    import { onMount } from "svelte";

    onMount(async() => {
        setToggleMenuEvent();
    })
</script>

<div class="flex">
    <!-- Sidebar -->
    <div class="fixed bg-gray-800 min-h-screen p-5 overflow-y-auto hidden md:block">
        <h2 class="text-white text-2xl mb-6">Naengs Test</h2>
        <nav class="text-white">

        </nav>
    </div>

    <!-- Small Sidebar -->
    <div id="js-smallSideMenu" class="fixed inset-0 bg-gray-800 p-5 overflow-y-auto hidden">
        <button id="js-closeSmallSideMenu" class="absolute top-4 right-4 text-white text-2xl">
            &times;
        </button>
        <div class="p-5">
            <h2 class="text-white text-2xl mb-6">Naengs Test</h2>
            <nav class="text-white">

            </nav>
        </div>
    </div>

    <!-- Right Content -->
    <div class="flex-1 md:ml-52">
        <!-- Top Header Bar -->
        <div class="bg-slate-300 h-16 flex flex-row px-4">
            <!-- svelte-ignore a11y_consider_explicit_label -->
            <button id="js-openSmallSideMenu" class="md:hidden">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7" />
                </svg>
            </button>
            <div class="flex flex-col ml-auto">
            </div>
            <button class="ml-auto">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-10">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M17.982 18.725A7.488 7.488 0 0 0 12 15.75a7.488 7.488 0 0 0-5.982 2.975m11.963 0a9 9 0 1 0-11.963 0m11.963 0A8.966 8.966 0 0 1 12 21a8.966 8.966 0 0 1-5.982-2.275M15 9.75a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
                </svg>
            </button>
        </div>

        <!-- Main Contents -->
        <main class="flex-1 p-5 pb-3 min-h-screen bg-gray-500 text-gray-300 overflow-y-auto"><slot /></main>

    </div>
</div>