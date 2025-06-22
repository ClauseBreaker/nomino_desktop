<script lang="ts">
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
		Settings,
		HelpCircle,
		X,
		Square,
		FileImage
	} from 'lucide-svelte';

	// State variables
	let mainFolderPath = '';
	let subfolderName = 'images';
	let deleteFiles = 'desktop.ini, thumbs.db, .DS_Store';
	let subfolders: any[] = [];
	let isProcessing = false;
	let isPaused = false;
	let progress = 0;
	let processLog: any[] = [];
	let currentStep = '';
	let showHelpModal = false;

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

	async function selectMainFolder() {
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
				title: 'Əsas qovluğunu seçin (Select Main Folder)'
			});
			
			console.log('Dialog result:', selected);
			
			if (selected && selected !== null && selected !== '') {
				mainFolderPath = Array.isArray(selected) ? selected[0] : selected;
				console.log('Selected folder:', mainFolderPath);
				await loadSubfolders();
			}
		} catch (error) {
			console.error('Dialog error:', error);
			alert(`Dialog xətası: ${error}`);
		}
	}

	async function loadSubfolders() {
		if (!mainFolderPath) return;
		
		try {
			const result = await invoke('get_pdf_subfolders', { 
				mainFolder: mainFolderPath
			});
			subfolders = result as any[];
		} catch (error) {
			console.error('Failed to load subfolders:', error);
			subfolders = [];
		}
	}

	async function startPdfCreation() {
		if (!mainFolderPath || !subfolderName.trim()) {
			alert('Bütün sahələri doldurun');
			return;
		}

		isProcessing = true;
		isPaused = false;
		progress = 0;
		processLog = [];
		currentStep = 'Başlanılır...';

		try {
			// Parse delete files list
			const deleteFilesList = deleteFiles.split(',').map(f => f.trim()).filter(f => f.length > 0);
			
			const config = {
				mainFolder: mainFolderPath,
				subfolderName: subfolderName.trim(),
				deleteFiles: deleteFilesList
			};

			const result = await invoke('create_pdf_from_images', { config });
			
			// Process completed successfully
			currentStep = 'Tamamlandı!';
			console.log('PDF creation completed:', result);
			
		} catch (error) {
			console.error('PDF creation failed:', error);
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
		mainFolderPath = '';
		subfolderName = 'images';
		deleteFiles = 'desktop.ini, thumbs.db, .DS_Store';
		subfolders = [];
		isProcessing = false;
		isPaused = false;
		progress = 0;
		processLog = [];
		currentStep = '';
	}

	function showHelp() {
		showHelpModal = true;
	}

	function closeHelp() {
		showHelpModal = false;
	}

	function getFileName(path: string) {
		return path.split(/[\\/]/).pop() || path;
	}
</script>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold text-text-primary mb-2">PDF Yaradılması</h1>
			<p class="text-text-secondary">Şəkillərdən PDF faylları yaradın və qovluq strukturunu təmizləyin</p>
			
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
		<!-- Main Folder Selection -->
		<div class="card">
			<h3 class="text-lg font-semibold text-text-primary mb-4 flex items-center space-x-2">
				<FolderOpen size={20} class="text-accent-cyan" />
				<span>Əsas Qovluq</span>
			</h3>
			
			<div class="space-y-4">
				<div>
					<button
						on:click={selectMainFolder}
						class="btn-primary w-full"
						disabled={isProcessing}
					>
						Qovluğu Seçin
					</button>
					{#if mainFolderPath}
						<p class="text-text-secondary text-sm mt-2 break-all">{mainFolderPath}</p>
						<p class="text-accent-green text-sm">📁 {subfolders.length} alt qovluq tapıldı</p>
					{/if}
				</div>
			</div>
		</div>

		<!-- PDF Settings -->
		<div class="card">
			<h3 class="text-lg font-semibold text-text-primary mb-4 flex items-center space-x-2">
				<Settings size={20} class="text-accent-orange" />
				<span>PDF Parametrləri</span>
			</h3>
			
			<div class="space-y-4">
				<!-- Subfolder Name -->
				<div>
					<label class="block text-text-secondary text-sm font-medium mb-2">
						Şəkil Qovluğu Adı
					</label>
					<input
						bind:value={subfolderName}
						type="text"
						disabled={isProcessing}
						class="input-field w-full"
						placeholder="images"
					/>
				</div>

				<!-- Delete Files -->
				<div>
					<label class="block text-text-secondary text-sm font-medium mb-2">
						Silinəcək Fayllar (vergüllə ayırın)
					</label>
					<input
						bind:value={deleteFiles}
						type="text"
						disabled={isProcessing}
						class="input-field w-full"
						placeholder="desktop.ini, thumbs.db, .DS_Store"
					/>
				</div>
			</div>
		</div>
	</div>

	<!-- Subfolders Preview -->
	{#if subfolders.length > 0}
		<div class="card">
			<h3 class="text-lg font-semibold text-text-primary mb-4">
				Alt Qovluqlar Önizləməsi ({subfolders.length} ədəd)
			</h3>
			<div class="max-h-48 overflow-y-auto">
				<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-2">
					{#each subfolders.slice(0, 12) as subfolder, index}
						<div class="bg-dark-secondary rounded-lg p-3 flex items-center space-x-2">
							<span class="text-text-muted text-sm">{index + 1}.</span>
							<div class="flex items-center space-x-2 flex-1">
								<FileImage size={16} class={subfolder.size > 0 ? 'text-accent-green' : 'text-text-muted'} />
								<span class="text-text-primary text-sm truncate">{subfolder.name}</span>
								{#if subfolder.size > 0}
									<span class="text-accent-green text-xs">✓</span>
								{/if}
							</div>
						</div>
					{/each}
					{#if subfolders.length > 12}
						<div class="text-text-muted text-sm p-3">
							və daha {subfolders.length - 12} qovluq...
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
						on:click={startPdfCreation}
						disabled={!mainFolderPath || !subfolderName.trim() || subfolders.length === 0}
						class="btn-primary flex items-center space-x-2 disabled:opacity-50"
					>
						<Play size={16} />
						<span>PDF Yaradılması</span>
					</button>
				{:else}
					<button
						on:click={pauseProcessing}
						class="btn-secondary flex items-center space-x-2"
					>
						{#if isPaused}
							<Play size={16} />
							<span>Davam et</span>
						{:else}
							<Pause size={16} />
							<span>Fasilə</span>
						{/if}
					</button>
					<button
						on:click={stopProcessing}
						class="bg-accent-red hover:bg-red-600 text-white px-4 py-2 rounded-lg flex items-center space-x-2 transition-colors"
					>
						<Square size={16} />
						<span>Dayandır</span>
					</button>
				{/if}
			</div>
		</div>

		<!-- Progress Section -->
		{#if isProcessing || progress > 0}
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
							<span class={log.type === 'success' ? 'text-accent-green' : 'text-accent-red'}>
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
	<div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
		<div class="bg-dark-card rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-[80vh] overflow-y-auto">
			<div class="flex items-center justify-between p-6 border-b border-dark-border">
				<h2 class="text-xl font-bold text-text-primary">📚 PDF Yaradılması Modulu</h2>
				<button 
					on:click={closeHelp}
					class="text-text-muted hover:text-text-primary"
				>
					<X size={24} />
				</button>
			</div>
			
			<div class="p-6 space-y-4 text-text-secondary">
				<div>
					<h3 class="text-lg font-semibold text-text-primary mb-2">🎯 BU MODUL NƏ EDİR?</h3>
					<p>Bu modul hər alt qovluqdakı şəkilləri PDF faylına çevirib, fayl strukturunu təmizləyir.</p>
				</div>
				
				<div>
					<h3 class="text-lg font-semibold text-text-primary mb-2">📋 NECƏ İŞLƏYİR?</h3>
					<ol class="list-decimal list-inside space-y-1">
						<li>Əsas qovluğu seçin - burada çoxlu alt qovluqlar var</li>
						<li>Şəkil qovluğu adını yazın (məs: "images", "photos")</li>
						<li>Silinəcək fayl adlarını yazın (məs: desktop.ini, thumbs.db)</li>
						<li>"PDF Yaradılması" düyməsini basın</li>
					</ol>
				</div>
				
				<div>
					<h3 class="text-lg font-semibold text-text-primary mb-2">💡 MİSAL:</h3>
					<div class="bg-dark-secondary p-3 rounded">
						<p><strong>Əsas qovluq:</strong> "Məhsullar"</p>
						<p><strong>Alt qovluqlar:</strong> "Məhsul1", "Məhsul2", "Məhsul3"</p>
						<p><strong>Hər birində "images" qovluğu var:</strong> şəkil1.jpg, şəkil2.jpg...</p>
						<br>
						<p><strong>Nəticə:</strong></p>
						<ul class="list-disc list-inside ml-4 space-y-1">
							<li>"Məhsul1_picture.pdf" yaradılacaq</li>
							<li>Bütün şəkillər PDF-ə əlavə olunacaq</li>
							<li>Orijinal şəkillər silinəcək</li>
							<li>PDF və digər fayllar "Məhsul1" qovluğuna köçürüləcək</li>
							<li>"images" qovluğu silinəcək</li>
						</ul>
					</div>
				</div>
				
				<div>
					<h3 class="text-lg font-semibold text-text-primary mb-2">⚠️ QEYD:</h3>
					<ul class="list-disc list-inside space-y-1">
						<li>Şəkil formatları: JPG, PNG, BMP, GIF, TIFF, WEBP</li>
						<li>PDF A4 formatında yaradılır</li>
						<li>Boş qovluqlar avtomatik silinir</li>
						<li class="text-accent-red font-medium">Proses geri qaytarıla bilməz!</li>
					</ul>
				</div>
			</div>
			
			<div class="p-6 border-t border-dark-border">
				<button 
					on:click={closeHelp}
					class="btn-primary w-full"
				>
					Anladım
				</button>
			</div>
		</div>
	</div>
{/if} 