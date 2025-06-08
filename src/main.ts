import { createApp } from "vue";
import App from "./App.vue";
import { createRouter, createWebHistory } from "vue-router";

// Import views to be used in the router
import AnalyseMailView from "./Views/AnalyseMailView.vue";
import Connexion from "./Views/connexion.vue";

// Create routes
const routes = [
    { path: "/", component: AnalyseMailView },
    { path: "/user/connexion", component: Connexion },
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
