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

  // Search filter
  const filteredItems = clipboardItems.filter((item) => {
    if (item.content_type === "image") {
      return true; // Always show images
    }
    return item.content.toLowerCase().includes(searchQuery.toLowerCase());
  });

  const resetSearch = () => {
    setSearchQuery("");
    setSelectedIndex(0);
  };

  // Keyboard navigation
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

  // Hotkey display listener
  useEffect(() => {
    const unlistenShow = listen("show-clipboard", () => {
      loadClipboardHistory();
      resetSearch(); // Reset search

      // scroll to top
      if (clipboardListRef.current) {
        clipboardListRef.current.scrollTop = 0;
      }

      // Focus on search field
      setTimeout(() => {
        searchInputRef.current?.focus();
      }, 100);
    });

    return () => {
      unlistenShow.then((fn) => fn());
    };
  }, [loadClipboardHistory, resetSearch]);

  // Adjust selected index
  useEffect(() => {
    if (selectedIndex >= filteredItems.length) {
      setSelectedIndex(Math.max(0, filteredItems.length - 1));
    }
  }, [filteredItems.length, selectedIndex]);

  // Auto-scroll to display selected item within the screen
  useEffect(() => {
    if (selectedItemRef.current && clipboardListRef.current) {
      const selectedElement = selectedItemRef.current;
      const containerElement = clipboardListRef.current;

      // Scroll to top when selected index is 0
      if (selectedIndex === 0) {
        containerElement.scrollTop = 0;
        return;
      }

      // Calculate container's relative position
      const selectedTop = selectedElement.offsetTop;
      const selectedBottom = selectedTop + selectedElement.offsetHeight;
      const containerScrollTop = containerElement.scrollTop;
      const containerHeight = containerElement.clientHeight;

      // Check if scrolling is needed
      if (selectedTop < containerScrollTop) {
        // Scroll up
        containerElement.scrollTop = selectedTop - 8; // 8px margin
      } else if (selectedBottom > containerScrollTop + containerHeight) {
        // Scroll down
        containerElement.scrollTop = selectedBottom - containerHeight + 8; // 8px margin
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
    resetSearch(); // Reset search
  };

  const handleSettingsSave = async (newConfig: typeof config) => {
    await saveConfig(newConfig);
    setShowSettings(false);
  };

  const handleSettingsCancel = () => {
    setShowSettings(false);
    // Reset settings (reload original settings)
    // loadConfig(); // Not needed here as it's managed by useClipboard hook
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

      {filteredItems.length > 0 && (
        <Footer hotkey={config.hotkey} itemCount={filteredItems.length} />
      )}
    </div>
  );
}

export default App;
