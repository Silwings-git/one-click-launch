<template>
    <div :class="['settings-container', theme]">
        <h2 class="setting-title">设置</h2>
        <!-- 主题切换 -->
        <div class="setting-item">
            <label for="theme-select">主题:</label>
            <select id="theme-select" v-model="theme" @change="changeTheme">
                <option value="light">浅色主题</option>
                <option value="dark">深色主题</option>
            </select>
        </div>

        <div class="setting-item">
            <label class="checkbox-label">开机启动
                <input type="checkbox" v-model="autoLaunch" @change="toggleAutoLaunch" />
            </label>
        </div>

        <!-- 保存按钮 -->
        <button @click="fetchAutoLaunchStatus">刷新</button>

    </div>
</template>

<script>
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';
import { invoke } from "@tauri-apps/api/core";

export default {
    data() {
        return {
            // selectedTheme: 'light', // 默认主题
            notificationsEnabled: true, // 默认开启通知
            // 开机启动状态
            autoLaunch: false,
            // 开机启动锁
            toggleLock: false,
            theme: 'light' // 默认主题为亮色
        };
    },
    methods: {
        // 切换主题
        async changeTheme() {
            await invoke("save_setting", { key: "theme", value: this.theme });
            // this.theme = this.selectedTheme;
            this.$emit("settings-updated");
        },
        // 切换通知开关
        toggleNotifications() {
            if (this.notificationsEnabled) {
                console.log('通知已开启');
            } else {
                console.log('通知已关闭');
            }
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
                this.$emit("settings-updated");
            }
        },
        // 获取当前开机启动状态
        async fetchAutoLaunchStatus() {
            try {
                this.autoLaunch = await isEnabled();
            } catch (error) {
                console.error("Failed to fetch auto launch status:", error);
            }
        },
        async reloadSettings() {
            const themeSetting = await invoke("read_setting", { key: "theme" });
            if (themeSetting?.value) {
                this.theme = themeSetting.value;
            }
        }
    },
    mounted() {
        // 初始化时获取开机启动状态
        this.fetchAutoLaunchStatus();
        this.reloadSettings();
    },
};
</script>

<style scoped>
.settings-container {
    max-width: 500px;
    /* 最大宽度 */
    max-height: 400px;
    /* 最大高度 */
    overflow-y: auto;
    /* 纵向滚动条 */
    padding: 20px;
    border: 1px solid #ccc;
    /* 可选：添加边框 */
    border-radius: 8px;
    /* 可选：圆角 */
}


.setting-title {
    margin-top: 0px;
}

.setting-item {
    margin-bottom: 15px;
}

label {
    margin-right: 10px;
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

.checkbox-label {
    display: flex;
    align-items: center;
    font-size: 16px;
    cursor: pointer;
    width: 100px;
}

input[type="checkbox"] {
    margin-left: 10px;
    transform: scale(1.2);
    /* 放大复选框大小 */
    cursor: pointer;
}

.hsettings-container.light {
    background-color: #ffffff;
    color: #000000;
}

.settings-container.dark {
    background-color: #1a1a1a;
    color: #ffffff;
}
</style>