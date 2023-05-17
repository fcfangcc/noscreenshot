import Screenshot from "@/views/Screenshot.vue";
import * as VueRouter from "vue-router";

const routes = [
  { path: "/", name: "Home", component: () => import("@/views/Home.vue") },
  { path: "/screenshot", name: "Screenshot", component: Screenshot },
];

const router = VueRouter.createRouter({
  history: VueRouter.createWebHistory(),
  routes,
});

export default router;