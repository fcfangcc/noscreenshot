import { invoke } from "@tauri-apps/api";

export const listFormats = async (): Promise<string[]> => {
  return await invoke("list_formats");
};

export const logger = {
  error: (message: string): Promise<void> => {
    return invoke("logger", { level: "error", message: message });
  },
  info: (message: string): Promise<void> => {
    return invoke("logger", { level: "info", message: message });
  },
  warning: (message: string): Promise<void> => {
    return invoke("logger", { level: "warning", message: message });
  },
};
