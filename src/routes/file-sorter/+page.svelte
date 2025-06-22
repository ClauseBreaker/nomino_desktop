<script lang="ts">
	import { onMount } from "svelte";
	import { invoke } from "@tauri-apps/api/tauri";
	import { open } from "@tauri-apps/api/dialog";
	import { ArrowRightLeft, FolderOpen, FileText, Play, Pause, Square, RotateCcw, HelpCircle, X, Hash } from "lucide-svelte";

	// State variables
	let isTauriApp = false;
	let filesFolder = "";
	let foldersFolder = "";
	let charCount = 7;
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
	let totalFiles = 0;
	let movedFiles = 0;
	let notMatchedFiles = 0;

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
						type: data.success ? "success" : (data.message.includes("Nəzərdə tutulmur") ? "info" : "error"),
						message: data.message,
						timestamp: new Date().toLocaleTimeString()
					}];
					
					if (data.success) {
						movedFiles++;
					} else if (data.message.includes("Nəzərdə tutulmur")) {
						notMatchedFiles++;
					}
					
					console.log("Process result:", data);
				});
			});
		}
	});

	async function selectFilesFolder() {
		if (!isTauriApp) {
			alert("Bu funksiyanın işləməsi üçün Tauri tətbiqi lazımdır.");
			return;
		}
		
		try {
			const selected = await open({
				directory: true,
				multiple: false,
				title: "Fayllar qovluğunu seçin"
			});
			
			if (selected && selected !== null && selected !== "") {
				filesFolder = Array.isArray(selected) ? selected[0] : selected;
				console.log("Selected files folder:", filesFolder);
			}
		} catch (error) {
			console.error("Folder dialog error:", error);
			alert(`Dialog xətası: ${error}`);
		}
	}

	async function selectFoldersFolder() {
		if (!isTauriApp) {
			alert("Bu funksiyanın işləməsi üçün Tauri tətbiqi lazımdır.");
			return;
		}
		
		try {
			const selected = await open({
				directory: true,
				multiple: false,
				title: "Hədəf qovluqlar qovluğunu seçin"
			});
			
			if (selected && selected !== null && selected !== "") {
				foldersFolder = Array.isArray(selected) ? selected[0] : selected;
				console.log("Selected folders folder:", foldersFolder);
			}
		} catch (error) {
			console.error("Folder dialog error:", error);
			alert(`Dialog xətası: ${error}`);
		}
	}

	async function startSorting() {
		if (!filesFolder) {
			alert("Fayllar qovluğunu seçin");
			return;
		}
		
		if (!foldersFolder) {
			alert("Hədəf qovluqlar qovluğunu seçin");
			return;
		}

		if (charCount < 1 || charCount > 50) {
			alert("Simvol sayı 1-50 arasında olmalıdır");
			return;
		}

		isProcessing = true;
		progress = 0;
		processLog = [];
		currentStep = "Başlanılır...";
		totalFiles = 0;
		movedFiles = 0;
		notMatchedFiles = 0;

		try {
			const config = {
				files_folder: filesFolder,
				folders_folder: foldersFolder,
				char_count: charCount
			};

			const result = await invoke("sort_files_by_folders", { config });
			
			console.log("Sort operation completed:", result);
			currentStep = "Tamamlandı!";
			
			// Update statistics
			const results = result as any[];
			totalFiles = results.length;
			
		} catch (error) {
			console.error("Sort operation failed:", error);
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

	async function pauseProcess() {
		if (!isTauriApp) return;
		
		try {
			await invoke("pause_process");
			isPaused = true;
		} catch (error) {
			console.error("Pause failed:", error);
		}
	}

	async function resumeProcess() {
		if (!isTauriApp) return;
		
		try {
			await invoke("resume_process");
			isPaused = false;
		} catch (error) {
			console.error("Resume failed:", error);
		}
	}

	async function stopProcess() {
		if (!isTauriApp) return;
		
		try {
			await invoke("stop_process");
			isProcessing = false;
			isPaused = false;
		} catch (error) {
			console.error("Stop failed:", error);
		}
	}

	function resetProcess() {
		filesFolder = "";
		foldersFolder = "";
		charCount = 7;
		isProcessing = false;
		isPaused = false;
		progress = 0;
		processLog = [];
		currentStep = "";
		totalFiles = 0;
		movedFiles = 0;
		notMatchedFiles = 0;
	}

	function showHelp() {
		showHelpModal = true;
	}

	function closeHelp() {
		showHelpModal = false;
	}

	function handleCharCountInput(event: Event) {
		const target = event.target as HTMLInputElement;
		let value = parseInt(target.value);
		
		if (isNaN(value) || value < 1) {
			charCount = 1;
		} else if (value > 50) {
			charCount = 50;
		} else {
			charCount = value;
		}
	}
</script>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold text-text-primary mb-2">Fayl Sıralayıcı</h1>
			<p class="text-text-secondary">Faylları qovluqlara simvol uyğunluğuna görə sıralayın</p>
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
		<!-- Files Folder Selection -->
		<div class="card">
			<h3 class="text-lg font-semibold text-text-primary mb-4 flex items-center space-x-2">
				<FileText size={20} class="text-accent-cyan" />
				<span>Fayllar Qovluğu</span>
			</h3>
			
			<div class="space-y-4">
				<button
					on:click={selectFilesFolder}
					class="btn-primary w-full"
					disabled={isProcessing}
				>
					Fayllar Qovluğunu Seçin
				</button>
				{#if filesFolder}
					<div class="bg-dark-secondary rounded-lg p-3">
						<p class="text-text-secondary text-sm mb-1">Seçilmiş qovluq:</p>
						<p class="text-text-primary font-medium break-all">{filesFolder}</p>
					</div>
				{/if}
			</div>
		</div>

		<!-- Target Folders Selection -->
		<div class="card">
			<h3 class="text-lg font-semibold text-text-primary mb-4 flex items-center space-x-2">
				<FolderOpen size={20} class="text-accent-orange" />
				<span>Hədəf Qovluqlar</span>
			</h3>
			
			<div class="space-y-4">
				<button
					on:click={selectFoldersFolder}
					class="btn-primary w-full"
					disabled={isProcessing}
				>
					Hədəf Qovluqlar Qovluğunu Seçin
				</button>
				{#if foldersFolder}
					<div class="bg-dark-secondary rounded-lg p-3">
						<p class="text-text-secondary text-sm mb-1">Seçilmiş qovluq:</p>
						<p class="text-text-primary font-medium break-all">{foldersFolder}</p>
					</div>
				{/if}
			</div>
		</div>
	</div>

	<!-- Character Count Setting -->
	<div class="card">
		<h3 class="text-lg font-semibold text-text-primary mb-4 flex items-center space-x-2">
			<Hash size={20} class="text-accent-purple" />
			<span>Müqayisə Parametrləri</span>
		</h3>
		
		<div class="flex items-center space-x-4">
			<div class="flex items-center space-x-2">
				<label class="text-text-secondary">Simvol sayı:</label>
				<input
					type="number"
					bind:value={charCount}
					on:input={handleCharCountInput}
					min="1"
					max="50"
					class="w-20 px-3 py-2 bg-dark-secondary border border-dark-border rounded-lg text-text-primary focus:ring-2 focus:ring-accent-orange focus:border-accent-orange"
					disabled={isProcessing}
				/>
			</div>
			<p class="text-text-muted text-sm">
				Fayl və qovluq adlarının ilk <span class="text-accent-orange font-medium">{charCount}</span> simvolu müqayisə ediləcək
			</p>
		</div>
	</div>

	<!-- Statistics -->
	{#if totalFiles > 0 || isProcessing}
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
			<div class="card bg-gradient-to-r from-blue-600/20 to-cyan-600/20 border-blue-500/30">
				<div class="flex items-center space-x-3">
					<div class="w-10 h-10 bg-blue-500/20 rounded-full flex items-center justify-center">
						<FileText size={20} class="text-blue-400" />
					</div>
					<div>
						<p class="text-text-secondary text-sm">Ümumi Fayl</p>
						<p class="text-2xl font-bold text-blue-400">{totalFiles}</p>
					</div>
				</div>
			</div>
			
			<div class="card bg-gradient-to-r from-green-600/20 to-emerald-600/20 border-green-500/30">
				<div class="flex items-center space-x-3">
					<div class="w-10 h-10 bg-green-500/20 rounded-full flex items-center justify-center">
						<ArrowRightLeft size={20} class="text-green-400" />
					</div>
					<div>
						<p class="text-text-secondary text-sm">Köçürülən</p>
						<p class="text-2xl font-bold text-green-400">{movedFiles}</p>
					</div>
				</div>
			</div>
			
			<div class="card bg-gradient-to-r from-yellow-600/20 to-orange-600/20 border-yellow-500/30">
				<div class="flex items-center space-x-3">
					<div class="w-10 h-10 bg-yellow-500/20 rounded-full flex items-center justify-center">
						<X size={20} class="text-yellow-400" />
					</div>
					<div>
						<p class="text-text-secondary text-sm">Uyğun Deyil</p>
						<p class="text-2xl font-bold text-yellow-400">{notMatchedFiles}</p>
					</div>
				</div>
			</div>
		</div>
	{/if}

	<!-- Control Panel -->
	<div class="card">
		<div class="flex items-center justify-between mb-4">
			<h3 class="text-lg font-semibold text-text-primary">İdarəetmə</h3>
			
			{#if !isProcessing}
				<button
					on:click={startSorting}
					disabled={!filesFolder || !foldersFolder}
					class="btn-primary flex items-center space-x-2 disabled:opacity-50"
				>
					<ArrowRightLeft size={16} />
					<span>Sıralamağa Başla</span>
				</button>
			{:else}
				<div class="flex items-center space-x-2">
					{#if isPaused}
						<button
							on:click={resumeProcess}
							class="btn-success flex items-center space-x-2"
						>
							<Play size={16} />
							<span>Davam Et</span>
						</button>
					{:else}
						<button
							on:click={pauseProcess}
							class="btn-warning flex items-center space-x-2"
						>
							<Pause size={16} />
							<span>Fasilə</span>
						</button>
					{/if}
					
					<button
						on:click={stopProcess}
						class="btn-danger flex items-center space-x-2"
					>
						<Square size={16} />
						<span>Dayandır</span>
					</button>
				</div>
			{/if}
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
							<span class={log.type === 'success' ? 'text-accent-green' : log.type === 'error' ? 'text-accent-red' : 'text-accent-yellow'}>
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
				<h2 class="text-xl font-bold text-text-primary">📁 Fayl Sıralayıcı Modulu</h2>
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
					<p class="leading-relaxed">Bu modul faylları qovluqlara simvol uyğunluğuna görə avtomatik sıralayır. Fayl adının ilk N simvolu qovluq adının ilk N simvolu ilə uyğun gələrsə, fayl həmin qovluğa köçürülür.</p>
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
								<p class="font-medium text-text-primary">Fayllar qovluğunu seçin</p>
								<p class="text-sm text-text-muted">Sıralanacaq faylların olduğu qovluğu seçin.</p>
							</div>
						</div>
						<div class="flex items-start space-x-3">
							<span class="bg-accent-orange text-white rounded-full w-6 h-6 flex items-center justify-center text-sm font-bold mt-0.5">2</span>
							<div>
								<p class="font-medium text-text-primary">Hədəf qovluqlar qovluğunu seçin</p>
								<p class="text-sm text-text-muted">Faylların köçürüləcəyi qovluqların olduğu əsas qovluğu seçin.</p>
							</div>
						</div>
						<div class="flex items-start space-x-3">
							<span class="bg-accent-orange text-white rounded-full w-6 h-6 flex items-center justify-center text-sm font-bold mt-0.5">3</span>
							<div>
								<p class="font-medium text-text-primary">Simvol sayını təyin edin</p>
								<p class="text-sm text-text-muted">Müqayisə ediləcək simvol sayını seçin (1-50 arası).</p>
							</div>
						</div>
						<div class="flex items-start space-x-3">
							<span class="bg-accent-orange text-white rounded-full w-6 h-6 flex items-center justify-center text-sm font-bold mt-0.5">4</span>
							<div>
								<p class="font-medium text-text-primary">"Sıralamağa Başla" düyməsinə klikləyin</p>
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
						<div class="space-y-3">
							<div>
								<p class="text-accent-cyan font-medium mb-2">📋 Parametrlər:</p>
								<div class="text-sm space-y-1 text-text-muted">
									<p><strong class="text-text-primary">Simvol sayı:</strong> 7</p>
									<p><strong class="text-text-primary">Fayl:</strong> document_2024.pdf (ilk 7 simvol: "documen")</p>
									<p><strong class="text-text-primary">Qovluq:</strong> document_archive (ilk 7 simvol: "documen")</p>
								</div>
							</div>
							<div>
								<p class="text-accent-green font-medium mb-2">✅ Nəticə:</p>
								<div class="text-sm text-text-muted">
									<p>Fayl "document_archive" qovluğuna köçürülür, çünki ilk 7 simvol uyğun gəlir.</p>
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
								<span><strong>Sürətli işləmə:</strong> Optimizasiya edilmiş fayl köçürməsi</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-accent-green">✓</span>
								<span><strong>Real vaxt izləmə:</strong> Proses və progress göstəricisi</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-accent-green">✓</span>
								<span><strong>Azərbaycan dili dəstəyi:</strong> Ə, Ş, Ç, Ö, Ü, Ğ, İ hərfləri</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-accent-green">✓</span>
								<span><strong>Proses idarəetməsi:</strong> Fasilə, davam etdirmə, dayandırma</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-accent-green">✓</span>
								<span><strong>Təfərrüatlı statistika:</strong> Köçürülən və uyğun olmayan fayllar</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-accent-green">✓</span>
								<span><strong>Təfərrüatlı log:</strong> Hər fayl üçün nəticə məlumatı</span>
							</li>
						</ul>
					</div>
				</div>
				
				<div>
					<h3 class="text-lg font-semibold text-text-primary mb-3 flex items-center space-x-2">
						<span class="bg-red-100 text-red-600 w-6 h-6 rounded-full flex items-center justify-center text-sm font-bold">⚠</span>
						<span>Vacib Qeydlər</span>
					</h3>
					<div class="bg-red-900/20 border border-red-600/30 rounded-lg p-4">
						<ul class="space-y-2 text-sm">
							<li class="flex items-start space-x-2">
								<span class="text-red-400">•</span>
								<span>Fayllar köçürülür (orijinal yerdən silinir)</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-red-400">•</span>
								<span>Eyni adlı fayl varsa, əvəz edilə bilər</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-red-400">•</span>
								<span>Uyğunluq olmayan fayllar öz yerində qalır</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-red-400">•</span>
								<span>Prosesi başlatmadan əvvəl yedəkləmə tövsiyə olunur</span>
							</li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Remove spinner buttons from number inputs */
	input[type="number"]::-webkit-outer-spin-button,
	input[type="number"]::-webkit-inner-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}

	input[type="number"] {
		-moz-appearance: textfield;
	}
</style> 