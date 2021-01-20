<template>
  <div class="manage fill-height">
    <v-app-bar app clipped-left>
      <v-app-bar-nav-icon
        v-if="user"
        class="hidden-lg-and-up"
        @click.stop="drawer = !drawer"
      />
      <h1>
        <router-link
          class="text-md-h4 text-h5 text-center"
          style="text-decoration: none; color: inherit;"
          :to="user ? { name: 'manage:create' } : { name: 'home' }"
        >
          <Logo />
        </router-link>
      </h1>
      <v-spacer />
      <v-menu
        v-if="user"
        right
        nudge-bottom="12px"
        offset-y
        min-width="280px"
        max-width="280px"
      >
        <template v-slot:activator="{ on, attrs }">
          <v-avatar
            color="primaryDim"
            size="32"
            v-on="on"
            v-bind="attrs"
            style="user-select: none"
          >
            {{ user.given_name[0] + user.family_name[0] }}
          </v-avatar>
        </template>
        <v-list rounded>
          <v-list-item-group color="primary">
            <v-list-item two-line disabled>
              <v-list-item-avatar color="primaryDim" size="48">
                <div class="text-center flex-fill text--primary">
                  {{ user.given_name[0] + user.family_name[0] }}
                </div>
              </v-list-item-avatar>
              <v-list-item-content>
                <v-list-item-title class="text--primary">
                  {{ user.given_name + " " + user.family_name }}
                </v-list-item-title>
                <v-list-item-subtitle>
                  {{ user.preferred_username }}
                </v-list-item-subtitle>
              </v-list-item-content>
            </v-list-item>
            <v-list-item :to="{ name: 'logout' }" dense>
              <v-list-item-icon>
                <v-icon color="error">mdi-logout-variant</v-icon>
              </v-list-item-icon>
              <v-list-item-content>
                <v-list-item-title class="error--text">
                  Logout
                </v-list-item-title>
              </v-list-item-content>
            </v-list-item>
          </v-list-item-group>
        </v-list>
      </v-menu>
      <v-btn
        v-else
        :to="{ name: 'login' }"
        elevation="6"
        rounded
        v-text="'Sign In'"
        color="primary darken-2"
      />
    </v-app-bar>
    <v-navigation-drawer v-if="user" app clipped v-model="drawer">
      <v-list nav dense rounded>
        <v-list-item-group color="primary">
          <v-list-item :to="{ name: 'manage:create' }">
            <v-list-item-icon>
              <v-icon v-text="'mdi-pencil'" />
            </v-list-item-icon>
            <v-list-item-title v-text="'Create Cut'" />
          </v-list-item>
          <v-list-item :to="{ name: 'manage:list' }">
            <v-list-item-icon>
              <v-icon v-text="'mdi-clipboard-list-outline'" />
            </v-list-item-icon>
            <v-list-item-title v-text="'My Cuts'" />
          </v-list-item>
        </v-list-item-group>
      </v-list>
    </v-navigation-drawer>
    <v-main class="fill-height">
      <v-container
        class="mx-auto pa-4 pa-sm-6 pa-md-8"
        style="max-width: 960px"
        fluid
      >
        <router-view></router-view>
      </v-container>
    </v-main>
  </div>
</template>

<script>
import Vue from "vue";
import Logo from "@/components/Logo.vue";
import { authManager } from "@/auth";

export default Vue.extend({
  components: { Logo },

  data() {
    return {
      drawer: null
    };
  },

  computed: {
    user: () => authManager.getUser()
  }
});
</script>
