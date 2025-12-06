use std::{io, path::Path, process::Command};

#[cfg(target_os = "windows")]
const GS_BIN: &str = "gswin64c";

#[cfg(not(target_os = "windows"))]
const GS_BIN: &str = "gs";

/// Kompres PDF menggunakan GhostScript.
/// Fungsi ini tidak menyentuh state global, hanya tergantung pada input & output path.
pub fn compress_pdf_high(input: &Path, output: &Path) -> io::Result<()> {
    let input_str = input
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid input path"))?;
    let output_str = output
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid output path"))?;

    let status = Command::new(GS_BIN)
        .args([
            "-sDEVICE=pdfwrite",
            "-dPDFSETTINGS=/screen",
            "-dCompatibilityLevel=1.5",
            "-dNOPAUSE",
            "-dQUIET",
            "-dBATCH",
            &format!("-sOutputFile={}", output_str),
            input_str,
        ])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "GhostScript compression failed",
        ))
    }
}
