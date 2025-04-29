import { createApp } from "vue";
import App from "./App.vue";
import { createRouter, createWebHistory } from "vue-router";

// Import views to be used in the router
import MainView from "./Views/MainView.vue"
import AnalyseMailView from "./Views/AnalyseMailView.vue";

// Create routes
const routes = [
    { path: "/", component: MainView },
    { path: "/analyse", component: AnalyseMailView },
];

//create router
const router = createRouter(
    {
        history: createWebHistory(),
        routes,
    }
)

createApp(App)
    .use(router) // Use the router just created in the app
    .mount("#app");
