<template>
    <div :class="['settings-container', theme]">
        <h2 class="setting-title">设置</h2>

        <div class="setting-item">
            <p class="setting-label">开机启动</p>
            <input type="checkbox" v-model="autoLaunch" @change="toggleAutoLaunch" class="setting-checkbox" />
        </div>

        <div class="setting-item">
            <p class="setting-label">开机启动后最小化到系统托盘</p>
            <input type="checkbox" v-model="hideAfterAutoStart" @change="toggleHideAfterAutoStart"
                class="setting-checkbox" />
        </div>

        <div class="setting-item">
            <div class="setting-label with-tooltip">
                <p class="setting-label">启动编组后退出</p>
                <el-tooltip content="在启动某个编组后退出应用程序" placement="top">
                    <Help :class="['help', theme]" theme="outline" size="15" />
                </el-tooltip>
            </div>
            <input type="checkbox" v-model="launchThenExit" @change="toggleLaunchThenExit" class="setting-checkbox" />
        </div>

        <div class="setting-item">
            <div class="setting-label with-tooltip">
                <p class="setting-label">自动启动编组</p>
                <el-tooltip content="当应用程序被设置为开机启动时, 所选择的编组将在开机启动后自动启动" placement="top">
                    <Help :class="['help', theme]" theme="outline" size="15" />
                </el-tooltip>
            </div>
            <el-select append-to=".home" v-model="autoStartLauncherIds" @change="saveAutoStartLauncher"
                class="setting-select" multiple collapse-tags collapse-tags-tooltip placeholder="请选择">
                <el-option v-for="item in launchers" :key="item.id" :label="item.name" :value="item.id">
                </el-option>
            </el-select>
        </div>

        <div class="setting-item">
            <p class="setting-label">主题</p>
            <el-select append-to=".home" v-model="theme" @change="changeTheme" placeholder="请选择" class="setting-select">
                <el-option v-for="item in themes" :key="item.id" :label="item.value" :value="item.id" />
            </el-select>
        </div>

        <div class="setting-item">
            <p class="setting-label">关闭主面板</p>
            <el-radio-group v-model="closeMainPanel" @change="saveCloseMainPanel" class="setting-radio-group">
                <el-radio value="m1">最小化到系统托盘</el-radio>
                <el-radio value="m2">退出一键启动</el-radio>
            </el-radio-group>
        </div>

        <div class="version-display" :class="theme" title="点击访问项目主页" @click="openGitHubRepo">
            <span>当前版本：{{ appVersion }}</span>
        </div>
    </div>
</template>

<script>
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';
import { invoke } from "@tauri-apps/api/core";
import { onMounted, inject, ref } from 'vue';
import { Help } from '@icon-park/vue-next';
import { getVersion } from '@tauri-apps/api/app';

