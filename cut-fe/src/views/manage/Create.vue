<template>
  <div class="create">
    <v-row class="mb-8" align="center">
      <v-col cols="12" sm="">
        <h1 class="text-h2">
          Create
        </h1>
      </v-col>
    </v-row>
    <v-row>
      <v-col cols="12">
        <v-card elevation="8" :loading="formLoadStatus === STATUS.LOADING">
          <template v-slot:progress>
            <v-progress-linear
              :value="formLoadProgress"
              :indeterminate="variant !== 2"
              absolute
              top
              color="primary"
            />
          </template>
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
            <v-card-text class="px-0">
              <v-tabs-items v-model="variant">
                <v-tab-item key="snippet">
                  <v-col class="px-6">
                    <v-row>
                      <v-col cols="12" sm="8">
                        <v-text-field
                          v-model.trim="snippet.name"
                          label="Name"
                          outlined
                          dense
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
                          dense
                          background-color="#2d2d2d"
                          hide-details="auto"
                          style="border: none !important"
                          required
                          :disabled="formLoadStatus === STATUS.LOADING"
                        />
                      </v-col>
                      <v-col>
                        <v-expand-transition>
                          <div v-if="snippet.emptyError">
                            <v-alert type="error" text class="mb-6" dense>
                              Snippet may not be empty!
                            </v-alert>
                          </div>
                        </v-expand-transition>
                        <div>
                          <prism-editor
                            v-model="snippet.data"
                            :highlight="highlighter(snippet.language)"
                            line-numbers
                            class="snippet-editor rounded"
                            @input="() => (snippet.emptyError = false)"
                          />
                        </div>
                      </v-col>
                    </v-row>
                  </v-col>
                </v-tab-item>
                <v-tab-item key="url">
                  <v-col class="px-6">
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
                  <v-col class="px-6">
                    <v-row>
                      <v-col cols="12">
                        <v-expand-transition>
                          <div v-if="file.emptyError">
                            <v-alert type="error" text class="mb-6" dense>
                              File required!
                            </v-alert>
                          </div>
                        </v-expand-transition>
                        <v-expand-transition>
                          <div v-if="file.files.length && !fileValid()">
                            <v-alert type="error" text class="mb-6" dense>
                              File too large! Maximum size is
                              {{ formatUnit(file.maxSize) }}.
                            </v-alert>
                          </div>
                        </v-expand-transition>
                        <div id="file-drop-area" class="rounded pa-2">
                          <div
                            :class="
                              `py-16 rounded ${
                                $refs.upload && $refs.upload.dropActive
                                  ? 'file-drop-active'
                                  : 'file-drop-inactive'
                              }`
                            "
                          >
                            <div v-if="!file.files.length" class="text-center">
                              <h3 class="text-button text-light">
                                Drop File
                              </h3>
                              <div
                                class="text-subtitle-2 font-weight-light mb-4"
                              >
                                or
                              </div>
                              <file-upload
                                :multiple="false"
                                :directory="false"
                                :thread="4"
                                drop="#file-drop-area"
                                :drop-directory="false"
                                v-model="file.files"
                                ref="upload"
                                @input="() => (file.emptyError = false)"
                              >
                                <v-btn
                                  block
                                  outlined
                                  style="height: 40px"
                                  color="primary"
                                >
                                  Select File
                                </v-btn>
                              </file-upload>
                            </div>
                            <div v-else class="text-center">
                              <v-icon
                                v-text="'mdi-file'"
                                class="mt-6 mb-2 text-h1"
                              />
                              <h3 class="text-h6 font-weight-normal">
                                {{ file.files[0].file.name }}
                              </h3>
                              <div class="text-subtitle font-weight-light">
                                {{ formatUnit(file.files[0].file.size) }}
                              </div>
                              <v-btn
                                plain
                                color="error"
                                class="mt-4"
                                :disabled="formLoadStatus === STATUS.LOADING"
                                @click="fileReset"
                              >
                                Delete
                              </v-btn>
                            </div>
                          </div>
                        </div>
                      </v-col>
                    </v-row>
                  </v-col>
                </v-tab-item>
              </v-tabs-items>
              <v-col class="px-6">
                <v-expand-transition>
                  <div v-if="formLoadStatus === STATUS.ERROR">
                    <v-alert type="error" text class="mb-6">
                      Failed creating cut!
                    </v-alert>
                  </div>
                </v-expand-transition>
                <v-row>
                  <v-col cols="12" sm="4" md="3">
                    <v-select
                      v-model="expiry"
                      :items="Object.keys(expirySelect)"
                      label="Expiry"
                      outlined
                      dense
                      background-color="#2d2d2d"
                      hide-details="auto"
                      style="border: none !important"
                      required
                      :disabled="formLoadStatus === STATUS.LOADING"
                    />
                  </v-col>
                  <v-col cols="12" sm="8" md="9">
                    <v-btn
                      block
                      outlined
                      style="height: 40px"
                      color="primary"
                      :disabled="formLoadStatus === STATUS.LOADING"
                      :loading="formLoadStatus === STATUS.LOADING"
                      @click="create"
                    >
                      Cut
                    </v-btn>
                  </v-col>
                </v-row>
              </v-col>
            </v-card-text>
          </v-card>
        </v-card>
        <v-dialog :value="formLoadStatus === STATUS.COMPLETE" width="420">
          <v-card>
            <v-card-title>
              <v-row no-gutters align="center">
                <v-col cols="auto">
                  Cut Created
                </v-col>
                <v-spacer />
                <v-col cols="auto">
                  <v-btn
                    class="text--secondary"
                    icon
                    @click="() => (formLoadStatus = STATUS.IDLE)"
                  >
                    <v-icon>mdi-close</v-icon>
                  </v-btn>
                </v-col>
              </v-row>
            </v-card-title>
            <v-divider inset />
            <div class="v-card__body pt-6">
              <v-row align="center">
                <v-col>
                  <div class="mb-1">
                    Use the following link to share your cut.
                  </div>
                  <div>
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
                  <div class="mb-1">
                    To get view the raw cut, use the following.
                  </div>
                  <div>
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
import FileUpload from "vue-upload-component";
import { maxLength, required, url } from "vuelidate/lib/validators";
import api from "@/apis/api";
import { expiries, languages, STATUS } from "@/constants";
import { highlighter } from "@/utils/highlighter";
import { formatUnit } from "@/utils/formatter";

