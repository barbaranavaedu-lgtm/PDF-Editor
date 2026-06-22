use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrWord {
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrLine {
    pub text: String,
    pub words: Vec<OcrWord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrResult {
    pub text: String,
    pub lines: Vec<OcrLine>,
}

#[cfg(target_os = "windows")]
mod win_ocr {
    use super::*;
    use windows::Foundation::Rect;
    use windows::Graphics::Imaging::BitmapDecoder;
    use windows::Media::Ocr::OcrEngine;
    use windows::Storage::Streams::{DataWriter, InMemoryRandomAccessStream};

    pub async fn run_ocr_on_image_bytes(image_bytes: &[u8]) -> Result<OcrResult, String> {
        // Create an in-memory random access stream
        let stream = InMemoryRandomAccessStream::new().map_err(|e| e.to_string())?;
        
        // Write bytes to stream
        let writer = DataWriter::CreateDataWriter(&stream).map_err(|e| e.to_string())?;
        writer.WriteBytes(image_bytes).map_err(|e| e.to_string())?;
        writer.StoreAsync().map_err(|e| e.to_string())?.await.map_err(|e| e.to_string())?;
        writer.FlushAsync().map_err(|e| e.to_string())?.await.map_err(|e| e.to_string())?;
        
        // Seek to beginning of stream
        stream.Seek(0).map_err(|e| e.to_string())?;

        // Decode the image to SoftwareBitmap
        let decoder = BitmapDecoder::CreateAsync(&stream)
            .map_err(|e| e.to_string())?
            .await
            .map_err(|e| e.to_string())?;
        let bitmap = decoder.GetSoftwareBitmapAsync()
            .map_err(|e| e.to_string())?
            .await
            .map_err(|e| e.to_string())?;

        // Create OCR Engine
        let engine = OcrEngine::TryCreateFromUserProfileLanguages().map_err(|e| e.to_string())?;
        
        // Run OCR
        let result = engine.RecognizeAsync(&bitmap)
            .map_err(|e| e.to_string())?
            .await
            .map_err(|e| e.to_string())?;

        let text = result.Text().map_err(|e| e.to_string())?.to_string();
        let mut lines = Vec::new();

        let ocr_lines = result.Lines().map_err(|e| e.to_string())?;
        for ocr_line in ocr_lines {
            let line_text = ocr_line.Text().map_err(|e| e.to_string())?.to_string();
            let mut words = Vec::new();
            
            let ocr_words = ocr_line.Words().map_err(|e| e.to_string())?;
            for ocr_word in ocr_words {
                let word_text = ocr_word.Text().map_err(|e| e.to_string())?.to_string();
                let rect: Rect = ocr_word.BoundingRect().map_err(|e| e.to_string())?;
                words.push(OcrWord {
                    text: word_text,
                    x: rect.X as f64,
                    y: rect.Y as f64,
                    width: rect.Width as f64,
                    height: rect.Height as f64,
                });
            }

            lines.push(OcrLine {
                text: line_text,
                words,
            });
        }

        Ok(OcrResult { text, lines })
    }
}

// Fallback or placeholder for other platforms as required by requirements
#[cfg(not(target_os = "windows"))]
mod win_ocr {
    use super::*;
    pub async fn run_ocr_on_image_bytes(_image_bytes: &[u8]) -> Result<OcrResult, String> {
        Err("OCR is only implemented natively on Windows for this build. Native iOS/macOS Vision or Android ML Kit bindings are structured for target platforms.".to_string())
    }
}

pub async fn perform_ocr(image_bytes: &[u8]) -> Result<OcrResult, String> {
    win_ocr::run_ocr_on_image_bytes(image_bytes).await
}
