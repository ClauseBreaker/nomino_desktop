<script>
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/dialog';
	
	// Check if running in Tauri environment
	let isTauriApp = false;
	import { 
		FolderOpen, 
		FileSpreadsheet, 
		Play,
		Pause,
		RotateCcw,
		CheckCircle,
		AlertCircle,
		ArrowRight,
		Settings
	} from 'lucide-svelte';

	// State variables
	let sourceFolderPath = '';
	let destinationFolderPath = '';
	let excelFilePath = '';
	let folders = [];
	let sortOrder = 'default'; // default, numeric, azerbaijani
	let excelStartRow = 1;
	let excelColumn = 'A';
	let isProcessing = false;
	let isPaused = false;
	let progress = 0;
	let processLog = [];
	let currentStep = '';

	// Sort options
	const sortOptions = [
		{ value: 'default', label: 'Orijinal sıralama (Original filesystem order)' },
		{ value: 'name', label: 'Ada görə (By Name)' },
		{ value: 'date', label: 'Tarixə görə (By Date)' },
		{ value: 'size', label: 'Ölçüyə görə (By Size)' }
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
				listen('progress-update', (event) => {
					const data = event.payload;
					progress = data.percentage;
					currentStep = data.current_step;
					console.log('Progress:', data);
				});
				
				listen('process-result', (event) => {
					const data = event.payload;
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

	async function selectSourceFolder() {
		// Check if Tauri is available
		if (!isTauriApp) {
			alert('Bu funksiyanın işləməsi üçün Tauri tətbiqi lazımdır.\nXahiş olunur "npm run tauri:dev" ilə tətbiqi işə salın.\n\n(This function requires Tauri app. Please run with "npm run tauri:dev")');
			return;
		}
		
		try {
			console.log('Attempting to open folder dialog...');
			
			const selected = await open({
				directory: true,
				multiple: false,
				title: 'Mənbə qovluğunu seçin (Select Source Folder)'
			});
			
			console.log('Dialog result:', selected);
			
			if (selected && selected !== null && selected !== '') {
				sourceFolderPath = Array.isArray(selected) ? selected[0] : selected;
				console.log('Selected folder:', sourceFolderPath);
				await loadFolders();
			}
		} catch (error) {
			console.error('Dialog error:', error);
			alert(`Dialog xətası: ${error}`);
		}
	}

	async function selectDestinationFolder() {
		if (!isTauriApp) {
			alert('Bu funksiyanın işləməsi üçün Tauri tətbiqi lazımdır.\nXahiş olunur "npm run tauri:dev" ilə tətbiqi işə salın.');
			return;
		}
		
		try {
			const selected = await open({
				directory: true,
				multiple: false,
				title: 'Hədəf qovluğunu seçin (Select Destination Folder)'
			});
			
			if (selected && selected !== null && selected !== '') {
				destinationFolderPath = Array.isArray(selected) ? selected[0] : selected;
			}
		} catch (error) {
			console.error('Failed to select destination folder:', error);
			alert(`Hədəf qovluq seçmə xətası: ${error}`);
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

	async function loadFolders() {
		if (!sourceFolderPath) return;
		
		try {
			// Use backend sorting function
			const result = await invoke('get_folders_with_sorting', { 
				path: sourceFolderPath,
				sortOrder: sortOrder
			});
			folders = result;
		} catch (error) {
			console.error('Failed to load folders:', error);
			folders = [];
		}
	}



	async function startProcessing() {
		if (!sourceFolderPath || !destinationFolderPath || !excelFilePath || folders.length === 0) {
			alert('Bütün sahələri doldurun (Fill all required fields)');
			return;
		}

		isProcessing = true;
		isPaused = false;
		progress = 0;
		processLog = [];
		currentStep = 'Başlanılır... (Starting...)';

		try {
			const result = await invoke('rename_folders_from_excel', {
				sourcePath: sourceFolderPath,
				destinationPath: destinationFolderPath,
				excelPath: excelFilePath,
				startRow: excelStartRow,
				column: excelColumn,
				sortOrder: sortOrder,
				folders: folders.map(f => f.name)
			});
			
			// Process completed successfully
			currentStep = 'Tamamlandı! (Completed!)';
			console.log('Process completed:', result);
			
		} catch (error) {
			console.error('Process failed:', error);
			currentStep = 'Xəta baş verdi (Error occurred)';
			processLog = [...processLog, {
				type: 'error',
				message: `❌ Xəta: ${error}`,
				timestamp: new Date().toLocaleTimeString()
			}];
		} finally {
			isProcessing = false;
		}
	}



	function pauseProcessing() {
		isPaused = !isPaused;
	}

	function stopProcessing() {
		isProcessing = false;
		isPaused = false;
		currentStep = 'Dayandırıldı (Stopped)';
	}

	function resetProcess() {
		// Reset all selections and data
		sourceFolderPath = '';
		destinationFolderPath = '';
		excelFilePath = '';
		folders = [];
		excelStartRow = 1;
		excelColumn = 'A';
		isProcessing = false;
		isPaused = false;
		progress = 0;
		processLog = [];
		currentStep = '';
		sortOrder = 'default';
	}

	// Format column input to uppercase Latin letters only
	function formatColumnInput(event) {
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
	$: if (sortOrder && folders.length > 0 && sourceFolderPath) {
		loadFolders();
	}

	function getFileName(path) {
		return path.split(/[\\/]/).pop() || path;
	}


</script>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold text-text-primary mb-2">Qovluqların Adını Dəyişdirmək</h1>
			<p class="text-text-secondary">Excel faylından istifadə edərək qovluqların adlarını dəyişdirin</p>
			
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
			<div class="flex space-x-2">
				<button 
					on:click={resetProcess}
					class="btn-secondary flex items-center space-x-2"
					title="Sıfırla (Reset)"
				>
					<RotateCcw size={16} />
					<span>Sıfırla</span>
				</button>
			</div>
		{/if}
	</div>

	<!-- Configuration Panel -->
	<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
		<!-- Source Folder Selection -->
		<div class="card">
			<h3 class="text-lg font-semibold text-text-primary mb-4 flex items-center space-x-2">
				<FolderOpen size={20} class="text-accent-cyan" />
				<span>Mənbə Qovluğu (Source Folder)</span>
			</h3>
			
			<div class="space-y-4">
				<div>
					<button
						on:click={selectSourceFolder}
						class="btn-primary w-full"
						disabled={isProcessing}
					>
						Qovluq Seçin (Select Folder)
					</button>
					{#if sourceFolderPath}
						<p class="text-text-secondary text-sm mt-2 break-all">{sourceFolderPath}</p>
						<p class="text-accent-green text-sm">📁 {folders.length} qovluq tapıldı</p>
					{/if}
				</div>

				{#if sourceFolderPath}
					<div>
						<label class="block text-text-secondary text-sm font-medium mb-2">
							Sıralama Qayda (Sort Order)
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
				<span>Parametrlər (Settings)</span>
			</h3>
			
			<div class="space-y-4">
				<!-- Destination Folder -->
				<div>
					<label class="block text-text-secondary text-sm font-medium mb-2">
						Hədəf Qovluğu (Destination Folder)
					</label>
					<button
						on:click={selectDestinationFolder}
						class="btn-secondary w-full"
						disabled={isProcessing}
					>
						Hədəf Seçin (Select Destination)
					</button>
					{#if destinationFolderPath}
						<p class="text-text-secondary text-sm mt-2 break-all">{destinationFolderPath}</p>
					{/if}
				</div>

				<!-- Excel File -->
				<div>
					<label class="block text-text-secondary text-sm font-medium mb-2">
						Excel Faylı (Excel File)
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
								Başlanğıc Sətir (Start Row)
							</label>
							<input
								bind:value={excelStartRow}
								type="number"
								min="1"
								disabled={isProcessing}
								class="input-field w-full"
								placeholder="1"
							/>
						</div>
						<div>
							<label class="block text-text-secondary text-sm font-medium mb-2">
								Sütun (Column)
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

	<!-- Folders Preview -->
	{#if folders.length > 0}
		<div class="card">
			<h3 class="text-lg font-semibold text-text-primary mb-4">
				Qovluqlar Önizləməsi ({folders.length} ədəd)
			</h3>
			<div class="max-h-48 overflow-y-auto">
				<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-2">
					{#each folders.slice(0, 12) as folder, index}
						<div class="bg-dark-secondary rounded-lg p-3 flex items-center space-x-2">
							<span class="text-text-muted text-sm">{index + 1}.</span>
							<span class="text-text-primary text-sm truncate">{folder.name}</span>
						</div>
					{/each}
					{#if folders.length > 12}
						<div class="text-text-muted text-sm p-3">
							və daha {folders.length - 12} qovluq...
						</div>
					{/if}
				</div>
			</div>
		</div>
	{/if}

	<!-- Control Panel -->
	<div class="card">
		<div class="flex items-center justify-between mb-4">
			<h3 class="text-lg font-semibold text-text-primary">İdarəetmə (Control)</h3>
			
			<div class="flex space-x-2">
				{#if !isProcessing}
					<button
						on:click={startProcessing}
						disabled={!sourceFolderPath || !destinationFolderPath || !excelFilePath || folders.length === 0}
						class="btn-primary flex items-center space-x-2 disabled:opacity-50"
					>
						<Play size={16} />
						<span>Başla (Start)</span>
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
						<span>Dayandır</span>
					</button>
				{/if}
			</div>
		</div>

		<!-- Progress Bar -->
		{#if isProcessing || progress > 0}
			<div class="space-y-3">
				<div class="flex justify-between text-sm">
					<span class="text-text-secondary">Gedişat (Progress): {progress}%</span>
					<span class="text-text-primary">{currentStep}</span>
				</div>
				<div class="w-full bg-dark-secondary rounded-full h-3">
					<div 
						class="bg-gradient-to-r from-accent-cyan to-accent-orange h-3 rounded-full transition-all duration-300"
						style="width: {progress}%"
					></div>
				</div>
			</div>
		{/if}
	</div>

	<!-- Process Log -->
	{#if processLog.length > 0}
		<div class="card">
			<h3 class="text-lg font-semibold text-text-primary mb-4">Proses Jurnalı (Process Log)</h3>
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