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
                <button class="delete-launcher" @click="deleteLauncher">
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
            <div class="move-launcher" @click="moveLauncher(0)"><</div>
            <div class="add-left" @click="addRow(false)">+ 添加</div>
            <div class="add-folder-button" @click="addRow(true)">添加文件夹</div>
            <div class="add-url-button" @click="showAddUrlDialog">添加网址</div>
            <div class="move-launcher" @click="moveLauncher(1)">></div>
        </div>
        <div class="content">
            <!-- 弹框部分 -->
            <div v-if="showDialog" class="dialog-overlay">
                <div class="dialog">
                    <h3>添加网址</h3>
                    <label for="url-name">名称:</label>
                    <input type="text" id="url-name" v-model="addUrlName" />
                    <label for="url-content">网址:</label>
                    <input type="text" id="url-content" v-model="addUrlContent" />
                    <div class="dialog-actions">
                        <button @click="addUrl">确认</button>
                        <button @click="closeDialog">取消</button>
                    </div>
                </div>
            </div>

            <div class="data-row" v-for="(item, index) in data.resources" :key="item.id" :title="item.path">
                <span class="data-text">
                    <!-- <strong>{{ item.name }}:</strong> -->
                    <span v-if="!isEditingResourceName" @dblclick="editResourceName(item)" title="双击修改名称">
                        <strong>{{ item.name }}:</strong>
                    </span>
                    <input v-if="isEditingResourceName" v-model="newResourceName" class="name-input" @blur="saveResourceName(item.id)"
                        @keyup.enter="saveResourceName(item.id)" />
                    <span>{{ item.path }}</span>
                </span>
                <button class="delete-button" @click="deleteRow(item.id)">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="16"
                        height="16">
                        <path
                            d="M9 3v1H4v2h16V4h-5V3H9zM5 7v12c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V7H5zm4 2h2v8H9V9zm4 0h2v8h-2V9z" />
                    </svg>
                </button>
            </div>
        </div>
        <!-- <button class="launch-button" @click="launch">启动</button> -->
        <button 
            class="launch-button" 
            :disabled="isLaunching" 
            @click="launch">
            <span v-if="!isLaunching">启动</span>
            <span v-else>
                <svg 
                    class="loading-icon" 
                    xmlns="http://www.w3.org/2000/svg" 
                    viewBox="0 0 24 24" 
                    fill="none" 
                    stroke="currentColor" 
                    stroke-width="2" 
                    stroke-linecap="round" 
                    stroke-linejoin="round" 
                    width="18" 
                    height="18">
                    <circle cx="12" cy="12" r="10" stroke-dasharray="80" stroke-dashoffset="60">
                        <animateTransform
                            attributeName="transform"
                            type="rotate"
                            from="0 12 12"
                            to="360 12 12"
                            dur="1s"
                            repeatCount="indefinite" />
                    </circle>
                </svg>
                启动中...
            </span>
        </button>
    </div>
</template>

