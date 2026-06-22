use lopdf::{Document, Object, ObjectId, Dictionary};
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::path::Path;
use image::ImageOutputFormat;
use std::io::Cursor;

/// Merges a list of PDF file paths into a single output file path.
pub fn merge_pdfs_impl(input_paths: Vec<String>, output_path: String) -> Result<(), String> {
    if input_paths.is_empty() {
        return Err("No input files provided for merging.".to_string());
    }

    // Load documents in parallel using rayon
    let docs: Vec<Document> = input_paths
        .par_iter()
        .map(|path| Document::load(path))
        .collect::<Result<Vec<Document>, _>>()
        .map_err(|e| format!("Failed to load PDF: {}", e))?;

    let mut max_id = 1;
    let mut documents_pages = Vec::new();
    let mut documents_objects = BTreeMap::new();

    for mut doc in docs {
        doc.renumber_objects_with(max_id);
        max_id = doc.max_id + 1;
        
        let pages = doc.get_pages();
        documents_pages.extend(pages.values().cloned());

        for (id, object) in doc.objects {
            let is_catalog_or_pages = if let Ok(dict) = object.as_dict() {
                dict.type_name().map_or(false, |t| t == "Catalog" || t == "Pages")
            } else {
                false
            };
            if !is_catalog_or_pages {
                documents_objects.insert(id, object);
            }
        }
    }

    let mut catalog_id = None;
    let mut pages_id = None;

    // Find or create Catalog and Pages objects
    for (&id, object) in &documents_objects {
        if let Ok(dict) = object.as_dict() {
            if dict.type_name().map_or(false, |t| t == "Catalog") {
                catalog_id = Some(id);
            } else if dict.type_name().map_or(false, |t| t == "Pages") {
                pages_id = Some(id);
            }
        }
    }

    // If they don't exist, we generate new ones
    let catalog_id = catalog_id.unwrap_or_else(|| {
        let id = (max_id, 0);
        max_id += 1;
        id
    });

    let pages_id = pages_id.unwrap_or_else(|| {
        let id = (max_id, 0);
        max_id += 1;
        id
    });

    let mut catalog = Dictionary::new();
    catalog.set("Type", Object::Name("Catalog".as_bytes().to_vec()));
    catalog.set("Pages", Object::Reference(pages_id));

    let mut pages = Dictionary::new();
    pages.set("Type", Object::Name("Pages".as_bytes().to_vec()));
    pages.set("Count", Object::Integer(documents_pages.len() as i64));
    
    let mut kids = Vec::new();
    for page_ref in documents_pages {
        kids.push(Object::Reference(page_ref));
    }
    pages.set("Kids", Object::Array(kids));

    documents_objects.insert(catalog_id, Object::Dictionary(catalog));
    documents_objects.insert(pages_id, Object::Dictionary(pages));

    // Update parent reference for all pages to point to the new combined Pages object
    for (_, object) in &mut documents_objects {
        if let Ok(dict) = object.as_dict_mut() {
            if dict.type_name().map_or(false, |t| t == "Page") {
                dict.set("Parent", Object::Reference(pages_id));
            }
        }
    }

    let mut merged_doc = Document::new();
    merged_doc.version = "1.7".to_string();
    merged_doc.objects = documents_objects;
    merged_doc.trailer.set("Root", Object::Reference(catalog_id));
    merged_doc.trailer.set("Size", Object::Integer(max_id as i64));
    merged_doc.max_id = max_id;

    merged_doc.save(output_path).map_err(|e| format!("Failed to save merged PDF: {}", e))?;

    Ok(())
}

