<template>
    <div class="launcher">
        <div class="header">
            <span v-if="!isEditing" class="name" @dblclick="editLauncherName" title="双击修改名称">
                {{ this.data.name }}
            </span>
            <input v-if="isEditing" v-model="newLauncherName" class="name-input" @blur="saveLauncherName"
                @keyup.enter="saveLauncherName" />
            <div class="button-container">
                <button class="copy-button" @click="copyLauncher">复制</button>
                <button class="delete-launcher" @click="deleteLauncher" title="删除">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="16"
                        height="16">
                        <path
                            d="M9 3v1H4v2h16V4h-5V3H9zM5 7v12c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V7H5zm4 2h2v8H9V9zm4 0h2v8h-2V9z" />
                    </svg>
                </button>
            </div>
        </div>
        <hr />
        <div class="add-row">
            <div class="move-launcher" @click="moveLauncher(0)" title="向左移动">&lt;</div>
            <div class="add-left" @click="addRow(false)">+ 添加</div>
            <div class="add-folder-button" @click="addRow(true)">添加文件夹</div>
            <div class="add-url-button" @click="showAddUrlDialog">添加网址</div>
            <div class="move-launcher" @click="moveLauncher(1)" title="向右移动">&gt;</div>
        </div>
        <div class="content">
            <!-- 弹框部分 -->
            <div v-if="showDialog" class="dialog-overlay">
                <div class="dialog">
                    <h3>添加网址</h3>
                    <label for="url-name">名称:</label>
                    <input type="text" id="url-name" v-model="addUrlName" @keydown.enter="addUrl"
                        :placeholder="addUrlNamePlaceholder" />
                    <label for="url-content">网址:</label>
                    <input type="text" id="url-content" v-model="addUrlContent" @keydown.enter="addUrl" />
                    <div class="dialog-actions">
                        <button @click="addUrl">确认</button>
                        <button @click="closeDialog">取消</button>
                    </div>
                </div>
            </div>

            <div class="data-row" v-for="(item, index) in data.resources" :key="item.id" :title="item.path">
                <span class="data-text">
                    <span v-if="!editingResourceState.get(item.id)" @dblclick="editResourceName(item)" title="双击修改名称">
                        <strong>{{ item.name }}:</strong>
                    </span>
                    <input v-if="editingResourceState.get(item.id)" v-model="newResourceName" class="name-input"
                        @blur="saveResourceName(item)" @keyup.enter="saveResourceName(item)" />
                    <span>{{ item.path }}</span>
                </span>
                <button class="delete-button" @click="deleteRow(item.id)" title="删除">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="16"
                        height="16">
                        <path
                            d="M9 3v1H4v2h16V4h-5V3H9zM5 7v12c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V7H5zm4 2h2v8H9V9zm4 0h2v8h-2V9z" />
                    </svg>
                </button>
            </div>
        </div>
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
import { confirm, message, open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { useToast } from "vue-toastification";
import { platform } from '@tauri-apps/plugin-os'
import { ref, reactive, onMounted, nextTick, onBeforeMount } from 'vue';

const toast = useToast()

export default {
    props: {
        launcherData: {
            type: Object,
            required: true, // 确保传入数据
        },
    },
    setup(props, { emit }) {

        // 控制下拉菜单的显示
        const dropdownVisible = ref(false);
        // 控制网址弹框的显示
        const showDialog = ref(false);
        // 临时存储的新启动器名称
        const newLauncherName = ref("");
        // 是否处于编辑模式
        const isEditing = ref(false);
        const addUrlName = ref("");
        const addUrlContent = ref("");
        const addUrlNamePlaceholder = ref("网页");
        // 是否正在启动
        const isLaunching = ref(false);
        const editingResourceState = ref(new Map());
        const newResourceName = ref("");
        const launcherNameInputRef = ref(null);
        const resourceNameInputRef = ref(null);
        const debouncedLaunch = ref(null);

        const editLauncherName = () => {
            isEditing.value = true; // 进入编辑模式
            newLauncherName.value = props.launcherData.name; // 预填当前名称
            nextTick(() => {
                launcherNameInputRef.value && launcherNameInputRef.value.focus();
            });
        };
        const saveLauncherName = async () => {
            if (newLauncherName.value.trim()) {
                newLauncherName.value = newLauncherName.value.trim(); // 保存修改后的名称
            }
            await invoke("modify_launcher_name", { launcherId: props.launcherData.id, name: newLauncherName.value });
            isEditing.value = false; // 退出编辑模式
            emit("launcher-updated", props.launcherData.id);
        };
        const editResourceName = (item) => {
            editingResourceState.value.set(item.id, true); // 进入编辑模式
            newResourceName.value = item.name; // 预填当前名称
            nextTick(() => {
                // 自动聚焦到输入框
                resourceNameInputRef.value && resourceNameInputRef.value.focus();
            });
        };
        const saveResourceName = async (item) => {
            if (newResourceName.value.trim()) {
                const trimName = newResourceName.value.trim();
                if (item.name === trimName) {
                    editingResourceState.value.set(item.id, false); // 退出编辑模式
                    return;
                }
                newResourceName.value = trimName; // 保存修改后的名称
            }
            await invoke("modify_resource_name", { resourceId: item.id, name: newResourceName.value });
            editingResourceState.value.set(item.id, false); // 退出编辑模式
            emit("launcher-updated", props.launcherData.id);
        };
        const addRow = async (directory) => {
            try {

                let defaultPath = ""; // 默认不设置路径

                if (!directory) {
                    // 检测操作系统
                    const currentPlatform = await platform();
                    if (currentPlatform === "windows") {
                        // 只有在 Windows 系统时设置预定义路径
                        defaultPath = "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs";
                    }
                }

                const filePath = await open({
                    multiple: false, // 禁止多选
                    directory: directory, // 选择文件而不是文件夹
                    defaultPath: defaultPath, // 动态设置默认路径
                });

                if (filePath) {
                    // 调用后端存储路径的命令
                    await invoke("add_resource", { launcherId: props.launcherData.id, path: filePath });
                    emit("launcher-updated", props.launcherData.id);
                }
            } catch (error) {
                console.error("文件选择错误:", error);
            }
        };
        const deleteRow = async (resourceId) => {
            await invoke("delete_resource", { resourceId: resourceId })
            emit("launcher-updated", props.launcherData.id);
        };
        const deleteLauncher = async () => {
            const userConfirmed = await confirm(
                "您确定要删除这一行吗？此操作无法撤销。",
                { title: "确认删除", type: "question" }
            );
            if (userConfirmed) {
                await invoke("delete_launcher", { "launcherId": props.launcherData.id });
                emit("launcher-updated", props.launcherData.id);
            }
        };
        const copyLauncher = async () => {
            await invoke("copy_launcher", { launcherId: props.launcherData.id });
            emit("launcher-updated", props.launcherData.id);
        };
        const launch = async () => {
            isLaunching.value = true;
            try {
                await invoke("launch", { launcherId: props.launcherData.id });
                toast.success("启动成功！所有内容已激活！");
            } catch (error) {
                console.error("启动失败:", error);
                toast.error("启动失败！");
            } finally {
                isLaunching.value = false; // 恢复按钮状态
            }
        };
        const showAddUrlDialog = () => {
            showDialog.value = true; // 打开添加网址的对话框
            dropdownVisible.value = false; // 关闭下拉菜单
        };
        const addUrl = async () => {
            if (addUrlContent.value) {
                let urlName = addUrlName.value;
                if (!urlName) {
                    urlName = addUrlNamePlaceholder.value;
                }
                await invoke("add_resource", { launcherId: props.launcherData.id, name: urlName, path: addUrlContent.value });
                emit("launcher-updated", props.launcherData.id);
                await closeDialog();
            } else {
                await message("请输入名称和网址！");
            }
        };
        const closeDialog = async () => {
            showDialog.value = false; // 关闭对话框
            addUrlName.value = "";
            addUrlContent.value = "";
        };
        const moveLauncher = (type) => {
            emit("launcher-moved", props.launcherData.id, type);
        };
        const debounce = (func, delay) => {
            let timer;
            return function (...args) {
                if (timer) clearTimeout(timer);
                timer = setTimeout(() => func.apply(this, args), delay);
            };
        };
        onBeforeMount(() => {
            // 包装 launch 方法为防抖函数
            debouncedLaunch.value = debounce(launch, 2000); // 2秒防抖
        });

        return {
            data: props.launcherData,
            dropdownVisible,
            showDialog,
            newLauncherName,
            isEditing,
            addUrlName,
            addUrlContent,
            addUrlNamePlaceholder,
            isLaunching,
            editingResourceState,
            newResourceName,
            editLauncherName,
            saveLauncherName,
            editResourceName,
            saveResourceName,
            addRow,
            deleteRow,
            deleteLauncher,
            copyLauncher,
            launch,
            showAddUrlDialog,
            addUrl,
            closeDialog,
            moveLauncher,
            debounce,
            debouncedLaunch
        };
    }
};
</script>
<style scoped>
.launcher {
    width: 300px;
    height: 500px;
    border: 1px solid #ccc;
    border-radius: 8px;
    padding: 10px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
}

.header {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.name {
    font-size: 18px;
    font-weight: bold;
}

.copy-button {
    background-color: #007bff;
    color: white;
    border: none;
    padding: 5px 10px;
    border-radius: 4px;
    cursor: pointer;
    width: 50px;
}

.copy-button:hover {
    background-color: #0056b3;
}

hr {
    margin: 10px 0;
    border: none;
    border-top: 1px solid #ddd;
}

.content {
    flex: 1;
    overflow-y: auto;

}

.add-row {
    font-size: 14px;
    color: #007bff;
    cursor: pointer;
    margin-bottom: 10px;
    display: flex;
    justify-content: start;
    align-items: center;
    /* 给按钮添加间距 */
    gap: 20px;
}

.move-launcher:hover,
.add-left:hover,
.add-folder-button:hover,
.add-url-button:hover {
    text-decoration: underline;
}

.data-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 5px;
    padding: 5px;
    border: 1px solid #ddd;
    border-radius: 4px;
}

.data-text {
    flex: 1 0 0;
    width: 0;
    word-break: break-all;
    display: flex;
    flex-flow: column nowrap;
}

.delete-button {
    flex-shrink: 0;
}

.data-text {
    font-size: 14px;
}

.data-text span {
    /* 隐藏溢出内容 */
    overflow: hidden;
    /* 超出部分显示省略号 */
    text-overflow: ellipsis;
    /* 不换行 */
    white-space: nowrap;
}

.delete-button,
.delete-launcher {
    /* 无背景色 */
    background-color: transparent;
    /* 默认蓝色 */
    color: #BFBFBF;
    border: none;
    /* 固定按钮大小 */
    width: 30px;
    height: 30px;
    /* 圆形按钮 */
    border-radius: 50%;
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 0;
    /* 平滑过渡 */
    transition: color 0.2s ease;
}

.delete-button:hover,
.delete-launcher:hover {
    /* 鼠标悬浮时变为红色 */
    color: #dc3545;
}

.launch-button {
    /* 绿色背景 */
    background-color: #28a745;
    /* 白色文字 */
    color: white;
    border: none;
    /* 占满宽度 */
    width: 100%;
    /* 高度 */
    height: 50px;
    /* 较大的文字 */
    font-size: 18px;
    /* 加粗文字 */
    font-weight: bold;
    /* 圆角 */
    border-radius: 8px;
    cursor: pointer;
    /* 与上方内容保持距离 */
    margin-top: 10px;
    /* 平滑过渡效果 */
    transition: background-color 0.3s ease, transform 0.2s ease;
}

.launch-button:hover {
    /* 鼠标悬浮时更深的绿色 */
    background-color: #218838;
    /* 鼠标悬浮时放大效果 */
    transform: scale(1.05);
}

.launch-button:active {
    /* 点击时更深的绿色 */
    background-color: #1e7e34;
    /* 点击时缩小效果 */
    transform: scale(0.95);
}

/* 按钮容器 */
.button-container {
    display: flex;
    /* 设置按钮之间的间距 */
    gap: 10px;
}

.dialog-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
}


.dialog {
    background-color: white;
    padding: 20px;
    border-radius: 8px;
    width: 300px;
}

.dialog h3 {
    margin: 0;
    font-size: 18px;
}

.dialog label {
    display: block;
    margin-top: 10px;
}

.dialog input {
    width: 100%;
    padding: 5px;
    margin-top: 5px;
    border: 1px solid #ddd;
    border-radius: 4px;
}

.dialog-actions {
    margin-top: 20px;
    text-align: center;
}

.dialog-actions button {
    margin: 0 5px;
    padding: 6px 12px;
    border-radius: 4px;
    border: none;
    cursor: pointer;
}

.dialog-actions button:hover {
    background-color: #007bff;
    color: white;
}

.name-input {
    font-size: 18px;
    font-weight: bold;
    border: 1px solid #ddd;
    border-radius: 4px;
    padding: 5px;
    width: 100%;
    box-sizing: border-box;
}

.mo-launcher {
    font-size: 18px;
}
</style>