<script>
import { confirm, message ,open} from "@tauri-apps/plugin-dialog";
import {invoke} from "@tauri-apps/api/core";        
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
            data: this.launcherData, // 初始化内容
            dropdownVisible: false, // 控制下拉菜单的显示
            showDialog: false, // 控制网址弹框的显示
            newLauncherName: "", // 临时存储的新启动器名称
            isEditing: false, // 是否处于编辑模式
            addUrlName:"",
            addUrlContent:"",
            isLaunching: false, // 是否正在启动
            isEditingResourceName: false,
            newResourceName: "",
        };
    },
    created() {
        // 包装 launch 方法为防抖函数
        this.debouncedLaunch = this.debounce(this.launch, 2000); // 2秒防抖
    },
    methods: {
        editLauncherName() {
            this.isEditing = true; // 进入编辑模式
            this.newLauncherName = this.data.name; // 预填当前名称
            this.$nextTick(() => {
                // 自动聚焦到输入框
                const input = this.$el.querySelector(".name-input");
                input && input.focus();
            });
        },
        async saveLauncherName() {
            if (this.newLauncherName.trim()) {
                this.launcherName = this.newLauncherName.trim(); // 保存修改后的名称
            }
            await invoke("modify_launcher_name", { launcherId: this.data.id, name: this.launcherName });
            this.isEditing = false; // 退出编辑模式
            this.$emit("launcher-updated", this.data.id);
        },
        editResourceName(item) {
            this.isEditingResourceName = true; // 进入编辑模式
            this.newResourceName = item.name; // 预填当前名称
            this.$nextTick(() => {
                // 自动聚焦到输入框
                const input = this.$el.querySelector(".name-input");
                input && input.focus();
            });
        },
        async saveResourceName(resourceId) {
            if (this.newResourceName.trim()) {
                this.newResourceName = this.newResourceName.trim(); // 保存修改后的名称
            }
            await invoke("modify_resource_name", { resourceId: resourceId, name: this.newResourceName });
            this.isEditingResourceName = false; // 退出编辑模式
            this.$emit("launcher-updated", this.data.id);
        },
        async addRow(directory) {
            try {
                const filePath = await open({
                    multiple: false, // 禁止多选
                    directory: directory, // 选择文件而不是文件夹
                });

                if (filePath) {
                    // 调用后端存储路径的命令
                    await invoke("add_resource", { launcherId: this.data.id, path: filePath });
                    this.$emit("launcher-updated", this.data.id);
                }
            } catch (error) {
                console.error("文件选择错误:", error);
            }
        },
        async deleteRow(resourceId) {
            await invoke("delete_resource",{resourceId: resourceId})
            this.$emit("launcher-updated", this.data.id);
        },
        async deleteLauncher() {
            const userConfirmed = await confirm(
                "您确定要删除这一行吗？此操作无法撤销。",
                { title: "确认删除", type: "question" }
            );
            if (userConfirmed) {
                 await invoke("delete_launcher",{"launcherId":this.data.id});
                 this.$emit("launcher-updated", this.data.id);
            }
        },
        async copyLauncher() {
            await invoke("copy_launcher",{launcherId: this.data.id});
            this.$emit("launcher-updated", this.data.id);
        },
        async launch() {
            this.isLaunching = true;
            await invoke("launch", { launcherId: this.data.id });
            toast.success("启动成功！所有内容已激活！");
            this.isLaunching = false;
        },
        showAddUrlDialog() {
            this.showDialog = true; // 打开添加网址的对话框
            this.dropdownVisible = false; // 关闭下拉菜单
        },
        async addUrl() {
            if (this.addUrlName && this.addUrlContent) {
                await invoke("add_resource", { launcherId: this.data.id, name: this.addUrlName, path: this.addUrlContent });
                this.$emit("launcher-updated", this.data.id);
                await this.closeDialog();
            } else {
                await message("请输入名称和网址！");
            }
        },
        async closeDialog() {
            this.showDialog = false; // 关闭对话框
            this.addUrlName = "";
            this.addUrlContent = "";
        },
        moveLauncher(type){
            this.$emit("launcher-moved", this.data.id, type);
        },
        debounce(func, delay) {
            let timer;
            return function (...args) {
                if (timer) clearTimeout(timer);
                timer = setTimeout(() => func.apply(this, args), delay);
            };
        },
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
    gap: 20px;
    /* 给按钮添加间距 */
}

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
    overflow: hidden;
    /* 隐藏溢出内容 */
    text-overflow: ellipsis;
    /* 超出部分显示省略号 */
    white-space: nowrap;
    /* 不换行 */
}

.delete-button,
.delete-launcher {
    background-color: transparent;
    /* 无背景色 */
    color: #BFBFBF;
    /* 默认蓝色 */
    border: none;
    width: 30px;
    /* 固定按钮大小 */
    height: 30px;
    border-radius: 50%;
    /* 圆形按钮 */
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 0;
    transition: color 0.2s ease;
    /* 平滑过渡 */
}

.delete-button:hover,
.delete-launcher:hover {
    color: #dc3545;
    /* 鼠标悬浮时变为红色 */
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
    margin-top: 10px;
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

/* 按钮容器 */
.button-container {
    display: flex;
    gap: 10px;
    /* 设置按钮之间的间距 */
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