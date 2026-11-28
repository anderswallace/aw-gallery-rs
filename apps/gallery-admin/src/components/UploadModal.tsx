import styles from "./UploadModal.module.scss";
import { useState, useEffect } from "react";
import { cameraOptions, digitalCameraOptions } from "../constants/constants.ts";
import TextField from "@mui/material/TextField";
import AutoComplete from "@mui/material/Autocomplete";
import Button from "@mui/material/Button";
import CloudUploadIcon from "@mui/icons-material/CloudUpload";
import { styled } from "@mui/material/styles";
import imageCompression from "browser-image-compression";

interface UploadModalProps {
  isOpen: boolean;
  onUploadComplete: (result: "success" | "error", message: string) => void;
}

type UploadStatus = "idle" | "uploading" | "success" | "error";

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

export default function UploadModal({
  isOpen,
  onUploadComplete,
}: UploadModalProps) {
  const [cameraValue, setCameraValue] = useState<string | null>(
    cameraOptions[0],
  );
  const [filmValue, setFilmValue] = useState<string>("");
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [status, setStatus] = useState<UploadStatus>("idle");
  const [previewUrl, setPreviewUrl] = useState<string | null>(null);

  const compressionOptions = {
    maxSizeMB: 2,
    useWebWorker: true,
    fileType: "image/webp",
  };

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

  const handleUpload = async (event: React.SubmitEvent<HTMLFormElement>) => {
    console.log("Upload pressed");
    event.preventDefault();

    if (status === "uploading" || !cameraValue || !selectedFile) {
      return;
    }

    setStatus("uploading");

    try {
      const compressedFile = await imageCompression(
        selectedFile,
        compressionOptions,
      );

      const formData = new FormData();
      formData.append("camera", cameraValue);
      formData.append("film", filmValue);
      formData.append("file", compressedFile);

      const response = await fetch("/api/photos", {
        method: "POST",
        body: formData,
      });

      if (!response.ok) {
        throw new Error("Failed to upload photo");
      }

      setStatus("success");
      onUploadComplete("success", "Photo uploaded successfully");

      // reset metadata values
      setCameraValue(cameraOptions[0]);
      setFilmValue("");
      setSelectedFile(null);
      setPreviewUrl("");
    } catch (error) {
      setStatus("error");
      console.error("Photo upload failed", error);

      onUploadComplete(
        "error",
        error instanceof Error ? error.message : "An unexpected error occurred",
      );
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
        <form onSubmit={handleUpload}>
          <div style={{ width: "100%" }}>
            <AutoComplete
              value={cameraValue}
              onChange={(
                _event: React.InputEvent<HTMLInputElement>,
                newValue: string | null,
              ) => {
                setCameraValue(newValue);

                if (digitalCameraOptions.includes(newValue)) {
                  setFilmValue("");
                }
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
              type="button"
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
            <div>
              <Button
                type="submit"
                loading={status === "uploading"}
                disabled={
                  !selectedFile || !cameraValue || status === "uploading"
                }
              >
                Upload
              </Button>
            </div>
          </div>
        </form>
      </div>
    </div>
  );
}
