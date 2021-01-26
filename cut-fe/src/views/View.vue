<template>
  <div class="view">
    <v-fade-transition>
      <div v-show="pageLoadStatus === STATUS.COMPLETE">
        <v-row class="mb-2" align="center">
          <v-col cols="12" sm="">
            <h1 class="text-h2 text-truncate" v-if="variant === 'snippet'">
              {{ snippet.name }}
            </h1>
            <h1 class="text-h2" v-if="variant === 'url'">
              Redirecting...
            </h1>
            <h1 class="text-h2" v-if="variant === 'file'">
              Download File
            </h1>
            <div class="mx-n2 mt-2">
              <v-chip color="secondary" class="ma-2" outlined small>
                <v-avatar left>
                  <v-icon v-text="'mdi-calendar-edit'" small />
                </v-avatar>
                {{
                  Intl.DateTimeFormat("default", {
                    year: "numeric",
                    month: "long",
                    day: "numeric",
                    hour: "numeric",
                    minute: "numeric",
                    second: "numeric"
                  }).format(createdAt)
                }}
              </v-chip>
              <v-chip color="primary" class="ma-2" outlined small>
                <v-avatar left>
                  <v-icon v-text="'mdi-eye'" small />
                </v-avatar>
                {{ views }}
              </v-chip>
              <v-chip
                v-if="variant === 'snippet'"
                color="accent"
                class="ma-2"
                outlined
                small
              >
                <v-avatar left>
                  <v-icon v-text="'mdi-code-braces'" small />
                </v-avatar>
                {{ snippet.language }}
              </v-chip>
            </div>
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
                          :highlight="highlighter(snippet.language)"
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
import api from "@/apis/api";
import { STATUS } from "@/constants/status";
import { highlighter } from "@/utils/highlighter";

import "@/styles/Create.sass";
import "@/styles/prism-atom-dark.css";

export default Vue.extend({
  data() {
    return {
      variant: "",
      createdAt: new Date(),
      views: 0,
      snippet: {
        name: "",
        language: "Plaintext",
        data: ""
      },
      url: {
        target: ""
      },
      pageLoadStatus: STATUS.LOADING
    };
  },

  created() {
    this.getCut(this.$route.params.hash || "");
  },

  methods: {
    getCut(hash: string) {
      this.pageLoadStatus = STATUS.LOADING;
      api.cut
        .get(hash)
        .then(response => {
          const data = response.data;
          const metadata = JSON.parse(data.metadata);
          this.variant = data.variant;
          this.views = data.views;
          this.createdAt = new Date(data.created_at * 1000);
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
        });
    },
    highlighter: (language: string) => highlighter(language)
  }
});
</script>
