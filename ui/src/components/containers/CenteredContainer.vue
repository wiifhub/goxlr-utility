<template>
  <div class="centered">
    <slot></slot>
  </div>
</template>

<script>
export default {
  name: "CenteredContainer"
}
</script>

<style scoped>
  .centered {
    margin: auto;
    width: max-content;
    min-width: min-content;

    display: flex;
    flex-direction: row;
    gap: 15px;
  }

  @media (max-width: 900px) {
    .centered {
      margin: 0;
    }
  }
</style>
