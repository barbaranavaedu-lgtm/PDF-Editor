<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onDestroy, onMount } from "svelte";
  import { 
    Plus, 
    Layers, 
    Merge, 
    Split, 
    FileDown, 
    Scan, 
    Settings, 
    Search, 
    Bell, 
    LayoutGrid, 
    Upload, 
    FileText, 
    Check, 
    Loader2, 
    MoreVertical,
    ArrowUp,
    ArrowDown,
    Trash2,
    X,
    AlertTriangle,
    Info,
    CheckCircle
  } from '@lucide/svelte';
  import * as pdfjsLib from 'pdfjs-dist';
  import pdfjsWorkerUrl from 'pdfjs-dist/build/pdf.worker.mjs?url';

  pdfjsLib.GlobalWorkerOptions.workerSrc = pdfjsWorkerUrl;

  // Language & Translation Dictionary
  let lang = $state<'es' | 'en'>('es'); // Default to Spanish

  const t = {
    es: {
      appName: "PDFSwift",
      proBadge: "ESPACIO PRO",
      addFile: "Añadir Archivo",
      batchProcess: "Procesar por Lotes",
      mergePdfs: "Unir PDFs",
      splitRanges: "Dividir Rangos",
      compressSize: "Comprimir Tamaño",
      windowsOcr: "OCR de Windows",
      settings: "Configuración",
      localUser: "Usuario Local",
      proWorkspace: "Área de Trabajo Pro",
      searchPlaceholder: "Buscar archivos en la cola...",
      activeSystem: "Sistema Activo",
      systemReady: "Sistema Listo",
      cpuLoad: "Carga de CPU",
      
      // Batch View
      batchTitle: "Panel de Procesamiento por Lotes",
      batchSubtitle: "Sube y gestiona las tareas activas en la cola de PDFs.",
      dropzoneTitle: "Arrastra y suelta tus archivos aquí",
      dropzoneSubtitle: "o haz clic para explorar en tu ordenador local",
      dropzoneLimits: "Soporta archivos PDF, JPG y PNG de hasta 2.4 GB",
      queueTitle: "Cola de Tareas Activas",
      clearQueue: "Limpiar Cola",
      completed: "Completado",
      processing: "Procesando",
      queued: "En cola",
      emptyQueue: "No hay tareas en la cola. Arrastra archivos o usa 'Añadir Archivo' para comenzar.",
      
      // Merge View
      mergeTitle: "Unir PDFs",
      mergeSubtitle: "Combina múltiples archivos PDF en un único PDF de alta fidelidad.",
      mergeOrderLabel: "Orden de unión de los archivos:",
      mergeEmptyList: "Añade archivos usando el botón de la barra lateral o selecciónalos a continuación.",
      addMergeDocs: "Añadir Documentos a Unir",
      outputFileLabel: "Ubicación del Archivo de Salida",
      mergeBtn: "Unir Documentos",
      
      // Split View
      splitTitle: "Dividir PDF",
      splitSubtitle: "Extrae rangos de páginas específicos en archivos individuales.",
      sourcePdfLabel: "Ruta del Archivo PDF de Origen",
      browseBtn: "Examinar",
      outputDirLabel: "Directorio de Salida Destino",
      pageRangesLabel: "Rangos de Páginas (separados por comas, e.g. 1-3, 4)",
      splitExample: "Ejemplo: \"1-3, 4-5\" generará 2 archivos separados: uno con las páginas 1 a 3 y otro con las 4 a 5.",
      splitBtn: "Dividir Documento PDF",
      
      // Compress View
      compressTitle: "Comprimir PDF",
      compressSubtitle: "Reduce el tamaño del PDF recomprimiendo imágenes de alta resolución.",
      outputPdfLabel: "Ruta del Archivo PDF de Salida",
      compressQualityLabel: "Calidad de Compresión (Menor calidad = menor peso)",
      maxCompression: "10% (Compresión Máxima)",
      highQuality: "100% (Alta Calidad)",
      compressBtn: "Comprimir Documento",
      
      // OCR View
      ocrTitle: "OCR Nativo de Windows",
      ocrSubtitle: "Extrae texto de PDFs basados en imagen o fotos usando la API local de Windows.",
      ocrOptions: "Opciones de OCR",
      sourceImageLabel: "Imagen de Origen",
      choosePhoto: "Elegir Foto / Escaneo",
      imageLoaded: "Imagen Cargada",
      clear: "Limpiar",
      runOcrBtn: "Ejecutar OCR de Windows",
      ocrWorkspace: "Espacio de Trabajo de Escaneo OCR",
      ocrOverlayHint: "Selecciona una imagen para ver los cuadros delimitadores sobre las palabras",
      ocrOutputLabel: "Salida de Texto Reconocido",
      ocrOutputPlaceholder: "Los resultados procesados por el OCR se mostrarán aquí...",
      
      // Settings View
      settingsTitle: "Configuración del Sistema",
      settingsSubtitle: "Configura los parámetros de procesamiento y las preferencias visuales.",
      backendTitle: "Procesamiento del Backend",
      threadPoolLabel: "Grupo de hilos en paralelo (Rayon)",
      threadPoolDesc: "Permite distribuir el procesamiento en múltiples hilos de tu CPU.",
      automatic: "Automático (Recomendado)",
      threads: "Hilos",
      ocrLanguageLabel: "Idioma por defecto del OCR",
      ocrLanguageDesc: "Idioma que usará el motor nativo de Windows al reconocer texto.",
      spanish: "Español (ES)",
      english: "Inglés (US)",
      french: "Francés (FR)",
      aestheticsTitle: "Afectaciones Estéticas",
      glassIntensityLabel: "Intensidad del Glassmorphism",
      glassIntensityDesc: "Ajusta la opacidad y difuminado de los paneles de la interfaz.",
      highBlur: "Alta (Desenfoque Premium)",
      mediumBlur: "Media",
      lowPerformance: "Baja (Rendimiento)",
      uiLanguageLabel: "Idioma de la Interfaz",
      uiLanguageDesc: "Configura el idioma de visualización de este panel de control."
    },
    en: {
      appName: "PDFSwift",
      proBadge: "PRO WORKSPACE",
      addFile: "Add File",
      batchProcess: "Batch Process",
      mergePdfs: "Merge PDFs",
      splitRanges: "Split Ranges",
      compressSize: "Compress Size",
      windowsOcr: "Windows OCR",
      settings: "Settings",
      localUser: "Local User",
      proWorkspace: "Pro Workspace",
      searchPlaceholder: "Search queue files...",
      activeSystem: "Active System",
      systemReady: "System Ready",
      cpuLoad: "CPU Load",
      
      // Batch View
      batchTitle: "Batch Processing Dashboard",
      batchSubtitle: "Upload and manage active PDF queue tasks.",
      dropzoneTitle: "Drag & Drop files here",
      dropzoneSubtitle: "or click to browse from local computer",
      dropzoneLimits: "Supports PDF, JPG, PNG files up to 2.4 GB",
      queueTitle: "Active Task Queue",
      clearQueue: "Clear Queue",
      completed: "Completed",
      processing: "Processing",
      queued: "Queued",
      emptyQueue: "No active tasks in the queue. Drag files or use 'Add File' to get started.",
      
      // Merge View
      mergeTitle: "Merge PDFs",
      mergeSubtitle: "Combine multiple PDF files into a single, high-fidelity PDF.",
      mergeOrderLabel: "File merging order:",
      mergeEmptyList: "Add files using the sidebar button or select files below.",
      addMergeDocs: "Add Merge Documents",
      outputFileLabel: "Output File Location",
      mergeBtn: "Merge Documents",
      
      // Split View
      splitTitle: "Split PDF",
      splitSubtitle: "Extract page ranges into separate files.",
      sourcePdfLabel: "Source PDF File Path",
      browseBtn: "Browse",
      outputDirLabel: "Target Output Directory",
      pageRangesLabel: "Page Ranges (comma-separated, e.g. 1-3, 4)",
      splitExample: "Example: \"1-3, 4-5\" generates 2 files: one with pages 1 to 3, and one with pages 4 to 5.",
      splitBtn: "Split PDF Document",
      
      // Compress View
      compressTitle: "Compress PDF",
      compressSubtitle: "Shrink PDF size by re-compressing high-resolution images.",
      outputPdfLabel: "Output PDF File Path",
      compressQualityLabel: "Compression Quality (Lower quality = smaller size)",
      maxCompression: "10% (Maximum Compression)",
      highQuality: "100% (High Quality)",
      compressBtn: "Compress Document",
      
      // OCR View
      ocrTitle: "Windows Native OCR",
      ocrSubtitle: "Extract text overlays from image-based PDFs or raw images using local Windows API.",
      ocrOptions: "OCR Options",
      sourceImageLabel: "Source Image",
      choosePhoto: "Choose Photo / Scan",
      imageLoaded: "Image Loaded",
      clear: "Clear",
      runOcrBtn: "Run Windows OCR",
      ocrWorkspace: "OCR Scan Workspace",
      ocrOverlayHint: "Select an image to see overlay bounding boxes",
      ocrOutputLabel: "Recognized Text Output",
      ocrOutputPlaceholder: "OCR outputs will print here...",
      
      // Settings View
      settingsTitle: "System Settings",
      settingsSubtitle: "Configure backend processing parameters and UI preferences.",
      backendTitle: "Backend Processing",
      threadPoolLabel: "Parallel Thread pool (Rayon)",
      threadPoolDesc: "Allow parallel processing across multiple CPU threads.",
      automatic: "Automatic (Recommended)",
      threads: "Threads",
      ocrLanguageLabel: "Default OCR Target Language",
      ocrLanguageDesc: "Configure language mappings for the OCR subsystem.",
      spanish: "Spanish (ES)",
      english: "English (US)",
      french: "French (FR)",
      aestheticsTitle: "Aesthetics",
      glassIntensityLabel: "Glassmorphism Intensity",
      glassIntensityDesc: "Tweak backdrop blur layers of the dashboard cards.",
      highBlur: "High (Premium Blur)",
      mediumBlur: "Medium",
      lowPerformance: "Low (Performance)",
      uiLanguageLabel: "Interface Language",
      uiLanguageDesc: "Set the preferred interface display language."
    }
  };

  // Navigation
  type ActiveView = 'batch' | 'merge' | 'split' | 'compress' | 'ocr' | 'settings';
  let activeView = $state<ActiveView>('batch');

  // App Theme/State
  let cpuLoad = $state(12);
  let statusText = $state("Sistema Listo");
  
  // Real File System State (No mocks)
  let queue = $state<ProcessedFile[]>([]);

  // Toast System
  interface Toast {
    id: string;
    message: string;
    type: 'success' | 'error' | 'info';
    fileToOpen?: string;
    dirToOpen?: string;
  }
  let toasts = $state<Toast[]>([]);

  function showToast(message: string, type: 'success' | 'error' | 'info' = 'success', fileToOpen?: string, dirToOpen?: string) {
    const id = Math.random().toString();
    toasts = [...toasts, { id, message, type, fileToOpen, dirToOpen }];
    setTimeout(() => {
      toasts = toasts.filter(t => t.id !== id);
    }, 8000); // Give 8 seconds to allow user time to click the file/folder actions
  }

  // Confirmation Modal System
  let confirmModal = $state<{
    isOpen: boolean;
    title: string;
    message: string;
    onConfirm: () => void;
  }>({
    isOpen: false,
    title: '',
    message: '',
    onConfirm: () => {}
  });

  function openConfirm(title: string, message: string, onConfirm: () => void) {
    confirmModal = {
      isOpen: true,
      title,
      message,
      onConfirm: () => {
        onConfirm();
        confirmModal.isOpen = false;
      }
    };
  }

  // Merge View States
  let filesToMerge = $state<{ path: string; name: string }[]>([]);
  let mergeOutputPath = $state("C:\\Users\\fredy\\Documents\\merged_document.pdf");
  
  // Split View States
  let splitInputPath = $state("");
  let splitOutputDir = $state("C:\\Users\\fredy\\Documents");
  let splitRanges = $state<string>("1-3, 4-5");
  
  // Compress View States
  let compressInputPath = $state("");
  let compressOutputPath = $state("");
  let compressQuality = $state<number>(70);

  // OCR View States
  let ocrImageBase64 = $state("");
  let ocrResultText = $state("");
  let ocrLines = $state<any[]>([]);
  let ocrImageWidth = $state(0);
  let ocrImageHeight = $state(0);
  let isOcrLoading = $state(false);

  // Stats simulation
  let intervalId: any;
  onMount(() => {
    intervalId = setInterval(() => {
      cpuLoad = Math.floor(Math.random() * 10) + 3;
    }, 2500);
  });

  onDestroy(() => {
    if (intervalId) clearInterval(intervalId);
  });

  // Reorder Merge list
  function moveMergeItemUp(idx: number) {
    if (idx === 0) return;
    const temp = filesToMerge[idx];
    filesToMerge[idx] = filesToMerge[idx - 1];
    filesToMerge[idx - 1] = temp;
    filesToMerge = [...filesToMerge];
    showToast(lang === 'es' ? "Orden modificado" : "Order changed", "info");
  }

  function moveMergeItemDown(idx: number) {
    if (idx === filesToMerge.length - 1) return;
    const temp = filesToMerge[idx];
    filesToMerge[idx] = filesToMerge[idx + 1];
    filesToMerge[idx + 1] = temp;
    filesToMerge = [...filesToMerge];
    showToast(lang === 'es' ? "Orden modificado" : "Order changed", "info");
  }

  function addFilesToQueue(event: any) {
    const files = event.target.files;
    if (!files || files.length === 0) return;
    
    let addedCount = 0;
    for (let i = 0; i < files.length; i++) {
      const file = files[i];
      // Grab real path from Tauri webview file reference
      const realPath = file.path || "";
      if (!realPath) {
        showToast(lang === 'es' ? "No se pudo obtener la ruta del archivo" : "Could not retrieve file path", "error");
        continue;
      }
      
      const newFile: ProcessedFile = {
        id: Math.random().toString(),
        name: file.name,
        path: realPath,
        size: `${(file.size / (1024 * 1024)).toFixed(2)} MB`,
        status: 'Queued',
        progress: 0,
        detail: lang === 'es' ? 'Listo' : 'Ready'
      };
      queue = [...queue, newFile];
      
      if (activeView === 'merge') {
        filesToMerge = [...filesToMerge, { path: newFile.path, name: newFile.name }];
      } else if (activeView === 'split') {
        splitInputPath = newFile.path;
      } else if (activeView === 'compress') {
        compressInputPath = newFile.path;
        compressOutputPath = newFile.path.replace(".pdf", "_compressed.pdf");
      }
      addedCount++;
    }
    
    if (addedCount > 0) {
      showToast(lang === 'es' ? `Cargado ${addedCount} archivo(s) correctamente` : `Loaded ${addedCount} file(s) successfully`, "success");
    }
  }

  function dragOverHandler(e: DragEvent) {
    e.preventDefault();
  }

  function dropHandler(e: DragEvent) {
    e.preventDefault();
    if (e.dataTransfer?.files) {
      const files = e.dataTransfer.files;
      let addedCount = 0;
      for (let i = 0; i < files.length; i++) {
        const file = files[i];
        const realPath = (file as any).path || "";
        if (!realPath) continue;

        const newFile: ProcessedFile = {
          id: Math.random().toString(),
          name: file.name,
          path: realPath,
          size: `${(file.size / (1024 * 1024)).toFixed(2)} MB`,
          status: 'Queued',
          progress: 0,
          detail: lang === 'es' ? 'Listo' : 'Ready'
        };
        queue = [...queue, newFile];
        if (activeView === 'merge') {
          filesToMerge = [...filesToMerge, { path: newFile.path, name: newFile.name }];
        }
        addedCount++;
      }
      if (addedCount > 0) {
        showToast(lang === 'es' ? `Soltado ${addedCount} archivo(s) correctamente` : `Dropped ${addedCount} file(s) successfully`, "success");
      }
    }
  }

  async function triggerMerge() {
    if (filesToMerge.length < 2) {
      showToast(lang === 'es' ? "Añada al menos 2 archivos para unir." : "Please add at least 2 files to merge.", "error");
      return;
    }
    
    openConfirm(
      lang === 'es' ? "Confirmar Unión de PDFs" : "Confirm PDF Merge",
      lang === 'es' ? `¿Desea unir los ${filesToMerge.length} archivos en el destino especificado?` : `Do you want to merge these ${filesToMerge.length} files into the specified destination?`,
      async () => {
        statusText = lang === 'es' ? "Uniendo PDFs..." : "Merging PDFs...";
        try {
          await invoke("merge_pdfs", {
            inputPaths: filesToMerge.map(f => f.path),
            outputPath: mergeOutputPath
          });
          statusText = lang === 'es' ? "¡Unión completada!" : "Merge completed!";
          const sepIdx = Math.max(mergeOutputPath.lastIndexOf('\\'), mergeOutputPath.lastIndexOf('/'));
          const dir = sepIdx !== -1 ? mergeOutputPath.substring(0, sepIdx) : mergeOutputPath;
          showToast(
            lang === 'es' ? "PDFs unidos correctamente" : "PDFs merged successfully", 
            "success", 
            mergeOutputPath, 
            dir
          );
        } catch (e: any) {
          statusText = lang === 'es' ? "Error al unir." : "Merge failed.";
          showToast(lang === 'es' ? `Error al unir: ${e}` : `Merge failed: ${e}`, "error");
        }
      }
    );
  }

  async function triggerSplit() {
    if (!splitInputPath) {
      showToast(lang === 'es' ? "Especifique un archivo PDF de origen." : "Please specify a source PDF path.", "error");
      return;
    }
    
    openConfirm(
      lang === 'es' ? "Confirmar División de PDF" : "Confirm PDF Split",
      lang === 'es' ? `¿Desea dividir el PDF con los rangos "${splitRanges}"?` : `Do you want to split the PDF with ranges "${splitRanges}"?`,
      async () => {
        statusText = lang === 'es' ? "Dividiendo PDF..." : "Splitting PDF...";
        try {
          const parsedRanges = splitRanges.split(",").map(r => {
            const parts = r.trim().split("-");
            return [parseInt(parts[0]), parseInt(parts[1] || parts[0])];
          });

          const result: string[] = await invoke("split_pdf", {
            inputPath: splitInputPath,
            outputDir: splitOutputDir,
            ranges: parsedRanges
          });
          statusText = lang === 'es' ? "¡División completada!" : "Split completed!";
          
          showToast(
            lang === 'es' ? `División exitosa. Se crearon ${result.length} archivos` : `Split successful. Created ${result.length} files`, 
            "success", 
            result[0], // Open the first split document
            splitOutputDir
          );
        } catch (e: any) {
          statusText = lang === 'es' ? "Error en la división." : "Split failed.";
          showToast(lang === 'es' ? `Error al dividir: ${e}` : `Split failed: ${e}`, "error");
        }
      }
    );
  }

  async function triggerCompress() {
    if (!compressInputPath || !compressOutputPath) {
      showToast(lang === 'es' ? "Especifique las rutas de entrada y salida." : "Please specify input and output PDF paths.", "error");
      return;
    }
    
    openConfirm(
      lang === 'es' ? "Confirmar Compresión de PDF" : "Confirm PDF Compression",
      lang === 'es' ? `¿Desea comprimir el PDF con una calidad de ${compressQuality}%?` : `Do you want to compress the PDF with ${compressQuality}% quality?`,
      async () => {
        statusText = lang === 'es' ? "Comprimiendo PDF..." : "Compressing PDF...";
        try {
          await invoke("compress_pdf", {
            inputPath: compressInputPath,
            outputPath: compressOutputPath,
            quality: compressQuality
          });
          statusText = lang === 'es' ? "¡Compresión completada!" : "Compression completed!";
          const sepIdx = Math.max(compressOutputPath.lastIndexOf('\\'), compressOutputPath.lastIndexOf('/'));
          const dir = sepIdx !== -1 ? compressOutputPath.substring(0, sepIdx) : compressOutputPath;
          showToast(
            lang === 'es' ? "Compresión completada con éxito" : "Compression completed successfully", 
            "success", 
            compressOutputPath, 
            dir
          );
        } catch (e: any) {
          statusText = lang === 'es' ? "Error en la compresión." : "Compression failed.";
          showToast(lang === 'es' ? `Error al comprimir: ${e}` : `Compression failed: ${e}`, "error");
        }
      }
    );
  }

  function handleOcrFileSelect(event: any) {
    const file = event.target.files[0];
    if (!file) return;
    
    const reader = new FileReader();
    reader.onload = (e) => {
      const result = e.target?.result as string;
      ocrImageBase64 = result;
      
      const img = new Image();
      img.onload = () => {
        ocrImageWidth = img.width;
        ocrImageHeight = img.height;
      };
      img.src = result;
    };
    reader.readAsDataURL(file);
  }

  async function triggerOcr() {
    if (!ocrImageBase64) {
      showToast(lang === 'es' ? "Seleccione una imagen primero." : "Please upload/select an image first.", "error");
      return;
    }
    isOcrLoading = true;
    statusText = lang === 'es' ? "Ejecutando escaneo OCR..." : "Running OCR scan...";
    try {
      const response: any = await invoke("perform_ocr_command", {
        imageBase64: ocrImageBase64
      });
      ocrResultText = response.text;
      ocrLines = response.lines || [];
      statusText = lang === 'es' ? "¡OCR procesado correctamente!" : "OCR processed successfully!";
      showToast(lang === 'es' ? "Texto reconocido con éxito" : "Text recognized successfully", "success");
    } catch (e: any) {
      statusText = lang === 'es' ? "Error en el procesamiento OCR." : "OCR processing failed.";
      showToast(lang === 'es' ? `Error en OCR: ${e}` : `OCR failed: ${e}`, "error");
    } finally {
      isOcrLoading = false;
    }
  }

  // Metadata Modal State
  let selectedFileMetadata = $state<any>(null);
  let isMetadataOpen = $state(false);
  let previewPages = $state<string[]>([]);
  let isRenderingPages = $state(false);

  async function viewFileMetadata(filepath: string) {
    statusText = lang === 'es' ? "Cargando metadatos..." : "Loading metadata...";
    previewPages = [];
    isRenderingPages = true;
    isMetadataOpen = true;
    try {
      const meta: any = await invoke("get_pdf_metadata", { filepath });
      selectedFileMetadata = meta;
      statusText = lang === 'es' ? "Renderizando páginas..." : "Rendering pages...";
      
      // Load file bytes and render page thumbnails
      const bytes: number[] = await invoke("read_pdf_file", { path: filepath });
      const uint8Array = new Uint8Array(bytes);
      const loadingTask = pdfjsLib.getDocument({ data: uint8Array });
      const pdf = await loadingTask.promise;
      
      const pagesToRender = Math.min(pdf.numPages, 12);
      const renderedUrls: string[] = [];
      
      for (let pageNum = 1; pageNum <= pagesToRender; pageNum++) {
        const page = await pdf.getPage(pageNum);
        const viewport = page.getViewport({ scale: 0.35 }); // Clear thumbnail size
        const canvas = document.createElement('canvas');
        const context = canvas.getContext('2d');
        canvas.height = viewport.height;
        canvas.width = viewport.width;
        
        if (context) {
          await page.render({
            canvasContext: context,
            viewport: viewport
          }).promise;
          renderedUrls.push(canvas.toDataURL());
        }
      }
      previewPages = renderedUrls;
      statusText = lang === 'es' ? "Listo" : "Ready";
    } catch (e: any) {
      statusText = lang === 'es' ? "Error al cargar información" : "Failed to load info";
      showToast(lang === 'es' ? `Error al leer PDF: ${e}` : `Error reading PDF: ${e}`, "error");
    } finally {
      isRenderingPages = false;
    }
  }

  function clearQueue() {
    queue = [];
    showToast(lang === 'es' ? "Cola vaciada" : "Queue cleared", "info");
  }
