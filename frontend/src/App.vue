<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router';
</script>

<template>
    <header>
        <nav>
            <RouterLink to="/">Home</RouterLink>
        </nav>
    </header>

    <main>
        <RouterView />
    </main>
</template>
