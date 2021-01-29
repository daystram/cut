<template>
  <div class="list">
    <v-row class="mb-8" align="center">
      <v-col cols="12" sm="">
        <h1 class="text-h2">
          My Cuts
        </h1>
      </v-col>
    </v-row>
    <v-fade-transition>
      <div v-show="pageLoadStatus === STATUS.COMPLETE">
        <v-row>
          <v-col cols="12">
            <v-data-table
              :headers="headers"
              :items="cuts"
              hide-default-footer
              class="elevation-4"
            >
              <template v-slot:no-data>No cuts created</template>
              <template v-slot:item.name="{ item }">
                <span
                  class="d-inline-block text-truncate"
                  style="max-width: 240px;"
                >
                  {{ item.name }}
                </span>
              </template>
              <template v-slot:item.variant="{ item }">
                <v-chip
                  v-if="item.variant === 'snippet'"
                  color="primary"
                  outlined
                  small
                >
                  <v-avatar left>
                    <v-icon v-text="'mdi-code-braces'" small />
                  </v-avatar>
                  Snippet
                </v-chip>
                <v-chip
                  v-if="item.variant === 'url'"
                  color="secondary"
                  outlined
                  small
                >
                  <v-avatar left>
                    <v-icon v-text="'mdi-web'" small />
                  </v-avatar>
                  URL
                </v-chip>
                <v-chip
                  v-if="item.variant === 'file'"
                  color="accent"
                  outlined
                  small
                >
                  <v-avatar left>
                    <v-icon v-text="'mdi-file-upload'" small />
                  </v-avatar>
                  File
                </v-chip>
              </template>
              <template v-slot:item.created_at="{ item }">
                {{
                  Intl.DateTimeFormat("default", {
                    year: "numeric",
                    month: "long",
                    day: "numeric",
                    hour: "numeric",
                    minute: "numeric",
                    second: "numeric"
                  }).format(item.created_at * 1000)
                }}
              </template>
              <template v-slot:item.expires="{ item }">
                {{
                  item.expiry >= 0
                    ? Intl.DateTimeFormat("default", {
                        year: "numeric",
                        month: "long",
                        day: "numeric",
                        hour: "numeric",
                        minute: "numeric",
                        second: "numeric"
                      }).format(item.expires * 1000)
                    : item.variant === "snippet"
                    ? "After viewed"
                    : item.variant === "url"
                    ? "After used"
                    : "After downloaded"
                }}
              </template>
              <template v-slot:item.action="{ item }">
                <v-menu
                  v-model="item.menu"
                  :close-on-content-click="false"
                  offset-x
                  left
                >
                  <template v-slot:activator="{ on, attrs }">
                    <v-btn icon v-bind="attrs" v-on="on">
                      <v-icon>mdi-dots-vertical</v-icon>
                    </v-btn>
                  </template>

                  <v-card width="420">
                    <v-card-title>
                      <v-row no-gutters align="center">
                        <v-col cols="auto">
                          Manage Cut
                        </v-col>
                        <v-spacer />
                        <v-col cols="auto">
                          <v-btn-toggle dense>
                            <v-btn
                              outlined
                              color="primary lighten-1"
                              :href="item.linkView"
                              target="_blank"
                              style="width: 50%"
                            >
                              View
                            </v-btn>
                            <v-btn
                              outlined
                              color="primary lighten-1"
                              :href="item.linkRaw"
                              target="_blank"
                              style="width: 50%"
                            >
                              Raw
                            </v-btn>
                          </v-btn-toggle>
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
                              v-model="item.linkView"
                              outlined
                              dense
                              class="rounded code-field"
                              background-color="#2d2d2d"
                              append-icon="mdi-clipboard-text-multiple"
                              :id="`view-${item.hash}`"
                              @click:append="
                                intoClipboard(item, true, item.hash)
                              "
                              readonly
                              persistent-hint
                              :hint="
                                item.copiedTooltip.view
                                  ? 'Copied to clipboard!'
                                  : ''
                              "
                            >
                            </v-text-field>
                          </div>
                          <div class="mb-1">
                            To get view the raw cut, use the following.
                          </div>
                          <div>
                            <v-text-field
                              v-model="item.linkRaw"
                              outlined
                              dense
                              class="rounded code-field"
                              background-color="#2d2d2d"
                              append-icon="mdi-clipboard-text-multiple"
                              :id="`raw-${item.hash}`"
                              @click:append="
                                intoClipboard(item, false, item.hash)
                              "
                              readonly
                              persistent-hint
                              :hint="
                                item.copiedTooltip.raw
                                  ? 'Copied to clipboard!'
                                  : ''
                              "
                            />
                          </div>
                        </v-col>
                      </v-row>
                      <v-row>
                        <v-col>
                          <v-expand-transition>
                            <div v-show="item.formLoadStatus === STATUS.ERROR">
                              <v-alert
                                type="error"
                                text
                                dense
                                transition="scroll-y-transition"
                                class="mt-n2"
                              >
                                Failed deleting cut!
                              </v-alert>
                            </div>
                          </v-expand-transition>
                          <v-btn
                            dense
                            block
                            text
                            outlined
                            color="error"
                            :disabled="item.formLoadStatus === STATUS.LOADING"
                            :loading="item.formLoadStatus === STATUS.LOADING"
                            @click="deleteCut(item)"
                          >
                            Delete
                          </v-btn>
                        </v-col>
                      </v-row>
                    </div>
                  </v-card>
                </v-menu>
              </template>
            </v-data-table>
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
          Failed loading cut list!
        </v-alert>
      </div>
    </v-expand-transition>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import api from "@/apis/api";
