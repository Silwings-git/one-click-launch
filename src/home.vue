<template>
  <div class="home" style="margin: 0; padding: 0;">
    <div class="topbar">
      <button class="create-launcher-button" @click="createLauncher">创建启动器</button>
      <div class="topbar-button">
        <div class="edit-mode-container">
          <label class="checkbox-label">
            <input type="checkbox" v-model="editMode" @change="toggleEditMode" />
            编辑模式
          </label>
        </div>
        <div class="edit-mode-container">
          <div class="setting" @click="openSetting">设置</div>
        </div>
      </div>
    </div>
    <div class="launcher-container" v-if="editMode">
      <launcher v-for="(item, index) in launchers" :key="index" :launcherData="item"
        @launcher-updated="refreshLaunchers" @launcher-moved="moveLauncher" @settings-updated="refreshLaunchers" />
    </div>
    <div :class="['launcher-lite-container', theme]" v-if="!editMode">
      <launcher-lite v-for="(item, index) in launchers" :key="index" :launcherData="item"
        :class="['launcher-lite-container-item', theme]" @launcher-updated="refreshLaunchers"
        @launcher-moved="moveLauncher" />
    </div>
    <!-- 悬浮框 -->
    <div v-if="showSetting" :class="['modal-overlay', theme]" @click="closeSetting">
      <div :class="['modal-content', theme]" @click.stop>
        <span class="close-btn" @click="closeSetting">&times;</span>
        <settings />
      </div>
    </div>
  </div>
</template>

<script>
import Launcher from './Launcher.vue';
import LauncherLite from './LauncherLite.vue';
import Settings from './Settings.vue';
import { invoke } from "@tauri-apps/api/core";
import { useToast } from "vue-toastification";
import { listen } from '@tauri-apps/api/event';
import { inject } from 'vue';

const toast = useToast()

export default {
  components: {
    Launcher,
    LauncherLite,
    Settings
  },
  data() {
    return {
      // 用于存储从后端获取的启动器列表
      launchers: [],
      editMode: true,
      showSetting: false,
    };
  },
  setup() {
    const theme = inject('theme');
    return {
      theme,
    };
  },
  methods: {
    async setupEventListener() {
      listen('launch', async (event) => {
        console.log("收到消息: ", event);
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
      const data = await invoke("query_launchers"); 
      this.launchers = []; 
      this.$nextTick(() => {
        this.launchers = [...data]; 
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
    async toggleEditMode() {
      await invoke("save_setting", { key: "editMode", value: this.editMode ? "true" : "false" })
      this.fetchEditModeStatus();
    },
    // 获取当前编辑模式设置
    async fetchEditModeStatus() {
      const em = await invoke("read_setting", { key: "editMode" })
      this.editMode = em == null || em.value === "true";
    },
    async reflush_tray() {
      await invoke("reflush_tray");
    },
    async openSetting() {
      this.showSetting = true;
    },
    async closeSetting() {
      this.showSetting = false;
    },
  },
  mounted() {
    this.reflush_tray();
    this.fetchEditModeStatus();
    // 页面加载时刷新 Launcher 列表
    this.refreshLaunchers();
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
  /* 水平排列 */
  flex-direction: row;
  /* 每个 Launcher 之间的间距 */
  gap: 10px;
  /* 开启水平滚动 */
  overflow-x: auto;
  padding: 10px 10px 10px 10px;
  scrollbar-width: auto;
  /* 调整滚动条宽度 */
  /* flex:1; */
  /* height: clac(100vh -50px); */
}

.launcher-container::-webkit-scrollbar {
  /* 滚动条高度 */
  height: 10px;
}

.launcher-container::-webkit-scrollbar-thumb {
  /* 滚动条颜色 */
  background-color: #8B8B8B;
  /* 滚动条圆角 */
  border-radius: 5px;
}

/* 确保每个 launcher 的宽度固定 */
.launcher-container>* {
  flex: 0 0 300px;
  width: 0;
  /* 高度固定为 500px，和原始组件一致 */
  height: 500px;
  padding: 10px 10px 10px 10px;
}

/* 确保每个 launcher 的宽度固定 */
.launcher-lite-container {
  margin-top: 10px;
  width: 100%;
  height: 100%;
  display: flex;
  flex-wrap: wrap;
  /* 水平排列并均匀分布 */
  justify-content: flex-start;
  /* 控制多行间的对齐方式 */
  align-content: flex-start;
  gap: 10px;
  overflow-y: auto;
}

.launcher-lite-container-item {
  aspect-ratio: 2 / 1;
  /* 宽度为容器的四分之一，减去间距 */
  flex: calc(25%);
}

/* 顶部栏样式 */
.topbar {
  width: 100%;
  /* 顶部栏高度 */
  height: 50px;
  /* 浅灰背景色 */
  /* background-color: #f8f9fa; */
  /* 分隔线 */
  border-bottom: 1px solid #ddd;
  display: flex;
  align-items: center;
  /* 左对齐按钮 */
  justify-content: space-between;
  /* 内边距 */
  padding: 0 10px;
  box-sizing: border-box;
}

.home.light,
.topbar.light,
.launcher-lite-container.light,
.launcher-lite-container-item.light,
.modal-content.light {
  background-color: #ffffff;
  color: #000000;
}

.home.dark,
.topbar.dark,
.launcher-lite-container.dark,
.launcher-lite-container-item.dark,
.modal-content.dark {
  background-color: rgba(30, 31, 34);
  color: rgba(188, 190, 196);
}

.modal-overlay.light {
  background-color: rgba(255, 255, 255, 0.5);
  color: #000000;
}

.modal-overlay.dark {
  background-color: rgba(0, 0, 0, 0.5);
  color: rgba(188, 190, 196);
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
  /* 鼠标悬停背景色 */
  background-color: #0056b3;
}

.create-launcher-button:active {
  /* 鼠标按下背景色 */
  background-color: #003d80;
}

.edit-mode-container {
  display: flex;
  justify-content: center;
  align-items: center;
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
  /* 放大复选框大小 */
  transform: scale(1.2);
  cursor: pointer;
}

.topbar-button {
  display: flex;
  gap: 10px;
}

.setting {
  display: flex;
  align-items: center;
  font-size: 16px;
  cursor: pointer;
}

.setting:hover {
  text-decoration: underline;
}

.home-container {
  padding: 20px;
}

button {
  padding: 10px 20px;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 5px;
  cursor: pointer;
}

button:hover {
  background-color: #0056b3;
}

/* 悬浮框样式 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
}

.modal-content {
  background-color: white;
  padding: 20px;
  border-radius: 8px;
  position: relative;
  max-width: 500px;
  width: 90%;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.close-btn {
  position: absolute;
  top: 0px;
  right: 7px;
  font-size: 24px;
  cursor: pointer;
  color: #333;
}

.close-btn:hover {
  color: #000;
}
</style>