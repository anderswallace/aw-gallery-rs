import styles from "./UploadModal.module.scss";
import { useState, useEffect } from "react";
import { cameraOptions, digitalCameraOptions } from "../constants/constants.ts";
import TextField from "@mui/material/TextField";
import AutoComplete from "@mui/material/Autocomplete";
import Button from "@mui/material/Button";
import CloudUploadIcon from "@mui/icons-material/CloudUpload";
import { styled } from "@mui/material/styles";

interface UploadModalProps {
  isOpen: boolean;
  //onClose: () => void;
}

//type UploadStatus = "idle" | "uploading" | "success" | "error";

const VisuallyHiddenInput = styled("input")({
  clipPath: "inset(50%)",
  height: 1,
  overflow: "hidden",
  position: "absolute",
  bottom: 0,
  left: 0,
  whiteSpace: "nowrap",
  width: 1,
});

export default function UploadModal({ isOpen }: UploadModalProps) {
  const [cameraValue, setCameraValue] = useState<string | null>(
    cameraOptions[0],
  );
  const [filmValue, setFilmValue] = useState<string>("");
  const [selectedFile, setSelectedFile] = useState(null);
  //const [status, setStatus] = useState<UploadStatus>("idle");
  const [previewUrl, setPreviewUrl] = useState(null);

  // const compressionOptions = {
  //   maxSizeMB: 2,
  //   useWebWorker: true,
  //   fileType: "image/webp",
  // };

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files && event.target.files[0];

    if (!file) {
      alert("Please upload a file");
      setSelectedFile(null);
    } else {
      setSelectedFile(file);
      setPreviewUrl(URL.createObjectURL(file));
    }
  };

  useEffect(() => {
    return () => {
      if (previewUrl) {
        URL.revokeObjectURL(previewUrl);
      }
    };
  }, [previewUrl]);

  return (
    <div className={styles.overlay} data-open={isOpen} inert={!isOpen}>
      <div className={styles.modal}>
        <div style={{ width: "100%" }}>
          <AutoComplete
            value={cameraValue}
            onChange={(
              _event: React.InputEvent<HTMLInputElement>,
              newValue: string | null,
            ) => {
              setCameraValue(newValue);
            }}
            options={cameraOptions}
            fullWidth
            renderInput={(params) => <TextField {...params} label="Camera" />}
          />
        </div>
        <div style={{ width: "100%" }}>
          <TextField
            id="outlined-basic"
            value={filmValue}
            label="Film"
            variant="outlined"
            disabled={digitalCameraOptions.includes(cameraValue)}
            onChange={(e) => setFilmValue(e.target.value)}
            fullWidth
          />
        </div>
        {selectedFile && previewUrl && (
          <div className={styles.previewImage}>
            <img src={previewUrl} alt={"Preview of uploaded file"} />
          </div>
        )}
        <div className={styles.photoButtons}>
          <Button
            component="label"
            role={undefined}
            variant="contained"
            tabIndex={-1}
            startIcon={<CloudUploadIcon />}
          >
            Select Photo
            <VisuallyHiddenInput
              type="file"
              onChange={handleFileChange}
              accept=".jpg, .png"
            />
          </Button>
          <Button disabled={!selectedFile || !filmValue || !cameraValue}>
            Upload
          </Button>
        </div>
      </div>
    </div>
  );
}
