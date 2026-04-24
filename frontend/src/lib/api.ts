export const IS_DEV = import.meta.env.MODE === "development";

export const API_URL: string = IS_DEV
  ? "http://127.0.0.1:3000/api"
  : "/api";
