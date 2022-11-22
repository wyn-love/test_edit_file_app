<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Dialog from "./components/Dialog.vue";
</script>

<template>
  <div class="container">
  <main>
    <h1 class="py-5 text-center">Welcome to Tauri!</h1>
    <em id="code_url">https://github.com/wyn-love/test_edit_file_app</em>
    <h2>Used to batch operate files</h2>
    

    <p id="tip">Tip:目前该功能尚不完善,请在指定路径时注意不要有空格</p>
    <Dialog />
  </main>
  </div>
</template>

<style lang="less" scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

#tip {
  color: rgb(255, 1, 1);
  font-size: small;
  font-weight: bold;
}
</style>
