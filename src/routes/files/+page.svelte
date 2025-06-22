<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/dialog';
	
	// Check if running in Tauri environment
	let isTauriApp = false;
	import { 
		FileText, 
		FileSpreadsheet, 
		Play,
		Pause,
		RotateCcw,
		CheckCircle,
		AlertCircle,
		ArrowRight,
		Settings,
		HelpCircle,
		X,
		Square
	} from 'lucide-svelte';

	// State variables
	let sourceDirectoryPath = '';
	let destinationDirectoryPath = '';
	let excelFilePath = '';
	let files: any[] = [];
	let sortOrder = 'default'; // default, name, date, size
	let excelStartRow = 1;
	let excelColumn = 'A';
	let isProcessing = false;
	let isPaused = false;
	let progress = 0;
	let processLog: any[] = [];
	let currentStep = '';
	let showHelpModal = false;

	// Sort options with corrected translations
	const sortOptions = [
		{ value: 'default', label: 'Orijinal' },
		{ value: 'name', label: 'Ada görə' },
		{ value: 'date', label: 'Tarixə görə' },
		{ value: 'size', label: 'Ölçüyə görə' }
	];

	onMount(() => {
		// Check if Tauri is available
		isTauriApp = typeof window !== 'undefined' && typeof window['__TAURI_IPC__'] === 'function';
		console.log('Tauri available:', isTauriApp);
		
		if (!isTauriApp) {
			console.warn('Tauri not available. Please run with: npm run tauri:dev');
		}
		
		// Listen for progress updates
		if (isTauriApp) {
			import('@tauri-apps/api/event').then(({ listen }) => {
				listen('progress-update', (event: any) => {
					const data = event.payload as any;
					progress = Math.round(data.percentage); // Make progress integer
					currentStep = data.current_step;
					console.log('Progress:', data);
				});
				
				listen('process-result', (event: any) => {
					const data = event.payload as any;
					processLog = [...processLog, {
						type: data.success ? 'success' : 'error',
						message: data.message,
						timestamp: new Date().toLocaleTimeString()
					}];
					console.log('Process result:', data);
				});
			});
		}
		
		loadInitialData();
	});

	async function loadInitialData() {
		// Initialize any required data
	}

	async function selectSourceDirectory() {
		// Check if Tauri is available
		if (!isTauriApp) {
			alert('Bu funksiyanın işləməsi üçün Tauri tətbiqi lazımdır.\nXahiş olunur "npm run tauri:dev" ilə tətbiqi işə salın.\n\n(This function requires Tauri app. Please run with "npm run tauri:dev")');
			return;
		}
		
		try {
			console.log('Attempting to open directory dialog...');
			
			const selected = await open({
				directory: true,
				multiple: false,
				title: 'Əsas qovluğunu seçin (Select Source Directory)'
			});
			
			console.log('Dialog result:', selected);
			
			if (selected && selected !== null && selected !== '') {
				sourceDirectoryPath = Array.isArray(selected) ? selected[0] : selected;
				console.log('Selected directory:', sourceDirectoryPath);
				await loadFiles();
			}
		} catch (error) {
			console.error('Dialog error:', error);
			alert(`Dialog xətası: ${error}`);
		}
	}

	async function selectDestinationDirectory() {
		if (!isTauriApp) {
			alert('Bu funksiyanın işləməsi üçün Tauri tətbiqi lazımdır.\nXahiş olunur "npm run tauri:dev" ilə tətbiqi işə salın.');
			return;
		}
		
		try {
			const selected = await open({
				directory: true,
				multiple: false,
				title: 'Təyinat qovluğunu seçin (Select Destination Directory)'
			});
			
			if (selected && selected !== null && selected !== '') {
				destinationDirectoryPath = Array.isArray(selected) ? selected[0] : selected;
			}
		} catch (error) {
			console.error('Failed to select destination directory:', error);
			alert(`Təyinat qovluq seçmə xətası: ${error}`);
		}
	}

	async function selectExcelFile() {
		if (!isTauriApp) {
			alert('Bu funksiyanın işləməsi üçün Tauri tətbiqi lazımdır.\nXahiş olunur "npm run tauri:dev" ilə tətbiqi işə salın.');
			return;
		}
		
		try {
			const selected = await open({
				multiple: false,
				filters: [{
					name: 'Excel Files',
					extensions: ['xlsx', 'xls', 'csv']
				}],
				title: 'Excel faylını seçin (Select Excel File)'
			});
			
			if (selected && selected !== null && selected !== '') {
				excelFilePath = Array.isArray(selected) ? selected[0] : selected;
			}
		} catch (error) {
			console.error('Failed to select Excel file:', error);
			alert(`Excel fayl seçmə xətası: ${error}`);
		}
	}

	async function loadFiles() {
		if (!sourceDirectoryPath) return;
		
		try {
			// Use backend function to get files
			const result = await invoke('get_files_with_sorting', { 
				path: sourceDirectoryPath,
				sortOrder: sortOrder
			});
			files = result as any[];
		} catch (error) {
			console.error('Failed to load files:', error);
			files = [];
		}
	}

	async function startProcessing() {
		if (!sourceDirectoryPath || !destinationDirectoryPath || !excelFilePath || files.length === 0) {
			alert('Bütün sahələri doldurun');
			return;
		}

		isProcessing = true;
		isPaused = false;
		progress = 0;
		processLog = [];
		currentStep = 'Başlanılır...';

		try {
			const result = await invoke('rename_files_from_excel', {
				sourcePath: sourceDirectoryPath,
				destinationPath: destinationDirectoryPath,
				excelPath: excelFilePath,
				startRow: excelStartRow,
				column: excelColumn,
				sortOrder: sortOrder,
				files: files.map((f: any) => f.name)
			});
			
			// Process completed successfully
			currentStep = 'Tamamlandı!';
			console.log('Process completed:', result);
			
		} catch (error) {
			console.error('Process failed:', error);
			currentStep = 'Xəta baş verdi';
			processLog = [...processLog, {
				type: 'error',
				message: `❌ Xəta: ${error}`,
				timestamp: new Date().toLocaleTimeString()
			}];
		} finally {
			isProcessing = false;
			isPaused = false;
		}
	}

	async function pauseProcessing() {
		if (!isProcessing) return;
		
		try {
			if (isPaused) {
				await invoke('resume_process');
				isPaused = false;
				currentStep = 'Davam edir...';
			} else {
				await invoke('pause_process');
				isPaused = true;
				currentStep = 'Fasilə verildi';
			}
		} catch (error) {
			console.error('Failed to pause/resume:', error);
		}
	}

	async function stopProcessing() {
		if (!isProcessing) return;
		
		try {
			await invoke('stop_process');
			isProcessing = false;
			isPaused = false;
			currentStep = 'Dayandırıldı';
		} catch (error) {
			console.error('Failed to stop process:', error);
		}
	}

	function resetProcess() {
		// Reset all selections and data
		sourceDirectoryPath = '';
		destinationDirectoryPath = '';
		excelFilePath = '';
		files = [];
		excelStartRow = 1;
		excelColumn = 'A';
		isProcessing = false;
		isPaused = false;
		progress = 0;
		processLog = [];
		currentStep = '';
		sortOrder = 'default';
	}

	function showHelp() {
		showHelpModal = true;
	}

	function closeHelp() {
		showHelpModal = false;
	}

	// Format column input to uppercase Latin letters only
	function formatColumnInput(event: any) {
		const input = event.target;
		let value = input.value;
		
		// Remove non-Latin letters and convert to uppercase
		value = value.replace(/[^A-Za-z]/g, '').toUpperCase();
		
		// Limit to 2 characters max (for columns like AA, AB, etc.)
		if (value.length > 2) {
			value = value.substring(0, 2);
		}
		
		// Update the bound value
		excelColumn = value;
		input.value = value;
	}

	// Watch for sort order changes (only when user changes it)
	$: if (sortOrder && files.length > 0 && sourceDirectoryPath) {
		loadFiles();
	}

	function getFileName(path: string) {
		return path.split(/[\\/]/).pop() || path;
	}

	function formatFileSize(bytes: number) {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
	}
