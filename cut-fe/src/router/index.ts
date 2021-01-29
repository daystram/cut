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
import { Manage, Create, View, List } from "@/views";

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
    component: Manage,
    children: [
      {
        path: "create",
        name: "manage:create",
        beforeEnter: authenticatedOnly,
        component: Create,
        meta: {
          title: "Create | Cut"
        }
      },
      {
        path: "list",
        name: "manage:list",
        beforeEnter: authenticatedOnly,
        component: List,
        meta: {
          title: "My Cuts | Cut"
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
  {
    path: "*",
    component: Manage,
    children: [
      {
        path: "/:hash",
        name: "view",
        component: View,
        meta: {
          title: "Cut"
        }
      },
      {
        path: "*",
        redirect: { name: "home" }
      }
    ]
  }
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
