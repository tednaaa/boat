import "@fontsource/roboto/400.css";
import "@fontsource/roboto/500.css";
import "@fontsource/roboto/700.css";

import "@/assets/css/main.css";

import { createApp } from "vue";
import App from "./App.vue";

import PrimeVue from "primevue/config";
import { router } from "@/views/router";

const app = createApp(App);

app.use(PrimeVue, { unstyled: true });

app.use(router);

app.mount("#app");