</script>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold text-text-primary mb-2">Fayl Adlandırıcı</h1>
			<p class="text-text-secondary">Excel faylından istifadə edərək faylların adlarını dəyişdirin</p>
			
			{#if !isTauriApp}
				<div class="mt-3 p-3 bg-accent-red/20 border border-accent-red/40 rounded-lg">
					<p class="text-accent-red text-sm font-medium">
						⚠️ Tauri tətbiqi mövcud deyil. File dialog-lar işləməyəcək.
						<br>
						Xahiş olunur: <code class="bg-dark-card px-2 py-1 rounded">npm run tauri:dev</code> ilə tətbiqi işə salın
					</p>
				</div>
			{/if}
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
		<!-- Source Directory Selection -->
		<div class="card">
			<h3 class="text-lg font-semibold text-text-primary mb-4 flex items-center space-x-2">
				<FileText size={20} class="text-accent-cyan" />
				<span>Əsas Qovluq</span>
			</h3>
			
			<div class="space-y-4">
				<div>
					<button
						on:click={selectSourceDirectory}
						class="btn-primary w-full"
						disabled={isProcessing}
					>
						Qovluğu Seçin
					</button>
					{#if sourceDirectoryPath}
						<p class="text-text-secondary text-sm mt-2 break-all">{sourceDirectoryPath}</p>
						<p class="text-accent-green text-sm">📄 {files.length} fayl tapıldı</p>
					{/if}
				</div>

				{#if sourceDirectoryPath}
					<div>
						<label class="block text-text-secondary text-sm font-medium mb-2">
							Sıralama Qaydası
						</label>
						<select 
							bind:value={sortOrder}
							disabled={isProcessing}
							class="input-field w-full"
						>
							{#each sortOptions as option}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
					</div>
				{/if}
			</div>
		</div>

		<!-- Destination & Excel -->
		<div class="card">
			<h3 class="text-lg font-semibold text-text-primary mb-4 flex items-center space-x-2">
				<Settings size={20} class="text-accent-orange" />
				<span>Parametrlər</span>
			</h3>
			
			<div class="space-y-4">
				<!-- Destination Directory -->
				<div>
					<label class="block text-text-secondary text-sm font-medium mb-2">
						Təyinat Qovluq
					</label>
					<button
						on:click={selectDestinationDirectory}
						class="btn-secondary w-full"
						disabled={isProcessing}
					>
						Qovluğu Seçin
					</button>
					{#if destinationDirectoryPath}
						<p class="text-text-secondary text-sm mt-2 break-all">{destinationDirectoryPath}</p>
					{/if}
				</div>

				<!-- Excel File -->
				<div>
					<label class="block text-text-secondary text-sm font-medium mb-2">
						Excel Faylı
					</label>
					<button
						on:click={selectExcelFile}
						class="btn-secondary w-full flex items-center justify-center space-x-2"
						disabled={isProcessing}
					>
						<FileSpreadsheet size={16} />
						<span>Excel Seçin</span>
					</button>
					{#if excelFilePath}
						<p class="text-text-secondary text-sm mt-2">{getFileName(excelFilePath)}</p>
					{/if}
				</div>

				<!-- Excel Parameters -->
				{#if excelFilePath}
					<div class="grid grid-cols-2 gap-4">
						<div>
							<label class="block text-text-secondary text-sm font-medium mb-2">
								Başlanğıc Sətir
							</label>
							<input
								bind:value={excelStartRow}
								type="number"
								min="1"
								disabled={isProcessing}
								class="input-field w-full [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
								placeholder="1"
							/>
						</div>
						<div>
							<label class="block text-text-secondary text-sm font-medium mb-2">
								Sütun
							</label>
							<input
								bind:value={excelColumn}
								type="text"
								disabled={isProcessing}
								class="input-field w-full"
								placeholder="A"
								maxlength="2"
								on:input={formatColumnInput}
							/>
						</div>
					</div>
				{/if}
			</div>
		</div>
	</div>

	<!-- Files Preview -->
	{#if files.length > 0}
		<div class="card">
			<h3 class="text-lg font-semibold text-text-primary mb-4">
				Fayllar Önizləməsi ({files.length} ədəd)
			</h3>
			<div class="max-h-48 overflow-y-auto">
				<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-2">
					{#each files.slice(0, 12) as file, index}
						<div class="bg-dark-secondary rounded-lg p-3 flex items-center space-x-2">
							<span class="text-text-muted text-sm">{index + 1}.</span>
							<div class="flex-1 min-w-0">
								<span class="text-text-primary text-sm truncate block">{file.name}</span>
								<span class="text-text-muted text-xs">{formatFileSize(file.size)}</span>
							</div>
						</div>
					{/each}
					{#if files.length > 12}
						<div class="text-text-muted text-sm p-3">
							və daha {files.length - 12} fayl...
						</div>
					{/if}
				</div>
			</div>
		</div>
	{/if}

	<!-- Control Panel -->
	<div class="card">
		<div class="flex items-center justify-between mb-4">
			<h3 class="text-lg font-semibold text-text-primary">İdarəetmə</h3>
			
			<div class="flex space-x-2">
				{#if !isProcessing}
					<button
						on:click={startProcessing}
						disabled={!sourceDirectoryPath || !destinationDirectoryPath || !excelFilePath || files.length === 0}
						class="btn-primary flex items-center space-x-2 disabled:opacity-50"
					>
						<Play size={16} />
						<span>Başlat</span>
					</button>
				{:else}
					<button
						on:click={pauseProcessing}
						class="btn-secondary flex items-center space-x-2"
					>
						<Pause size={16} />
						<span>{isPaused ? 'Davam Et' : 'Fasilə'}</span>
					</button>
					<button
						on:click={stopProcessing}
						class="btn-secondary bg-accent-red hover:bg-red-600 flex items-center space-x-2"
					>
						<Square size={16} />
						<span>Dayandır</span>
					</button>
				{/if}
			</div>
		</div>

		<!-- Progress Bar -->
		{#if isProcessing || progress > 0}
			<div class="space-y-3">
				<div class="flex justify-between text-sm">
					<span class="text-text-secondary">Gedişat: {progress}%</span>
					<span class="text-text-primary">{currentStep}</span>
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
			<h3 class="text-lg font-semibold text-text-primary mb-4">Proses Jurnalı</h3>
			<div class="max-h-64 overflow-y-auto space-y-2">
				{#each processLog as log}
					<div class="flex items-start space-x-3 p-3 bg-dark-secondary rounded-lg">
						{#if log.type === 'success'}
							<CheckCircle size={16} class="text-accent-green mt-0.5" />
						{:else}
							<AlertCircle size={16} class="text-accent-red mt-0.5" />
						{/if}
						<div class="flex-1">
							<p class="text-text-primary text-sm">{log.message}</p>
							<p class="text-text-muted text-xs">{log.timestamp}</p>
						</div>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>

<!-- Help Modal -->
{#if showHelpModal}
	<div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" on:click={closeHelp}>
		<div class="bg-dark-card border border-dark-border rounded-xl p-6 max-w-2xl w-full m-4 max-h-[80vh] overflow-y-auto" on:click|stopPropagation>
			<div class="flex items-center justify-between mb-4">
				<h2 class="text-xl font-bold text-text-primary">Fayl Adlandırıcı - İstifadəçi Bələdçisi</h2>
				<button 
					on:click={closeHelp}
					class="w-8 h-8 rounded-full flex items-center justify-center hover:bg-dark-secondary transition-colors"
				>
					<X size={18} class="text-text-muted" />
				</button>
			</div>
			
			<div class="space-y-4 text-text-secondary text-sm">
				<div>
					<h3 class="text-accent-orange font-semibold mb-2">Bu modul nə edir?</h3>
					<p>Bu modul sizə bir Excel faylındakı məlumatlardan istifadə edərək bir neçə faylın adını dəyişməyə imkan verir. Excel faylından fayl adlarını oxuyur, mövcud faylların adını uyğun olaraq dəyişdirir və onları təyinat qovluğuna köçürür.</p>
				</div>
				
				<div>
					<h3 class="text-accent-orange font-semibold mb-2">Addım-addım istifadə qaydası:</h3>
					<ol class="list-decimal list-inside space-y-2 pl-4">
						<li><strong>Mənbə Qovluğu Seçin:</strong> Adını dəyişmək istədiyiniz faylları ehtiva edən əsas qovluğu seçin.</li>
						<li><strong>Sıralama Seçin:</strong> Faylların necə sıralanacağını seçin:
							<ul class="list-disc list-inside mt-1 ml-4">
								<li>Orijinal (dəyişməz)</li>
								<li>Ada görə</li>
								<li>Tarixə görə</li>
								<li>Ölçüyə görə</li>
							</ul>
						</li>
						<li><strong>Təyinat Qovluğunu Seçin:</strong> Adları dəyişdirilmiş faylların köçürüləcəyi qovluğu seçin.</li>
						<li><strong>Excel Faylını Seçin:</strong> Yeni adları ehtiva edən Excel faylını seçin (.xlsx, .xls, .csv formatlarında).</li>
						<li><strong>Excel Ayarlarını Tənzimləyin:</strong>
							<ul class="list-disc list-inside mt-1 ml-4">
								<li>Başlanğıc Sətri: Oxumağa başlanacaq sətir nömrəsi (standart: 1)</li>
								<li>Sütun: Adların yerləşdiyi sütun (A, B, C və s.)</li>
							</ul>
						</li>
						<li><strong>Ön Baxış:</strong> Emal olunacaq faylları öncədən gözdən keçirin.</li>
						<li><strong>Prosesi Başladın:</strong> "Başlat" düyməsinə klikləyərək adların dəyişdirilməsi və köçürülmə prosesinə başlayın.</li>
					</ol>
				</div>
				
				<div>
					<h3 class="text-accent-orange font-semibold mb-2">Prosesə Nəzarət Düymələri:</h3>
					<ul class="list-disc list-inside space-y-1 pl-4">
						<li><strong>Fasilə / Davam Et:</strong> Prosesi müvəqqəti olaraq dayandırıb sonra davam edə bilərsiniz.</li>
						<li><strong>Dayandır:</strong> Prosesi tamamilə dayandırır (yenidən başlamaq mümkün deyil).</li>
						<li><strong>Sıfırla:</strong> Bütün seçimləri təmizləyir və yenidən başlamağa imkan verir.</li>
					</ul>
				</div>
				
				<div>
					<h3 class="text-accent-orange font-semibold mb-2">Vacib Qeydlər:</h3>
					<ul class="list-disc list-inside space-y-1 pl-4">
						<li>Excel faylındakı adların sayı, mənbə qovluğundakı faylların sayı ilə eyni olmalıdır.</li>
						<li>Fayllar seçilmiş sıralama üsuluna əsasən emal olunur.</li>
						<li>Adlarda icazəsiz simvollar avtomatik olaraq alt xətlərlə (_) əvəz edilir.</li>
						<li>Proses jurnalı hər əməliyyat barədə ətraflı məlumat verir.</li>
						<li>Fayllar təyinat yerinə köçürülür (surət çıxarılmır).</li>
						<li>Fayl uzantıları qorunur və yeni adlara avtomatik əlavə edilir.</li>
					</ul>
				</div>
				
				<div>
					<h3 class="text-accent-orange font-semibold mb-2">Problemlərin Həlli:</h3>
					<ul class="list-disc list-inside space-y-1 pl-4">
						<li>Həm mənbə, həm də təyinat qovluqlarının mövcud və əlçatan olduğundan əmin olun.</li>
						<li>Excel faylının başqa proqramda açıq olmadığından əmin olun.</li>
						<li>Göstərilən sütun və sətrin gözlənilən məlumatı ehtiva etdiyini yoxlayın.</li>
						<li>Təyinat qovluğuna yazmaq icazənizin olduğunu təsdiqləyin.</li>
						<li>Eyni adlı faylların üzərinə yazılmaması üçün təyinat qovluğunu yoxlayın.</li>
					</ul>
				</div>
			</div>
		</div>
	</div>
{/if} 