import "@/styles/Create.sass";
import "@/styles/prism-atom-dark.css";

export default Vue.extend({
  components: {
    FileUpload
  },

  data() {
    return {
      variant: 0,
      expiry: Object.keys(expiries)[1],
      expirySelect: expiries,
      snippet: {
        name: "",
        language: Object.keys(languages)[0],
        languageSelect: languages,
        data: "",
        emptyError: false
      },
      url: {
        target: ""
      },
      file: {
        maxSize: 50 * 1000 * 1000,
        files: new Array<{
          file: File;
        }>(), // vue-upload-component uses its own file struct in the array v-model
        emptyError: false
      },
      formLoadStatus: STATUS.IDLE,
      formLoadProgress: 0,
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
    create() {
      this.formLoadProgress = 0;
      switch (this.variant) {
        case 0: {
          this.$v.snippet.$touch();
          if (this.$v.snippet.$invalid || !this.snippet.data.trim()) {
            this.snippet.emptyError = !this.snippet.data.trim();
            return;
          }
          this.formLoadStatus = STATUS.LOADING;
          api.cut
            .create({
              name: this.snippet.name,
              variant: "snippet",
              metadata: JSON.stringify({ language: this.snippet.language }),
              data: this.snippet.data,
              expiry: expiries[this.expiry]
            })
            .then(response => {
              this.formLoadStatus = STATUS.COMPLETE;
              this.snippet.name = "";
              this.snippet.data = "";
              this.linkView = `${window.origin}/${response.data.hash}`;
              this.linkRaw = `${window.origin}/raw/${response.data.hash}`;
              this.$v.snippet.$reset();
            })
            .catch(() => {
              this.formLoadStatus = STATUS.ERROR;
            });
          break;
        }
        case 1: {
          this.$v.url.$touch();
          if (this.$v.url.$invalid) return;
          this.formLoadStatus = STATUS.LOADING;
          api.cut
            .create({
              name: this.url.target,
              variant: "url",
              metadata: JSON.stringify({}),
              data: this.url.target,
              expiry: expiries[this.expiry]
            })
            .then(response => {
              this.formLoadStatus = STATUS.COMPLETE;
              this.url.target = "";
              this.linkView = `${window.origin}/${response.data.hash}`;
              this.linkRaw = `${window.origin}/raw/${response.data.hash}`;
              this.$v.url.$reset();
            })
            .catch(() => {
              this.formLoadStatus = STATUS.ERROR;
            });
          break;
        }
        case 2: {
          if (!this.fileValid()) {
            this.file.emptyError = !this.file.files.length;
            return;
          }
          this.formLoadStatus = STATUS.LOADING;
          const file: File = this.file.files[0].file;
          const form = new FormData();
          form.append("name", file.name);
          form.append("expiry", expiries[this.expiry].toString());
          form.append(
            "metadata",
            JSON.stringify({
              type: file.type,
              size: file.size
            })
          );
          form.append("data", file.type);
          form.append("file", file);
          api.cut
            .createFile(form, this.loadProgress)
            .then(response => {
              this.formLoadStatus = STATUS.COMPLETE;
              this.file.files = [];
              this.linkView = `${window.origin}/${response.data.hash}`;
              this.linkRaw = `${window.origin}/raw/${response.data.hash}`;
            })
            .catch(() => {
              this.formLoadStatus = STATUS.ERROR;
            });
          break;
        }
        default:
          this.formLoadStatus = STATUS.IDLE;
      }
    },
    highlighter: (language: string) => highlighter(language),
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
    },
    fileValid() {
      return (
        this.file.files.length === 1 &&
        this.file.files[0].file.size <= this.file.maxSize
      );
    },
    fileReset() {
      this.file.files = [];
    },
    formatUnit(size: number) {
      return formatUnit(size);
    },
    loadProgress(progressEvent: { loaded: number }) {
      this.formLoadProgress =
        (progressEvent.loaded / this.file.files[0].file.size) * 100;
    }
  },

  beforeRouteLeave(to, from, next) {
    if (this.formLoadStatus !== STATUS.LOADING) next();
  }
});
</script>
