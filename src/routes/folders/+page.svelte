<script>
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/dialog';
	import { 
		Folder, 
		FolderOpen, 
		Search, 
		AlertCircle,
		CheckCircle 
	} from 'lucide-svelte';

	let selectedDirectory = '';
	let folders = [];
	let filteredFolders = [];
	let searchQuery = '';
	let isLoading = false;
	let renamePattern = '';
	let renameReplacement = '';
	let results = [];
	let showResults = false;

	async function selectDirectory() {
		try {
			const selected = await open({
				directory: true,
				multiple: false,
				title: 'Select Parent Directory'
			});
			
			if (selected) {
				selectedDirectory = selected;
				await loadFolders();
			}
		} catch (error) {
			console.error('Failed to select directory:', error);
		}
	}

	async function loadFolders() {
		if (!selectedDirectory) return;
		
		isLoading = true;
		try {
			const result = await invoke('get_folders_in_directory', { 
				path: selectedDirectory 
			});
			folders = result;
			filterFolders();
		} catch (error) {
			console.error('Failed to load folders:', error);
			folders = [];
		} finally {
			isLoading = false;
		}
	}

	function filterFolders() {
		if (!searchQuery) {
			filteredFolders = folders;
		} else {
			filteredFolders = folders.filter(folder => 
				folder.name.toLowerCase().includes(searchQuery.toLowerCase())
			);
		}
	}

	async function renameFolders() {
		if (!selectedDirectory || !renamePattern) return;
		
		isLoading = true;
		try {
			const result = await invoke('rename_folders', {
				directory: selectedDirectory,
				pattern: renamePattern,
				replacement: renameReplacement
			});
			
			results = result;
			showResults = true;
			await loadFolders(); // Refresh folder list
		} catch (error) {
			console.error('Failed to rename folders:', error);
			results = [`Error: ${error}`];
			showResults = true;
		} finally {
			isLoading = false;
		}
	}

	$: searchQuery && filterFolders();
</script>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold text-text-primary mb-2">Folder Management</h1>
			<p class="text-text-secondary">Rename and organize your folders</p>
		</div>
		<button 
			on:click={selectDirectory}
			class="btn-primary flex items-center space-x-2"
		>
			<FolderOpen size={20} />
			<span>Select Directory</span>
		</button>
	</div>

	<!-- Selected Directory -->
	{#if selectedDirectory}
		<div class="card">
			<div class="flex items-center space-x-3">
				<FolderOpen size={20} class="text-accent-cyan" />
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
			<h3 class="text-lg font-semibold text-text-primary mb-4">Batch Rename Folders</h3>
			<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
				<div>
					<label class="block text-text-secondary text-sm font-medium mb-2">
						Find Pattern
					</label>
					<input
						bind:value={renamePattern}
						type="text"
						placeholder="e.g., Folder_"
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
						placeholder="e.g., Directory_"
						class="input-field w-full"
					/>
				</div>
				<div class="flex items-end">
					<button
						on:click={renameFolders}
						disabled={!renamePattern || isLoading}
						class="btn-primary w-full disabled:opacity-50 disabled:cursor-not-allowed"
					>
						{#if isLoading}
							<span>Processing...</span>
						{:else}
							<span>Rename Folders</span>
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

	<!-- Folder List -->
	{#if selectedDirectory}
		<div class="card">
			<div class="flex items-center justify-between mb-4">
				<h3 class="text-lg font-semibold text-text-primary">
					Folders ({filteredFolders.length})
				</h3>
				<div class="relative">
					<input
						bind:value={searchQuery}
						type="text"
						placeholder="Search folders..."
						class="input-field pl-10 w-64"
					/>
					<Search size={20} class="absolute left-3 top-1/2 transform -translate-y-1/2 text-text-muted" />
				</div>
			</div>

			{#if isLoading}
				<div class="flex items-center justify-center py-12">
					<div class="text-text-secondary">Loading folders...</div>
				</div>
			{:else if filteredFolders.length === 0}
				<div class="flex items-center justify-center py-12">
					<div class="text-center">
						<Folder size={48} class="text-text-muted mx-auto mb-4" />
						<p class="text-text-secondary">No folders found</p>
					</div>
				</div>
			{:else}
				<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
					{#each filteredFolders as folder}
						<div class="bg-dark-secondary rounded-lg p-4 border border-dark-border hover:border-accent-cyan transition-colors">
							<div class="flex items-center space-x-3">
								<Folder size={24} class="text-accent-cyan" />
								<div class="flex-1 min-w-0">
									<p class="text-text-primary font-medium truncate">{folder.name}</p>
									<p class="text-text-muted text-sm">Folder</p>
								</div>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}
</div> 