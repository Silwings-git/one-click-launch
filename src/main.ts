import { createApp } from "vue";
import { createPinia } from 'pinia';
import ElementPlus from 'element-plus'
import App from "./App.vue";
import '@fortawesome/fontawesome-free/css/all.css';
import '@fortawesome/fontawesome-free/js/all.js';
import '@icon-park/vue-next/styles/index.css';

import Toast, { PluginOptions, POSITION } from "vue-toastification";
import "vue-toastification/dist/index.css";

const options: PluginOptions = {
    position: POSITION.TOP_CENTER, // 设置Toast的位置
    timeout: 3000, // 自动关闭的时间，单位为毫秒
    closeOnClick: true, // 是否点击关闭Toast
    pauseOnFocusLoss: false, // 当窗口失去焦点时暂停倒计时
    pauseOnHover: true, // 鼠标悬停时暂停倒计时
    draggable: true, // 是否可以拖动Toast
    draggablePercent: 0.6, // 拖动百分比
    showCloseButtonOnHover: true, // 是否在鼠标悬停时显示关闭按钮
    hideProgressBar: false, // 是否隐藏进度条
    closeButton: 'button', // 关闭按钮类型
    icon: true, // 是否显示图标
    rtl: false, // 是否支持从右向左的语言
};

const app = createApp(App);

app.use(Toast, options);
app.use(createPinia());
app.use(ElementPlus);

app.mount("#app");
