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
                          v-model.trim="snippet.name"
                          label="Name"
                          filled
                          dense
                          maxlength="50"
                          class="rounded"
                          background-color="#2d2d2d"
                          hide-details="auto"
                          :error-messages="snippetNameErrors"
                          required
                          :disabled="formLoadStatus === STATUS.LOADING"
                          @input="$v.snippet.name.$touch()"
                          @blur="$v.snippet.name.$touch()"
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
                          required
                          :disabled="formLoadStatus === STATUS.LOADING"
                        />
                      </v-col>
                    </v-row>
                    <v-row>
                      <v-col class="py-0">
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
              <v-col class="pt-2">
                <v-expand-transition>
                  <div v-if="formLoadStatus === STATUS.ERROR">
                    <v-alert type="error" text class="mb-6">
                      Failed creating cut!
                    </v-alert>
                  </div>
                </v-expand-transition>
                <v-btn
                  block
                  large
                  color="primary darken-1"
                  :disabled="formLoadStatus === STATUS.LOADING"
                  :loading="formLoadStatus === STATUS.LOADING"
                  @click="create"
                >
                  Create
                </v-btn>
              </v-col>
            </v-card-text>
          </v-card>
        </v-card>
        <v-dialog
          :value="formLoadStatus === STATUS.COMPLETE"
          width="500"
          persistent
        >
          <v-card>
            <v-card-title>
              <v-row no-gutters align="center">
                <v-col cols="auto">
                  Cut Created
                </v-col>
                <v-spacer />
                <v-col cols="auto">
                  <v-btn
                    text
                    icon
                    color="grey"
                    @click="() => (formLoadStatus = STATUS.IDLE)"
                  >
                    <v-icon v-text="'mdi-close'" />
                  </v-btn>
                </v-col>
              </v-row>
            </v-card-title>
            <v-divider inset />
            <div class="v-card__body">
              <v-row align="center">
                <v-col>
                  <div class="mb-4">
                    Use the following link to share your cut.
                  </div>
                  <div class="mt-4">
                    <v-text-field
                      v-model="linkView"
                      filled
                      dense
                      class="rounded code-field"
                      background-color="#2d2d2d"
                      append-icon="mdi-clipboard-text-multiple"
                      readonly
                      hide-details="auto"
                    />
                  </div>
                  <div class="my-4">
                    To get view the raw cut, use the following.
                  </div>
                  <div class="mt-4">
                    <v-text-field
                      v-model="linkRaw"
                      filled
                      dense
                      class="rounded code-field"
                      background-color="#2d2d2d"
                      append-icon="mdi-clipboard-text-multiple"
                      readonly
                      hide-details="auto"
                    />
                  </div>
                </v-col>
              </v-row>
            </div>
          </v-card>
        </v-dialog>
      </v-col>
    </v-row>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import { STATUS } from "@/constants/status";
import { languages } from "@/constants/languages";
import { maxLength, required } from "vuelidate/lib/validators";
import "@/styles/Create.sass";

import Prism from "prismjs";
import "@/styles/prism-atom-dark.css";
import api from "@/apis/api";

export default Vue.extend({
  data() {
    return {
      variant: 0,
      snippet: {
        name: "",
        language: "Plaintext",
        languageSelect: languages,
        data: ""
      },
      formLoadStatus: STATUS.IDLE,
      linkView: "",
      linkRaw: ""
    };
  },

  computed: {
    snippetNameErrors() {
      const errors: string[] = [];
      if (!this.$v.snippet.name?.$dirty) return errors;
      !this.$v.snippet.name.required && errors.push("Name required");
      !this.$v.snippet.name.maxLength && errors.push("Name too long");
      return errors;
    }
  },

  validations: {
    snippet: {
      name: { required, maxLength: maxLength(50) }
    }
  },

  methods: {
    highlighter(code: string): string {
      if (this.snippet.language === "Plaintext") return code;
      Prism.highlightAll();
      return Prism.highlight(
        code,
        languages[this.snippet.language].grammar,
        languages[this.snippet.language].language
      );
    },
    create() {
      switch (this.variant) {
        case 0:
          this.$v.snippet.$touch();
          if (this.$v.snippet.$invalid) return;
          this.formLoadStatus = STATUS.LOADING;
          console.log("Creating SNIPPET");
          api.cut
            .create({
              name: this.snippet.name,
              variant: "snippet",
              metadata: JSON.stringify({ language: this.snippet.language }),
              data: this.snippet.data
            })
            .then(response => {
              this.formLoadStatus = STATUS.COMPLETE;
              this.snippet.name = "";
              this.snippet.data = "";
              this.linkView = `${window.origin}/${response.data.hash}`;
              this.linkRaw = `${window.origin}/raw/${response.data.hash}`;
              this.$v.snippet.$reset();
            })
            .catch(err => {
              this.formLoadStatus = STATUS.ERROR;
              console.error(err);
            });
          break;
        default:
          this.formLoadStatus = STATUS.IDLE;
      }
    }
  }
});
</script>
