<template>
    <div class="home">
      <div class="launcher-container">
        <launcher v-for="index in 10" :key="index" :id="index" />
      </div>
    </div>
  </template>
  
  <script>
  import Launcher from './Launcher.vue'; // 导入 Launcher 组件
  
  export default {
    components: {
      Launcher,
    },
  };
  </script>
  
  <style scoped>
  .home {
    width: 770px;
    height: 584px;
    padding: 10px;
    box-sizing: border-box;
  }
  
  /* 容器样式 */
  .launcher-container {
    display: flex;
    flex-direction: row; /* 水平排列 */
    gap: 30px; /* 每个 Launcher 之间的间距 */
    overflow-x: auto; /* 开启水平滚动 */
    padding: 10px 0px 10px 10px;
    scrollbar-width: thin; /* 调整滚动条宽度 */
  }
  
  .launcher-container::-webkit-scrollbar {
    height: 8px; /* 滚动条高度 */
  }
  
  .launcher-container::-webkit-scrollbar-thumb {
    background-color: #007bff; /* 滚动条颜色 */
    border-radius: 4px; /* 滚动条圆角 */
  }
  
  /* 确保每个 launcher 的宽度固定 */
  .launcher-container > * {
    flex: 0 0 300px; /* 宽度固定为 300px，不随容器调整 */
    height: 500px; /* 高度固定为 500px，和原始组件一致 */
  }
  
  </style>
  