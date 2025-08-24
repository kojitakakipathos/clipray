import { useState, useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";
import { useClipboard } from "./hooks/useClipboard";
import { useKeyboardNavigation } from "./hooks/useKeyboardNavigation";
import { useTheme } from "./hooks/useTheme";
import Header from "./components/Header";
import Settings from "./components/Settings";
import ExitConfirmModal from "./components/ExitConfirmModal";
import ClipboardList from "./components/ClipboardList";
import Footer from "./components/Footer";
import "./App.css";

function App() {
  const [searchQuery, setSearchQuery] = useState("");
  const [showSettings, setShowSettings] = useState(false);
  const [showExitModal, setShowExitModal] = useState(false);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [activeTab, setActiveTab] = useState<"pinned" | "history">("history"); // Add: tab state
  const [originalConfig, setOriginalConfig] = useState<typeof config | null>(
    null
  );
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
    exitApp,
  } = useClipboard();

  // Apply theme
  useTheme(config.theme);

  // Search filter
  const filteredItems = clipboardItems.filter((item) => {
    if (item.content_type === "image") {
      return false; // Hide images from search results
    }
    return item.content.toLowerCase().includes(searchQuery.toLowerCase());
  });

  // Tab filter - filter pinned/history items based on activeTab
  const tabFilteredItems = filteredItems.filter((item) => {
    if (activeTab === "pinned") {
      return item.pinned;
    } else {
      return !item.pinned;
    }
  });

  const resetSearch = () => {
    setSearchQuery("");
    setSelectedIndex(0);
    setActiveTab("history"); // Return to history tab when resetting search
  };

  const handleTabChange = (tab: "pinned" | "history") => {
    setActiveTab(tab);
    setSelectedIndex(0); // Reset selected index when switching tabs
  };

  // Keyboard navigation
  useKeyboardNavigation({
    showSettings,
    selectedIndex,
    setSelectedIndex,
    filteredItems: tabFilteredItems,
    copyAndHide,
    deleteItem,
    togglePin,
    hideWindow,
    setShowSettings,
    resetSearch,
    setShowExitModal,
    activeTab,
    handleTabChange,
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
    if (selectedIndex >= tabFilteredItems.length) {
      setSelectedIndex(Math.max(0, tabFilteredItems.length - 1));
    }
  }, [tabFilteredItems.length, selectedIndex]);

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
  }, [selectedIndex, tabFilteredItems]);

  const handleSearchChange = (query: string) => {
    setSearchQuery(query);
    setSelectedIndex(0);
  };

  const handleItemClick = (index: number) => {
    setSelectedIndex(index);
    const item = tabFilteredItems[index];
    copyAndHide(item.content, item.content_type);
    resetSearch(); // Reset search
  };

  const handleSettingsSave = async (newConfig: typeof config) => {
    await saveConfig(newConfig);
    setOriginalConfig(null);
    setShowSettings(false);
  };

  const handleSettingsCancel = () => {
    setConfig(originalConfig || config);
    setOriginalConfig(null);
    setShowSettings(false);
  };

  const handleSettingsToggle = () => {
    if (!showSettings) {
      setOriginalConfig(config);
    }
    setShowSettings(!showSettings);
  };

  const handleExitRequest = () => {
    setShowExitModal(true);
  };

  const handleExitConfirm = () => {
    setShowExitModal(false);
    exitApp();
  };

  const handleExitCancel = () => {
    setShowExitModal(false);
  };

  // Count pinned items
  const pinnedItemsCount = clipboardItems.filter((item) => item.pinned).length;

  return (
    <div className="app">
      <Header
        searchQuery={searchQuery}
        onSearchChange={handleSearchChange}
        onSettingsToggle={handleSettingsToggle}
        onHide={hideWindow}
        searchInputRef={searchInputRef}
      />

      {/* Tab UI */}
      <div className="tabs-container">
        <div className="tabs">
          <button
            className={`tab ${activeTab === "history" ? "active" : ""}`}
            onClick={() => handleTabChange("history")}
          >
            History ({clipboardItems.filter((item) => !item.pinned).length})
          </button>
          <button
            className={`tab ${activeTab === "pinned" ? "active" : ""}`}
            onClick={() => handleTabChange("pinned")}
          >
            Pinned ({pinnedItemsCount})
          </button>
        </div>
      </div>

      {showSettings && (
        <Settings
          config={config}
          onConfigChange={setConfig}
          onSave={handleSettingsSave}
          onCancel={handleSettingsCancel}
          onExit={handleExitRequest}
        />
      )}

      <ExitConfirmModal
        isOpen={showExitModal}
        onConfirm={handleExitConfirm}
        onCancel={handleExitCancel}
        pinnedItemsCount={pinnedItemsCount}
      />

      <ClipboardList
        items={tabFilteredItems}
        selectedIndex={selectedIndex}
        onItemClick={handleItemClick}
        onPin={togglePin}
        onCopy={copyToClipboard}
        onDelete={deleteItem}
        selectedItemRef={selectedItemRef}
        listRef={clipboardListRef}
        activeTab={activeTab}
      />

      {tabFilteredItems.length > 0 && (
        <Footer hotkey={config.hotkey} itemCount={tabFilteredItems.length} />
      )}
    </div>
  );
}

export default App;
