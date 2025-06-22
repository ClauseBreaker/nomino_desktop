<script>
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/dialog';
	import { 
		Folder, 
		FileText, 
		Search, 
		Settings,
		Download,
		AlertCircle,
		CheckCircle 
	} from 'lucide-svelte';

	let selectedDirectory = '';
	let files = [];
	let filteredFiles = [];
	let searchQuery = '';
	let isLoading = false;
	let renamePattern = '';
	let renameReplacement = '';
	let results = [];
	let showResults = false;

	onMount(() => {
		// Initialize with default directory if needed
	});

	async function selectDirectory() {
		try {
			const selected = await open({
				directory: true,
				multiple: false,
				title: 'Select Directory'
			});
			
			if (selected) {
				selectedDirectory = selected;
				await loadFiles();
			}
		} catch (error) {
			console.error('Failed to select directory:', error);
		}
	}

	async function loadFiles() {
		if (!selectedDirectory) return;
		
		isLoading = true;
		try {
			const result = await invoke('get_files_in_directory', { 
				path: selectedDirectory 
			});
			files = result.filter(f => !f.is_directory);
			filterFiles();
		} catch (error) {
			console.error('Failed to load files:', error);
			files = [];
		} finally {
			isLoading = false;
		}
	}

	function filterFiles() {
		if (!searchQuery) {
			filteredFiles = files;
		} else {
			filteredFiles = files.filter(file => 
				file.name.toLowerCase().includes(searchQuery.toLowerCase())
			);
		}
	}

	async function renameFiles() {
		if (!selectedDirectory || !renamePattern) return;
		
		isLoading = true;
		try {
			const result = await invoke('rename_files', {
				directory: selectedDirectory,
				pattern: renamePattern,
				replacement: renameReplacement
			});
			
			results = result;
			showResults = true;
			await loadFiles(); // Refresh file list
		} catch (error) {
			console.error('Failed to rename files:', error);
			results = [`Error: ${error}`];
			showResults = true;
		} finally {
			isLoading = false;
		}
	}

	function formatFileSize(bytes) {
		if (bytes === 0) return '0 Bytes';
		const k = 1024;
		const sizes = ['Bytes', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
	}

	$: searchQuery && filterFiles();
</script>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold text-text-primary mb-2">File Management</h1>
			<p class="text-text-secondary">Rename and organize your files</p>
		</div>
		<button 
			on:click={selectDirectory}
			class="btn-primary flex items-center space-x-2"
		>
			<Folder size={20} />
			<span>Select Directory</span>
		</button>
	</div>

	<!-- Selected Directory -->
	{#if selectedDirectory}
		<div class="card">
			<div class="flex items-center space-x-3">
				<Folder size={20} class="text-accent-cyan" />
				<div>
					<p class="text-text-secondary text-sm">Selected Directory:</p>
					<p class="text-text-primary font-medium">{selectedDirectory}</p>
				</div>
			</div>
		</div>
	{/if}

	<!-- Rename Controls -->
	{#if selectedDirectory}
		<div class="card">
			<h3 class="text-lg font-semibold text-text-primary mb-4">Batch Rename Files</h3>
			<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
				<div>
					<label class="block text-text-secondary text-sm font-medium mb-2">
						Find Pattern
					</label>
					<input
						bind:value={renamePattern}
						type="text"
						placeholder="e.g., IMG_"
						class="input-field w-full"
					/>
				</div>
				<div>
					<label class="block text-text-secondary text-sm font-medium mb-2">
						Replace With
					</label>
					<input
						bind:value={renameReplacement}
						type="text"
						placeholder="e.g., Photo_"
						class="input-field w-full"
					/>
				</div>
				<div class="flex items-end">
					<button
						on:click={renameFiles}
						disabled={!renamePattern || isLoading}
						class="btn-primary w-full disabled:opacity-50 disabled:cursor-not-allowed"
					>
						{#if isLoading}
							<span>Processing...</span>
						{:else}
							<span>Rename Files</span>
						{/if}
					</button>
				</div>
			</div>
		</div>
	{/if}

	<!-- Results -->
	{#if showResults}
		<div class="card">
			<div class="flex items-center justify-between mb-4">
				<h3 class="text-lg font-semibold text-text-primary">Results</h3>
				<button
					on:click={() => showResults = false}
					class="text-text-muted hover:text-text-primary"
				>
					Close
				</button>
			</div>
			<div class="space-y-2 max-h-64 overflow-y-auto">
				{#each results as result}
					<div class="flex items-center space-x-2 p-2 bg-dark-secondary rounded-lg">
						{#if result.includes('Error')}
							<AlertCircle size={16} class="text-accent-red" />
						{:else}
							<CheckCircle size={16} class="text-accent-green" />
						{/if}
						<span class="text-text-primary text-sm">{result}</span>
					</div>
				{/each}
			</div>
		</div>
	{/if}

	<!-- File List -->
	{#if selectedDirectory}
		<div class="card">
			<div class="flex items-center justify-between mb-4">
				<h3 class="text-lg font-semibold text-text-primary">
					Files ({filteredFiles.length})
				</h3>
				<div class="relative">
					<input
						bind:value={searchQuery}
						type="text"
						placeholder="Search files..."
						class="input-field pl-10 w-64"
					/>
					<Search size={20} class="absolute left-3 top-1/2 transform -translate-y-1/2 text-text-muted" />
				</div>
			</div>

			{#if isLoading}
				<div class="flex items-center justify-center py-12">
					<div class="text-text-secondary">Loading files...</div>
				</div>
			{:else if filteredFiles.length === 0}
				<div class="flex items-center justify-center py-12">
					<div class="text-center">
						<FileText size={48} class="text-text-muted mx-auto mb-4" />
						<p class="text-text-secondary">No files found</p>
					</div>
				</div>
			{:else}
				<div class="overflow-x-auto">
					<table class="w-full">
						<thead>
							<tr class="border-b border-dark-border">
								<th class="text-left py-3 px-4 text-text-secondary font-medium">Name</th>
								<th class="text-left py-3 px-4 text-text-secondary font-medium">Size</th>
								<th class="text-left py-3 px-4 text-text-secondary font-medium">Extension</th>
							</tr>
						</thead>
						<tbody>
							{#each filteredFiles as file}
								<tr class="border-b border-dark-border hover:bg-dark-secondary/50 transition-colors">
									<td class="py-3 px-4">
										<div class="flex items-center space-x-3">
											<FileText size={20} class="text-accent-cyan" />
											<span class="text-text-primary">{file.name}</span>
										</div>
									</td>
									<td class="py-3 px-4 text-text-secondary">
										{formatFileSize(file.size)}
									</td>
									<td class="py-3 px-4 text-text-secondary">
										{file.extension || 'None'}
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			{/if}
		</div>
	{/if}
</div> 