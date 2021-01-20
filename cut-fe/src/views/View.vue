<template>
  <div class="view">
    <v-fade-transition>
      <div v-show="pageLoadStatus === STATUS.COMPLETE">
        <v-row class="mb-8" align="center">
          <v-col cols="12" sm="">
            <h1 class="text-h2">
              <div v-if="variant === 'snippet'">
                {{ snippet.name }}
              </div>
              <div v-if="variant === 'url'">
                Redirecting...
              </div>
              <div v-if="variant === 'file'">
                Download File
              </div>
            </h1>
          </v-col>
        </v-row>
        <v-row>
          <v-col cols="12">
            <v-card elevation="8">
              <v-card-text class="px-0">
                <div v-if="variant === 'snippet'">
                  <v-col class="px-6">
                    <v-row>
                      <v-col>
                        <prism-editor
                          v-model="snippet.data"
                          :highlight="highlighter"
                          line-numbers
                          readonly
                          class="snippet-editor rounded"
                        />
                      </v-col>
                    </v-row>
                  </v-col>
                </div>
                <div v-if="variant === 'url'">
                  <v-col class="px-4">
                    <v-row>
                      <v-col cols="12">
                        <h2 class="text-button text-center text--disabled">
                          Redirecting you to
                        </h2>
                        <p class="text-h6 text-center">
                          <a :href="url.target">{{ url.target }}</a>
                        </p>
                      </v-col>
                    </v-row>
                  </v-col>
                </div>
                <div v-if="variant === 'file'">
                  <v-col class="px-4">
                    <v-row>
                      FILE VIEWER
                    </v-row>
                  </v-col>
                </div>
              </v-card-text>
            </v-card>
          </v-col>
        </v-row>
      </div>
    </v-fade-transition>
    <v-fade-transition>
      <v-overlay
        v-show="
          pageLoadStatus === STATUS.PRE_LOADING ||
            pageLoadStatus === STATUS.LOADING
        "
        opacity="0"
        absolute
      >
        <v-progress-circular indeterminate size="64" />
      </v-overlay>
    </v-fade-transition>
    <v-expand-transition>
      <div v-show="pageLoadStatus === STATUS.ERROR">
        <v-alert
          type="error"
          text
          transition="scroll-y-transition"
          class="mt-0"
        >
          Cut not found!
        </v-alert>
      </div>
    </v-expand-transition>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import { STATUS } from "@/constants/status";
import { languages } from "@/constants/languages";
import "@/styles/Create.sass";

import Prism from "prismjs";
import "@/styles/prism-atom-dark.css";
import api from "@/apis/api";

export default Vue.extend({
  data() {
    return {
      variant: "",
      snippet: {
        name: "",
        language: "Plaintext",
        languageSelect: languages,
        data: ""
      },
      url: {
        target: ""
      },
      pageLoadStatus: STATUS.IDLE,
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

  created() {
    this.getCut(this.$route.params.hash || "");
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
    getCut(hash: string) {
      this.pageLoadStatus = STATUS.LOADING;
      api.cut
        .get(hash)
        .then(response => {
          const data = response.data;
          const metadata = JSON.parse(data.metadata);
          this.variant = data.variant;
          switch (response.data.variant) {
            case "snippet":
              this.snippet.name = data.name;
              this.snippet.data = data.data;
              this.snippet.language = metadata.language;
              break;
            case "url":
              this.url.target = data.data;
              setTimeout(() => (window.location.href = data.data), 2000);
              break;
            default:
              this.pageLoadStatus = STATUS.ERROR;
              return;
          }
          this.pageLoadStatus = STATUS.COMPLETE;
        })
        .catch(err => {
          this.pageLoadStatus = STATUS.ERROR;
          console.error(err);
        });
    }
  }
});
</script>
