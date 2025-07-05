import { useState, useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";
import { useClipboard } from "./hooks/useClipboard";
import { useKeyboardNavigation } from "./hooks/useKeyboardNavigation";
import Header from "./components/Header";
import Settings from "./components/Settings";
import ClipboardList from "./components/ClipboardList";
import Footer from "./components/Footer";
import "./App.css";

function App() {
  const [searchQuery, setSearchQuery] = useState("");
  const [showSettings, setShowSettings] = useState(false);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const searchInputRef = useRef<HTMLInputElement>(null);
  const clipboardListRef = useRef<HTMLDivElement>(null);
  const selectedItemRef = useRef<HTMLDivElement>(null);

  const {
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
  } = useClipboard();

  // 検索フィルタ
  const filteredItems = clipboardItems.filter((item) => {
    if (item.content_type === "image") {
      return true; // 画像は常に表示
    }
    return item.content.toLowerCase().includes(searchQuery.toLowerCase());
  });

  const resetSearch = () => {
    setSearchQuery("");
    setSelectedIndex(0);
  };

  // キーボードナビゲーション
  useKeyboardNavigation({
    showSettings,
    selectedIndex,
    setSelectedIndex,
    filteredItems,
    copyAndHide,
    deleteItem,
    togglePin,
    hideWindow,
    setShowSettings,
    resetSearch,
  });

  // ホットキーでの表示リスナー
  useEffect(() => {
    const unlistenShow = listen("show-clipboard", () => {
      loadClipboardHistory();
      resetSearch(); // 検索をリセット

      // scroll to top
      if (clipboardListRef.current) {
        clipboardListRef.current.scrollTop = 0;
      }

      // 検索フィールドにフォーカス
      setTimeout(() => {
        searchInputRef.current?.focus();
      }, 100);
    });

    return () => {
      unlistenShow.then((fn) => fn());
    };
  }, [loadClipboardHistory, resetSearch]);

  // 選択インデックスを調整
  useEffect(() => {
    if (selectedIndex >= filteredItems.length) {
      setSelectedIndex(Math.max(0, filteredItems.length - 1));
    }
  }, [filteredItems.length, selectedIndex]);

  // 選択されたアイテムを画面内に表示するための自動スクロール
  useEffect(() => {
    if (selectedItemRef.current && clipboardListRef.current) {
      const selectedElement = selectedItemRef.current;
      const containerElement = clipboardListRef.current;

      // コンテナの相対位置を計算
      const selectedTop = selectedElement.offsetTop;
      const selectedBottom = selectedTop + selectedElement.offsetHeight;
      const containerScrollTop = containerElement.scrollTop;
      const containerHeight = containerElement.clientHeight;

      // スクロールが必要かチェック
      if (selectedTop < containerScrollTop) {
        // 上にスクロール
        containerElement.scrollTop = selectedTop - 8; // 8pxのマージン
      } else if (selectedBottom > containerScrollTop + containerHeight) {
        // 下にスクロール
        containerElement.scrollTop = selectedBottom - containerHeight + 8; // 8pxのマージン
      }
    }
  }, [selectedIndex, filteredItems]);

  const handleSearchChange = (query: string) => {
    setSearchQuery(query);
    setSelectedIndex(0);
  };

  const handleItemClick = (index: number) => {
    setSelectedIndex(index);
    const item = filteredItems[index];
    copyAndHide(item.content, item.content_type);
    resetSearch(); // 検索をリセット
  };

  const handleSettingsSave = async (newConfig: typeof config) => {
    await saveConfig(newConfig);
    setShowSettings(false);
  };

  const handleSettingsCancel = () => {
    setShowSettings(false);
    // 設定をリセット（元の設定を再読み込み）
    // loadConfig(); // useClipboardフックで管理されているため、ここでは不要
  };

  return (
    <div className="app">
      <Header
        searchQuery={searchQuery}
        onSearchChange={handleSearchChange}
        onSettingsToggle={() => setShowSettings(!showSettings)}
        onHide={hideWindow}
        searchInputRef={searchInputRef}
      />

      {showSettings && (
        <Settings
          config={config}
          onConfigChange={setConfig}
          onSave={handleSettingsSave}
          onCancel={handleSettingsCancel}
        />
      )}

      <ClipboardList
        items={filteredItems}
        selectedIndex={selectedIndex}
        onItemClick={handleItemClick}
        onPin={togglePin}
        onCopy={copyToClipboard}
        onDelete={deleteItem}
        selectedItemRef={selectedItemRef}
        listRef={clipboardListRef}
      />

      <Footer hotkey={config.hotkey} itemCount={filteredItems.length} />
    </div>
  );
}

export default App;
