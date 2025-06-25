<script lang="ts">
	import { onMount } from "svelte";
	import { invoke } from "@tauri-apps/api/tauri";
	import { open } from "@tauri-apps/api/dialog";
	import { Copy, FolderOpen, FileText, Play, Pause, Square, RotateCcw, HelpCircle, X } from "lucide-svelte";

	// State variables
	let isTauriApp = false;
	let sourceFilePath = "";
	let targetFolderPath = "";
	let isProcessing = false;
	let isPaused = false;
	let progress = 0;
	let currentStep = "";
	let showHelpModal = false;

	// Process log
	interface LogEntry {
		type: "success" | "error" | "info";
		message: string;
		timestamp: string;
	}
	let processLog: LogEntry[] = [];

	// Statistics
	let totalFolders = 0;
	let successCount = 0;
	let errorCount = 0;

	onMount(() => {
		// Check if Tauri is available
		isTauriApp = typeof window !== "undefined" && typeof window["__TAURI_IPC__"] === "function";
		console.log("Tauri available:", isTauriApp);
		
		// Listen for progress updates
		if (isTauriApp) {
			import("@tauri-apps/api/event").then(({ listen }) => {
				listen("progress-update", (event: any) => {
					const data = event.payload as any;
					progress = Math.round(data.percentage);
					currentStep = data.current_step;
					console.log("Progress:", data);
				});
				
				listen("process-result", (event: any) => {
					const data = event.payload as any;
					processLog = [...processLog, {
						type: data.success ? "success" : "error",
						message: data.message,
						timestamp: new Date().toLocaleTimeString()
					}];
					
					if (data.success) {
						successCount++;
					} else {
						errorCount++;
					}
					
					console.log("Process result:", data);
				});
			});
		}
	});

	async function selectSourceFile() {
		if (!isTauriApp) {
			alert("Bu funksiyanın işləməsi üçün Tauri tətbiqi lazımdır.");
			return;
		}
		
		try {
			const selected = await open({
				multiple: false,
				title: "Kopyalanacaq faylı seçin"
			});
			
			if (selected && selected !== null && selected !== "") {
				sourceFilePath = Array.isArray(selected) ? selected[0] : selected;
				console.log("Selected file:", sourceFilePath);
			}
		} catch (error) {
			console.error("File dialog error:", error);
			alert(`Dialog xətası: ${error}`);
		}
	}

	async function selectTargetFolder() {
		if (!isTauriApp) {
			alert("Bu funksiyanın işləməsi üçün Tauri tətbiqi lazımdır.");
			return;
		}
		
		try {
			const selected = await open({
				directory: true,
				multiple: false,
				title: "Hədəf qovluğunu seçin"
			});
			
			if (selected && selected !== null && selected !== "") {
				targetFolderPath = Array.isArray(selected) ? selected[0] : selected;
				console.log("Selected folder:", targetFolderPath);
			}
		} catch (error) {
			console.error("Folder dialog error:", error);
			alert(`Dialog xətası: ${error}`);
		}
	}

	async function startCopying() {
		if (!sourceFilePath) {
			alert("Mənbə faylı seçin");
			return;
		}
		
		if (!targetFolderPath) {
			alert("Hədəf qovluğunu seçin");
			return;
		}

		isProcessing = true;
		progress = 0;
		processLog = [];
		currentStep = "Başlanılır...";
		totalFolders = 0;
		successCount = 0;
		errorCount = 0;

		try {
			const result = await invoke("copy_file_to_all_subfolders", {
				sourceFile: sourceFilePath,
				targetFolder: targetFolderPath
			});
			
			console.log("Copy operation completed:", result);
			currentStep = "Tamamlandı!";
			
			// Update statistics
			const results = result as any[];
			totalFolders = results.length;
			successCount = results.filter(r => r.success).length;
			errorCount = results.filter(r => !r.success).length;
			
		} catch (error) {
			console.error("Copy operation failed:", error);
			currentStep = "Xəta baş verdi";
			processLog = [...processLog, {
				type: "error",
				message: `❌ Xəta: ${error}`,
				timestamp: new Date().toLocaleTimeString()
			}];
		} finally {
			isProcessing = false;
		}
	}

	function resetProcess() {
		sourceFilePath = "";
		targetFolderPath = "";
		isProcessing = false;
		progress = 0;
		processLog = [];
		currentStep = "";
		totalFolders = 0;
		successCount = 0;
		errorCount = 0;
	}

	function getFileName(path: string) {
		return path.split(/[\\/]/).pop() || path;
	}

	function showHelp() {
		showHelpModal = true;
	}

	function closeHelp() {
		showHelpModal = false;
	}
