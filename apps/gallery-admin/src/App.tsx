import styles from "./app.module.scss";
import Button from "@mui/material/Button";
import { useState } from "react";
import UploadModal from "./components/UploadModal.tsx";
import Snackbar from "@mui/material/Snackbar";
import Alert from "@mui/material/Alert";

type Notification = {
  severity: "success" | "error";
  message: string;
};

function App() {
  const [isUploadOpen, setIsUploadOpen] = useState(false);
  const [notification, setNotification] = useState<Notification | null>(null);

  const handleUploadComplete = (
    result: "success" | "error",
    message: string,
  ) => {
    setNotification({ severity: result, message });

    if (result === "success") {
      setIsUploadOpen(false);
    }
  };
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
          onUploadComplete={handleUploadComplete}
        />
        <Snackbar
          open={notification !== null}
          autoHideDuration={5000}
          onClose={() => setNotification(null)}
          anchorOrigin={{
            vertical: "bottom",
            horizontal: "right",
          }}
        >
          <Alert
            severity={notification?.severity ?? "success"}
            variant="filled"
            onClose={() => setNotification(null)}
          >
            {notification?.message}
          </Alert>
        </Snackbar>
      </div>
    </>
  );
}

export default App;
