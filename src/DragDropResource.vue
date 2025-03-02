<template>
    <div class="container">
        <h2 class="title">添加资源</h2>

        <div class="m-4" style="display: flex; justify-content: space-between;align-items: center;">
            <div style="display: flex;align-items: center;">
                <p style="margin-right: 10px;">资源:</p>
            </div>
            <el-select append-to=".home" v-model="selectPathList" style="width: 240px;" multiple collapse-tags
                collapse-tags-tooltip placeholder="Select">
                <el-option v-for="item in pathList" :key="item" :label="item" :value="item">
                </el-option>
            </el-select>
        </div>

        <div class="m-4" style="display: flex; justify-content: space-between;align-items: center;">
            <div style="display: flex;align-items: center;">
                <p style="margin-right: 10px;">保存至:</p>
            </div>
            <div style="display: flex;align-items: center; gap: 10px;">

                <el-tooltip v-if="isNewLauncher" content="取消新建" placement="top">
                    <Close theme="outline" size="20" fill="#a19797" @click="handleNewLauncher" />
                </el-tooltip>
                <el-tooltip v-else content="新建编组" placement="top">
                    <Add theme="outline" size="20" fill="#a19797" @click="handleNewLauncher" />
                </el-tooltip>

                <el-select v-if="!isNewLauncher" append-to=".home" v-model="targetLauncher" style="width: 240px;"
                    collapse-tags collapse-tags-tooltip placeholder="Select">
                    <el-option v-for="item in launchers" :key="item.id" :label="item.name" :value="item.id">
                    </el-option>
                </el-select>
                <el-input v-else v-model="newLauncherName" style="width: 240px;" placeholder="请输入新编组名称"></el-input>
            </div>
        </div>

        <div style="display: flex; justify-content: flex-end;gap:10px;align-items: center; margin-top: 20px;">
            <button class="cancel-button" @click="cancelDragDrop">取消</button>
            <button class="confirm-button" @click="confirmDragDrop">确定</button>
        </div>

    </div>
</template>

<script>
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';
import { invoke } from "@tauri-apps/api/core";
import { onMounted, inject, ref, nextTick } from 'vue';
import { Help, Add, Close } from '@icon-park/vue-next';

export default {
    props: {
        pathList: {
            type: Array,
            required: true,
        },
    },
    components: {
        Help,
        Add,
        Close
    },
    setup(props, { emit }) {
        const theme = inject('theme');
        const selectPathList = ref(props.pathList);
        const launchers = ref([]);
        const targetLauncher = ref();
        const isNewLauncher = ref(false);
        const newLauncherName = ref("");

        const queryLaunchers = async () => {
            const launchers_data = await invoke("query_launchers");
            launchers.value = [...launchers_data];
        };

        const initTargetLauncher = async () => {
            if (launchers.value.length > 0) {
                targetLauncher.value = launchers.value[0].id;
            } else if (launchers.value.length == 0) {
                isNewLauncher.value = true;
            }
        };

        const handleNewLauncher = async () => {
            isNewLauncher.value = !isNewLauncher.value;

        }

        const cancelDragDrop = async () => {
            emit("cancel_drag_drop");
        }

        const confirmDragDrop = async () => {
            let launcherId;
            if (isNewLauncher.value) {
                launcherId = await invoke("craete_launcher", { "name": newLauncherName.value });
            } else {
                launcherId = targetLauncher.value;
            }

            await invoke("add_resources", { "launcherId": launcherId, "resources": selectPathList.value.map(path => ({ "path": path })) });

            emit("confirm_drag_drop");
        }

        onMounted(() => {
            queryLaunchers().then(() => {
                initTargetLauncher();
            })
        });

        return {
            selectPathList,
            launchers,
            targetLauncher,
            isNewLauncher,
            handleNewLauncher,
            newLauncherName,
            cancelDragDrop,
            confirmDragDrop
        };
    }
};
</script>

<style scoped>
.container {
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


.title {
    margin-top: 0px;
}

label {
    margin-right: 10px;
}

button {
    padding: 10px 20px;
    background-color: #409eff;
    ;
    color: white;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    border-color: #409eff;
}

button:hover {
    background-color: #0056b3;
}

.cancel-button {
    background-color: #fff;
    /* 背景色为白色 */
    border-color: #ccc;
    /* 边框颜色为浅灰色 */
    color: #666;
    /* 字体颜色为深灰色 */
}

.cancel-button:hover {
    background-color: #f5f5f5;
    /* 悬停时背景色为浅灰色 */
    border-color: #bbb;
    /* 悬停时边框颜色为灰色 */
    color: #333;
    /* 悬停时字体颜色为更深的灰色 */
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

.confirm-button {
    background-color: #409eff;
    border-color: #409eff;
    color: #fff;
}

.confirm-button:hover {
    background-color: #66b1ff;
    border-color: #66b1ff;
}
</style>