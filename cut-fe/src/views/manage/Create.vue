<template>
  <div class="create">
    <v-row class="mb-8" align="center">
      <v-col cols="12" sm="">
        <h1 class="text-h2">
          Cuts
        </h1>
      </v-col>
    </v-row>
    <v-row>
      <v-col cols="12">
        MOCK CREATE
      </v-col>
    </v-row>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import { authManager } from "@/auth";

export default Vue.extend({
  computed: {
    user: () => authManager.getUser()
  }
});
</script>
