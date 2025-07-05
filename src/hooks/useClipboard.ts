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

  // クリップボード履歴を取得
  const loadClipboardHistory = async () => {
    try {
      const items = await invoke<ClipboardItem[]>("get_clipboard_history");
      setClipboardItems(items);
    } catch (error) {
      console.error("Failed to load clipboard history:", error);
    }
  };

  // 設定を取得
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

  // アイテムをコピーして、ウィンドウを非表示にする
  const copyAndHide = async (content: string, contentType: string = "text") => {
    try {
      await invoke("copy_and_hide", { content, contentType });
    } catch (error) {
      console.error("Failed to copy and hide:", error);
    }
  };

  // アイテムを削除
  const deleteItem = async (id: number) => {
    try {
      await invoke("delete_clipboard_item", { id });
      await loadClipboardHistory();
    } catch (error) {
      console.error("Failed to delete item:", error);
    }
  };

  // ピン留めをトグル
  const togglePin = async (id: number) => {
    try {
      await invoke("toggle_pin", { id });
      await loadClipboardHistory();
    } catch (error) {
      console.error("Failed to toggle pin:", error);
    }
  };

  // 設定を保存
  const saveConfig = async (newConfig: AppConfig) => {
    try {
      await invoke("update_config", { config: newConfig });
      setConfig(newConfig);
    } catch (error) {
      console.error("Failed to save config:", error);
    }
  };

  // ウィンドウを非表示
  const hideWindow = async () => {
    try {
      await invoke("hide_window");
    } catch (error) {
      console.error("Failed to hide window:", error);
    }
  };

  // 初期化
  useEffect(() => {
    loadClipboardHistory();
    loadConfig();

    // クリップボード更新のリスナー
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
  };
};
