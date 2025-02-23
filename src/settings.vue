<template>
    <div class="settings-container">
        <h2 class="setting-title">设置</h2>
        <div class="setting-item">
            <label for="theme-select">主题:</label>
            <select v-model="theme" @change="changeTheme" :class="['theme-select', theme]">
                <option value="light">浅色主题</option>
                <option value="dark">深色主题</option>
            </select>
        </div>

        <div class="setting-item">
            <label class="checkbox-label">开机启动
                <input type="checkbox" v-model="autoLaunch" @change="toggleAutoLaunch" />
            </label>
        </div>

        <div class="setting-item">
            <label for="auto-start-launcher-select">自动启动编组:</label>
            <select id="auto-start-launcher-select" v-model="autoStartLaunchers" multiple>
                <option v-for="item in launchers" :key="item.id" :value="item.id">
                    {{ item.name }}
                </option>
            </select>
        </div>

        <!-- 按钮 -->
        <button >刷新</button>

    </div>
</template>

<script>
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';
import { invoke } from "@tauri-apps/api/core";
import { onMounted, inject,ref } from 'vue';

export default {
    setup(props, { emit }) {
        const theme = inject('theme');
        const autoLaunch = ref(false);
        const toggleLock = ref(false);
        const launchers = ref([]);
        const autoStartLaunchers = ref([]);

        // 从后端加载主题
        const loadTheme = async () => {
            const savedTheme = await invoke("read_setting", { key: "theme" });
            if (savedTheme?.value) {
                theme.value = savedTheme.value;
                window.setTheme(savedTheme.value);
            }
        };

        // 定义切换主题的方法
        const changeTheme = async () => {
            await invoke("save_setting", { key: "theme", value: theme.value });
            await invoke("change_windows_theme", { theme: theme.value });
            window.setTheme(theme.value);
        };

        // 切换开机启动状态
        const toggleAutoLaunch = async () => {
            if (toggleLock.value) {
                return; // 如果已有任务在执行，直接返回
            }
            toggleLock.value = true;
            try {
                if (await isEnabled()) {
                    await disable();
                } else {
                    await enable();
                }
                // 更新当前状态
                autoLaunch.value = await isEnabled();
            } catch (error) {
                console.error("Failed to toggle auto-launch:", error);
                toast.error("调整开机启动失败！");
            } finally {
                toggleLock.value = false; // 释放锁
                emit("settings-updated");
            }
        };
        // 获取当前开机启动状态
        const fetchAutoLaunchStatus = async () => {
            try {
                autoLaunch.value = await isEnabled();
            } catch (error) {
                console.error("Failed to fetch auto launch status:", error);
            }
        };

        // 在组件挂载时加载主题
        onMounted(() => {
            loadTheme();
            fetchAutoLaunchStatus();
        });

        return {
            theme,
            autoLaunch,
            launchers,
            autoStartLaunchers,
            changeTheme,
            toggleAutoLaunch
        };
    }
};
</script>

<style scoped>
.settings-container {
    /* 最大宽度 */
    max-width: 500px;
    /* 最大高度 */
    max-height: 400px;
    /* 纵向滚动条 */
    overflow-y: auto;
    padding: 20px;
    /* 可选：添加边框 */
    border: 1px solid #ccc;
    /* 可选：圆角 */
    border-radius: 8px;
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

.theme-select.light {
    background-color: #ffffff;
    color: #000000;
}

.theme-select.dark {
    background-color: rgba(30, 31, 34);
    color: rgba(188, 190, 196);
}
</style>