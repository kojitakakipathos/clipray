import React from "react";
import { ClipboardItem as ClipboardItemType } from "../types";
import ClipboardItem from "./ClipboardItem";
import EmptyState from "./EmptyState";

interface ClipboardListProps {
  items: ClipboardItemType[];
  selectedIndex: number;
  onItemClick: (index: number) => void;
  onPin: (id: number) => void;
  onCopy: (content: string, contentType: string) => void;
  onDelete: (id: number) => void;
  selectedItemRef: React.RefObject<HTMLDivElement>;
  listRef: React.RefObject<HTMLDivElement>;
}

const ClipboardList: React.FC<ClipboardListProps> = ({
  items,
  selectedIndex,
  onItemClick,
  onPin,
  onCopy,
  onDelete,
  selectedItemRef,
  listRef,
}) => {
  if (items.length === 0) {
    return <EmptyState />;
  }

  return (
    <div className="clipboard-list" ref={listRef}>
      {items.map((item, index) => (
        <div
          key={item.id}
          ref={index === selectedIndex ? selectedItemRef : null}
        >
          <ClipboardItem
            item={item}
            isSelected={index === selectedIndex}
            onClick={() => onItemClick(index)}
            onPin={onPin}
            onCopy={onCopy}
            onDelete={onDelete}
          />
        </div>
      ))}
    </div>
  );
};

export default ClipboardList;
