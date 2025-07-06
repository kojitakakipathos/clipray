import React from "react";

interface ExitConfirmModalProps {
  isOpen: boolean;
  onConfirm: () => void;
  onCancel: () => void;
  pinnedItemsCount: number;
}

const ExitConfirmModal: React.FC<ExitConfirmModalProps> = ({
  isOpen,
  onConfirm,
  onCancel,
  pinnedItemsCount,
}) => {
  if (!isOpen) return null;

  return (
    <div className="modal-overlay">
      <div className="modal-content">
        <div className="modal-header">
          <h3>Exit Clipray</h3>
        </div>
        <div className="modal-body">
          <p className="warning-text">Are you sure you want to exit Clipray?</p>

          <div className="warning-list">
            <div className="warning-item">
              <span className="warning-icon">⚠️</span>
              <span>
                Clipboard monitoring will stop and new clipboard data will not
                be recorded until the application is restarted.
              </span>
            </div>

            {pinnedItemsCount > 0 && (
              <div className="warning-item">
                <span className="warning-icon">📌</span>
                <span>
                  You have {pinnedItemsCount} pinned item
                  {pinnedItemsCount > 1 ? "s" : ""} that will remain saved.
                </span>
              </div>
            )}

            <div className="warning-item">
              <span className="warning-icon">💾</span>
              <span>
                Your clipboard history is saved and will be available when you
                restart the application.
              </span>
            </div>
          </div>
        </div>

        <div className="modal-actions">
          <button onClick={onCancel} className="cancel-button">
            Cancel
          </button>
          <button onClick={onConfirm} className="exit-button">
            Exit Application
          </button>
        </div>
      </div>
    </div>
  );
};

export default ExitConfirmModal;
