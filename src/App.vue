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

</style>