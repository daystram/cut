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
          <v-card elevation="0">
            <v-card-text>
              <v-tabs-items v-model="variant">
                <v-tab-item key="snippet">
                  <v-col>
                    <v-row>
                      <v-col cols="12" sm="8">
                        <v-text-field
                          v-model.trim="snippet.name"
                          label="Name"
                          outlined
                          maxlength="50"
                          background-color="#2d2d2d"
                          hide-details="auto"
                          :error-messages="snippetNameErrors"
                          required
                          :disabled="formLoadStatus === STATUS.LOADING"
                          @blur="$v.snippet.name.$touch()"
                        />
                      </v-col>
                      <v-col cols="12" sm="4">
                        <v-select
                          v-model="snippet.language"
                          :items="Object.keys(snippet.languageSelect)"
                          label="Language"
                          outlined
                          background-color="#2d2d2d"
                          hide-details="auto"
                          style="border: none !important"
                          required
                          :disabled="formLoadStatus === STATUS.LOADING"
                        />
                      </v-col>
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
                </v-tab-item>
                <v-tab-item key="url">
                  <v-col>
                    <v-row>
                      <v-col cols="12">
                        <v-text-field
                          v-model.trim="url.target"
                          label="Target URL"
                          outlined
                          maxlength="512"
                          background-color="#2d2d2d"
                          hide-details="auto"
                          :error-messages="urlTargetErrors"
                          required
                          :disabled="formLoadStatus === STATUS.LOADING"
                          @blur="$v.url.target.$touch()"
                        />
                      </v-col>
                    </v-row>
                  </v-col>
                </v-tab-item>
                <v-tab-item key="file">
                  <v-col>
                    <v-row>
                      FILE EDITOR
                    </v-row>
                  </v-col>
                </v-tab-item>
              </v-tabs-items>
              <v-col>
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
                      outlined
                      dense
                      class="rounded code-field"
                      background-color="#2d2d2d"
                      append-icon="mdi-clipboard-text-multiple"
                      id="view-link"
                      @click:append="intoClipboard('view-link')"
                      readonly
                      persistent-hint
                      :hint="copiedTooltip.view ? 'Copied to clipboard!' : ''"
                    >
                    </v-text-field>
                  </div>
                  <div class="my-4">
                    To get view the raw cut, use the following.
                  </div>
                  <div class="mt-4">
                    <v-text-field
                      v-model="linkRaw"
                      outlined
                      dense
                      class="rounded code-field"
                      background-color="#2d2d2d"
                      append-icon="mdi-clipboard-text-multiple"
                      id="raw-link"
                      @click:append="intoClipboard('raw-link')"
                      readonly
                      persistent-hint
                      :hint="copiedTooltip.raw ? 'Copied to clipboard!' : ''"
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
import { maxLength, required, url } from "vuelidate/lib/validators";
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
      url: {
        target: ""
      },
      formLoadStatus: STATUS.IDLE,
      linkView: "",
      linkRaw: "",
      copiedTooltip: {
        view: false,
        raw: false,
        viewTimeout: 0,
        rawTimeout: 0
      }
    };
  },

  computed: {
    snippetNameErrors() {
      const errors: string[] = [];
      if (!this.$v.snippet.name?.$dirty) return errors;
      !this.$v.snippet.name.required && errors.push("Name required");
      !this.$v.snippet.name.maxLength && errors.push("Name too long");
      return errors;
    },
    urlTargetErrors() {
      const errors: string[] = [];
      if (!this.$v.url.target?.$dirty) return errors;
      !this.$v.url.target.required && errors.push("Target URL required");
      !this.$v.url.target.url && errors.push("Invalid URL");
      !this.$v.url.target.maxLength && errors.push("URL too long");
      return errors;
    }
  },

  validations: {
    snippet: {
      name: { required, maxLength: maxLength(50) }
    },
    url: {
      target: { required, url, maxLength: maxLength(512) }
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
        case 1:
          this.$v.url.$touch();
          if (this.$v.url.$invalid) return;
          this.formLoadStatus = STATUS.LOADING;
          console.log("Creating URL");
          api.cut
            .create({
              name: this.url.target,
              variant: "url",
              metadata: JSON.stringify({}),
              data: this.url.target
            })
            .then(response => {
              this.formLoadStatus = STATUS.COMPLETE;
              this.url.target = "";
              this.linkView = `${window.origin}/${response.data.hash}`;
              this.linkRaw = `${window.origin}/raw/${response.data.hash}`;
              this.$v.url.$reset();
            })
            .catch(err => {
              this.formLoadStatus = STATUS.ERROR;
              console.error(err);
            });
          break;
        default:
          this.formLoadStatus = STATUS.IDLE;
      }
    },
    intoClipboard(id: string) {
      const target: HTMLTextAreaElement | null = document.querySelector(
        `#${id}`
      );
      if (!target) return;
      target.select();
      document.execCommand("copy");
      if (id === "view-link") {
        this.copiedTooltip.view = true;
        clearTimeout(this.copiedTooltip.viewTimeout);
        this.copiedTooltip.viewTimeout = setTimeout(
          () => (this.copiedTooltip.view = false),
          5000
        );
      } else if (id === "raw-link") {
        this.copiedTooltip.raw = true;
        clearTimeout(this.copiedTooltip.rawTimeout);
        this.copiedTooltip.rawTimeout = setTimeout(
          () => (this.copiedTooltip.raw = false),
          5000
        );
      }
    }
  }
});
</script>
