<template>
  <div class="home">
    <div class="topbar">
      <button class="create-launcher-button" @click="createLauncher">创建启动器</button>
    </div>
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
  data() {
    return {
      launchers: Array.from({ length: 10 }, (_, i) => i + 1), // 初始化 10 个启动器
    };
  },
  methods: {
    createLauncher() {
      // 创建一个新启动器，ID 为数组长度 + 1
      const newId = this.launchers.length + 1;
      this.launchers.push(newId);
    },
  },
};
</script>

<style scoped>
.home {
  width: 770px;
  height: 584px;
  padding: 0px;
  box-sizing: border-box;
}

/* 容器样式 */
.launcher-container {
  display: flex;
  flex-direction: row;
  /* 水平排列 */
  gap: 30px;
  /* 每个 Launcher 之间的间距 */
  overflow-x: auto;
  /* 开启水平滚动 */
  padding: 10px 0px 10px 10px;
  scrollbar-width: thin;
  /* 调整滚动条宽度 */
}

.launcher-container::-webkit-scrollbar {
  height: 8px;
  /* 滚动条高度 */
}

.launcher-container::-webkit-scrollbar-thumb {
  background-color: #007bff;
  /* 滚动条颜色 */
  border-radius: 4px;
  /* 滚动条圆角 */
}

/* 确保每个 launcher 的宽度固定 */
.launcher-container>* {
  flex: 0 0 300px;
  /* 宽度固定为 300px，不随容器调整 */
  height: 500px;
  /* 高度固定为 500px，和原始组件一致 */
}

/* 顶部栏样式 */
.topbar {
  width: 100%;
  height: 50px;
  /* 顶部栏高度 */
  background-color: #f8f9fa;
  /* 浅灰背景色 */
  border-bottom: 1px solid #ddd;
  /* 分隔线 */
  display: flex;
  align-items: center;
  justify-content: start;
  /* 左对齐按钮 */
  padding: 0 10px;
  /* 内边距 */
  box-sizing: border-box;
}

.create-launcher-button {
  background-color: #007bff;
  /* 按钮背景色 */
  color: white;
  /* 按钮文字颜色 */
  border: none;
  border-radius: 4px;
  padding: 10px 15px;
  cursor: pointer;
  font-size: 16px;
  font-weight: bold;
  transition: background-color 0.3s ease;
}

.create-launcher-button:hover {
  background-color: #0056b3;
  /* 鼠标悬停背景色 */
}

.create-launcher-button:active {
  background-color: #003d80;
  /* 鼠标按下背景色 */
}
</style>