</script>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold text-text-primary mb-2">Fayl Kopyalama</h1>
			<p class="text-text-secondary">Seçilmiş faylı bütün alt qovluqlara kopyalayın</p>
		</div>
		
		{#if !isProcessing}
			<div class="flex items-center space-x-2">
				<button 
					on:click={showHelp}
					class="w-10 h-10 rounded-full flex items-center justify-center transition-all duration-300 text-text-muted hover:text-accent-orange hover:bg-accent-orange/10 hover:scale-105 border border-transparent hover:border-accent-orange"
					title="Kömək"
				>
					<HelpCircle size={20} />
				</button>
				<button 
					on:click={resetProcess}
					class="btn-secondary flex items-center space-x-2"
					title="Sıfırla"
				>
					<RotateCcw size={16} />
					<span>Sıfırla</span>
				</button>
			</div>
		{/if}
	</div>

	<!-- Configuration Panel -->
	<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
		<!-- Source File Selection -->
		<div class="card">
			<h3 class="text-lg font-semibold text-text-primary mb-4 flex items-center space-x-2">
				<FileText size={20} class="text-accent-cyan" />
				<span>Mənbə Fayl</span>
			</h3>
			
			<div class="space-y-4">
				<button
					on:click={selectSourceFile}
					class="btn-primary w-full"
					disabled={isProcessing}
				>
					Fayl Seçin
				</button>
				{#if sourceFilePath}
					<div class="bg-dark-secondary rounded-lg p-3">
						<p class="text-text-secondary text-sm mb-1">Seçilmiş fayl:</p>
						<p class="text-text-primary font-medium break-all">{getFileName(sourceFilePath)}</p>
					</div>
				{/if}
			</div>
		</div>

		<!-- Target Folder Selection -->
		<div class="card">
			<h3 class="text-lg font-semibold text-text-primary mb-4 flex items-center space-x-2">
				<FolderOpen size={20} class="text-accent-orange" />
				<span>Hədəf Qovluq</span>
			</h3>
			
			<div class="space-y-4">
				<button
					on:click={selectTargetFolder}
					class="btn-primary w-full"
					disabled={isProcessing}
				>
					Qovluğu Seçin
				</button>
				{#if targetFolderPath}
					<div class="bg-dark-secondary rounded-lg p-3">
						<p class="text-text-secondary text-sm mb-1">Seçilmiş qovluq:</p>
						<p class="text-text-primary font-medium break-all">{targetFolderPath}</p>
					</div>
				{/if}
			</div>
		</div>
	</div>

	<!-- Statistics -->
	{#if totalFolders > 0 || isProcessing}
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
			<div class="card bg-gradient-to-r from-blue-600/20 to-cyan-600/20 border-blue-500/30">
				<div class="flex items-center space-x-3">
					<div class="w-10 h-10 bg-blue-500/20 rounded-full flex items-center justify-center">
						<FolderOpen size={20} class="text-blue-400" />
					</div>
					<div>
						<p class="text-text-secondary text-sm">Ümumi Qovluq</p>
						<p class="text-2xl font-bold text-blue-400">{totalFolders}</p>
					</div>
				</div>
			</div>
			
			<div class="card bg-gradient-to-r from-green-600/20 to-emerald-600/20 border-green-500/30">
				<div class="flex items-center space-x-3">
					<div class="w-10 h-10 bg-green-500/20 rounded-full flex items-center justify-center">
						<Copy size={20} class="text-green-400" />
					</div>
					<div>
						<p class="text-text-secondary text-sm">Uğurlu</p>
						<p class="text-2xl font-bold text-green-400">{successCount}</p>
					</div>
				</div>
			</div>
			
			<div class="card bg-gradient-to-r from-red-600/20 to-pink-600/20 border-red-500/30">
				<div class="flex items-center space-x-3">
					<div class="w-10 h-10 bg-red-500/20 rounded-full flex items-center justify-center">
						<X size={20} class="text-red-400" />
					</div>
					<div>
						<p class="text-text-secondary text-sm">Xəta</p>
						<p class="text-2xl font-bold text-red-400">{errorCount}</p>
					</div>
				</div>
			</div>
		</div>
	{/if}

	<!-- Control Panel -->
	<div class="card">
		<div class="flex items-center justify-between mb-4">
			<h3 class="text-lg font-semibold text-text-primary">İdarəetmə</h3>
			
			<button
				on:click={startCopying}
				disabled={!sourceFilePath || !targetFolderPath || isProcessing}
				class="btn-primary flex items-center space-x-2 disabled:opacity-50"
			>
				<Copy size={16} />
				<span>Kopyalamağa Başla</span>
			</button>
		</div>

		<!-- Progress Section -->
		{#if isProcessing}
			<div class="space-y-4">
				<div class="flex items-center justify-between">
					<span class="text-text-secondary text-sm">{currentStep}</span>
					<span class="text-text-primary font-medium">{progress}%</span>
				</div>
				
				<div class="w-full bg-dark-secondary rounded-full h-3">
					<div 
						class="bg-accent-orange h-3 rounded-full transition-all duration-300"
						style="width: {progress}%"
					></div>
				</div>
			</div>
		{/if}
	</div>

	<!-- Process Log -->
	{#if processLog.length > 0}
		<div class="card">
			<h3 class="text-lg font-semibold text-text-primary mb-4">Proses Loqu</h3>
			<div class="bg-dark-secondary rounded-lg p-4 max-h-64 overflow-y-auto">
				<div class="space-y-2">
					{#each processLog as log}
						<div class="flex items-start space-x-2 text-sm">
							<span class="text-text-muted text-xs">{log.timestamp}</span>
							<span class={log.type === 'success' ? 'text-accent-green' : log.type === 'error' ? 'text-accent-red' : 'text-text-secondary'}>
								{log.message}
							</span>
						</div>
					{/each}
				</div>
			</div>
		</div>
	{/if}
</div>

<!-- Help Modal -->
{#if showHelpModal}
	<div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" on:click={closeHelp}>
		<div class="bg-dark-card rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-[80vh] overflow-y-auto" on:click|stopPropagation>
			<div class="flex items-center justify-between p-6 border-b border-dark-border">
				<h2 class="text-xl font-bold text-text-primary">📁 Fayl Kopyalama Modulu</h2>
				<button 
					on:click={closeHelp}
					class="text-text-muted hover:text-text-primary"
				>
					<X size={24} />
				</button>
			</div>
			
			<div class="p-6 space-y-6 text-text-secondary">
				<div>
					<h3 class="text-lg font-semibold text-text-primary mb-3 flex items-center space-x-2">
						<span>🎯</span>
						<span>Modulun Təyinatı</span>
					</h3>
					<p class="leading-relaxed">Bu modul seçilmiş bir faylı hədəf qovluğun içindəki bütün alt qovluqlara avtomatik olaraq kopyalayır. Bu, eyni faylı çoxlu qovluqlara tez bir zamanda yaymaq üçün çox faydalıdır.</p>
				</div>
				
				<div>
					<h3 class="text-lg font-semibold text-text-primary mb-3 flex items-center space-x-2">
						<span>📋</span>
						<span>İş Prinsipi — Addım-addım</span>
					</h3>
					<div class="space-y-3">
						<div class="flex items-start space-x-3">
							<span class="bg-accent-orange text-white rounded-full w-6 h-6 flex items-center justify-center text-sm font-bold mt-0.5">1</span>
							<div>
								<p class="font-medium text-text-primary">Mənbə faylı seçin</p>
								<p class="text-sm text-text-muted">Kopyalanacaq faylı seçin (istənilən format ola bilər).</p>
							</div>
						</div>
						<div class="flex items-start space-x-3">
							<span class="bg-accent-orange text-white rounded-full w-6 h-6 flex items-center justify-center text-sm font-bold mt-0.5">2</span>
							<div>
								<p class="font-medium text-text-primary">Hədəf qovluğunu seçin</p>
								<p class="text-sm text-text-muted">Faylın kopyalanacağı əsas qovluğu seçin.<br>→ Modul bu qovluğun bütün alt qovluqlarını tapacaq.</p>
							</div>
						</div>
						<div class="flex items-start space-x-3">
							<span class="bg-accent-orange text-white rounded-full w-6 h-6 flex items-center justify-center text-sm font-bold mt-0.5">3</span>
							<div>
								<p class="font-medium text-text-primary">"Kopyalamağa Başla" düyməsinə klikləyin</p>
								<p class="text-sm text-text-muted">Proses avtomatik olaraq başlayacaq və real vaxtda izlənə biləcək.</p>
							</div>
						</div>
					</div>
				</div>
				
				<div>
					<h3 class="text-lg font-semibold text-text-primary mb-3 flex items-center space-x-2">
						<span>💡</span>
						<span>İstifadə Nümunəsi</span>
					</h3>
					<div class="bg-dark-secondary p-4 rounded-lg space-y-3">
						<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
							<div>
								<p class="text-accent-cyan font-medium mb-2">📁 Başlanğıc struktur:</p>
								<div class="text-sm space-y-1 text-text-muted">
									<p><strong class="text-text-primary">Mənbə fayl:</strong> readme.txt</p>
									<p><strong class="text-text-primary">Hədəf qovluq:</strong> Layihələr</p>
									<p><strong class="text-text-primary">Alt qovluqlar:</strong> Layihə1, Layihə2, Layihə3...</p>
								</div>
							</div>
							<div>
								<p class="text-accent-green font-medium mb-2">✅ Nəticə:</p>
								<div class="text-sm space-y-1 text-text-muted">
									<p>• readme.txt faylı Layihə1 qovluğuna kopyalanır</p>
									<p>• readme.txt faylı Layihə2 qovluğuna kopyalanır</p>
									<p>• readme.txt faylı Layihə3 qovluğuna kopyalanır</p>
									<p>• Bütün alt qovluqlarda eyni fayl olur</p>
								</div>
							</div>
						</div>
					</div>
				</div>
				
				<div>
					<h3 class="text-lg font-semibold text-text-primary mb-3 flex items-center space-x-2">
						<span>⚡</span>
						<span>Xüsusiyyətlər</span>
					</h3>
					<div class="bg-emerald-900/20 border border-emerald-600/30 rounded-lg p-4">
						<ul class="space-y-2 text-sm">
							<li class="flex items-start space-x-2">
								<span class="text-accent-green">✓</span>
								<span><strong>Sürətli işləmə:</strong> Optimizasiya edilmiş kopyalama</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-accent-green">✓</span>
								<span><strong>Real vaxt izləmə:</strong> Proses və progress göstəricisi</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-accent-green">✓</span>
								<span><strong>Rekursiv axtarış:</strong> Bütün alt qovluqlar avtomatik tapılır</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-accent-green">✓</span>
								<span><strong>Təfərrüatlı statistika:</strong> Uğurlu və xətalı əməliyyatların sayı</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-accent-green">✓</span>
								<span><strong>Təfərrüatlı log:</strong> Hər qovluq üçün nəticə məlumatı</span>
							</li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}
 