import axios, { AxiosInstance, AxiosResponse } from "axios";
import { ACCESS_TOKEN } from "@daystram/ratify-client";
import { authManager, refreshAuth } from "@/auth";
import router from "@/router";

const apiClient: AxiosInstance = axios.create({
  baseURL: `${process.env.NODE_ENV === "development" &&
    process.env.VUE_APP_DEV_BASE_API}/api/v1/`
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
    create: function(cut: object): Promise<AxiosResponse> {
      return apiClient.post(`cut`, cut, withAuth());
    }
  }
};
