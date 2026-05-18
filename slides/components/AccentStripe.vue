<template>
  <div class="accent-stripe" />
</template>

<style scoped>
.accent-stripe {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 6px;
  background: linear-gradient(
    to right,
    #6b3fa0 0 33%,
    #3f7a5b 33% 66%,
    #b88a2c 66% 100%
  );
  z-index: 10;
  pointer-events: none;
}
</style>
