import styles from "./app.module.scss";
import Button from "@mui/material/Button";
import { useState } from "react";
import UploadModal from "./components/UploadModal.tsx";

function App() {
  const [isUploadOpen, setIsUploadOpen] = useState(false);
  return (
    <>
      <div className={styles.contentWrapper}>
        <div className={styles.addPhoto}>
          <Button variant="contained" onClick={() => setIsUploadOpen(true)}>
            Add photo
          </Button>
        </div>

        <UploadModal
          isOpen={isUploadOpen}
          onClose={() => setIsUploadOpen(false)}
        />
      </div>
    </>
  );
}

export default App;
