<template>
    <div>
        <Home />
    </div>
</template>

<script>
import { defineComponent } from "vue";
import Launcher from "./launcher.vue";
import LauncherLite from "./launcherLite.vue";
import Settings from "./settings.vue";
import Home from "./home.vue";

export default defineComponent({
    components: {
        Launcher,
        LauncherLite,
        Settings,
        Home
    },
});
</script>