import { STATUS } from "@/constants";

import "@/styles/Create.sass";
import "@/styles/prism-atom-dark.css";

export default Vue.extend({
  data() {
    return {
      pageLoadStatus: STATUS.PRE_LOADING,
      headers: [
        {
          text: "Name",
          align: "start",
          sortable: false,
          value: "name"
        },
        { text: "Type", value: "variant" },
        { text: "Created At", value: "created_at" },
        { text: "Expires At", value: "expires" },
        {
          text: "",
          align: "end",
          sortable: false,
          value: "action"
        }
      ],
      cuts: new Array<{}>()
    };
  },

  created() {
    api.cut
      .list()
      .then(response => {
        for (let i = 0; i < response.data.length; i++) {
          const data = response.data;
          this.cuts.push({
            ...data[i],
            expires: data[i].created_at + data[i].expiry,
            linkView: `${window.origin}/${data[i].hash}`,
            linkRaw: `${window.origin}/raw/${data[i].hash}`,
            menu: false,
            copiedTooltip: {
              view: false,
              raw: false,
              viewTimeout: 0,
              rawTimeout: 0
            },
            formLoadStatus: STATUS.IDLE
          });
        }
        this.pageLoadStatus = STATUS.COMPLETE;
      })
      .catch(() => {
        this.pageLoadStatus = STATUS.ERROR;
      });
  },

  methods: {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    intoClipboard(item: any, view: boolean, hash: string) {
      const target: HTMLTextAreaElement | null = document.querySelector(
        `#${view ? "view-" : "raw-"}${hash}`
      );
      if (!target) return;
      target.select();
      document.execCommand("copy");
      if (view) {
        item.copiedTooltip.view = true;
        clearTimeout(item.copiedTooltip.viewTimeout);
        item.copiedTooltip.viewTimeout = setTimeout(
          () => (item.copiedTooltip.view = false),
          5000
        );
      } else {
        item.copiedTooltip.raw = true;
        clearTimeout(item.copiedTooltip.rawTimeout);
        item.copiedTooltip.rawTimeout = setTimeout(
          () => (item.copiedTooltip.raw = false),
          5000
        );
      }
    },
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    deleteCut(item: any) {
      item.formLoadStatus = STATUS.LOADING;
      api.cut
        .delete(item.hash)
        .then(() => {
          this.cuts.splice(this.cuts.indexOf(item), 1);
        })
        .catch(() => {
          item.formLoadStatus = STATUS.IDLE;
        });
    }
  }
});
</script>
