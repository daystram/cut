import Vue from "vue";
import VueRouter, { RouteConfig } from "vue-router";
import Home from "../views/Home.vue";
import {
  authenticatedOnly,
  callback,
  login,
  logout,
  unAuthenticatedOnly
} from "@/auth";
import { Manage, Create } from "@/views";

Vue.use(VueRouter);

const routes: Array<RouteConfig> = [
  {
    path: "/",
    name: "home",
    component: Home,
    beforeEnter: unAuthenticatedOnly,
    meta: {
      title: "Cut"
    }
  },
  {
    path: "/",
    beforeEnter: authenticatedOnly,
    component: Manage,
    redirect: "/create",
    children: [
      {
        path: "create",
        name: "manage:create",
        component: Create,
        meta: {
          title: "Create | Cut"
        }
      }
    ]
  },
  {
    path: "/login",
    name: "login",
    beforeEnter: unAuthenticatedOnly,
    component: login
  },
  {
    path: "/logout",
    name: "logout",
    beforeEnter: authenticatedOnly,
    component: logout
  },
  {
    path: "/callback",
    name: "callback",
    beforeEnter: unAuthenticatedOnly,
    component: callback
  },
  { path: "*", redirect: { name: "home", query: {} } }
];

const router = new VueRouter({
  mode: "history",
  base: process.env.BASE_URL,
  routes
});

router.beforeEach((to, from, next) => {
  document.title = to.meta.title || "Cut";
  next();
});

export default router;