</script>

<div class="flex h-screen w-screen bg-[#050b18] overflow-hidden text-slate-100 selection:bg-blue-500/30 selection:text-white relative">
  
  <!-- Sidebar Panel with Glassmorphism -->
  <aside class="w-[280px] h-screen bg-[#091024]/60 backdrop-blur-xl border-r border-white/5 flex flex-col p-6 z-50">
    <div class="mb-8 px-2">
      <h1 class="text-2xl font-black tracking-wider bg-gradient-to-r from-blue-400 via-indigo-300 to-indigo-500 bg-clip-text text-transparent">{t[lang].appName}</h1>
      <p class="text-slate-400 font-mono text-[10px] tracking-widest uppercase mt-1">{t[lang].proBadge}</p>
    </div>

    <!-- Quick action/New file trigger -->
    <div class="relative group mb-6">
      <label class="w-full flex items-center justify-center gap-2.5 cursor-pointer py-3 rounded-xl glass-button-primary font-semibold text-sm hover:brightness-110 active:scale-95 transition-all shadow-lg">
        <Plus size={16} strokeWidth={2.5} />
        {t[lang].addFile}
        <input type="file" multiple class="hidden" onchange={addFilesToQueue} />
      </label>
    </div>

    <!-- Sidebar Navigation -->
    <nav class="flex-1 space-y-2 overflow-y-auto pr-1">
      <button 
        class="w-full flex items-center gap-4 px-4 py-3 rounded-xl transition-all text-left {activeView === 'batch' ? 'bg-blue-500/15 border border-blue-500/30 text-blue-300 font-semibold' : 'text-slate-400 hover:bg-white/5 hover:text-white'}"
        onclick={() => activeView = 'batch'}
      >
        <Layers size={18} />
        <span class="text-sm">{t[lang].batchProcess}</span>
      </button>

      <button 
        class="w-full flex items-center gap-4 px-4 py-3 rounded-xl transition-all text-left {activeView === 'merge' ? 'bg-blue-500/15 border border-blue-500/30 text-blue-300 font-semibold' : 'text-slate-400 hover:bg-white/5 hover:text-white'}"
        onclick={() => activeView = 'merge'}
      >
        <Merge size={18} />
        <span class="text-sm">{t[lang].mergePdfs}</span>
      </button>

      <button 
        class="w-full flex items-center gap-4 px-4 py-3 rounded-xl transition-all text-left {activeView === 'split' ? 'bg-blue-500/15 border border-blue-500/30 text-blue-300 font-semibold' : 'text-slate-400 hover:bg-white/5 hover:text-white'}"
        onclick={() => activeView = 'split'}
      >
        <Split size={18} />
        <span class="text-sm">{t[lang].splitRanges}</span>
      </button>

      <button 
        class="w-full flex items-center gap-4 px-4 py-3 rounded-xl transition-all text-left {activeView === 'compress' ? 'bg-blue-500/15 border border-blue-500/30 text-blue-300 font-semibold' : 'text-slate-400 hover:bg-white/5 hover:text-white'}"
        onclick={() => activeView = 'compress'}
      >
        <FileDown size={18} />
        <span class="text-sm">{t[lang].compressSize}</span>
      </button>

      <button 
        class="w-full flex items-center gap-4 px-4 py-3 rounded-xl transition-all text-left {activeView === 'ocr' ? 'bg-blue-500/15 border border-blue-500/30 text-blue-300 font-semibold' : 'text-slate-400 hover:bg-white/5 hover:text-white'}"
        onclick={() => activeView = 'ocr'}
      >
        <Scan size={18} />
        <span class="text-sm">{t[lang].windowsOcr}</span>
      </button>
    </nav>

    <!-- Sidebar footer -->
    <div class="mt-auto pt-6 border-t border-white/5 space-y-4">
      <button 
        class="w-full flex items-center gap-4 px-4 py-2.5 rounded-xl text-slate-400 hover:bg-white/5 hover:text-white transition-all text-left"
        onclick={() => activeView = 'settings'}
      >
        <Settings size={16} />
        <span class="text-xs">{t[lang].settings}</span>
      </button>

      <div class="flex items-center gap-3 px-2 py-1">
        <div class="w-8 h-8 rounded-full bg-blue-500/20 border border-blue-400/30 flex items-center justify-center font-bold text-xs text-blue-300">ADMIN</div>
        <div class="flex flex-col">
          <span class="text-xs font-bold text-slate-200">{t[lang].localUser}</span>
          <span class="text-[9px] font-mono text-slate-500 uppercase">{t[lang].proWorkspace}</span>
        </div>
      </div>
    </div>
  </aside>

  <!-- Main Content Space -->
  <main class="flex-1 h-screen flex flex-col overflow-hidden relative">
    
    <!-- Decorative Glowing Ambient Orbs behind panels -->
    <div class="absolute -top-40 -right-40 w-96 h-96 bg-blue-500/10 rounded-full blur-[120px] pointer-events-none"></div>
    <div class="absolute -bottom-45 -left-20 w-80 h-80 bg-indigo-500/10 rounded-full blur-[100px] pointer-events-none"></div>

    <!-- Header bar -->
    <header class="h-16 flex items-center justify-between px-8 border-b border-white/5 bg-[#070e1f]/30 backdrop-blur-md z-45">
      <div class="flex items-center gap-4">
        <div class="relative w-80">
          <Search size={14} class="text-slate-500 absolute left-3 top-1/2 -translate-y-1/2" />
          <input 
            type="text" 
            placeholder={t[lang].searchPlaceholder} 
            class="w-full pl-9 pr-4 py-1.5 rounded-lg glass-input text-xs outline-none"
          />
        </div>
      </div>

      <div class="flex items-center gap-6">
        <div class="flex items-center gap-4 border-r border-white/5 pr-6">
          <button class="text-slate-400 hover:text-blue-400 transition-colors">
            <Bell size={18} />
          </button>
          <button class="text-slate-400 hover:text-blue-400 transition-colors">
            <LayoutGrid size={18} />
          </button>
        </div>
        <div class="flex items-center gap-2">
          <span class="w-1.5 h-1.5 rounded-full bg-emerald-500 glow-green"></span>
          <span class="font-mono text-[9px] text-slate-400 uppercase tracking-widest">{t[lang].activeSystem}</span>
        </div>
      </div>
    </header>

    <!-- Scrollable content -->
    <div class="flex-1 overflow-y-auto p-8 relative z-10">

      <!-- View: BATCH PROCESS -->
      {#if activeView === 'batch'}
        <div class="space-y-6">
          <div>
            <h2 class="text-2xl font-bold text-white tracking-tight">{t[lang].batchTitle}</h2>
            <p class="text-xs text-slate-400 mt-1">{t[lang].batchSubtitle}</p>
          </div>

          <!-- Dropzone Card -->
          <div 
            class="glass-card rounded-2xl p-8 flex flex-col items-center justify-center min-h-[220px] dropzone-pattern cursor-pointer relative"
            ondragover={dragOverHandler}
            ondrop={dropHandler}
          >
            <input type="file" multiple class="absolute inset-0 opacity-0 cursor-pointer" onchange={addFilesToQueue} />
            <div class="w-16 h-16 bg-blue-500/10 rounded-full flex items-center justify-center mb-4 border border-dashed border-blue-400/30">
              <Upload size={32} class="text-blue-400" />
            </div>
            <h3 class="text-sm font-semibold text-slate-200">{t[lang].dropzoneTitle}</h3>
            <p class="text-xs text-slate-500 mt-1">{t[lang].dropzoneSubtitle}</p>
            <p class="text-[10px] text-slate-600 font-mono mt-3">{t[lang].dropzoneLimits}</p>
          </div>

          <!-- Queue Panel -->
          <div class="glass-card rounded-2xl overflow-hidden">
            <div class="px-6 py-4 border-b border-white/5 bg-slate-900/40 flex justify-between items-center">
              <h3 class="text-xs font-bold uppercase tracking-wider text-slate-300">{t[lang].queueTitle}</h3>
              <div class="flex items-center gap-4">
                <button class="text-xs text-blue-400 hover:underline" onclick={clearQueue}>{t[lang].clearQueue}</button>
              </div>
            </div>
            <div class="divide-y divide-white/5">
              {#each queue as item}
                <div class="px-6 py-4 flex items-center justify-between hover:bg-white/[0.02] transition-colors">
                  <div class="flex items-center gap-4 flex-1">
                    <div class="w-10 h-10 rounded-lg bg-blue-500/10 border border-blue-500/20 flex items-center justify-center">
                      <FileText size={20} class="text-blue-400" />
                    </div>
                    <div>
                      <div class="text-xs font-semibold text-slate-200">{item.name}</div>
                      <div class="text-[10px] font-mono text-slate-500 mt-0.5">{item.size} &bull; {item.detail}</div>
                    </div>
                  </div>

                  <div class="flex items-center gap-6">
                    {#if item.status === 'Completed'}
                      <span class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-[10px] font-medium bg-emerald-500/10 border border-emerald-500/20 text-emerald-400">
                        <Check size={12} class="text-emerald-400" />
                        {t[lang].completed}
                      </span>
                    {:else if item.status === 'Processing'}
                      <div class="flex items-center gap-3">
                        <span class="text-[10px] font-mono text-blue-300">{item.progress}%</span>
                        <div class="w-24 h-1.5 bg-white/5 rounded-full overflow-hidden">
                          <div class="h-full bg-blue-500 transition-all duration-300" style="width: {item.progress}%"></div>
                        </div>
                      </div>
                    {:else}
                      <span class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-[10px] font-medium bg-slate-500/10 border border-white/5 text-slate-400">
                        <span class="w-1.5 h-1.5 rounded-full bg-slate-500"></span> {t[lang].queued}
                      </span>
                    {/if}
                    <button 
                      class="text-slate-500 hover:text-blue-400 p-1 rounded transition-colors"
                      title={lang === 'es' ? "Ver metadatos" : "View metadata"}
                      onclick={() => viewFileMetadata(item.path)}
                    >
                      <Info size={16} />
                    </button>
                    <button class="text-slate-500 hover:text-slate-300">
                      <MoreVertical size={16} />
                    </button>
                  </div>
                </div>
              {/each}
              {#if queue.length === 0}
                <div class="px-6 py-8 text-center text-xs text-slate-500">{t[lang].emptyQueue}</div>
              {/if}
            </div>
          </div>
        </div>

      <!-- View: MERGE -->
      {:else if activeView === 'merge'}
        <div class="space-y-6 max-w-3xl">
          <div>
            <h2 class="text-2xl font-bold text-white tracking-tight">{t[lang].mergeTitle}</h2>
            <p class="text-xs text-slate-400 mt-1">{t[lang].mergeSubtitle}</p>
          </div>

          <div class="glass-card rounded-2xl p-6 space-y-4">
            <div class="space-y-2">
              <label class="text-xs font-semibold text-slate-300">{t[lang].mergeOrderLabel}</label>
              
              <!-- Selected list -->
              <div class="space-y-2">
                {#each filesToMerge as file, idx}
                  <div class="flex items-center justify-between p-3 rounded-lg bg-slate-950/40 border border-white/5">
                    <div class="flex items-center gap-3">
                      <span class="font-mono text-xs text-slate-500">#{idx + 1}</span>
                      <span class="text-xs text-slate-200">{file.name}</span>
                    </div>
                    <div class="flex items-center gap-2">
                      <button 
                        class="p-1 hover:bg-white/5 rounded text-slate-400 hover:text-white"
                        onclick={() => moveMergeItemUp(idx)}
                        disabled={idx === 0}
                      >
                        <ArrowUp size={14} />
                      </button>
                      <button 
                        class="p-1 hover:bg-white/5 rounded text-slate-400 hover:text-white"
                        onclick={() => moveMergeItemDown(idx)}
                        disabled={idx === filesToMerge.length - 1}
                      >
                        <ArrowDown size={14} />
                      </button>
                      <button 
                        class="p-1 hover:bg-rose-500/10 rounded text-rose-400"
                        onclick={() => filesToMerge = filesToMerge.filter((_, i) => i !== idx)}
                      >
                        <Trash2 size={14} />
                      </button>
                    </div>
                  </div>
                {/each}

                {#if filesToMerge.length === 0}
                  <div class="text-center py-6 text-xs text-slate-500 border border-dashed border-white/5 rounded-lg bg-slate-950/20">
                    {t[lang].mergeEmptyList}
                  </div>
                {/if}
              </div>
            </div>

            <!-- Choose file trigger in page -->
            <label class="inline-flex items-center gap-2 cursor-pointer px-4 py-2 rounded-lg glass-button font-medium text-xs">
              <Plus size={14} /> {t[lang].addMergeDocs}
              <input type="file" multiple class="hidden" onchange={addFilesToQueue} />
            </label>

            <hr class="border-white/5 my-4" />

            <div class="space-y-2">
              <label class="text-xs font-semibold text-slate-300 block">{t[lang].outputFileLabel}</label>
              <div class="flex gap-2">
                <input 
                  type="text" 
                  bind:value={mergeOutputPath}
                  class="flex-1 px-4 py-2 rounded-lg glass-input text-xs outline-none font-mono"
                />
                <button 
                  type="button"
                  class="px-4 py-2 rounded-lg glass-button font-medium text-xs flex items-center justify-center transition-all"
                  onclick={async () => {
                    const selected = await invoke("select_save_file", { defaultName: "merged_document.pdf" });
                    if (selected) mergeOutputPath = selected;
                  }}
                >
                  {t[lang].browseBtn}
                </button>
              </div>
            </div>

            <button 
              class="w-full py-2.5 rounded-lg glass-button-primary font-bold text-xs shadow-lg uppercase tracking-wider"
              onclick={triggerMerge}
            >
              {t[lang].mergeBtn}
            </button>
          </div>
        </div>

      <!-- View: SPLIT -->
      {:else if activeView === 'split'}
        <div class="space-y-6 max-w-3xl">
          <div>
            <h2 class="text-2xl font-bold text-white tracking-tight">{t[lang].splitTitle}</h2>
            <p class="text-xs text-slate-400 mt-1">{t[lang].splitSubtitle}</p>
          </div>

          <div class="glass-card rounded-2xl p-6 space-y-4">
            <div class="space-y-2">
              <label class="text-xs font-semibold text-slate-300 block">{t[lang].sourcePdfLabel}</label>
              <div class="flex gap-2">
                <input 
                  type="text" 
                  bind:value={splitInputPath}
                  placeholder="e.g. C:\Users\fredy\Documents\document.pdf"
                  class="flex-1 px-4 py-2 rounded-lg glass-input text-xs outline-none font-mono"
                />
                <label class="px-4 py-2 rounded-lg glass-button font-medium text-xs cursor-pointer flex items-center justify-center">
                  {t[lang].browseBtn}
                  <input type="file" class="hidden" onchange={(e) => {
                    const file = (e.target as any).files[0];
                    if (file) {
                      splitInputPath = file.path || "";
                    }
                  }} />
                </label>
              </div>
            </div>

            <div class="space-y-2">
              <label class="text-xs font-semibold text-slate-300 block">{t[lang].outputDirLabel}</label>
              <div class="flex gap-2">
                <input 
                  type="text" 
                  bind:value={splitOutputDir}
                  class="flex-1 px-4 py-2 rounded-lg glass-input text-xs outline-none font-mono"
                />
                <button 
                  type="button"
                  class="px-4 py-2 rounded-lg glass-button font-medium text-xs flex items-center justify-center transition-all"
                  onclick={async () => {
                    const selected = await invoke("select_directory");
                    if (selected) splitOutputDir = selected;
                  }}
                >
                  {t[lang].browseBtn}
                </button>
              </div>
            </div>

            <div class="space-y-2">
              <label class="text-xs font-semibold text-slate-300 block">{t[lang].pageRangesLabel}</label>
              <input 
                type="text" 
                bind:value={splitRanges}
                placeholder="e.g. 1-3, 4-5, 8"
                class="w-full px-4 py-2 rounded-lg glass-input text-xs outline-none font-mono"
              />
              <p class="text-[10px] text-slate-500">{t[lang].splitExample}</p>
            </div>

            <button 
              class="w-full py-2.5 rounded-lg glass-button-primary font-bold text-xs shadow-lg uppercase tracking-wider"
              onclick={triggerSplit}
            >
              {t[lang].splitBtn}
            </button>
          </div>
        </div>

      <!-- View: COMPRESS -->
      {:else if activeView === 'compress'}
        <div class="space-y-6 max-w-3xl">
          <div>
            <h2 class="text-2xl font-bold text-white tracking-tight">{t[lang].compressTitle}</h2>
            <p class="text-xs text-slate-400 mt-1">{t[lang].compressSubtitle}</p>
          </div>

          <div class="glass-card rounded-2xl p-6 space-y-4">
            <div class="space-y-2">
              <label class="text-xs font-semibold text-slate-300 block">{t[lang].sourcePdfLabel}</label>
              <div class="flex gap-2">
                <input 
                  type="text" 
                  bind:value={compressInputPath}
                  placeholder="e.g. C:\Users\fredy\Documents\document.pdf"
                  class="flex-1 px-4 py-2 rounded-lg glass-input text-xs outline-none font-mono"
                />
                <label class="px-4 py-2 rounded-lg glass-button font-medium text-xs cursor-pointer flex items-center justify-center">
                  {t[lang].browseBtn}
                  <input type="file" class="hidden" onchange={(e) => {
                    const file = (e.target as any).files[0];
                    if (file) {
                      compressInputPath = file.path || "";
                      compressOutputPath = compressInputPath.replace(".pdf", "_compressed.pdf");
                    }
                  }} />
                </label>
              </div>
            </div>

            <div class="space-y-2">
              <label class="text-xs font-semibold text-slate-300 block">{t[lang].outputPdfLabel}</label>
              <div class="flex gap-2">
                <input 
                  type="text" 
                  bind:value={compressOutputPath}
                  class="flex-1 px-4 py-2 rounded-lg glass-input text-xs outline-none font-mono"
                />
                <button 
                  type="button"
                  class="px-4 py-2 rounded-lg glass-button font-medium text-xs flex items-center justify-center transition-all"
                  onclick={async () => {
                    const defaultName = compressOutputPath ? compressOutputPath.split('\\').pop() || "compressed.pdf" : "compressed.pdf";
                    const selected = await invoke("select_save_file", { defaultName });
                    if (selected) compressOutputPath = selected;
                  }}
                >
                  {t[lang].browseBtn}
                </button>
              </div>
            </div>

            <div class="space-y-2">
              <div class="flex justify-between items-center text-xs font-semibold text-slate-300">
                <label>{t[lang].compressQualityLabel}</label>
                <span class="text-blue-300 font-mono">{compressQuality}%</span>
              </div>
              <input 
                type="range" 
                min="10" 
                max="100" 
                bind:value={compressQuality}
                class="w-full h-1 bg-white/5 rounded-lg appearance-none cursor-pointer accent-blue-500"
              />
              <div class="flex justify-between text-[10px] text-slate-500 font-mono">
                <span>{t[lang].maxCompression}</span>
                <span>{t[lang].highQuality}</span>
              </div>
            </div>

            <button 
              class="w-full py-2.5 rounded-lg glass-button-primary font-bold text-xs shadow-lg uppercase tracking-wider"
              onclick={triggerCompress}
            >
              {t[lang].compressBtn}
            </button>
          </div>
        </div>

      <!-- View: OCR -->
      {:else if activeView === 'ocr'}
        <div class="space-y-6">
          <div>
            <h2 class="text-2xl font-bold text-white tracking-tight">{t[lang].ocrTitle}</h2>
            <p class="text-xs text-slate-400 mt-1">{t[lang].ocrSubtitle}</p>
          </div>

          <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <!-- OCR Controls -->
            <div class="lg:col-span-1 glass-card rounded-2xl p-6 space-y-4 h-fit">
              <h3 class="text-xs font-bold uppercase tracking-wider text-slate-300">{t[lang].ocrOptions}</h3>
              
              <div class="space-y-2">
                <label class="text-xs font-semibold text-slate-400 block">{t[lang].sourceImageLabel}</label>
                <label class="w-full flex flex-col items-center justify-center py-4 rounded-xl border border-dashed border-white/5 bg-slate-950/20 cursor-pointer hover:bg-slate-950/45 transition-colors">
                  <Upload size={32} class="text-slate-400 mb-1" />
                  <span class="text-xs text-slate-300">{t[lang].choosePhoto}</span>
                  <input type="file" accept="image/*" class="hidden" onchange={handleOcrFileSelect} />
                </label>
              </div>

              {#if ocrImageBase64}
                <div class="p-2 bg-slate-950/40 rounded-lg border border-white/5 flex items-center justify-between">
                  <span class="text-[10px] font-mono text-slate-400 truncate max-w-[140px]">{t[lang].imageLoaded}</span>
                  <button 
                    class="text-[10px] text-rose-400 hover:underline"
                    onclick={() => { ocrImageBase64 = ""; ocrResultText = ""; ocrLines = []; }}
                  >{t[lang].clear}</button>
                </div>
              {/if}

              <button 
                class="w-full py-2.5 rounded-xl glass-button-primary font-bold text-xs shadow-lg uppercase tracking-wider flex items-center justify-center gap-2"
                onclick={triggerOcr}
                disabled={isOcrLoading}
              >
                {#if isOcrLoading}
                  <Loader2 size={14} class="animate-spin" />
                  {lang === 'es' ? 'Procesando...' : 'Processing...'}
                {:else}
                  <Scan size={14} />
                  {t[lang].runOcrBtn}
                {/if}
              </button>
            </div>

            <!-- OCR Workspace / Canvas and Output -->
            <div class="lg:col-span-2 space-y-6">
              <div class="glass-card rounded-2xl p-6 min-h-[300px] flex flex-col">
                <h3 class="text-xs font-bold uppercase tracking-wider text-slate-300 mb-4">{t[lang].ocrWorkspace}</h3>
                
                <div class="flex-1 flex flex-col lg:flex-row gap-6">
                  <!-- Visual canvas mockup -->
                  <div class="flex-1 bg-slate-950/40 border border-white/5 rounded-xl relative overflow-hidden flex items-center justify-center min-h-[240px]">
                    {#if ocrImageBase64}
                      <div class="relative max-w-full max-h-[320px]">
                        <img 
                          src={ocrImageBase64} 
                          alt="OCR Preview" 
                          class="max-w-full max-h-[320px] object-contain rounded"
                        />
                        <!-- Render OCR word boundary boxes overlay -->
                        {#each ocrLines as line}
                          {#each line.words as word}
                            <div 
                              class="absolute bg-blue-500/20 border border-blue-400/40 text-[7px] text-white overflow-hidden pointer-events-none"
                              style="left: {(word.x / ocrImageWidth) * 100}%; top: {(word.y / ocrImageHeight) * 100}%; width: {(word.width / ocrImageWidth) * 100}%; height: {(word.height / ocrImageHeight) * 100}%"
                              title={word.text}
                            >
                            </div>
                          {/each}
                        {/each}
                      </div>
                    {:else}
                      <span class="text-xs text-slate-500 font-medium">{t[lang].ocrOverlayHint}</span>
                    {/if}
                  </div>

                  <!-- Text output -->
                  <div class="w-full lg:w-72 flex flex-col">
                    <h4 class="text-xs font-semibold text-slate-400 mb-2">{t[lang].ocrOutputLabel}</h4>
                    <textarea 
                      class="flex-1 w-full p-4 rounded-xl glass-input text-xs font-mono outline-none resize-none min-h-[160px]"
                      readonly
                      placeholder={t[lang].ocrOutputPlaceholder}
                      value={ocrResultText}
                    ></textarea>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

      <!-- View: SETTINGS -->
      {:else if activeView === 'settings'}
        <div class="space-y-6 max-w-2xl">
          <div>
            <h2 class="text-2xl font-bold text-white tracking-tight">{t[lang].settingsTitle}</h2>
            <p class="text-xs text-slate-400 mt-1">{t[lang].settingsSubtitle}</p>
          </div>

          <div class="glass-card rounded-2xl p-6 space-y-6">
            
            <!-- Language Selector block -->
            <div class="space-y-4">
              <h3 class="text-xs font-bold uppercase tracking-wider text-slate-300">{t[lang].uiLanguageLabel}</h3>
              <div class="flex items-center justify-between">
                <div>
                  <h4 class="text-xs font-semibold text-slate-200">{t[lang].uiLanguageLabel}</h4>
                  <p class="text-[10px] text-slate-500">{t[lang].uiLanguageDesc}</p>
                </div>
                <select 
                  bind:value={lang}
                  class="glass-input text-xs px-3 py-1.5 rounded-lg outline-none cursor-pointer"
                >
                  <option value="es">Español</option>
                  <option value="en">English</option>
                </select>
              </div>
            </div>

            <hr class="border-white/5" />

            <div class="space-y-4">
              <h3 class="text-xs font-bold uppercase tracking-wider text-slate-300">{t[lang].backendTitle}</h3>
              
              <div class="flex items-center justify-between">
                <div>
                  <h4 class="text-xs font-semibold text-slate-200">{t[lang].threadPoolLabel}</h4>
                  <p class="text-[10px] text-slate-500">{t[lang].threadPoolDesc}</p>
                </div>
                <select class="glass-input text-xs px-3 py-1.5 rounded-lg outline-none">
                  <option>{t[lang].automatic}</option>
                  <option>2 {t[lang].threads}</option>
                  <option>4 {t[lang].threads}</option>
                  <option>8 {t[lang].threads}</option>
                </select>
              </div>

              <hr class="border-white/5" />

              <div class="flex items-center justify-between">
                <div>
                  <h4 class="text-xs font-semibold text-slate-200">{t[lang].ocrLanguageLabel}</h4>
                  <p class="text-[10px] text-slate-500">{t[lang].ocrLanguageDesc}</p>
                </div>
                <select class="glass-input text-xs px-3 py-1.5 rounded-lg outline-none">
                  <option>{t[lang].spanish}</option>
                  <option>{t[lang].english}</option>
                  <option>{t[lang].french}</option>
                </select>
              </div>
            </div>

            <hr class="border-white/5" />

            <div class="space-y-4">
              <h3 class="text-xs font-bold uppercase tracking-wider text-slate-300">{t[lang].aestheticsTitle}</h3>
              
              <div class="flex items-center justify-between">
                <div>
                  <h4 class="text-xs font-semibold text-slate-200">{t[lang].glassIntensityLabel}</h4>
                  <p class="text-[10px] text-slate-500">{t[lang].glassIntensityDesc}</p>
                </div>
                <select class="glass-input text-xs px-3 py-1.5 rounded-lg outline-none">
                  <option>{t[lang].highBlur}</option>
                  <option>{t[lang].mediumBlur}</option>
                  <option>{t[lang].lowPerformance}</option>
                </select>
              </div>
            </div>
          </div>
        </div>
      {/if}

    </div>

    <!-- Status Bar / Footer -->
    <footer class="h-8 border-t border-white/5 bg-[#050b18]/60 backdrop-blur-md px-8 flex items-center justify-between relative z-10 text-[10px] text-slate-400 font-mono">
      <div class="flex items-center gap-6">
        <div class="flex items-center gap-2">
          <span class="w-1.5 h-1.5 rounded-full bg-emerald-500 glow-green"></span>
          <span class="text-[9px] uppercase tracking-wider">{statusText}</span>
        </div>
        <span class="text-white/10">|</span>
        <span class="text-[9px] uppercase tracking-wider">{t[lang].cpuLoad}: {cpuLoad}%</span>
      </div>
      <div class="flex items-center gap-4 text-[9px] uppercase tracking-wider">
        <span>v2.0.0-Stable</span>
      </div>
    </footer>

  </main>

  <!-- Interactive Toast Notification View Layer -->
  <div class="fixed bottom-6 right-6 z-[100] flex flex-col gap-2 max-w-sm pointer-events-none">
    {#each toasts as toast (toast.id)}
      <div 
        class="pointer-events-auto flex flex-col gap-2.5 px-4 py-3 rounded-xl shadow-xl border backdrop-blur-md transition-all duration-300 transform translate-y-0 min-w-[280px]
          {toast.type === 'success' ? 'bg-emerald-950/80 border-emerald-500/30 text-emerald-300' : ''}
          {toast.type === 'error' ? 'bg-rose-950/80 border-rose-500/30 text-rose-300' : ''}
          {toast.type === 'info' ? 'bg-slate-900/80 border-white/10 text-blue-300' : ''}"
      >
        <div class="flex items-center gap-3">
          {#if toast.type === 'success'}
            <CheckCircle size={16} class="shrink-0" />
          {:else if toast.type === 'error'}
            <AlertTriangle size={16} class="shrink-0" />
          {:else}
            <Info size={16} class="shrink-0" />
          {/if}
          <span class="text-xs font-semibold flex-1 leading-snug">{toast.message}</span>
          <button 
            class="p-0.5 hover:bg-white/5 rounded text-current opacity-70 hover:opacity-100 shrink-0 cursor-pointer"
            onclick={() => toasts = toasts.filter(t => t.id !== toast.id)}
          >
            <X size={14} />
          </button>
        </div>

        {#if toast.fileToOpen || toast.dirToOpen}
          <div class="flex gap-2 justify-end mt-0.5">
            {#if toast.fileToOpen}
              <button 
                class="px-2 py-1 rounded text-[10px] font-bold bg-white/10 hover:bg-white/20 transition-all border border-white/10 text-white cursor-pointer active:scale-95"
                onclick={async () => {
                  try {
                    await invoke("open_path", { path: toast.fileToOpen });
                  } catch (err) {
                    showToast(lang === 'es' ? "Error al abrir archivo" : "Failed to open file", "error");
                  }
                }}
              >
                {lang === 'es' ? 'Abrir archivo' : 'Open file'}
              </button>
            {/if}
            {#if toast.dirToOpen}
              <button 
                class="px-2 py-1 rounded text-[10px] font-bold bg-white/10 hover:bg-white/20 transition-all border border-white/10 text-white cursor-pointer active:scale-95"
                onclick={async () => {
                  try {
                    await invoke("open_path", { path: toast.dirToOpen });
                  } catch (err) {
                    showToast(lang === 'es' ? "Error al abrir carpeta" : "Failed to open folder", "error");
                  }
                }}
              >
                {lang === 'es' ? 'Abrir carpeta' : 'Open folder'}
              </button>
            {/if}
          </div>
        {/if}
      </div>
    {/each}
  </div>

  <!-- Professional Confirmation Modal Overlay -->
  {#if confirmModal.isOpen}
    <div class="fixed inset-0 z-[110] bg-slate-950/75 backdrop-blur-sm flex items-center justify-center p-4">
      <div class="w-full max-w-md bg-[#091024]/90 border border-white/10 rounded-2xl p-6 shadow-2xl space-y-4 backdrop-blur-xl">
        <div class="flex items-center gap-3 text-amber-400">
          <AlertTriangle size={24} />
          <h3 class="text-md font-bold text-white">{confirmModal.title}</h3>
        </div>
        <p class="text-xs text-slate-300 leading-relaxed">{confirmModal.message}</p>
        <div class="flex justify-end gap-3 pt-2">
          <button 
            class="px-4 py-2 rounded-lg glass-button text-xs font-semibold"
            onclick={() => confirmModal.isOpen = false}
          >
            {lang === 'es' ? 'Cancelar' : 'Cancel'}
          </button>
          <button 
            class="px-4 py-2 rounded-lg glass-button-primary text-xs font-semibold"
            onclick={confirmModal.onConfirm}
          >
            {lang === 'es' ? 'Confirmar' : 'Confirm'}
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- PDF Metadata & Page Previews Modal -->
  {#if isMetadataOpen && selectedFileMetadata}
    <div class="fixed inset-0 z-[110] bg-slate-950/75 backdrop-blur-sm flex items-center justify-center p-4">
      <div class="w-full max-w-4xl bg-[#091024]/90 border border-white/10 rounded-2xl p-6 shadow-2xl space-y-4 backdrop-blur-xl relative flex flex-col max-h-[85vh]">
        <button 
          class="absolute top-4 right-4 text-slate-400 hover:text-white p-1 hover:bg-white/5 rounded transition-all"
          onclick={() => isMetadataOpen = false}
        >
          <X size={16} />
        </button>
        
        <div class="flex items-center gap-3 text-blue-400">
          <Info size={24} />
          <h3 class="text-md font-bold text-white">{lang === 'es' ? 'Información y Páginas del PDF' : 'PDF Info & Pages'}</h3>
        </div>

        <div class="flex-1 grid grid-cols-1 md:grid-cols-5 gap-6 overflow-hidden min-h-0">
          <!-- Left: Metadata -->
          <div class="md:col-span-2 space-y-4 overflow-y-auto pr-2">
            <div class="p-3 rounded-lg bg-slate-950/45 border border-white/5">
              <span class="text-slate-500 block text-[10px] uppercase font-mono">{lang === 'es' ? 'Nombre de archivo' : 'Filename'}</span>
              <span class="text-slate-200 font-semibold truncate block mt-0.5" title={selectedFileMetadata.filename}>{selectedFileMetadata.filename}</span>
            </div>
            
            <div class="p-3 rounded-lg bg-slate-950/45 border border-white/5">
              <span class="text-slate-500 block text-[10px] uppercase font-mono">{lang === 'es' ? 'Ruta completa' : 'Full Path'}</span>
              <span class="text-slate-400 font-mono text-[10px] break-all block mt-0.5">{selectedFileMetadata.filepath}</span>
            </div>

            <div class="grid grid-cols-2 gap-3">
              <div class="p-3 rounded-lg bg-slate-950/45 border border-white/5">
                <span class="text-slate-500 block text-[10px] uppercase font-mono">{lang === 'es' ? 'Tamaño' : 'Size'}</span>
                <span class="text-slate-200 font-semibold block mt-0.5">{selectedFileMetadata.filesize}</span>
              </div>

              <div class="p-3 rounded-lg bg-slate-950/45 border border-white/5">
                <span class="text-slate-500 block text-[10px] uppercase font-mono">{lang === 'es' ? 'Número de Páginas' : 'Page Count'}</span>
                <span class="text-slate-200 font-semibold block mt-0.5">{selectedFileMetadata.pages}</span>
              </div>
            </div>

            <div class="p-3 rounded-lg bg-slate-950/45 border border-white/5">
              <span class="text-slate-500 block text-[10px] uppercase font-mono">{lang === 'es' ? 'Versión PDF' : 'PDF Version'}</span>
              <span class="text-slate-200 font-mono block mt-0.5">{selectedFileMetadata.version}</span>
            </div>

            <div class="p-3 rounded-lg bg-slate-950/45 border border-white/5">
              <span class="text-slate-500 block text-[10px] uppercase font-mono">{lang === 'es' ? 'Título' : 'Title'}</span>
              <span class="text-slate-200 font-semibold truncate block mt-0.5">{selectedFileMetadata.title || '-'}</span>
            </div>

            <div class="p-3 rounded-lg bg-slate-950/45 border border-white/5">
              <span class="text-slate-500 block text-[10px] uppercase font-mono">{lang === 'es' ? 'Autor' : 'Author'}</span>
              <span class="text-slate-200 font-semibold truncate block mt-0.5">{selectedFileMetadata.author || '-'}</span>
            </div>

            <div class="p-3 rounded-lg bg-slate-950/45 border border-white/5">
              <span class="text-slate-500 block text-[10px] uppercase font-mono">{lang === 'es' ? 'Creador' : 'Creator'}</span>
              <span class="text-slate-200 font-semibold truncate block mt-0.5">{selectedFileMetadata.creator || '-'}</span>
            </div>
          </div>

          <!-- Right: Page Previews -->
          <div class="md:col-span-3 flex flex-col overflow-hidden">
            <h4 class="text-xs font-semibold text-slate-400 mb-2">{lang === 'es' ? 'Páginas del Documento' : 'Document Pages'}</h4>
            
            <div class="flex-1 bg-slate-950/50 border border-white/5 rounded-xl p-4 overflow-y-auto">
              {#if isRenderingPages}
                <div class="h-full flex flex-col items-center justify-center gap-3">
                  <Loader2 size={24} class="animate-spin text-blue-400" />
                  <span class="text-xs text-slate-400">{lang === 'es' ? 'Renderizando páginas en tiempo real...' : 'Rendering pages in real time...'}</span>
                </div>
              {:else if previewPages.length > 0}
                <div class="grid grid-cols-2 sm:grid-cols-3 gap-4">
                  {#each previewPages as dataUrl, pIdx}
                    <div class="flex flex-col items-center gap-1.5 p-2 bg-slate-900/60 border border-white/5 rounded-lg">
                      <div class="relative w-full aspect-[1/1.4] bg-white rounded flex items-center justify-center overflow-hidden shadow-md">
                        <img src={dataUrl} alt="Pág {pIdx + 1}" class="w-full h-full object-contain" />
                      </div>
                      <span class="text-[10px] font-mono text-slate-500">{lang === 'es' ? 'Página' : 'Page'} {pIdx + 1}</span>
                    </div>
                  {/each}
                </div>
                {#if selectedFileMetadata.pages > 12}
                  <p class="text-[10px] font-mono text-slate-500 text-center mt-4">
                    {lang === 'es' ? `* Mostrando las primeras 12 páginas de ${selectedFileMetadata.pages}` : `* Showing first 12 pages of ${selectedFileMetadata.pages}`}
                  </p>
                {/if}
              {:else}
                <div class="h-full flex items-center justify-center text-xs text-slate-500">
                  {lang === 'es' ? 'No se pudieron generar previsualizaciones' : 'Could not generate previews'}
                </div>
              {/if}
            </div>
          </div>
        </div>

        <div class="flex justify-end pt-2 border-t border-white/5">
          <button 
            class="px-4 py-2 rounded-lg glass-button text-xs font-semibold"
            onclick={() => isMetadataOpen = false}
          >
            {lang === 'es' ? 'Cerrar' : 'Close'}
          </button>
        </div>
      </div>
    </div>
  {/if}

</div>

<style>
  main {
    transition: all 0.3s ease;
  }
</style>
