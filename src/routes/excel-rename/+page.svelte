<script lang="ts">
	import { onMount } from "svelte";
	import { invoke } from "@tauri-apps/api/tauri";
	import { open } from "@tauri-apps/api/dialog";
	import { FileSpreadsheet, FolderOpen, Play, RotateCcw, CheckSquare, Square, HelpCircle, X } from "lucide-svelte";

	// State variables
	let isTauriApp = false;
	let folderPath = "";
	let excelPath = "";
	let isProcessing = false;
	let progress = 0;
	let currentStep = "";
	
	// Configuration
	let mode = "original"; // "original" or "digits"
	let startRow = "2";
	let column = "C";
	let startFileName = "";
	let digitCount = "";
	let digitFromEnd = false;
	let limitFiles = false;
	let limitCount = "5";
	let limitChars = false;
	let charCount = "5";
	let charFromEnd = false;

	// Process log
	interface LogEntry {
		type: "success" | "error" | "info";
		message: string;
		timestamp: string;
	}
	let processLog: LogEntry[] = [];

	// Statistics
	let totalFiles = 0;
	let renamedCount = 0;
	let errorCount = 0;

	// Help modal
	let showHelpModal = false;

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
						renamedCount++;
					} else {
						errorCount++;
					}
					
					console.log("Process result:", data);
				});
			});
		}
	});

	async function selectFolder() {
		if (!isTauriApp) {
			alert("Bu funksiyanın işləməsi üçün Tauri tətbiqi lazımdır.");
			return;
		}
		
		try {
			const selected = await open({
				directory: true,
				multiple: false,
				title: "Fayllar olan qovluğu seçin"
			});
			
			if (selected && selected !== null && selected !== "") {
				folderPath = Array.isArray(selected) ? selected[0] : selected;
				console.log("Selected folder:", folderPath);
			}
		} catch (error) {
			console.error("Folder dialog error:", error);
			alert(`Dialog xətası: ${error}`);
		}
	}

	async function selectExcel() {
		if (!isTauriApp) {
			alert("Bu funksiyanın işləməsi üçün Tauri tətbiqi lazımdır.");
			return;
		}
		
		try {
			const selected = await open({
				filters: [{
					name: "Excel Files",
					extensions: ["xlsx"]
				}],
				multiple: false,
				title: "Excel faylını seçin"
			});
			
			if (selected && selected !== null && selected !== "") {
				excelPath = Array.isArray(selected) ? selected[0] : selected;
				console.log("Selected Excel:", excelPath);
			}
		} catch (error) {
			console.error("Excel dialog error:", error);
			alert(`Dialog xətası: ${error}`);
		}
	}

	async function startRenaming() {
		if (!folderPath) {
			alert("Əvvəlcə qovluq seçin");
			return;
		}

		if (!excelPath) {
			alert("Əvvəlcə Excel fayl seçin");
			return;
		}

		isProcessing = true;
		progress = 0;
		processLog = [];
		currentStep = "Başlanılır...";
		totalFiles = 0;
		renamedCount = 0;
		errorCount = 0;

		try {
			const config = {
				folder_path: folderPath,
				excel_path: excelPath,
				mode: mode,
				start_row: parseInt(startRow),
				column: column,
				start_file_name: startFileName,
				digit_count: digitCount ? parseInt(digitCount) : null,
				digit_from_end: digitFromEnd,
				limit_files: limitFiles,
				limit_count: limitFiles ? parseInt(limitCount) : null,
				limit_chars: limitChars,
				char_count: limitChars ? parseInt(charCount) : null,
				char_from_end: charFromEnd
			};

			const result = await invoke("rename_files_from_excel_advanced", { config });
			
			console.log("Excel renaming completed:", result);
			currentStep = "Tamamlandı!";
			
			// Update statistics
			const results = result as any[];
			totalFiles = results.length;
			renamedCount = results.filter(r => r.success).length;
			errorCount = results.filter(r => !r.success).length;
			
		} catch (error) {
			console.error("Excel renaming failed:", error);
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
		folderPath = "";
		excelPath = "";
		isProcessing = false;
		progress = 0;
		processLog = [];
		currentStep = "";
		totalFiles = 0;
		renamedCount = 0;
		errorCount = 0;
		// Reset configuration to defaults
		mode = "original";
		startRow = "2";
		column = "C";
		startFileName = "";
		digitCount = "";
		digitFromEnd = false;
		limitFiles = false;
		limitCount = "5";
		limitChars = false;
		charCount = "5";
		charFromEnd = false;
	}

	function showHelp() {
		showHelpModal = true;
	}

	function closeHelp() {
		showHelpModal = false;
	}

	function handleColumnInput(e: Event) {
		const target = e.target as HTMLInputElement;
		// Only allow Latin letters and convert to uppercase
		target.value = target.value.replace(/[^A-Za-z]/g, '').toUpperCase();
		column = target.value;
	}
</script>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold text-text-primary mb-2">Excel Fayl Adı Dəyişdirici</h1>
			<p class="text-text-secondary">Excel məlumatlarına əsasən faylların adını dəyişdirin</p>
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
	<div class="card">
		<h3 class="text-lg font-semibold text-text-primary mb-4 flex items-center space-x-2">
			<FileSpreadsheet size={20} class="text-accent-cyan" />
			<span>Əsas Parametrlər</span>
		</h3>
		
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<!-- Folder Selection -->
			<div class="space-y-2">
				<label class="text-text-primary text-sm font-medium">Fayllar olan qovluq:</label>
				<div class="flex space-x-2">
					<button
						on:click={selectFolder}
						class="btn-primary flex items-center space-x-2"
						disabled={isProcessing}
					>
						<FolderOpen size={16} />
						<span>Seçin</span>
					</button>
					{#if folderPath}
						<div class="flex-1 bg-dark-secondary rounded-lg p-2">
							<p class="text-text-primary text-sm break-all">{folderPath}</p>
						</div>
					{/if}
				</div>
			</div>

			<!-- Excel Selection -->
			<div class="space-y-2">
				<label class="text-text-primary text-sm font-medium">Excel fayl (.xlsx):</label>
				<div class="flex space-x-2">
					<button
						on:click={selectExcel}
						class="btn-primary flex items-center space-x-2"
						disabled={isProcessing}
					>
						<FileSpreadsheet size={16} />
						<span>Seçin</span>
					</button>
					{#if excelPath}
						<div class="flex-1 bg-dark-secondary rounded-lg p-2">
							<p class="text-text-primary text-sm break-all">{excelPath}</p>
						</div>
					{/if}
				</div>
			</div>
		</div>
	</div>

	<!-- Mode and Excel Parameters -->
	<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
		<!-- Mode Selection -->
		<div class="card">
			<h3 class="text-lg font-semibold text-text-primary mb-4">İş Rejimi</h3>
			<div class="space-y-4">
				<div class="flex items-center space-x-3">
					<input
						type="radio"
						id="mode-original"
						bind:group={mode}
						value="original"
						disabled={isProcessing}
						class="w-4 h-4 text-accent-orange"
					/>
					<label for="mode-original" class="text-text-primary font-medium cursor-pointer">
						Əsas rejim
					</label>
				</div>
				<div class="flex items-center space-x-3">
					<input
						type="radio"
						id="mode-digits"
						bind:group={mode}
						value="digits"
						disabled={isProcessing}
						class="w-4 h-4 text-accent-orange"
					/>
					<label for="mode-digits" class="text-text-primary font-medium cursor-pointer">
						Rəqəmlə rejim
					</label>
				</div>

				{#if mode === "original"}
					<div class="bg-dark-secondary p-3 rounded-lg">
						<label class="text-text-primary text-sm font-medium">Hansı fayldan başlasın:</label>
						<input
							type="text"
							bind:value={startFileName}
							disabled={isProcessing}
							placeholder="Fayl adı (uzantısız)"
							class="w-full mt-1 px-3 py-2 bg-dark-card border border-dark-border rounded-lg text-text-primary placeholder-text-muted focus:outline-none focus:ring-2 focus:ring-accent-orange"
						/>
					</div>
				{:else}
					<div class="bg-dark-secondary p-3 rounded-lg space-y-3">
						<div>
							<label class="text-text-primary text-sm font-medium">Neçə rəqəm:</label>
							<input
								type="number"
								bind:value={digitCount}
								disabled={isProcessing}
								placeholder="Rəqəm sayı"
								style="-moz-appearance: textfield;"
								class="w-full mt-1 px-3 py-2 bg-dark-card border border-dark-border rounded-lg text-text-primary placeholder-text-muted focus:outline-none focus:ring-2 focus:ring-accent-orange [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
							/>
						</div>
						<div class="flex items-center space-x-3">
							<button
								on:click={() => digitFromEnd = !digitFromEnd}
								disabled={isProcessing}
								class="flex items-center space-x-2 text-text-primary"
							>
								{#if digitFromEnd}
									<CheckSquare size={20} class="text-accent-orange" />
								{:else}
									<Square size={20} class="text-text-muted" />
								{/if}
								<span>Axırdan say</span>
							</button>
						</div>
					</div>
				{/if}
			</div>
		</div>

		<!-- Excel Parameters -->
		<div class="card">
			<h3 class="text-lg font-semibold text-text-primary mb-4">Excel Parametrləri</h3>
			<div class="space-y-4">
				<div>
					<label class="text-text-primary text-sm font-medium">Başlanğıc sətir:</label>
					<input
						type="number"
						bind:value={startRow}
						disabled={isProcessing}
						min="1"
						style="-moz-appearance: textfield;"
						class="w-full mt-1 px-3 py-2 bg-dark-card border border-dark-border rounded-lg text-text-primary focus:outline-none focus:ring-2 focus:ring-accent-orange [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
					/>
				</div>
				<div>
					<label class="text-text-primary text-sm font-medium">Sütun:</label>
					<input
						type="text"
						bind:value={column}
						disabled={isProcessing}
						placeholder="Məsələn: C"
						maxlength="3"
						on:input={handleColumnInput}
						class="w-full mt-1 px-3 py-2 bg-dark-card border border-dark-border rounded-lg text-text-primary placeholder-text-muted focus:outline-none focus:ring-2 focus:ring-accent-orange"
					/>
				</div>
			</div>
		</div>
	</div>

	<!-- Limits and Options -->
	<div class="card">
		<h3 class="text-lg font-semibold text-text-primary mb-4">Məhdudiyyətlər və Seçimlər</h3>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<!-- File Limit -->
			<div class="space-y-3">
				<div class="flex items-center space-x-3">
					<button
						on:click={() => limitFiles = !limitFiles}
						disabled={isProcessing}
						class="flex items-center space-x-2 text-text-primary"
					>
						{#if limitFiles}
							<CheckSquare size={20} class="text-accent-orange" />
						{:else}
							<Square size={20} class="text-text-muted" />
						{/if}
						<span>Fayl sayını məhdudlaşdır</span>
					</button>
				</div>
				{#if limitFiles}
					<input
						type="number"
						bind:value={limitCount}
						disabled={isProcessing}
						min="1"
						placeholder="Maksimum fayl sayı"
						style="-moz-appearance: textfield;"
						class="w-full px-3 py-2 bg-dark-card border border-dark-border rounded-lg text-text-primary placeholder-text-muted focus:outline-none focus:ring-2 focus:ring-accent-orange [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
					/>
				{/if}
			</div>

			<!-- Character Limit -->
			<div class="space-y-3">
				<div class="flex items-center space-x-3">
					<button
						on:click={() => limitChars = !limitChars}
						disabled={isProcessing}
						class="flex items-center space-x-2 text-text-primary"
					>
						{#if limitChars}
							<CheckSquare size={20} class="text-accent-orange" />
						{:else}
							<Square size={20} class="text-text-muted" />
						{/if}
						<span>Yalnız N simvol dəyiş</span>
					</button>
				</div>
				{#if limitChars}
					<div class="space-y-2">
						<input
							type="number"
							bind:value={charCount}
							disabled={isProcessing}
							min="1"
							placeholder="Simvol sayı"
							style="-moz-appearance: textfield;"
							class="w-full px-3 py-2 bg-dark-card border border-dark-border rounded-lg text-text-primary placeholder-text-muted focus:outline-none focus:ring-2 focus:ring-accent-orange [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
						/>
						<div class="flex items-center space-x-3">
							<button
								on:click={() => charFromEnd = !charFromEnd}
								disabled={isProcessing}
								class="flex items-center space-x-2 text-text-primary"
							>
								{#if charFromEnd}
									<CheckSquare size={20} class="text-accent-orange" />
								{:else}
									<Square size={20} class="text-text-muted" />
								{/if}
								<span>Axırdan</span>
							</button>
						</div>
					</div>
				{/if}
			</div>
		</div>
	</div>

	<!-- Information Panel -->
	<div class="card bg-gradient-to-r from-blue-600/10 to-cyan-600/10 border-blue-500/20">
		<h3 class="text-lg font-semibold text-text-primary mb-3 flex items-center space-x-2">
			<FileSpreadsheet size={20} class="text-blue-400" />
			<span>Necə işləyir?</span>
		</h3>
		<div class="space-y-2 text-text-secondary text-sm">
			<p>• Excel faylından məlumatları oxuyur və faylların adını dəyişdirir</p>
			<p>• İki rejim: Əsas (sıralı) və Rəqəmlə (rəqəmli fayl adları)</p>
			<p>• Fayl sayı və simvol sayını məhdudlaşdıra bilərsiniz</p>
			<p>• Excel sütunundakı məlumatlar fayl adı olur (boşluq _ ilə əvəz olunur)</p>
		</div>
	</div>

	<!-- Statistics -->
	{#if totalFiles > 0 || isProcessing}
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
			<div class="card bg-gradient-to-r from-blue-600/20 to-cyan-600/20 border-blue-500/30">
				<div class="flex items-center space-x-3">
					<div class="w-10 h-10 bg-blue-500/20 rounded-full flex items-center justify-center">
						<FileSpreadsheet size={20} class="text-blue-400" />
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
						<CheckSquare size={20} class="text-green-400" />
					</div>
					<div>
						<p class="text-text-secondary text-sm">Uğurlu</p>
						<p class="text-2xl font-bold text-green-400">{renamedCount}</p>
					</div>
				</div>
			</div>
			
			<div class="card bg-gradient-to-r from-red-600/20 to-pink-600/20 border-red-500/30">
				<div class="flex items-center space-x-3">
					<div class="w-10 h-10 bg-red-500/20 rounded-full flex items-center justify-center">
						<Square size={20} class="text-red-400" />
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
				on:click={startRenaming}
				disabled={!folderPath || !excelPath || isProcessing}
				class="btn-primary flex items-center space-x-2 disabled:opacity-50"
			>
				<Play size={16} />
				<span>Adlandırmağa Başla</span>
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
		<div class="bg-dark-card rounded-lg shadow-xl max-w-4xl w-full mx-4 max-h-[90vh] overflow-y-auto" on:click|stopPropagation>
			<div class="flex items-center justify-between p-6 border-b border-dark-border">
				<h2 class="text-xl font-bold text-text-primary">📊 Excel Fayl Adı Dəyişdirici - Təfərrüatlı Təlimat</h2>
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
					<p class="leading-relaxed">Bu modul Excel faylındakı məlumatları oxuyaraq faylların adını toplu şəkildə dəyişdirir. Hüquqi sənədlər, şəkillər və digər faylların sistemli adlandırılması üçün ideal həlldir.</p>
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
								<p class="font-medium text-text-primary">Excel faylını hazırlayın</p>
								<p class="text-sm text-text-muted">Excel faylında yeni adlar olan sütun yaradın (məsələn C sütunu).</p>
							</div>
						</div>
						<div class="flex items-start space-x-3">
							<span class="bg-accent-orange text-white rounded-full w-6 h-6 flex items-center justify-center text-sm font-bold mt-0.5">2</span>
							<div>
								<p class="font-medium text-text-primary">Fayllar olan qovluğu seçin</p>
								<p class="text-sm text-text-muted">Adını dəyişmək istədiyiniz faylların olduğu qovluğu seçin.</p>
							</div>
						</div>
						<div class="flex items-start space-x-3">
							<span class="bg-accent-orange text-white rounded-full w-6 h-6 flex items-center justify-center text-sm font-bold mt-0.5">3</span>
							<div>
								<p class="font-medium text-text-primary">Excel faylını seçin və parametrləri təyin edin</p>
								<p class="text-sm text-text-muted">Başlanğıc sətir və sütunu göstərin.</p>
							</div>
						</div>
						<div class="flex items-start space-x-3">
							<span class="bg-accent-orange text-white rounded-full w-6 h-6 flex items-center justify-center text-sm font-bold mt-0.5">4</span>
							<div>
								<p class="font-medium text-text-primary">Rejimi və məhdudiyyətləri seçin</p>
								<p class="text-sm text-text-muted">İş rejimi və lazımsa məhdudiyyətləri təyin edin.</p>
							</div>
						</div>
						<div class="flex items-start space-x-3">
							<span class="bg-accent-orange text-white rounded-full w-6 h-6 flex items-center justify-center text-sm font-bold mt-0.5">5</span>
							<div>
								<p class="font-medium text-text-primary">"Adlandırmağa Başla" düyməsinə klikləyin</p>
								<p class="text-sm text-text-muted">Proses avtomatik başlayacaq və nəticələri izləyə bilərsiniz.</p>
							</div>
						</div>
					</div>
				</div>
				
				<div>
					<h3 class="text-lg font-semibold text-text-primary mb-3 flex items-center space-x-2">
						<span>⚙️</span>
						<span>İş Rejimləri</span>
					</h3>
					<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
						<div class="bg-blue-900/20 border border-blue-600/30 rounded-lg p-4">
							<h4 class="font-semibold text-blue-400 mb-2">🔄 Əsas Rejim</h4>
							<ul class="space-y-1 text-sm">
								<li>• Bütün faylları təbii sıralama ilə işləyir</li>
								<li>• İstəsəniz müəyyən fayldan başlaya bilərsiniz</li>
								<li>• Excel sırasına uyğun adlandırır</li>
								<li>• Ən çox istifadə olunan rejimdir</li>
							</ul>
						</div>
						<div class="bg-green-900/20 border border-green-600/30 rounded-lg p-4">
							<h4 class="font-semibold text-green-400 mb-2">🔢 Rəqəmlə Rejim</h4>
							<ul class="space-y-1 text-sm">
								<li>• Yalnız rəqəmli adları olan faylları işləyir</li>
								<li>• Rəqəm sayını məhdudlaşdıra bilərsiniz</li>
								<li>• Başdan və ya axırdan rəqəmləri sayır</li>
								<li>• Xüsusi hallarda istifadə olunur</li>
							</ul>
						</div>
					</div>
				</div>
				
				<div>
					<h3 class="text-lg font-semibold text-text-primary mb-3 flex items-center space-x-2">
						<span>📊</span>
						<span>Excel Parametrləri</span>
					</h3>
					<div class="bg-dark-secondary p-4 rounded-lg">
						<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
							<div>
								<h4 class="font-semibold text-accent-cyan mb-2">Başlanğıc Sətir</h4>
								<p class="text-sm text-text-muted">Excel faylında məlumatların başladığı sətir nömrəsi. Adətən başlıq sətirindən sonra 2-ci sətir olur.</p>
							</div>
							<div>
								<h4 class="font-semibold text-accent-cyan mb-2">Sütun</h4>
								<p class="text-sm text-text-muted">Yeni fayl adlarının olduğu sütun (A, B, C və s.). Məsələn: C sütunu üçün "C" yazın.</p>
							</div>
						</div>
					</div>
				</div>
				
				<div>
					<h3 class="text-lg font-semibold text-text-primary mb-3 flex items-center space-x-2">
						<span>🎛️</span>
						<span>Məhdudiyyətlər və Seçimlər</span>
					</h3>
					<div class="space-y-4">
						<div class="bg-yellow-900/20 border border-yellow-600/30 rounded-lg p-4">
							<h4 class="font-semibold text-yellow-400 mb-2">📊 Fayl Sayını Məhdudlaşdır</h4>
							<p class="text-sm">Neçə fayl işlənəcəyini məhdudlaşdırır. Məsələn yalnız ilk 10 faylı adlandırmaq üçün istifadə edin.</p>
						</div>
						<div class="bg-purple-900/20 border border-purple-600/30 rounded-lg p-4">
							<h4 class="font-semibold text-purple-400 mb-2">✂️ Yalnız N Simvol Dəyiş</h4>
							<p class="text-sm">Fayl adının yalnız müəyyən hissəsini dəyişdirir:</p>
							<ul class="text-sm mt-2 space-y-1">
								<li>• <strong>Başdan:</strong> İlk N simvolu Excel adı ilə əvəz edir</li>
								<li>• <strong>Axırdan:</strong> Son N simvolu Excel adı ilə əvəz edir</li>
							</ul>
						</div>
					</div>
				</div>
				
				<div>
					<h3 class="text-lg font-semibold text-text-primary mb-3 flex items-center space-x-2">
						<span>💡</span>
						<span>İstifadə Nümunələri</span>
					</h3>
					<div class="space-y-4">
						<div class="bg-dark-secondary p-4 rounded-lg">
							<h4 class="font-semibold text-accent-green mb-3">📄 Nümunə 1: Sənəd Adlandırması</h4>
							<div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
								<div>
									<p class="text-accent-cyan font-medium mb-2">📁 Fayllar:</p>
									<div class="text-sm space-y-1 text-text-muted font-mono">
										<p>document1.pdf</p>
										<p>document2.pdf</p>
										<p>document3.pdf</p>
									</div>
								</div>
								<div>
									<p class="text-accent-cyan font-medium mb-2">📊 Excel (C sütunu):</p>
									<div class="text-sm space-y-1 text-text-muted font-mono">
										<p>Əli Məmmədov</p>
										<p>Leyla Əliyeva</p>
										<p>Rəşad Həsənov</p>
									</div>
								</div>
							</div>
							<div class="mt-4">
								<p class="text-accent-green font-medium mb-2">✅ Nəticə:</p>
								<div class="text-sm space-y-1 text-text-muted font-mono">
									<p>Əli_Məmmədov.pdf</p>
									<p>Leyla_Əliyeva.pdf</p>
									<p>Rəşad_Həsənov.pdf</p>
								</div>
							</div>
						</div>
						
						<div class="bg-dark-secondary p-4 rounded-lg">
							<h4 class="font-semibold text-accent-orange mb-3">🖼️ Nümunə 2: Şəkil Adlandırması (Hissəvi)</h4>
							<div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
								<div>
									<p class="text-accent-cyan font-medium mb-2">📁 Fayllar:</p>
									<div class="text-sm space-y-1 text-text-muted font-mono">
										<p>IMG_001.jpg</p>
										<p>IMG_002.jpg</p>
										<p>IMG_003.jpg</p>
									</div>
								</div>
								<div>
									<p class="text-accent-cyan font-medium mb-2">⚙️ Parametrlər:</p>
									<div class="text-sm space-y-1 text-text-muted">
										<p>• Yalnız 3 simvol dəyiş (başdan)</p>
										<p>• Excel: "AAA", "BBB", "CCC"</p>
									</div>
								</div>
							</div>
							<div class="mt-4">
								<p class="text-accent-green font-medium mb-2">✅ Nəticə:</p>
								<div class="text-sm space-y-1 text-text-muted font-mono">
									<p>AAA_001.jpg</p>
									<p>BBB_002.jpg</p>
									<p>CCC_003.jpg</p>
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
								<span><strong>Avtomatik təmizləmə:</strong> Excel adlarındakı boşluqlar "_" ilə əvəz olunur</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-accent-green">✓</span>
								<span><strong>Təhlükəsiz:</strong> Fayl uzantıları saxlanılır</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-accent-green">✓</span>
								<span><strong>Təbii sıralama:</strong> 1, 2, 3... (deyil 1, 10, 11, 2...)</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-accent-green">✓</span>
								<span><strong>Real vaxt izləmə:</strong> Proses və nəticələrin izlənməsi</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-accent-green">✓</span>
								<span><strong>Çevik parametrlər:</strong> İki rejim və çoxlu seçimlər</span>
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
						<span>⚠️</span>
						<span>Məsləhətlər və Diqqət</span>
					</h3>
					<div class="bg-red-900/20 border border-red-600/30 rounded-lg p-4">
						<ul class="space-y-2 text-sm">
							<li class="flex items-start space-x-2">
								<span class="text-red-400">⚠️</span>
								<span><strong>Ehtiyat nüsxə:</strong> Mühüm faylları adlandırmazdan əvvəl ehtiyat nüsxəsini çıxarın</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-yellow-400">📌</span>
								<span><strong>Excel formatı:</strong> Yalnız .xlsx faylları dəstəklənir</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-yellow-400">📌</span>
								<span><strong>Sütun formatı:</strong> Sütun adını hərflə yazın (A, B, C və s.)</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-yellow-400">📌</span>
								<span><strong>Fayl sayı:</strong> Excel sətir sayı ilə fayl sayı uyğun olmalıdır</span>
							</li>
							<li class="flex items-start space-x-2">
								<span class="text-blue-400">💡</span>
								<span><strong>Test edin:</strong> Əvvəlcə az sayda fayl ilə test edin</span>
							</li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if} 