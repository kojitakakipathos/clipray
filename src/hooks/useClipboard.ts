import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { ClipboardItem, AppConfig } from "../types";

export const useClipboard = () => {
  const [clipboardItems, setClipboardItems] = useState<ClipboardItem[]>([]);
  const [config, setConfig] = useState<AppConfig>({
    max_history_count: 50,
    hotkey: "CommandOrControl+Shift+V",
  });

  // Get clipboard history
  const loadClipboardHistory = async () => {
    try {
      const items = await invoke<ClipboardItem[]>("get_clipboard_history");
      setClipboardItems(items);
    } catch (error) {
      console.error("Failed to load clipboard history:", error);
    }
  };

  // Get configuration
  const loadConfig = async () => {
    try {
      const appConfig = await invoke<AppConfig>("get_config");
      setConfig(appConfig);
    } catch (error) {
      console.error("Failed to load config:", error);
    }
  };

  // アイテムをクリップボードにコピー
  const copyToClipboard = async (
    content: string,
    contentType: string = "text"
  ) => {
    try {
      await invoke("copy_to_clipboard", { content, contentType });
    } catch (error) {
      console.error("Failed to copy to clipboard:", error);
    }
  };

  // Copy item and hide window
  const copyAndHide = async (content: string, contentType: string = "text") => {
    try {
      await invoke("copy_and_hide", { content, contentType });
    } catch (error) {
      console.error("Failed to copy and hide:", error);
    }
  };

  // Delete item
  const deleteItem = async (id: number) => {
    try {
      await invoke("delete_clipboard_item", { id });
      await loadClipboardHistory();
    } catch (error) {
      console.error("Failed to delete item:", error);
    }
  };

  // Toggle pin
  const togglePin = async (id: number) => {
    try {
      await invoke("toggle_pin", { id });
      await loadClipboardHistory();
    } catch (error) {
      console.error("Failed to toggle pin:", error);
    }
  };

  // Save configuration
  const saveConfig = async (newConfig: AppConfig) => {
    try {
      await invoke("update_config", { config: newConfig });
      setConfig(newConfig);
    } catch (error) {
      console.error("Failed to save config:", error);
    }
  };

  // Hide window
  const hideWindow = async () => {
    try {
      await invoke("hide_window");
    } catch (error) {
      console.error("Failed to hide window:", error);
    }
  };

  // Exit application
  const exitApp = async () => {
    try {
      await invoke("exit_app");
    } catch (error) {
      console.error("Failed to exit app:", error);
    }
  };

  // Initialize
  useEffect(() => {
    loadClipboardHistory();
    loadConfig();

    // Clipboard update listener
    const unlistenClipboard = listen("clipboard-updated", () => {
      loadClipboardHistory();
    });

    return () => {
      unlistenClipboard.then((fn) => fn());
    };
  }, []);

  return {
    clipboardItems,
    config,
    setConfig,
    loadClipboardHistory,
    copyToClipboard,
    copyAndHide,
    deleteItem,
    togglePin,
    saveConfig,
    hideWindow,
    exitApp,
  };
};
