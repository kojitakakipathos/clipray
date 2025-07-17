import { useEffect } from "react";
import { ClipboardItem } from "../types";

interface UseKeyboardNavigationProps {
  showSettings: boolean;
  selectedIndex: number;
  setSelectedIndex: (index: number) => void;
  filteredItems: ClipboardItem[];
  copyAndHide: (content: string, contentType: string) => void;
  deleteItem: (id: number) => void;
  togglePin: (id: number) => void;
  hideWindow: () => void;
  setShowSettings: (show: boolean) => void;
  resetSearch: () => void;
  setShowExitModal: (show: boolean) => void;
  activeTab: "pinned" | "history";
  handleTabChange: (tab: "pinned" | "history") => void;
}

export const useKeyboardNavigation = ({
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
  setShowExitModal,
  activeTab,
  handleTabChange,
}: UseKeyboardNavigationProps) => {
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        setShowExitModal(false);
        if (showSettings) {
          setShowSettings(false);
        } else {
          hideWindow();
        }
        return;
      }

      // Disabled in settings screen
      if (showSettings) return;

      // Tab switching with Ctrl+Tab
      if (event.ctrlKey && event.key === "Tab") {
        event.preventDefault();
        const nextTab = activeTab === "history" ? "pinned" : "history";
        handleTabChange(nextTab);
        return;
      }

      if (event.key === "ArrowDown") {
        event.preventDefault();
        setSelectedIndex(Math.min(selectedIndex + 1, filteredItems.length - 1));
        return;
      }
      if (event.key === "ArrowUp") {
        event.preventDefault();
        setSelectedIndex(Math.max(selectedIndex - 1, 0));
        return;
      }

      if (event.key === "Enter") {
        event.preventDefault();
        if (filteredItems[selectedIndex]) {
          const item = filteredItems[selectedIndex];
          copyAndHide(item.content, item.content_type);
          resetSearch(); // Reset search
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
  }, [
    showSettings,
    selectedIndex,
    filteredItems,
    copyAndHide,
    deleteItem,
    togglePin,
    hideWindow,
    setShowSettings,
    resetSearch,
    setShowExitModal,
    activeTab,
    handleTabChange,
  ]);
};