export default {
    components: {
        Help
    },
    setup(props, { emit }) {
        // 状态管理
        const theme = inject('theme');
        const autoLaunch = ref(false);
        const toggleLock = ref(false);
        const hideAfterAutoStart = ref(false);
        const launchers = ref([]);
        const autoStartLauncherIds = ref([]);
        const launchThenExit = ref(false);
        const themes = ref([
            { "id": "light", "value": "浅色主题" },
            { "id": "dark", "value": "深色主题" }
        ]);
        // 默认值设为最小化到托盘
        const closeMainPanel = ref("m1");
        // 初始状态提示
        const appVersion = ref('加载中...');

        // 主题相关
        const loadTheme = async () => {
            try {
                const savedTheme = await invoke("read_setting", { key: "theme" });
                if (savedTheme?.value) {
                    theme.value = savedTheme.value;
                    window.setTheme?.(savedTheme.value);
                }
            } catch (error) {
                console.error("加载主题失败:", error);
            }
        };

        const changeTheme = async () => {
            try {
                await invoke("save_setting", { key: "theme", value: theme.value });
                window.setTheme?.(theme.value);
            } catch (error) {
                console.error("切换主题失败:", error);
            }
        };

        // 开机启动相关
        const toggleAutoLaunch = async () => {
            if (toggleLock.value) return;
            toggleLock.value = true;

            try {
                if (await isEnabled()) {
                    await disable();
                } else {
                    await enable();
                }
                autoLaunch.value = await isEnabled();
                emit("settings-updated");
            } catch (error) {
                console.error("调整开机启动失败:", error);
                // 失败时恢复之前的状态
                autoLaunch.value = await isEnabled().catch(() => autoLaunch.value);
            } finally {
                toggleLock.value = false;
            }
        };

        const fetchAutoLaunchStatus = async () => {
            try {
                autoLaunch.value = await isEnabled();
            } catch (error) {
                console.error("获取开机启动状态失败:", error);
            }
        };

        // 启动器相关
        const refreshAutoStartLaunchers = async () => {
            try {
                const launchers_data = await invoke("query_launchers");
                launchers.value = [...launchers_data];

                const at_launchers = await invoke("read_setting", { key: "auto_start_launcher_ids" });
                if (at_launchers?.value) {
                    autoStartLauncherIds.value = JSON.parse(at_launchers.value || '[]')
                        .filter(id => id && launchers.value.some(data => data.id === id));
                }
            } catch (error) {
                console.error("加载启动器列表失败:", error);
            }
        };

        const saveAutoStartLauncher = async () => {
            try {
                await invoke("save_setting", {
                    key: "auto_start_launcher_ids",
                    value: JSON.stringify(autoStartLauncherIds.value)
                });
            } catch (error) {
                console.error("保存自动启动编组失败:", error);
            }
        };

        // 其他设置
        const loadLaunchThenExit = async () => {
            try {
                const kv = await invoke("read_setting", { key: "launch_then_exit" });
                launchThenExit.value = kv?.value === "true";
            } catch (error) {
                console.error("加载启动后退出设置失败:", error);
            }
        };

        const toggleLaunchThenExit = async () => {
            try {
                await invoke("save_setting", {
                    key: "launch_then_exit",
                    value: launchThenExit.value ? "true" : "false"
                });
            } catch (error) {
                console.error("保存启动后退出设置失败:", error);
            }
        };

        const loadHideAfterAutoStart = async () => {
            try {
                const kv = await invoke("read_setting", { key: "hide_after_auto_start" });
                hideAfterAutoStart.value = kv?.value === "true";
            } catch (error) {
                console.error("加载自动启动后隐藏设置失败:", error);
            }
        };

        const toggleHideAfterAutoStart = async () => {
            try {
                await invoke("save_setting", {
                    key: "hide_after_auto_start",
                    value: hideAfterAutoStart.value ? "true" : "false"
                });
            } catch (error) {
                console.error("保存自动启动后隐藏设置失败:", error);
            }
        };

        const loadCloseMainPanel = async () => {
            try {
                const kv = await invoke("read_setting", { key: "close_main_panel" });
                closeMainPanel.value = kv?.value || "m1";
            } catch (error) {
                console.error("加载关闭主面板设置失败:", error);
            }
        };

        const saveCloseMainPanel = async () => {
            try {
                await invoke("save_setting", {
                    key: "close_main_panel",
                    value: closeMainPanel.value
                });
            } catch (error) {
                console.error("保存关闭主面板设置失败:", error);
            }
        };

        // 版本信息
        const getAppVersion = async () => {
            try {
                const version = await getVersion();
                appVersion.value = version;
            } catch (error) {
                console.error("获取版本信息失败:", error);
                appVersion.value = "未知版本";
            }
        };

        const openGitHubRepo = async (item) => {
            await invoke("open_path", { path: "https://github.com/Silwings-git/one-click-launch" });
        };

        // 组件初始化
        onMounted(async () => {
            // 并行加载设置，提高初始化速度
            await Promise.all([
                loadTheme(),
                fetchAutoLaunchStatus(),
                refreshAutoStartLaunchers(),
                loadLaunchThenExit(),
                loadCloseMainPanel(),
                loadHideAfterAutoStart(),
                getAppVersion()
            ]);
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
            appVersion,
            saveCloseMainPanel,
            hideAfterAutoStart,
            toggleHideAfterAutoStart,
            openGitHubRepo
        };
    }
};
</script>

<style scoped>
.settings-container {
    max-width: 1000px;
    max-height: 400px;
    overflow-y: auto;
    padding: 20px;
    border: 1px solid #ccc;
    border-radius: 8px;
}

.setting-title {
    margin: 0 0 16px 0;
    font-size: 1.2rem;
    font-weight: 600;
}

.version-display {
    margin: 0 0 24px 0;
    padding: 8px 12px;
    font-size: 0.9rem;
    border-radius: 4px;
    background-color: rgba(0, 0, 0, 0.05);
}

/* 主题适配样式 */
.version-display.light {
    color: #666;
    background-color: #f5f5f5;
}

.version-display.dark {
    color: #a3a1a1;
    background-color: #333;
}

/* 统一的设置项样式 */
.setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
    padding-bottom: 16px;
    border-bottom: 1px solid #eee;
}

.setting-item:last-child {
    border-bottom: none;
}

.setting-label {
    margin: 0;
    padding-right: 10px;
    flex: 1;
}

.with-tooltip {
    display: flex;
    align-items: center;
    gap: 8px;
}

.setting-checkbox {
    transform: scale(1.2);
    cursor: pointer;
}

.setting-select {
    width: 240px;
}

.setting-radio-group {
    display: flex;
    gap: 16px;
}

/* 帮助图标样式 */
.help.light {
    color: #333;
}

.help.dark {
    color: #a3a1a1;
}

/* 深色主题适配 */
.settings-container.dark {
    border-color: #444;
}

.dark .setting-item {
    border-bottom-color: #555;
}

.version-display {
    margin-top: auto;
    padding: 8px 12px;
    font-size: 0.9rem;
    border-radius: 4px;
    background-color: rgba(0, 0, 0, 0.05);
    /* 让鼠标悬浮时显示手型，提示可交互 */
    cursor: pointer;
}

.version-display:hover {
    background-color: rgba(0, 0, 0, 0.08);
}
</style>
