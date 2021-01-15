import axios, { AxiosInstance } from "axios";
import { authManager, refreshAuth } from "@/auth";
import { ACCESS_TOKEN } from "@/auth/AuthManager";
import router from "@/router";

const apiClient: AxiosInstance = axios.create({
  baseURL: "/api/v1/"
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

export default {};
