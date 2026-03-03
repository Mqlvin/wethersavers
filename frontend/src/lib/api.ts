const dev = import.meta.env.MODE === "development";

export const API_URL: string = dev
  ? "http://127.0.0.1:3000/api"
  : "/api";

