<template>
  <v-container fill-height fluid class="gradient-bg">
    <v-col>
      <v-row>
        <v-col>
          <h1 class="text-sm-h1 text-h3 text-center">
            <Logo />
          </h1>
          <p
            class="mt-12 mb-12 text-subtitle-1 ma-auto text-center app-subtitle"
          >
            URL, snippet, and file sharing with ease.
          </p>
        </v-col>
      </v-row>
      <v-row align="center" justify="center">
        <v-btn
          :to="{ name: 'login' }"
          elevation="6"
          rounded
          x-large
          class="ma-2"
          v-text="'Sign In'"
          color="primary darken-2"
        />
      </v-row>
      <v-row align="center" justify="center">
        <v-btn
          href="https://github.com/daystram/cut"
          x-large
          class="mt-12"
          text
          plain
          rounded
        >
          View on GitHub <v-icon class="ml-1" v-text="'mdi-github'" />
        </v-btn>
      </v-row>
    </v-col>
  </v-container>
</template>

<script lang="ts">
import Vue from "vue";
import Logo from "@/components/Logo.vue";
import { authManager } from "@/auth";

export default Vue.extend({
  components: { Logo },

  computed: {
    isAuthenticated() {
      return authManager.isAuthenticated();
    }
  }
});
</script>
