<script>
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { open, save } from '@tauri-apps/api/dialog';
	import { 
		FileText, 
		FolderOpen, 
		Download,
		Plus,
		Trash2,
		FileCheck,
		AlertCircle 
	} from 'lucide-svelte';

	let selectedFiles = [];
	let outputPath = '';
	let pdfTitle = 'Generated PDF';
	let isCreating = false;
	let result = '';
	let showResult = false;

	async function selectFiles() {
		try {
			const selected = await open({
				multiple: true,
				filters: [{
					name: 'Images and Documents',
					extensions: ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'pdf', 'txt', 'doc', 'docx']
				}]
			});
			
			if (selected) {
				const files = Array.isArray(selected) ? selected : [selected];
				selectedFiles = [...selectedFiles, ...files];
			}
		} catch (error) {
			console.error('Failed to select files:', error);
		}
	}

	async function selectOutputPath() {
		try {
			const path = await save({
				defaultPath: pdfTitle + '.pdf',
				filters: [{
					name: 'PDF Files',
					extensions: ['pdf']
				}]
			});
			
			if (path) {
				outputPath = path;
			}
		} catch (error) {
			console.error('Failed to select output path:', error);
		}
	}

	function removeFile(index) {
		selectedFiles = selectedFiles.filter((_, i) => i !== index);
	}

	function clearFiles() {
		selectedFiles = [];
	}

	async function createPDF() {
		if (!selectedFiles.length || !outputPath) return;
		
		isCreating = true;
		try {
			const response = await invoke('create_pdf', {
				files: selectedFiles,
				outputPath: outputPath,
				title: pdfTitle
			});
			
			result = response;
			showResult = true;
		} catch (error) {
			console.error('Failed to create PDF:', error);
			result = `Error: ${error}`;
			showResult = true;
		} finally {
			isCreating = false;
		}
	}

	function getFileName(path) {
		return path.split(/[\\/]/).pop() || path;
	}

	function getFileExtension(path) {
		const parts = path.split('.');
		return parts.length > 1 ? parts.pop().toUpperCase() : 'FILE';
	}
</script>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold text-text-primary mb-2">PDF Creator</h1>
			<p class="text-text-secondary">Convert multiple files into a single PDF document</p>
		</div>
		<button 
			on:click={selectFiles}
			class="btn-primary flex items-center space-x-2"
		>
			<Plus size={20} />
			<span>Add Files</span>
		</button>
	</div>

	<!-- PDF Settings -->
	<div class="card">
		<h3 class="text-lg font-semibold text-text-primary mb-4">PDF Settings</h3>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
			<div>
				<label class="block text-text-secondary text-sm font-medium mb-2">
					PDF Title
				</label>
				<input
					bind:value={pdfTitle}
					type="text"
					placeholder="Enter PDF title..."
					class="input-field w-full"
				/>
			</div>
			<div>
				<label class="block text-text-secondary text-sm font-medium mb-2">
					Output Path
				</label>
				<div class="flex space-x-2">
					<input
						bind:value={outputPath}
						type="text"
						placeholder="Select output path..."
						class="input-field flex-1"
						readonly
					/>
					<button
						on:click={selectOutputPath}
						class="btn-secondary flex items-center space-x-2"
					>
						<FolderOpen size={16} />
						<span>Browse</span>
					</button>
				</div>
			</div>
		</div>
	</div>

	<!-- Selected Files -->
	<div class="card">
		<div class="flex items-center justify-between mb-4">
			<h3 class="text-lg font-semibold text-text-primary">
				Selected Files ({selectedFiles.length})
			</h3>
			{#if selectedFiles.length > 0}
				<button
					on:click={clearFiles}
					class="text-accent-red hover:text-red-400 flex items-center space-x-1 text-sm"
				>
					<Trash2 size={16} />
					<span>Clear All</span>
				</button>
			{/if}
		</div>

		{#if selectedFiles.length === 0}
			<div class="flex flex-col items-center justify-center py-12 border-2 border-dashed border-dark-border rounded-lg">
				<FileText size={48} class="text-text-muted mb-4" />
				<p class="text-text-secondary mb-2">No files selected</p>
				<button 
					on:click={selectFiles}
					class="btn-secondary"
				>
					Select Files
				</button>
			</div>
		{:else}
			<div class="space-y-2">
				{#each selectedFiles as file, index}
					<div class="flex items-center justify-between p-3 bg-dark-secondary rounded-lg border border-dark-border">
						<div class="flex items-center space-x-3">
							<FileCheck size={20} class="text-accent-green" />
							<div>
								<p class="text-text-primary font-medium">{getFileName(file)}</p>
								<p class="text-text-muted text-sm">{getFileExtension(file)} File</p>
							</div>
						</div>
						<button
							on:click={() => removeFile(index)}
							class="text-accent-red hover:text-red-400 p-2"
						>
							<Trash2 size={16} />
						</button>
					</div>
				{/each}
			</div>
		{/if}
	</div>

	<!-- Create PDF Button -->
	{#if selectedFiles.length > 0 && outputPath}
		<div class="card">
			<div class="flex items-center justify-center">
				<button
					on:click={createPDF}
					disabled={isCreating}
					class="btn-primary flex items-center space-x-2 px-8 py-3 text-lg disabled:opacity-50 disabled:cursor-not-allowed"
				>
					{#if isCreating}
						<span>Creating PDF...</span>
					{:else}
						<Download size={20} />
						<span>Create PDF</span>
					{/if}
				</button>
			</div>
		</div>
	{/if}

	<!-- Result -->
	{#if showResult}
		<div class="card">
			<div class="flex items-center justify-between mb-4">
				<h3 class="text-lg font-semibold text-text-primary">Result</h3>
				<button
					on:click={() => showResult = false}
					class="text-text-muted hover:text-text-primary"
				>
					Close
				</button>
			</div>
			<div class="flex items-center space-x-2 p-4 bg-dark-secondary rounded-lg">
				{#if result.includes('Error')}
					<AlertCircle size={20} class="text-accent-red" />
					<span class="text-accent-red">{result}</span>
				{:else}
					<FileCheck size={20} class="text-accent-green" />
					<span class="text-text-primary">{result}</span>
				{/if}
			</div>
		</div>
	{/if}

	<!-- Instructions -->
	<div class="card bg-dark-secondary/50">
		<h3 class="text-lg font-semibold text-text-primary mb-4">How to use PDF Creator</h3>
		<div class="space-y-2 text-text-secondary">
			<p>1. Click "Add Files" to select images or documents</p>
			<p>2. Set a title for your PDF document</p>
			<p>3. Choose where to save the output PDF file</p>
			<p>4. Click "Create PDF" to generate your document</p>
		</div>
		<div class="mt-4 p-3 bg-accent-orange/10 rounded-lg border border-accent-orange/20">
			<p class="text-accent-orange text-sm">
				<strong>Note:</strong> This is a placeholder implementation. 
				In a full version, this would convert images and documents into a proper PDF.
			</p>
		</div>
	</div>
</div> 