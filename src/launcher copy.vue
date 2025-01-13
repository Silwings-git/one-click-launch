<template>
    <div class="launcher">
        <div class="header">
            <span class="name">启动器名称</span>
            <div class="button-container">
                <button class="copy-button" @click="copyName">复制</button>
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
        <div class="content">
            <div class="add-row" @click="addRow">+ 添加</div>
            <div class="data-row" v-for="(item, index) in data" :key="index">
                <input class="data-name" type="text" :value="item.name" @input="updateName(index, $event.target.value)"
                    @blur="onNameEditComplete(index)" />
                <span class="data-content">{{ item.content }}</span>
                <button class="delete-button" @click="deleteRow(index)">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="16"
                        height="16">
                        <path
                            d="M9 3v1H4v2h16V4h-5V3H9zM5 7v12c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V7H5zm4 2h2v8H9V9zm4 0h2v8h-2V9z" />
                    </svg>
                </button>
            </div>
        </div>
        <button class="launch-button" @click="launch">启动</button>
    </div>
</template>

<script>
import { confirm, message } from "@tauri-apps/plugin-dialog";
export default {
    data() {
        return {
            data: [
                { name: "示例 1", content: "这是浮可见完整内容", fullContent: "示例 1: 这是示例数据，可能会很长，鼠标悬浮可见完整内容" },
                { name: "示例 2", content: "短内容", fullContent: "示例 2: 短内容" },
                { name: "示例 3", content: "短内容", fullContent: "示例 2: 短内容" },
                { name: "示例 4", content: "短内容", fullContent: "示例 2: 短内容" },
                { name: "示例 5", content: "短内容", fullContent: "示例 2: 短内容" },
                { name: "示例 6", content: "短内容", fullContent: "示例 2: 短内容" },
                { name: "示例 7", content: "短内容", fullContent: "示例 2: 短内容" },
                { name: "示例 8", content: "短内容", fullContent: "示例 2: 短内容" },
                { name: "示例 9", content: "短内容", fullContent: "示例 2: 短内容" },
            ], // 初始数据
        };
    },
    methods: {
        addRow() {
            const newIndex = this.data.length + 1;
            const newItem = {
                name: `新数据 ${newIndex}`,
                content: `这是新数据 ${newIndex} 的详细信息，鼠标悬浮可见完整内容`,
                fullContent: `新数据 ${newIndex}: 这是新数据 ${newIndex} 的详细信息，鼠标悬浮可见完整内容`,
            };
            this.data.push(newItem);
        },
        async deleteRow(index) {
            this.data.splice(index, 1);
        },
        async deleteLauncher() {
            const userConfirmed = await confirm(
                "您确定要删除这一行吗？此操作无法撤销。",
                { title: "确认删除", type: "question" }
            );
            if (userConfirmed) {
                console.log("???");
            }
        },
        copyName() {
            const name = "启动器名称";
            navigator.clipboard.writeText(name).then(() => {
                alert("名称已复制！");
            });
        },
        async launch() {
            await message("启动成功！所有内容已激活！", {
                title: "启动通知", // 自定义弹窗标题
                type: "error", // 可选值：info, warning, error
            });
        },
        updateName(index, newName) {
            this.data[index].name = newName;
        },
        onNameEditComplete(index) {
            console.log(`行 ${index + 1} 的名称修改完成：${this.data[index].name}`);
            // 调用你的自定义逻辑
        },
    },
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
    font-size: 16px;
    color: #007bff;
    cursor: pointer;
    margin-bottom: 10px;
}

.add-row:hover {
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
    font-size: 14px;
}

.delete-button,
.delete-launcher {
    background-color: transparent;
    /* 无背景色 */
    color: #007bff;
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


.data-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 5px;
    padding: 5px;
    border: 1px solid #ddd;
    border-radius: 4px;
}

.data-name {
    width: 100px;
    margin-right: 10px;
    padding: 3px;
    border: 1px solid #ccc;
    border-radius: 4px;
}

.data-content {
    flex: 1;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
}
</style>