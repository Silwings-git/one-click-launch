<template>
    <div class="container">
        <div class="row">
            <span class="row-name" :title="data.name"> {{ data.name }}</span>
            <span class="move-buttons">
                <div @click="moveLauncher(0)" class="move-button" title="向左移动">&lt;</div>
                <div @click="moveLauncher(1)" class="move-button" title="向右移动">&gt;</div>
            </span>
        </div>
        <div class="spacer" :title="formattedResourceNames"></div>
        <button class="launch-button" :disabled="isLaunching" @click="launch">
            <span v-if="!isLaunching">启动</span>
            <span v-else>
                <svg class="loading-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
                    stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" width="18"
                    height="18">
                    <circle cx="12" cy="12" r="10" stroke-dasharray="80" stroke-dashoffset="60">
                        <animateTransform attributeName="transform" type="rotate" from="0 12 12" to="360 12 12" dur="1s"
                            repeatCount="indefinite" />
                    </circle>
                </svg>
                启动中...
            </span>
        </button>
    </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/core";
import { useToast } from "vue-toastification";

const toast = useToast()

export default {
    props: {
        launcherData: {
            type: Object,
            required: true, // 确保传入数据
        },
    },
    data() {
        return {
            data: this.launcherData,
            isLaunching: false, // 是否正在启动
        };
    },
    methods: {
        moveLauncher(type) {
            this.$emit("launcher-moved", this.data.id, type);
        },
        async launch() {
            this.isLaunching = true;
            try {
                await invoke("launch", { launcherId: this.data.id });
                toast.success("启动成功！所有内容已激活！");
                await invoke("hide_window", {});
            } catch (error) {
                console.error("启动失败:", error);
                toast.error("启动失败！");
            } finally {
                this.isLaunching = false; // 恢复按钮状态
            }
        },
    },
    computed: {
        formattedResourceNames() {
            return this.data.resources.map(resource => resource.name).join('\n');
        }
    }
};
</script>

<style scoped>
.container {
    display: flex;
    flex-direction: column;
    width: 200px;
    border: 1px solid #ccc;
    padding: 10px;
    background-color: #f9f9f9;
}

.row {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.name {
    font-size: 16px;
    font-weight: bold;
}

.move-buttons {
    width: 25%;
    font-size: 12px;
    color: #007bff;
    cursor: pointer;
    margin-bottom: 0;
    display: flex;
    align-items: center;
    gap: 0px;
    justify-content: space-between
}

.move-button {
    font-size: 20px;
    background: none;
    border: none;
    cursor: pointer;
    padding: 5px;
    margin: 0 5px;
}

.move-button:hover {
    text-decoration: underline;
}

.row-name {
    width: 75%;
    font-size: 18px;
    font-weight: bold;
}

.row span {
    overflow: hidden;
    /* 隐藏溢出内容 */
    text-overflow: ellipsis;
    /* 超出部分显示省略号 */
    white-space: nowrap;
    /* 不换行 */
}

.launch-button {
    background-color: #28a745;
    /* 绿色背景 */
    color: white;
    /* 白色文字 */
    border: none;
    width: 100%;
    /* 占满宽度 */
    height: 50px;
    /* 高度 */
    font-size: 18px;
    /* 较大的文字 */
    font-weight: bold;
    /* 加粗文字 */
    border-radius: 8px;
    /* 圆角 */
    cursor: pointer;
    margin-top: 0px;
    margin-bottom: 0px;
    /* 与上方内容保持距离 */
    transition: background-color 0.3s ease, transform 0.2s ease;
    /* 平滑过渡效果 */
}

.launch-button:hover {
    background-color: #218838;
    /* 鼠标悬浮时更深的绿色 */
    transform: scale(1.05);
    /* 鼠标悬浮时放大效果 */
}

.launch-button:active {
    background-color: #1e7e34;
    /* 点击时更深的绿色 */
    transform: scale(0.95);
    /* 点击时缩小效果 */
}

.spacer {
    flex-grow: 1;
    /* This pushes the launch button to the bottom */
}
</style>