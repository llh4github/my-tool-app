import {createRouter, createWebHashHistory, createWebHistory, type RouteRecordRaw} from "vue-router"

const Layout = () => import("@/layout/index.vue")
export const constantRoutes: RouteRecordRaw[] = [

]
const router = createRouter({
    history:
        import.meta.env.VITE_ROUTER_HISTORY === "hash"
            ? createWebHashHistory(import.meta.env.VITE_PUBLIC_PATH)
            : createWebHistory(import.meta.env.VITE_PUBLIC_PATH),
    routes: constantRoutes,
})

export default router
