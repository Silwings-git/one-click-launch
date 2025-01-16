<template>
    <Home />
</template>

<script>
import { defineComponent } from "vue";
import Launcher from "./launcher.vue";
import Home from "./home.vue";

export default defineComponent({
    components: {
        Demo,
        Launcher,
        Home
    },
});
</script>