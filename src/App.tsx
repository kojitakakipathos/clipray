import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  Search,
  Pin,
  Trash2,
  Copy,
  Settings,
  X,
  Image,
  FileText,
} from "lucide-react";
import { formatDistanceToNow } from "date-fns";
import { ja } from "date-fns/locale";
import "./App.css";

interface ClipboardItem {
  id: number;
  content: string;
  content_type: string;
  timestamp: string;
  pinned: boolean;
}

interface AppConfig {
  max_history_count: number;
  hotkey: string;
}

function App() {
  const [clipboardItems, setClipboardItems] = useState<ClipboardItem[]>([]);
  const [searchQuery, setSearchQuery] = useState("");
  const [showSettings, setShowSettings] = useState(false);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [config, setConfig] = useState<AppConfig>({
    max_history_count: 50,
    hotkey: "CommandOrControl+Shift+V",
  });
  const searchInputRef = useRef<HTMLInputElement>(null);

  // クリップボード履歴を取得
  const loadClipboardHistory = async () => {
    try {
      const items = await invoke<ClipboardItem[]>("get_clipboard_history");
      setClipboardItems(items);
      setSelectedIndex(0); // 最初のアイテムを選択
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
      setShowSettings(false);
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

  // 検索フィルタ
  const filteredItems = clipboardItems.filter((item) => {
    if (item.content_type === "image") {
      return true; // 画像は常に表示
    }
    return item.content.toLowerCase().includes(searchQuery.toLowerCase());
  });

  // 初期化
  useEffect(() => {
    loadClipboardHistory();
    loadConfig();

    // クリップボード更新のリスナー
    const unlistenClipboard = listen("clipboard-updated", () => {
      loadClipboardHistory();
    });

    // ホットキーでの表示リスナー
    const unlistenShow = listen("show-clipboard", () => {
      loadClipboardHistory();
      // 検索フィールドにフォーカス
      setTimeout(() => {
        searchInputRef.current?.focus();
      }, 100);
    });

    return () => {
      unlistenClipboard.then((fn) => fn());
      unlistenShow.then((fn) => fn());
    };
  }, []);

  // キーボードナビゲーション
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        if (showSettings) {
          setShowSettings(false);
        } else {
          hideWindow();
        }
        return;
      }

      // 設定画面では無効
      if (showSettings) return;

      if (event.key === "ArrowDown") {
        event.preventDefault();
        setSelectedIndex((prev) =>
          Math.min(prev + 1, filteredItems.length - 1)
        );
        return;
      }
      if (event.key === "ArrowUp") {
        event.preventDefault();
        setSelectedIndex((prev) => Math.max(prev - 1, 0));
        return;
      }

      if (event.key === "Enter") {
        event.preventDefault();
        if (filteredItems[selectedIndex]) {
          const item = filteredItems[selectedIndex];
          copyAndHide(item.content, item.content_type);
        }
        return;
      }

      if (event.key === "Delete") {
        event.preventDefault();
        if (filteredItems[selectedIndex]) {
          deleteItem(filteredItems[selectedIndex].id);
        }
        return;
      }

      if (event.ctrlKey && event.key === "p") {
        event.preventDefault();
        if (filteredItems[selectedIndex]) {
          togglePin(filteredItems[selectedIndex].id);
        }
        return;
      }

      if (event.ctrlKey && event.key === "i") {
        event.preventDefault();
        setShowSettings(!showSettings);
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [showSettings, selectedIndex, filteredItems]);

  // 選択インデックスを調整
  useEffect(() => {
    if (selectedIndex >= filteredItems.length) {
      setSelectedIndex(Math.max(0, filteredItems.length - 1));
    }
  }, [filteredItems.length, selectedIndex]);

  // テキストを短縮表示
  const truncateText = (text: string, maxLength: number = 80) => {
    if (text.length <= maxLength) return text;
    return text.substring(0, maxLength) + "...";
  };

  // 画像のプレビューを生成
  const getImagePreview = (base64Data: string) => {
    return `data:image/png;base64,${base64Data}`;
  };

  return (
    <div className="app">
      <div className="header">
        <div className="search-container">
          <Search className="search-icon" size={16} />
          <input
            ref={searchInputRef}
            type="text"
            placeholder="クリップボード履歴を検索... (↑↓で選択, Enterでコピー&閉じる)"
            value={searchQuery}
            onChange={(e) => {
              setSearchQuery(e.target.value);
              setSelectedIndex(0);
            }}
            className="search-input"
          />
        </div>
        <div className="header-actions">
          <button
            onClick={() => setShowSettings(!showSettings)}
            className="icon-button"
            title="設定 (Ctrl+I)"
          >
            <Settings size={16} />
          </button>
          <button
            onClick={hideWindow}
            className="icon-button"
            title="閉じる (Esc)"
          >
            <X size={16} />
          </button>
        </div>
      </div>

      {showSettings && (
        <div className="settings-panel">
          <h3>設定</h3>
          <div className="setting-item">
            <label>最大履歴数:</label>
            <input
              type="number"
              value={config.max_history_count}
              onChange={(e) =>
                setConfig({
                  ...config,
                  max_history_count: parseInt(e.target.value) || 50,
                })
              }
              min="1"
              max="1000"
            />
          </div>
          <div className="setting-item">
            <label>ホットキー:</label>
            <input
              type="text"
              value={config.hotkey}
              onChange={(e) => setConfig({ ...config, hotkey: e.target.value })}
              placeholder="CommandOrControl+Shift+V"
            />
            <small>例: CommandOrControl+Shift+V, Alt+V, Ctrl+Space</small>
          </div>
          <div className="setting-actions">
            <button onClick={() => saveConfig(config)} className="save-button">
              保存
            </button>
            <button
              onClick={() => {
                setShowSettings(false);
                loadConfig(); // 設定をリセット
              }}
              className="cancel-button"
            >
              キャンセル
            </button>
          </div>
        </div>
      )}

      <div className="clipboard-list">
        {filteredItems.length === 0 ? (
          <div className="empty-state">
            <p>クリップボード履歴がありません</p>
            <p className="empty-hint">
              何かをコピーすると、ここに表示されます
              <br />
              テキストや画像に対応しています
            </p>
          </div>
        ) : (
          filteredItems.map((item, index) => (
            <div
              key={item.id}
              className={`clipboard-item ${item.pinned ? "pinned" : ""} ${
                index === selectedIndex ? "selected" : ""
              }`}
              onClick={() => {
                setSelectedIndex(index);
                copyAndHide(item.content, item.content_type);
              }}
            >
              <div className="item-icon">
                {item.content_type === "image" ? (
                  <Image size={18} />
                ) : (
                  <FileText size={18} />
                )}
              </div>
              <div className="item-content">
                {item.content_type === "image" ? (
                  <div className="image-item">
                    <img
                      src={getImagePreview(item.content)}
                      alt="クリップボード画像"
                      className="image-preview"
                    />
                    <div className="image-meta">
                      <span>画像</span>
                      <div className="item-meta">
                        {formatDistanceToNow(new Date(item.timestamp), {
                          addSuffix: true,
                          locale: ja,
                        })}
                      </div>
                    </div>
                  </div>
                ) : (
                  <div className="text-item">
                    <div className="item-text">
                      {truncateText(item.content)}
                    </div>
                    <div className="item-meta">
                      {formatDistanceToNow(new Date(item.timestamp), {
                        addSuffix: true,
                        locale: ja,
                      })}
                    </div>
                  </div>
                )}
              </div>
              <div className="item-actions">
                <button
                  onClick={(e) => {
                    e.stopPropagation();
                    togglePin(item.id);
                  }}
                  className={`icon-button ${item.pinned ? "pinned" : ""}`}
                  title={
                    item.pinned
                      ? "ピン留めを解除 (Ctrl+P)"
                      : "ピン留め (Ctrl+P)"
                  }
                >
                  <Pin size={14} />
                </button>
                <button
                  onClick={(e) => {
                    e.stopPropagation();
                    copyToClipboard(item.content, item.content_type);
                  }}
                  className="icon-button"
                  title="コピーのみ"
                >
                  <Copy size={14} />
                </button>
                <button
                  onClick={(e) => {
                    e.stopPropagation();
                    deleteItem(item.id);
                  }}
                  className="icon-button delete"
                  title="削除 (Delete)"
                >
                  <Trash2 size={14} />
                </button>
              </div>
            </div>
          ))
        )}
      </div>

      <div className="footer">
        <span className="footer-text">
          {config.hotkey} で起動 | ↑↓で選択 | Enterでコピー&閉じる | Escで閉じる
          | {filteredItems.length} 件
        </span>
      </div>
    </div>
  );
}

export default App;