/// Splits a single PDF into one or more files based on page selections (1-indexed).
pub fn split_pdf_impl(input_path: String, output_dir: String, ranges: Vec<(usize, usize)>) -> Result<Vec<String>, String> {
    let doc = Document::load(&input_path).map_err(|e| format!("Failed to load source PDF: {}", e))?;
    let pages = doc.get_pages();
    let total_pages = pages.len();
    
    let mut created_files = Vec::new();
    let filename_stem = Path::new(&input_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("document");

    for (idx, (start, end)) in ranges.into_iter().enumerate() {
        if start < 1 || end > total_pages || start > end {
            return Err(format!("Invalid page range {}-{}. Total pages: {}", start, end, total_pages));
        }

        let mut split_doc = doc.clone();
        
        let mut pages_to_delete = Vec::new();
        for page_num in 1..=total_pages {
            if page_num < start || page_num > end {
                pages_to_delete.push(page_num as u32);
            }
        }
        
        if !pages_to_delete.is_empty() {
            split_doc.delete_pages(&pages_to_delete);
        }
        
        split_doc.prune_objects();
        split_doc.renumber_objects();
        
        let out_filename = format!("{}_part_{}.pdf", filename_stem, idx + 1);
        let out_path = Path::new(&output_dir).join(out_filename);
        let out_path_str = out_path.to_string_lossy().to_string();

        split_doc.save(&out_path_str).map_err(|e| format!("Failed to save split file: {}", e))?;
        created_files.push(out_path_str);
    }

    Ok(created_files)
}

/// Compresses a PDF by extracting XObject images, resizing/re-compressing them, and writing them back.
pub fn compress_pdf_impl(input_path: String, output_path: String, quality: u8) -> Result<(), String> {
    let mut doc = Document::load(&input_path).map_err(|e| format!("Failed to load source PDF: {}", e))?;

    // Find and compress image streams
    let keys: Vec<ObjectId> = doc.objects.keys().cloned().collect();

    for id in keys {
        if let Some(Object::Stream(ref mut stream)) = doc.objects.get_mut(&id) {
            let is_image = stream.dict.get(b"Subtype")
                .map_or(false, |obj| obj.as_name().map_or(false, |name| name == b"Image"));

            if is_image {
                if let Ok(bytes) = stream.decompressed_content() {
                    if let Ok(img) = image::load_from_memory(&bytes) {
                        // Compress/resize image by 30% if too large, and encode to jpeg/png with quality settings
                        let resized = if img.width() > 1200 {
                            let scale = 1200.0 / img.width() as f32;
                            img.resize((img.width() as f32 * scale) as u32, (img.height() as f32 * scale) as u32, image::imageops::FilterType::Triangle)
                        } else {
                            img
                        };

                        let rgb_img = resized.to_rgb8();
                        let mut compressed_bytes = Vec::new();
                        let mut cursor = Cursor::new(&mut compressed_bytes);
                        
                        // Save image as JPEG with configured quality
                        if let Err(_) = rgb_img.write_to(&mut cursor, ImageOutputFormat::Jpeg(quality)) {
                            continue;
                        }

                        // Update stream dictionary and contents
                        stream.dict.set("Width", Object::Integer(resized.width() as i64));
                        stream.dict.set("Height", Object::Integer(resized.height() as i64));
                        stream.dict.set("Filter", Object::Name(b"DCTDecode".to_vec()));
                        stream.dict.set("ColorSpace", Object::Name(b"DeviceRGB".to_vec()));
                        stream.dict.set("BitsPerComponent", Object::Integer(8));
                        stream.dict.remove(b"DecodeParms");
                        stream.set_content(compressed_bytes);
                    }
                }
            }
        }
    }

    doc.save(output_path).map_err(|e| format!("Failed to save compressed PDF: {}", e))?;
    Ok(())
}

#[derive(serde::Serialize)]
pub struct PdfMetadata {
    pub filepath: String,
    pub filename: String,
    pub filesize: String,
    pub pages: usize,
    pub version: String,
    pub title: String,
    pub author: String,
    pub creator: String,
    pub producer: String,
}

/// Extracts metadata from a PDF file path.
pub fn get_pdf_metadata_impl(filepath: String) -> Result<PdfMetadata, String> {
    let path = Path::new(&filepath);
    let filename = path.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown")
        .to_string();

    let metadata = std::fs::metadata(&filepath)
        .map_err(|e| format!("Failed to read file system metadata: {}", e))?;
    let filesize = format!("{:.2} MB", metadata.len() as f64 / (1024.0 * 1024.0));

    let doc = Document::load(&filepath).map_err(|e| format!("Failed to load PDF: {}", e))?;
    let pages = doc.get_pages().len();
    let version = doc.version.clone();

    let mut title = String::new();
    let mut author = String::new();
    let mut creator = String::new();
    let mut producer = String::new();

    if let Ok(info_obj) = doc.trailer.get(b"Info") {
        let info_dict = match info_obj {
            Object::Reference(id) => doc.get_object(*id).ok().and_then(|o| o.as_dict().ok().cloned()),
            Object::Dictionary(dict) => Some(dict.clone()),
            _ => None,
        };

        if let Some(dict) = info_dict {
            if let Ok(t_obj) = dict.get(b"Title") {
                if let Ok(s) = t_obj.as_reference().and_then(|id| doc.get_object(id)).unwrap_or(t_obj).as_string() {
                    title = s.to_string();
                }
            }
            if let Ok(a_obj) = dict.get(b"Author") {
                if let Ok(s) = a_obj.as_reference().and_then(|id| doc.get_object(id)).unwrap_or(a_obj).as_string() {
                    author = s.to_string();
                }
            }
            if let Ok(c_obj) = dict.get(b"Creator") {
                if let Ok(s) = c_obj.as_reference().and_then(|id| doc.get_object(id)).unwrap_or(c_obj).as_string() {
                    creator = s.to_string();
                }
            }
            if let Ok(p_obj) = dict.get(b"Producer") {
                if let Ok(s) = p_obj.as_reference().and_then(|id| doc.get_object(id)).unwrap_or(p_obj).as_string() {
                    producer = s.to_string();
                }
            }
        }
    }

    Ok(PdfMetadata {
        filepath,
        filename,
        filesize,
        pages,
        version,
        title,
        author,
        creator,
        producer,
    })
}

/// Reads a file from local system and returns its bytes.
pub fn read_pdf_file_impl(path: String) -> Result<Vec<u8>, String> {
    std::fs::read(path).map_err(|e| format!("Failed to read file: {}", e))
}


