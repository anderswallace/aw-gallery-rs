import styles from "./UploadModal.module.scss";
import { useState, useEffect } from "react";
import { cameraOptions } from "../constants/constants.ts";
import TextField from "@mui/material/TextField";
import AutoComplete from "@mui/material/Autocomplete";
import Button from "@mui/material/Button";
import CloudUploadIcon from "@mui/icons-material/CloudUpload";
import { styled } from "@mui/material/styles";

interface UploadModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export default function UploadModal({ isOpen, onClose }: UploadModalProps) {
  const [cameraValue, setCameraValue] = useState<string | null>(
    cameraOptions[0],
  );
  const [filmValue, setFilmValue] = useState<string>("");

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

  return (
    <div className={styles.overlay} data-open={isOpen} inert={!isOpen}>
      <div className={styles.modal}>
        <div style={{width: "100%"}}>
          <AutoComplete
            value={cameraValue}
            onChange={(_event: any, newValue: string | null) => {
              setCameraValue(newValue);
            }}
            options={cameraOptions}
            fullWidth
            renderInput={(params) => <TextField {...params} label="Camera" />}
          />
        </div>
        <div style={{width: "100%"}}>
          <TextField
            id="outlined-basic"
            value={filmValue}
            label="Film"
            variant="outlined"
            disabled={cameraValue === ("Leica Q2" || "Ricoh GR IIIx")}
            onChange={(e) => setFilmValue(e.target.value)}
            fullWidth
          />
        </div>
        <div className={styles.photoButtons}>
          <Button
            component="label"
            role={undefined}
            variant="contained"
            tabIndex={-1}
            startIcon={<CloudUploadIcon />}
          >
            Upload File
          </Button>
          <Button>Save Photo< /Button>
        </div>
      </div>
    </div>
  );
}
