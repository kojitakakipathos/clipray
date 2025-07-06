import React from "react";
import { Pin, Copy, Trash2, Image, FileText } from "lucide-react";
import { formatDistanceToNow } from "date-fns";
import { enUS } from "date-fns/locale";
import { ClipboardItem as ClipboardItemData } from "../types";

interface ClipboardItemProps {
  item: ClipboardItemData;
  isSelected: boolean;
  onClick: () => void;
  onPin: (id: number) => void;
  onCopy: (content: string, contentType: string) => void;
  onDelete: (id: number) => void;
}

const ClipboardItem: React.FC<ClipboardItemProps> = ({
  item,
  isSelected,
  onClick,
  onPin,
  onCopy,
  onDelete,
}) => {
  const truncateText = (text: string, maxLength: number = 80) => {
    if (text.length <= maxLength) return text;
    return text.substring(0, maxLength) + "...";
  };

  const getImagePreview = (base64Data: string) => {
    return `data:image/png;base64,${base64Data}`;
  };

  return (
    <div
      className={`clipboard-item ${item.pinned ? "pinned" : ""} ${
        isSelected ? "selected" : ""
      }`}
      onClick={onClick}
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
              alt="Clipboard image"
              className="image-preview"
            />
            <div className="image-meta">
              <span>Image</span>
              <div className="item-meta">
                {formatDistanceToNow(new Date(item.timestamp), {
                  addSuffix: true,
                  locale: enUS,
                })}
              </div>
            </div>
          </div>
        ) : (
          <div className="text-item">
            <div className="item-text">{truncateText(item.content)}</div>
            <div className="item-meta">
              {formatDistanceToNow(new Date(item.timestamp), {
                addSuffix: true,
                locale: enUS,
              })}
            </div>
          </div>
        )}
      </div>
      <div className="item-actions">
        <button
          onClick={(e) => {
            e.stopPropagation();
            onPin(item.id);
          }}
          className={`icon-button ${item.pinned ? "pinned" : ""}`}
          title={item.pinned ? "Unpin (Ctrl+P)" : "Pin (Ctrl+P)"}
        >
          <Pin size={14} />
        </button>
        <button
          onClick={(e) => {
            e.stopPropagation();
            onCopy(item.content, item.content_type);
          }}
          className="icon-button"
          title="Copy only"
        >
          <Copy size={14} />
        </button>
        <button
          onClick={(e) => {
            e.stopPropagation();
            onDelete(item.id);
          }}
          className="icon-button delete"
          title="Delete (Delete)"
        >
          <Trash2 size={14} />
        </button>
      </div>
    </div>
  );
};

export default ClipboardItem;
