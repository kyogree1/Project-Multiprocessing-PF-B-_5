    use std::{process::Command, path::Path, io};

    #[cfg(target_os = "windows")]
    const GS_BIN: &str = "gswin64c";

    #[cfg(not(target_os = "windows"))]
    const GS_BIN: &str = "gs";

    /// Kompres PDF kualitas tinggi menggunakan GhostScript (tanpa turunin kualitas)
    pub fn compress_pdf_high(input: &Path, output: &Path) -> io::Result<()> {
        let status = Command::new(GS_BIN)
            .args([
                "-sDEVICE=pdfwrite",
                "-dPDFSETTINGS=/screen",
                "-dCompatibilityLevel=1.5",
                "-dDownsampleColorImages=true",
                "-dColorImageResolution=72",
                "-dGrayImageResolution=72",
                "-dMonoImageResolution=72",
                "-dDetectDuplicateImages=true",
                "-dCompressFonts=true",
                "-dNOPAUSE",
                "-dQUIET",
                "-dBATCH",
                &format!("-sOutputFile={}", output.to_string_lossy()),
                &input.to_string_lossy(),
            ])
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "GhostScript failed"))
        }
    }
