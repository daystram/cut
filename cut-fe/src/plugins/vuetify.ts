import Vue from "vue";
import Vuetify from "vuetify/lib/framework";

Vue.use(Vuetify);

export default new Vuetify({
  theme: {
    dark: true,
    themes: {
      dark: {
        primary: "#00C777",
        primaryDim: "#237A57",
        secondary: "#C79930"
      }
    },
    options: { customProperties: true }
  },
  icons: {
    iconfont: "mdi"
  }
});
