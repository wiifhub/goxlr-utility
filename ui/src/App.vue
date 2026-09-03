<template>
  <GoXLR/>
</template>

<script>
import GoXLR from "@/components/GoXLR.vue";

export default {
  name: 'App',
  components: {
    GoXLR,
  },
}
</script>

<style>
:root {
  color-scheme: dark;
}

body {
  background-color: var(--surface-canvas);
  padding: 0;
  margin: 0;
}

@font-face {
  src: url('./assets/LeagueMono-NarrowRegular.ttf');
  font-family:'LeagueMonoCondensed';
  font-style: normal;
  font-stretch: 1% 500%; /* Required by Chrome */
}

@font-face {
  src: url('./assets/LeagueMono.ttf');
  font-family:'LeagueMono';
  font-style: normal;
  font-stretch: 1% 500%; /* Required by Chrome */
}

#app {
  font-family: LeagueMonoCondensed, sans-serif;

  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: var(--text-primary);
  min-height: 100dvh;
  user-select: none;
}

button {
  font-family: LeagueMonoCondensed, sans-serif;
}
</style>
