import { createApp } from "vue";
import App from "./App.vue";
import { createRouter, createWebHistory } from "vue-router";
import { createPinia } from "pinia";

// Import views to be used in the router
import AnalyseMailView from "./Views/AnalyseMailView.vue";
import Connexion from "./Views/connexion.vue";
import UITest from "./Views/UITest.vue";

// Create routes
const routes = [
    { path: "/", component: AnalyseMailView },
    { path: "/user/connexion", component: Connexion },
    { path: "/uitest", component: UITest },
];

//create router
const router = createRouter(
    {
        history: createWebHistory(),
        routes,
    }
)

const pinia = createPinia();

createApp(App)
    .use(router) // Use the router just created in the app
    .use(pinia)
    .mount("#app");
