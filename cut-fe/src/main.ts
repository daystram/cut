import Vue from "vue";
import App from "./App.vue";
import "./registerServiceWorker";
import router from "./router";
import store from "./store";
import vuetify from "./plugins/vuetify";
import "./plugins/prismjs";
import { StatusMixin } from "@/constants/status";

import "@/styles/App.sass";

Vue.config.productionTip = false;
Vue.mixin(StatusMixin);
new Vue({
  router,
  store,
  vuetify,
  render: h => h(App)
}).$mount("#app");
