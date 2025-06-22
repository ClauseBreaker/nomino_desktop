<script lang="ts">
	import { onMount } from "svelte";
	import { invoke } from "@tauri-apps/api/tauri";
	import { open } from "@tauri-apps/api/dialog";
	import { Calendar, FolderOpen, FileText, Play, Pause, Square, RotateCcw, HelpCircle, X, Search } from "lucide-svelte";

	// State variables
	let isTauriApp = false;
	let rootFolderPath = "";
	let newDate = "";
	let keyword = "İDDİA";
	let deleteOriginal = false;
	let isProcessing = false;
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
	let successCount = 0;
	let errorCount = 0;

	// Set default date to today
	onMount(() => {
		// Check if Tauri is available
		isTauriApp = typeof window !== "undefined" && typeof window["__TAURI_IPC__"] === "function";
		console.log("Tauri available:", isTauriApp);
		
		// Set default date to today in dd.MM.yyyy format
		const today = new Date();
		const day = String(today.getDate()).padStart(2, '0');
		const month = String(today.getMonth() + 1).padStart(2, '0');
		const year = today.getFullYear();
		newDate = `${day}.${month}.${year}`;
		
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
				title: "PDF faylları olan qovluğu seçin"
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

	async function startProcessing() {
		if (!rootFolderPath) {
			alert("Əvvəlcə qovluq seçin");
			return;
		}
		
		if (!newDate) {
			alert("Yeni tarix daxil edin");
			return;
		}
		
		if (!keyword.trim()) {
			alert("Açar söz daxil edin");
			return;
		}

		isProcessing = true;
		progress = 0;
		processLog = [];
		currentStep = "Başlanılır...";
		totalFiles = 0;
		successCount = 0;
		errorCount = 0;

		try {
			const config = {
				root_folder: rootFolderPath,
				new_date: newDate,
				keyword: keyword.trim(),
				delete_original: deleteOriginal
			};

			const result = await invoke("change_pdf_dates", { config });
			
			console.log("PDF date change completed:", result);
			currentStep = "Tamamlandı!";
			
			// Update statistics
			const results = result as any[];
			totalFiles = results.length;
			successCount = results.filter(r => r.success).length;
			errorCount = results.filter(r => !r.success).length;
			
		} catch (error) {
			console.error("PDF date change failed:", error);
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
		keyword = "İDDİA";
		deleteOriginal = false;
		isProcessing = false;
		progress = 0;
		processLog = [];
		currentStep = "";
		totalFiles = 0;
		successCount = 0;
		errorCount = 0;
		
		// Reset date to today
		const today = new Date();
		const day = String(today.getDate()).padStart(2, '0');
		const month = String(today.getMonth() + 1).padStart(2, '0');
		const year = today.getFullYear();
		newDate = `${day}.${month}.${year}`;
	}

	function showHelp() {
		showHelpModal = true;
	}

	function closeHelp() {
		showHelpModal = false;
	}

	function validateDate(input: string): boolean {
		const dateRegex = /^(\d{2})\.(\d{2})\.(\d{4})$/;
		const match = input.match(dateRegex);
		if (!match) return false;
		
		const day = parseInt(match[1], 10);
		const month = parseInt(match[2], 10);
		const year = parseInt(match[3], 10);
		
		if (month < 1 || month > 12) return false;
		if (day < 1 || day > 31) return false;
		if (year < 1900 || year > 2100) return false;
		
		return true;
	}

	function handleDateInput(event: Event) {
		const target = event.target as HTMLInputElement;
		newDate = target.value;
	}
</script>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold text-text-primary mb-2">PDF Tarix Dəyişdiricisi</h1>
			<p class="text-text-secondary">PDF fayllarındakı tarixi dəyişdirin</p>
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
		<!-- Root Folder Selection -->
		<div class="card">
			<h3 class="text-lg font-semibold text-text-primary mb-4 flex items-center space-x-2">
				<FolderOpen size={20} class="text-accent-cyan" />
				<span>Əsas Qovluq</span>
			</h3>
			
			<div class="space-y-4">
				<button
					on:click={selectRootFolder}
					class="btn-primary w-full"
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
			</div>
		</div>

		<!-- Configuration Options -->
		<div class="card">
			<h3 class="text-lg font-semibold text-text-primary mb-4 flex items-center space-x-2">
				<Calendar size={20} class="text-accent-orange" />
				<span>Konfiqurasiya</span>
			</h3>
			
			<div class="space-y-4">
				<!-- New Date Input -->
				<div>
					<label class="block text-text-secondary text-sm mb-2">Yeni Tarix (dd.MM.yyyy)</label>
					<input
						type="text"
						bind:value={newDate}
						on:input={handleDateInput}
						placeholder="01.01.2024"
						disabled={isProcessing}
						class="w-full px-3 py-2 bg-dark-secondary border border-dark-border rounded-lg text-text-primary focus:border-accent-orange focus:outline-none"
						class:border-red-500={newDate && !validateDate(newDate)}
					/>
					{#if newDate && !validateDate(newDate)}
						<p class="text-red-400 text-xs mt-1">Yanlış tarix formatı. dd.MM.yyyy formatında daxil edin</p>
					{/if}
				</div>

				<!-- Keyword Input -->
				<div>
					<label class="block text-text-secondary text-sm mb-2">Açar Söz</label>
					<div class="relative">
						<Search size={16} class="absolute left-3 top-1/2 transform -translate-y-1/2 text-text-muted" />
						<input
							type="text"
							bind:value={keyword}
							placeholder="İDDİA"
							disabled={isProcessing}
							class="w-full pl-10 pr-3 py-2 bg-dark-secondary border border-dark-border rounded-lg text-text-primary focus:border-accent-orange focus:outline-none"
						/>
					</div>
					<p class="text-text-muted text-xs mt-1">Fayl adında axtarılacaq söz</p>
				</div>

				<!-- Delete Original Checkbox -->
				<div class="flex items-center space-x-2">
					<input
						type="checkbox"
						id="deleteOriginal"
						bind:checked={deleteOriginal}
						disabled={isProcessing}
						class="w-4 h-4 rounded border-dark-border bg-dark-secondary text-accent-orange focus:ring-accent-orange focus:ring-2"
					/>
					<label for="deleteOriginal" class="text-text-secondary text-sm">
						Orijinal faylı sil
					</label>
				</div>
			</div>
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
						<p class="text-text-secondary text-sm">Ümumi PDF</p>
						<p class="text-2xl font-bold text-blue-400">{totalFiles}</p>
					</div>
				</div>
			</div>
			
			<div class="card bg-gradient-to-r from-green-600/20 to-emerald-600/20 border-green-500/30">
				<div class="flex items-center space-x-3">
					<div class="w-10 h-10 bg-green-500/20 rounded-full flex items-center justify-center">
						<Calendar size={20} class="text-green-400" />
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
				on:click={startProcessing}
				disabled={!rootFolderPath || !newDate || !validateDate(newDate) || !keyword.trim() || isProcessing}
				class="btn-primary flex items-center space-x-2 disabled:opacity-50"
			>
				<Calendar size={16} />
				<span>İşləməyə Başla</span>
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
				<h2 class="text-xl font-bold text-text-primary">📅 PDF Tarix Dəyişdiricisi</h2>
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
					<p class="leading-relaxed">Bu modul PDF fayllarında olan tarixi avtomatik olaraq yeni tarixlə əvəz edir. Xüsusilə hüquqi sənədlərdə tarix dəyişikliyi üçün istifadə olunur.</p>
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
								<p class="font-medium text-text-primary">Əsas qovluğu seçin</p>
								<p class="text-sm text-text-muted">PDF faylları olan qovluğu seçin. Program bütün alt qovluqlarda da axtarış edəcək.</p>
							</div>
						</div>
						<div class="flex items-start space-x-3">
							<span class="bg-accent-orange text-white rounded-full w-6 h-6 flex items-center justify-center text-sm font-bold mt-0.5">2</span>
							<div>
								<p class="font-medium text-text-primary">Yeni tarixi daxil edin</p>
								<p class="text-sm text-text-muted">dd.MM.yyyy formatında yeni tarix yazın (məsələn: 15.03.2024).</p>
							</div>
						</div>
						<div class="flex items-start space-x-3">
							<span class="bg-accent-orange text-white rounded-full w-6 h-6 flex items-center justify-center text-sm font-bold mt-0.5">3</span>
							<div>
								<p class="font-medium text-text-primary">Açar sözü təyin edin</p>
								<p class="text-sm text-text-muted">Fayl adında axtarılacaq söz (default: "İDDİA").</p>
							</div>
						</div>
						<div class="flex items-start space-x-3">
							<span class="bg-accent-orange text-white rounded-full w-6 h-6 flex items-center justify-center text-sm font-bold mt-0.5">4</span>
							<div>
								<p class="font-medium text-text-primary">"İşləməyə Başla" düyməsinə klikləyin</p>
								<p class="text-sm text-text-muted">Proses avtomatik başlayacaq və hər fayl üçün "_new" sufiksi ilə yeni fayl yaradacaq.</p>
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
								<p class="text-accent-cyan font-medium mb-2">📁 Giriş:</p>
								<div class="text-sm space-y-1 text-text-muted">
									<p><strong class="text-text-primary">Qovluq:</strong> Sənədlər/</p>
									<p><strong class="text-text-primary">Fayl:</strong> İDDİA_2023.pdf</p>
									<p><strong class="text-text-primary">Köhnə tarix:</strong> 15.12.2023</p>
									<p><strong class="text-text-primary">Yeni tarix:</strong> 20.03.2024</p>
								</div>
							</div>
							<div>
								<p class="text-accent-green font-medium mb-2">✅ Nəticə:</p>
								<div class="text-sm space-y-1 text-text-muted">
									<p>• PDF-də tarix 20.03.2024 olaraq dəyişir</p>
									<p>• Yeni fayl: İDDİA_2023_new.pdf</p>
									<p>• Orijinal fayl saxlanılır (seçimə görə)</p>
									<p>• Bütün alt qovluqlarda eyni əməliyyat</p>
								</div>
							</div>
						</div>
					</div>
				</div>
				
				<div>
					<h3 class="text-lg font-semibold text-text-primary mb-3 flex items-center space-x-2">
						<span>⚠️</span>
						<span>Diqqət</span>
					</h3>
					<div class="bg-orange-900/20 border border-orange-600/30 rounded-lg p-4">
						<ul class="space-y-2 text-sm">
							<li class="flex items-start space-x-2">
								<span class="text-orange-400">⚠️</span>
								<span><strong>Ehtiyat nüsxə:</strong> Mühüm faylları işləməzdən əvvəl nüsxələyin</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-orange-400">⚠️</span>
								<span><strong>Tarix formatı:</strong> Yalnız dd.MM.yyyy formatı dəstəklənir</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-orange-400">⚠️</span>
								<span><strong>Son tarix:</strong> PDF-də tapılan son tarix dəyişdirilir</span>
							</li>
						</ul>
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
								<span><strong>Rekursiv axtarış:</strong> Bütün alt qovluqlarda axtarış</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-accent-green">✓</span>
								<span><strong>Açar söz filtri:</strong> Yalnız müəyyən adlı faylları işləyir</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-accent-green">✓</span>
								<span><strong>Təhlükəsiz:</strong> Orijinal faylı qoruyur (_new sufiksi)</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-accent-green">✓</span>
								<span><strong>Real vaxt izləmə:</strong> Proses və nəticələrin izlənməsi</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-accent-green">✓</span>
								<span><strong>Statistika:</strong> Uğurlu və xətalı əməliyyatların sayı</span>
							</li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if} 