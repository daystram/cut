<template>
  <div class="create">
    <v-row class="mb-8" align="center">
      <v-col cols="12" sm="">
        <h1 class="text-h2">
          Create Cut
        </h1>
      </v-col>
    </v-row>
    <v-row>
      <v-col cols="12">
        <v-card elevation="8" :loading="formLoadStatus === STATUS.LOADING">
          <v-tabs v-model="variant" center-active grow>
            <v-tab key="snippet" :disabled="formLoadStatus === STATUS.LOADING">
              <v-icon v-text="'mdi-code-braces'" class="mr-2" />Snippet
            </v-tab>
            <v-tab key="url" :disabled="formLoadStatus === STATUS.LOADING">
              <v-icon v-text="'mdi-web'" class="mr-2" />URL
            </v-tab>
            <v-tab key="file" :disabled="formLoadStatus === STATUS.LOADING">
              <v-icon v-text="'mdi-file-upload'" class="mr-2" />File
            </v-tab>
          </v-tabs>
          <v-tabs-items v-model="variant">
            <v-tab-item key="snippet">
              <v-card elevation="0">
                <v-card-text>
                  <v-col>
                    <v-row>
                      <v-col cols="12" sm="8">
                        <v-text-field
                          label="Name"
                          filled
                          dense
                          counter
                          maxlength="50"
                          class="rounded"
                          background-color="#2d2d2d"
                        />
                      </v-col>
                      <v-col cols="12" sm="4">
                        <v-select
                          v-model="snippet.language"
                          :items="Object.keys(snippet.languageSelect)"
                          label="Language"
                          filled
                          dense
                          background-color="#2d2d2d"
                          class="rounded"
                          style="border: none !important"
                        />
                      </v-col>
                    </v-row>
                    <v-row>
                      <v-col>
                        <prism-editor
                          v-model="snippet.data"
                          :highlight="highlighter"
                          line-numbers
                          class="snippet-editor rounded"
                        />
                      </v-col>
                    </v-row>
                  </v-col>
                </v-card-text>
              </v-card>
            </v-tab-item>
            <v-tab-item key="url">
              <v-card flat>
                <v-card-text>URL EDITOR</v-card-text>
              </v-card>
            </v-tab-item>
            <v-tab-item key="file">
              <v-card flat>
                <v-card-text>FILE EDITOR</v-card-text>
              </v-card>
            </v-tab-item>
          </v-tabs-items>
          <v-card elevation="0">
            <v-card-text class="pt-0">
              <v-col>
                <v-btn block large color="primary darken-1">Create</v-btn>
              </v-col>
            </v-card-text>
          </v-card>
        </v-card>
      </v-col>
    </v-row>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import { authManager } from "@/auth";
import { STATUS } from "@/constants/status";
import { languages } from "@/constants/languages";
import "@/styles/Create.sass";

import Prism from "prismjs";
import "@/styles/prism-atom-dark.css";

export default Vue.extend({
  data() {
    return {
      variant: null,
      snippet: {
        name: "",
        language: "Plaintext",
        languageSelect: languages,
        data: ""
      },
      formLoadStatus: STATUS.IDLE
    };
  },

  computed: {
    user: () => authManager.getUser()
  },

  methods: {
    highlighter(code: string) {
      if (this.snippet.language === "Plaintext") return code;
      Prism.highlightAll();
      return Prism.highlight(
        code,
        (languages as any)[this.snippet.language].grammar,
        (languages as any)[this.snippet.language].language
      );
    }
  }
});
</script>
