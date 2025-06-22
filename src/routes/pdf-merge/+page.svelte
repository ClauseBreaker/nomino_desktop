<script lang="ts">
	import { onMount } from "svelte";
	import { invoke } from "@tauri-apps/api/tauri";
	import { open } from "@tauri-apps/api/dialog";
	import { Layers, FolderOpen, FileText, Play, Pause, Square, RotateCcw, Merge, X } from "lucide-svelte";

	// State variables
	let isTauriApp = false;
	let rootFolderPath = "";
	let isProcessing = false;
	let progress = 0;
	let currentStep = "";
	let deleteOriginalFiles = false;


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

	async function selectRootFolder() {
		if (!isTauriApp) {
			alert("Bu funksiyanın işləməsi üçün Tauri tətbiqi lazımdır.");
			return;
		}
		
		try {
			const selected = await open({
				directory: true,
				multiple: false,
				title: "PDF faylları olan əsas qovluğu seçin"
			});
			
			if (selected && selected !== null && selected !== "") {
				rootFolderPath = Array.isArray(selected) ? selected[0] : selected;
				console.log("Selected folder:", rootFolderPath);
			}
		} catch (error) {
			console.error("Folder dialog error:", error);
			alert(`Dialog xətası: ${error}`);
		}
	}

	async function startMerging() {
		if (!rootFolderPath) {
			alert("Əvvəlcə qovluq seçin");
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
			const config = {
				root_folder: rootFolderPath,
				delete_original_files: deleteOriginalFiles
			};

			const result = await invoke("merge_pdf_files", { config });
			
			console.log("PDF merge completed:", result);
			currentStep = "Tamamlandı!";
			
			// Update statistics
			const results = result as any[];
			totalFolders = results.length;
			successCount = results.filter(r => r.success).length;
			errorCount = results.filter(r => !r.success).length;
			
		} catch (error) {
			console.error("PDF merge failed:", error);
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
		rootFolderPath = "";
		isProcessing = false;
		progress = 0;
		processLog = [];
		currentStep = "";
		totalFolders = 0;
		successCount = 0;
		errorCount = 0;
		deleteOriginalFiles = false;
	}


</script>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold text-text-primary mb-2">PDF Birləşdirici</h1>
			<p class="text-text-secondary">Hər alt qovluqdakı PDF faylları birləşdirin</p>
		</div>
		
		{#if !isProcessing}
			<button 
				on:click={resetProcess}
				class="btn-secondary flex items-center space-x-2"
				title="Sıfırla"
			>
				<RotateCcw size={16} />
				<span>Sıfırla</span>
			</button>
		{/if}
	</div>

	<!-- Configuration Panel -->
	<div class="card">
		<h3 class="text-lg font-semibold text-text-primary mb-4 flex items-center space-x-2">
			<FolderOpen size={20} class="text-accent-cyan" />
			<span>Əsas Qovluq Seçimi</span>
		</h3>
		
		<div class="space-y-4">
			<button
				on:click={selectRootFolder}
				class="btn-primary"
				disabled={isProcessing}
			>
				Qovluğu Seçin
			</button>
			{#if rootFolderPath}
				<div class="bg-dark-secondary rounded-lg p-3">
					<p class="text-text-secondary text-sm mb-1">Seçilmiş qovluq:</p>
					<p class="text-text-primary font-medium break-all">{rootFolderPath}</p>
				</div>
			{/if}
			
			<!-- Delete Original Files Option -->
			<div class="flex items-center space-x-3 p-3 bg-dark-secondary rounded-lg">
				<input
					type="checkbox"
					id="deleteOriginal"
					bind:checked={deleteOriginalFiles}
					disabled={isProcessing}
					class="w-4 h-4 text-accent-orange bg-dark-card border-gray-600 rounded focus:ring-accent-orange focus:ring-2"
				/>
				<label for="deleteOriginal" class="text-text-primary text-sm font-medium cursor-pointer">
					Orijinal PDF faylları sil (birləşdirmədən sonra)
				</label>
			</div>
		</div>
	</div>

	<!-- Information Panel -->
	<div class="card bg-gradient-to-r from-blue-600/10 to-cyan-600/10 border-blue-500/20">
		<h3 class="text-lg font-semibold text-text-primary mb-3 flex items-center space-x-2">
			<Layers size={20} class="text-blue-400" />
			<span>Necə işləyir?</span>
		</h3>
		<div class="space-y-2 text-text-secondary text-sm">
			<p>• Seçilmiş qovluqdakı hər alt qovluq axtarılacaq</p>
			<p>• Hər alt qovluqdakı bütün PDF faylları bir yerə birləşdiriləcək</p>
			<p>• Nəticə faylı: <span class="text-accent-cyan font-mono">{'{qovluq_adı}'}_iddia_ərizəsi_və_əlavə_sənədlər.pdf</span></p>
			<p>• Orijinal fayllar: 
				{#if deleteOriginalFiles}
					<span class="text-accent-red">silinəcək</span>
				{:else}
					<span class="text-accent-green">saxlanılacaq</span>
				{/if}
			</p>
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
						<Layers size={20} class="text-green-400" />
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
				on:click={startMerging}
				disabled={!rootFolderPath || isProcessing}
				class="btn-primary flex items-center space-x-2 disabled:opacity-50"
			>
				<Layers size={16} />
				<span>Birləşdirməyə Başla</span>
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

 