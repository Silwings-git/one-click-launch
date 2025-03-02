<template>
    <div :class="['settings-container', theme]">
        <h2 class="setting-title">设置</h2>

        <div class="m-4" style="display: flex;justify-content: space-between; align-items: center;">
            <p style="margin-right: 10px;">开机启动</p>
            <input type="checkbox" v-model="autoLaunch" @change="toggleAutoLaunch" />
        </div>

        <div class="m-4" style="display: flex;justify-content: space-between; align-items: center;">
            <div style="display: flex;align-items: center;">
                <p style="margin-right: 10px;">启动编组后退出</p>
                <el-tooltip content="在启动某个编组后退出应用程序" placement="top">
                    <Help :class="['help', theme]" theme="outline" size="15"  />
                </el-tooltip>
            </div>
            <input type="checkbox" v-model="launchThenExit" @change="toggleLaunchThenExit" />
        </div>

        <div class="m-4" style="display: flex; justify-content: space-between;align-items: center;">
            <div style="display: flex;align-items: center;">
                <p style="margin-right: 10px;">自动启动编组</p>
                <el-tooltip content="当应用程序被设置为开机启动时, 所选择的编组将在开机启动后自动启动" placement="top">
                    <Help :class="['help', theme]" theme="outline" size="15"  />
                </el-tooltip>
            </div>
            <el-select append-to=".home" v-model="autoStartLauncherIds" @change="saveAutoStartLauncher"
                style="width: 240px;" multiple collapse-tags collapse-tags-tooltip placeholder="Select">
                <el-option v-for="item in launchers" :key="item.id" :label="item.name" :value="item.id">
                </el-option>
            </el-select>
        </div>

        <div class="m-4" style="display: flex; justify-content: space-between;align-items: center;">
            <p style="margin-right: 10px;">主题</p>
            <el-select append-to=".home" v-model="theme" @change="changeTheme" placeholder="Select"
                style="width: 240px">
                <el-option v-for="item in themes" :key="item.id" :label="item.value" :value="item.id" />
            </el-select>
        </div>

        <div class="m-4" style="display: flex;justify-content: space-between; align-items: center;">
            <div style="display: flex;align-items: center;">
                <p style="margin-right: 10px;">关闭主面板</p>
            </div>
            <div class="my-2 ml-4">
                <el-radio-group v-model="closeMainPanel" @change="saveCloseMainPanel">
                    <el-radio value="m1">最小化到系统托盘</el-radio>
                    <el-radio value="m2">退出一键启动</el-radio>
                </el-radio-group>
            </div>
        </div>

    </div>
</template>

<script>
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';
import { invoke } from "@tauri-apps/api/core";
import { onMounted, inject, ref, nextTick } from 'vue';
import { Help } from '@icon-park/vue-next';

export default {
    components: {
        Help
    },
    setup(props, { emit }) {
        const theme = inject('theme');
        const autoLaunch = ref(false);
        const toggleLock = ref(false);
        const launchers = ref([]);
        const autoStartLauncherIds = ref([]);
        const launchThenExit = ref(false);
        const themes = ref([{ "id": "light", "value": "浅色主题" }, { "id": "dark", "value": "深色主题" }]);
        const closeMainPanel = ref();

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

        const refreshAutoStartLaunchers = async () => {
            const launchers_data = await invoke("query_launchers");
            launchers.value = [...launchers_data];
            const at_launchers = await invoke("read_setting", { key: "auto_start_launcher_ids" });
            if (at_launchers?.value) {
                autoStartLauncherIds.value = JSON.parse(at_launchers.value || '[]')
                    .filter(id =>
                        id && launchers.value.some(data => data.id === id)
                    );
            }
        };

        const saveAutoStartLauncher = async () => {
            await invoke("save_setting", { key: "auto_start_launcher_ids", value: JSON.stringify(autoStartLauncherIds.value) });
        };

        const loadLaunchThenExit = async () => {
            const kv = await invoke("read_setting", { key: "launch_then_exit" });
            launchThenExit.value = kv == null || kv.value === "true";
        };

        const loadCloseMainPanel = async () => {
            const kv = await invoke("read_setting", { key: "close_main_panel" });
            if (null == kv?.value) {
                closeMainPanel.value = "m1";
            } else {
                closeMainPanel.value = kv.value;
            }
        };

        const saveCloseMainPanel = async () => {
            await invoke("save_setting", { key: "close_main_panel", value: closeMainPanel.value });
        };

        const toggleLaunchThenExit = async () => {
            await invoke("save_setting", { key: "launch_then_exit", value: launchThenExit.value ? "true" : "false" });
        };

        // 在组件挂载时加载主题
        onMounted(() => {
            loadTheme();
            fetchAutoLaunchStatus();
            refreshAutoStartLaunchers();
            loadLaunchThenExit();
            loadCloseMainPanel();
        });

        return {
            theme,
            autoLaunch,
            launchers,
            autoStartLauncherIds,
            saveAutoStartLauncher,
            themes,
            changeTheme,
            toggleAutoLaunch,
            launchThenExit,
            toggleLaunchThenExit,
            closeMainPanel,
            saveCloseMainPanel,
            loadCloseMainPanel
        };
    }
};
</script>

<style scoped>
.settings-container {
    /* 最大宽度 */
    max-width: 1000px;
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
    background-color: #409eff;
    ;
    border-color: #409eff;
    color: white;
    border: none;
    border-radius: 5px;
    cursor: pointer;
}

button:hover {
    background-color: #0056b3;
}

.checkbox-label {
    /* display: flex; */
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

.help.light {
    color: #333;
}

.help.dark {
    color: #a3a1a1;
}
</style>