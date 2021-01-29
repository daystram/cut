import axios, { AxiosInstance, AxiosResponse } from "axios";
import { ACCESS_TOKEN } from "@daystram/ratify-client";
import { authManager, refreshAuth } from "@/auth";
import router from "@/router";

const apiClient: AxiosInstance = axios.create({
  baseURL: `${
    process.env.NODE_ENV === "development"
      ? process.env.VUE_APP_DEV_BASE_API
      : ""
  }/api/v1/`
});

apiClient.interceptors.response.use(
  response => {
    return response;
  },
  error => {
    if (error.response.status === 401) {
      refreshAuth(router.currentRoute.fullPath);
    }
    return Promise.reject(error);
  }
);

const withAuth = () => ({
  headers: {
    Authorization: `Bearer ${authManager.getToken(ACCESS_TOKEN)}`
  }
});

export default {
  cut: {
    get: function(hash: string): Promise<AxiosResponse> {
      return apiClient.get(`cut/${hash}`);
    },
    list: function(): Promise<AxiosResponse> {
      return apiClient.get(`cut/list`, withAuth());
    },
    create: function(cut: object): Promise<AxiosResponse> {
      return apiClient.post(`cut`, cut, withAuth());
    },
    createFile: function(
      data: FormData,
      onUploadProgress: (progressEvent: { loaded: number }) => void
    ): Promise<AxiosResponse> {
      return apiClient.post(`cut/file`, data, {
        headers: {
          Authorization: `Bearer ${authManager.getToken(ACCESS_TOKEN)}`,
          "Content-Type": "multipart/form-data"
        },
        onUploadProgress
      });
    },
    delete: function(hash: string): Promise<AxiosResponse> {
      return apiClient.delete(`cut/${hash}`, withAuth());
    }
  }
};
