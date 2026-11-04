import styles from "./UploadModal.module.scss";

interface UploadModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export default function UploadModal({ isOpen, onClose }: UploadModalProps) {
  return (
    <div className={styles.overlay} data-open={isOpen} inert={!isOpen}>
      <div className={styles.modal}>Some text here</div>
    </div>
  );
}
