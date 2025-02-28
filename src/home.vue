<template>
  <div class="home">
    <div class="topbar">
      <button class="create-launcher-button" @click="createLauncher">创建启动器</button>
      <div class="topbar-button">
        <div class="edit-mode-container">
          <label class="checkbox-label">
            <input type="checkbox" v-model="editMode" @change="toggleEditMode" />
            编辑模式
          </label>
        </div>
        <div class="auto-start-container">
          <label class="checkbox-label">
            <input type="checkbox" v-model="autoLaunch" @change="toggleAutoLaunch" />
            开机启动
          </label>
        </div>
      </div>
    </div>
    <div class="launcher-container" v-if="editMode">
      <launcher v-for="(item, index) in launchers" :key="index" :launcherData="item"
        @launcher-updated="refreshLaunchers" @launcher-moved="moveLauncher" />
    </div>
    <div class="launcher-lite-container" v-if="!editMode">
      <launcher-lite v-for="(item, index) in launchers" :key="index" :launcherData="item"
        class="launcher-lite-container-item" @launcher-updated="refreshLaunchers" @launcher-moved="moveLauncher" />
    </div>
  </div>
</template>

<script>
import Launcher from './Launcher.vue';
import LauncherLite from './LauncherLite.vue';
import { invoke } from "@tauri-apps/api/core";
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';
import { useToast } from "vue-toastification";
import { listen } from '@tauri-apps/api/event';

const toast = useToast()

export default {
  components: {
    Launcher,
    LauncherLite
  },
  data() {
    return {
      launchers: [], // 用于存储从后端获取的启动器列表
      // 开机启动状态
      autoLaunch: false,
      // 开机启动锁
      toggleLock: false,
      editMode: true
    };
  },
  methods: {
    async setupEventListener() {
      listen('launch', async (event) => {
        console.log("收到消息: ",event);
        await this.launch(event.payload);
      });
      listen('launcher_basic_info_updated', async (event) => {
        await this.reflush_tray();
      });
    },
    async launch(launcherId) {
      await invoke("launch", { launcherId: launcherId });
    },
    async createLauncher() {
      await invoke("craete_launcher");
      this.editMode = true;
      this.refreshLaunchers();
    },
    async refreshLaunchers() {
      const data = await invoke("query_launchers"); // 调用 Tauri 后端命令
      this.launchers = []; // 先清空数组
      this.$nextTick(() => {
        this.launchers = [...data]; // 再赋值
      });
    },
    // type 0->左移,1-右移
    async moveLauncher(launcherId, type) {
      // 找到目标元素的索引
      const ids = this.launchers.map(launcher => launcher.id);
      const index = ids.findIndex(id => id === launcherId);

      // 如果未找到目标元素，直接返回原数组副本
      if (index === -1) {
        return;
      }

      if (type === 0 && index > 0) {
        // 向左移动（交换与左边的元素）
        [ids[index], ids[index - 1]] = [ids[index - 1], ids[index]];
      } else if (type === 1 && index < ids.length - 1) {
        // 向右移动（交换与右边的元素）
        [ids[index], ids[index + 1]] = [ids[index + 1], ids[index]];
      }

      const sortList = ids.map((id, index) => ({
        id: id,
        sort: index
      }));

      await invoke("modify_launcher_sort", { launchers: sortList });

      this.refreshLaunchers();
    },
    // 切换开机启动状态
    async toggleAutoLaunch() {
      if (this.toggleLock) {
        return; // 如果已有任务在执行，直接返回
      }
      this.toggleLock = true;
      try {
        if (await isEnabled()) {
          await disable();
        } else {
          await enable();
        }
        // 更新当前状态
        this.autoLaunch = await isEnabled();
      } catch (error) {
        console.error("Failed to toggle auto-launch:", error);
        toast.error("调整开机启动失败！");
      } finally {
        this.toggleLock = false; // 释放锁
      }
    },
    async toggleEditMode() {
      await invoke("save_setting", { key: "editMode", value: this.editMode ? "true" : "false" })
      this.fetchEditModeStatus();
    },
    // 获取当前编辑模式设置
    async fetchEditModeStatus() {
      const em = await invoke("read_setting", { key: "editMode" })
      this.editMode = em == null || em.value === "true";
    },
    // 获取当前开机启动状态
    async fetchAutoLaunchStatus() {
      try {
        this.autoLaunch = await isEnabled();
      } catch (error) {
        console.error("Failed to fetch auto launch status:", error);
      }
    },
    async reflush_tray() {
      await invoke("reflush_tray");
    }
  },
  mounted() {
    this.reflush_tray();
    // 初始化时获取开机启动状态
    this.fetchAutoLaunchStatus();
    this.refreshLaunchers(); // 页面加载时刷新 Launcher 列表
    this.fetchEditModeStatus();
    this.setupEventListener();
  },
};
</script>

<style scoped>
.home {
  width: 100%;
  height: 100%;
  padding: 0px;
  box-sizing: border-box;
  display: flex;
  flex-flow: column nowrap;
  overflow: hidden;
}

/* 容器样式 */
.launcher-container {
  display: flex;
  flex-direction: row;
  /* 水平排列 */
  gap: 10px;
  /* 每个 Launcher 之间的间距 */
  overflow-x: auto;
  /* 开启水平滚动 */
  padding: 10px 10px 10px 10px;
  scrollbar-width: auto;
  /* 调整滚动条宽度 */
  /* flex:1; */
  /* height: clac(100vh -50px); */
}

.launcher-container::-webkit-scrollbar {
  height: 10px;
  /* 滚动条高度 */
}

.launcher-container::-webkit-scrollbar-thumb {
  background-color: #8B8B8B;
  /* 滚动条颜色 */
  border-radius: 5px;
  /* 滚动条圆角 */
}

/* 确保每个 launcher 的宽度固定 */
.launcher-container>* {
  flex: 0 0 300px;
  width: 0;
  /* 宽度固定为 300px，不随容器调整 */
  height: 500px;
  /* 高度固定为 500px，和原始组件一致 */
  padding: 10px 10px 10px 10px;
}

/* 确保每个 launcher 的宽度固定 */
.launcher-lite-container {
  margin-top: 10px;
  width: 100%;
  height: 100%;
  display: flex;
  flex-wrap: wrap;
  justify-content: flex-start;
  /* 水平排列并均匀分布 */
  align-content: flex-start;
  /* 控制多行间的对齐方式 */
  gap: 10px;
  overflow-y: auto;
}

.launcher-lite-container-item {
  aspect-ratio: 2 / 1;
  flex: calc(25%);
  /* 宽度为容器的四分之一，减去间距 */
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
  /* justify-content: start; */
  justify-content: space-between;
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

.auto-start-container,
.edit-mode-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
  font-family: Arial, sans-serif;
}

.checkbox-label {
  display: flex;
  align-items: center;
  font-size: 16px;
  cursor: pointer;
}

input[type="checkbox"] {
  margin-right: 10px;
  transform: scale(1.2);
  /* 放大复选框大小 */
  cursor: pointer;
}

.topbar-button {
  display: flex;
  gap: 10px;
}
</style>