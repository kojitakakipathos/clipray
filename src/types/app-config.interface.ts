import { ThemePreset } from "./theme";

export interface ThemeConfig {
  preset: ThemePreset;
}

export interface AppConfig {
  max_history_count: number;
  hotkey: string;
  theme: ThemeConfig;
}
