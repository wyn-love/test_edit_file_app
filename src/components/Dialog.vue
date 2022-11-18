<script setup lang="ts">
import { Ref, ref } from "vue";
import { open } from '@tauri-apps/api/dialog';
import { message } from '@tauri-apps/api/dialog';
import { appDataDir } from '@tauri-apps/api/path';
import { invoke } from "@tauri-apps/api/tauri";


const selected:Ref<string | string[] | null> = ref("");
const prefix = ref("")

async function openDir() {
  
  console.log(prefix);

  await open({
    directory: true,
    multiple: true,
    defaultPath: await appDataDir(),
  }).then(val => {
    if (val === null || val.length === 0) {
      message("文件目录出错",{title:"tip",type: "error"});
      return;
    }
    selected.value = val;
    invoke("edit_file", { dir: selected.value[0],prefix: prefix.value });
    message("修改成功",{title:"tip",type: "info"});
  }).catch(_=> {
    message("修改失败",{title: "tip", type: "error"});
    return;
  });
  console.log(selected);
}
</script>

<template>
  <div class="card">

    

    <label for="name">指定前缀:</label>
    <input id="name" v-model="prefix" placeholder="please input prefix" maxlength="20" size="20" />

    <button type="button" @click="openDir()">打开</button>
  </div>

</template>


<style lang="less" scoped>
  .card {
    #name {
      margin: 20px;
    }

    button {
      background-color: rgb(7, 170, 108);
    }
  }

</style>