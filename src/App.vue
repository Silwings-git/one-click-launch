<template>
    <div :class="theme">
        <Home />
    </div>
</template>

<script>
import { defineComponent } from "vue";
import Launcher from "./launcher.vue";
import LauncherLite from "./launcherLite.vue";
import Settings from "./settings.vue";
import Home from "./home.vue";
import { invoke } from "@tauri-apps/api/core";

import { ref, provide } from 'vue';

export default defineComponent({
    setup() {
        const theme = ref('light'); // 默认主题

        // 从后端加载主题
        const loadTheme = async () => {
            const savedTheme = await invoke("read_setting", { key: "theme" });
            if (savedTheme?.value) {
                theme.value = savedTheme.value;
                window.setTheme(savedTheme.value);
            }
        };

        // 提供全局主题状态
        provide('theme', theme);

        return {
            theme,
            loadTheme
        };
    },
    components: {
        Launcher,
        LauncherLite,
        Settings,
        Home
    },
    mounted() {
        this.loadTheme();
    }
});
</script>

<style>
.light {
    background-color: #ffffff;
    color: #000000;
}

.dark {
    background-color: rgba(30, 31, 34);
    color: rgba(188, 190, 196);
}

.dark .el-select-dropdown {
    background-color: rgba(30, 31, 34);
    color: rgba(188, 190, 196);
}

.dark .el-select-dropdown__item.is-hovering {
    background-color: #020913;
}

.dark .el-tag {
    background-color: rgba(30, 31, 34);
    color: rgba(188, 190, 196);
}

/* 公共样式 */
input[type="checkbox"] {
    width: 16px;
    height: 16px;
    border: 1px solid #ccc;
    border-radius: 3px;
    background-color: #f0f0f0;
    cursor: pointer;
    outline: none;
}

/* 勾选标记 */
input[type="checkbox"]:checked::after,
.dark input[type="checkbox"]:checked::after {
    content: "✔";
    display: block;
    text-align: center;
    color: #fff;
    font-size: 12px;
    line-height: 16px;
}

/* 深色主题样式 */
.dark input[type="checkbox"] {
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
    border-color: #bcbec4;
    background-color: #1e1f22;
}

.dark input[type="checkbox"]:checked {
    background-color: #ccc;
    border-color: #ccc;
}

</style>