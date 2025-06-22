<script>
	import { onMount } from 'svelte';
	import { 
		Settings, 
		Palette, 
		Folder,
		Bell,
		Shield,
		Info,
		Save,
		RotateCcw
	} from 'lucide-svelte';

	// Settings state
	let settings = {
		theme: 'dark',
		language: 'en',
		autoSave: true,
		notifications: true,
		defaultDirectory: '',
		confirmActions: true,
		maxFileSize: 100, // MB
		fileExtensions: 'jpg,jpeg,png,gif,pdf,txt,doc,docx'
	};

	let hasChanges = false;

	onMount(() => {
		// Load settings from storage or backend
		loadSettings();
	});

	function loadSettings() {
		// This would normally load from Tauri backend or localStorage
		// For now, using default values
		console.log('Settings loaded');
	}

	function saveSettings() {
		// This would normally save to Tauri backend
		console.log('Saving settings:', settings);
		hasChanges = false;
		// Show success message
	}

	function resetSettings() {
		settings = {
			theme: 'dark',
			language: 'en',
			autoSave: true,
			notifications: true,
			defaultDirectory: '',
			confirmActions: true,
			maxFileSize: 100,
			fileExtensions: 'jpg,jpeg,png,gif,pdf,txt,doc,docx'
		};
		hasChanges = true;
	}

	function handleChange() {
		hasChanges = true;
	}

	async function selectDefaultDirectory() {
		try {
			// This would use Tauri's dialog API
			console.log('Selecting default directory...');
		} catch (error) {
			console.error('Failed to select directory:', error);
		}
	}

	const themeOptions = [
		{ value: 'dark', label: 'Dark Theme' },
		{ value: 'light', label: 'Light Theme' },
		{ value: 'auto', label: 'System Theme' }
	];

	const languageOptions = [
		{ value: 'en', label: 'English' },
		{ value: 'ru', label: 'Русский' },
		{ value: 'es', label: 'Español' },
		{ value: 'fr', label: 'Français' }
	];
</script>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold text-text-primary mb-2">Settings</h1>
			<p class="text-text-secondary">Configure your Nomino experience</p>
		</div>
		{#if hasChanges}
			<div class="flex space-x-2">
				<button 
					on:click={resetSettings}
					class="btn-secondary flex items-center space-x-2"
				>
					<RotateCcw size={16} />
					<span>Reset</span>
				</button>
				<button 
					on:click={saveSettings}
					class="btn-primary flex items-center space-x-2"
				>
					<Save size={16} />
					<span>Save Changes</span>
				</button>
			</div>
		{/if}
	</div>

	<!-- Appearance Settings -->
	<div class="card">
		<div class="flex items-center space-x-3 mb-6">
			<Palette size={24} class="text-accent-cyan" />
			<h3 class="text-lg font-semibold text-text-primary">Appearance</h3>
		</div>
		
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div>
				<label class="block text-text-secondary text-sm font-medium mb-2">
					Theme
				</label>
				<select 
					bind:value={settings.theme}
					on:change={handleChange}
					class="input-field w-full"
				>
					{#each themeOptions as option}
						<option value={option.value}>{option.label}</option>
					{/each}
				</select>
			</div>
			
			<div>
				<label class="block text-text-secondary text-sm font-medium mb-2">
					Language
				</label>
				<select 
					bind:value={settings.language}
					on:change={handleChange}
					class="input-field w-full"
				>
					{#each languageOptions as option}
						<option value={option.value}>{option.label}</option>
					{/each}
				</select>
			</div>
		</div>
	</div>

	<!-- File Management Settings -->
	<div class="card">
		<div class="flex items-center space-x-3 mb-6">
			<Folder size={24} class="text-accent-orange" />
			<h3 class="text-lg font-semibold text-text-primary">File Management</h3>
		</div>
		
		<div class="space-y-6">
			<div>
				<label class="block text-text-secondary text-sm font-medium mb-2">
					Default Directory
				</label>
				<div class="flex space-x-2">
					<input
						bind:value={settings.defaultDirectory}
						on:input={handleChange}
						type="text"
						placeholder="No default directory set"
						class="input-field flex-1"
						readonly
					/>
					<button
						on:click={selectDefaultDirectory}
						class="btn-secondary"
					>
						Browse
					</button>
				</div>
			</div>
			
			<div>
				<label class="block text-text-secondary text-sm font-medium mb-2">
					Supported File Extensions
				</label>
				<input
					bind:value={settings.fileExtensions}
					on:input={handleChange}
					type="text"
					placeholder="jpg,jpeg,png,gif,pdf,txt,doc,docx"
					class="input-field w-full"
				/>
				<p class="text-text-muted text-xs mt-1">Separate extensions with commas</p>
			</div>
			
			<div>
				<label class="block text-text-secondary text-sm font-medium mb-2">
					Maximum File Size (MB)
				</label>
				<input
					bind:value={settings.maxFileSize}
					on:input={handleChange}
					type="number"
					min="1"
					max="1000"
					class="input-field w-32"
				/>
			</div>
		</div>
	</div>

	<!-- Behavior Settings -->
	<div class="card">
		<div class="flex items-center space-x-3 mb-6">
			<Settings size={24} class="text-accent-green" />
			<h3 class="text-lg font-semibold text-text-primary">Behavior</h3>
		</div>
		
		<div class="space-y-4">
			<div class="flex items-center justify-between">
				<div>
					<p class="text-text-primary font-medium">Auto-save settings</p>
					<p class="text-text-secondary text-sm">Automatically save changes</p>
				</div>
				<label class="relative inline-flex items-center cursor-pointer">
					<input 
						type="checkbox" 
						bind:checked={settings.autoSave}
						on:change={handleChange}
						class="sr-only peer"
					/>
					<div class="w-11 h-6 bg-dark-border peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-accent-orange"></div>
				</label>
			</div>
			
			<div class="flex items-center justify-between">
				<div>
					<p class="text-text-primary font-medium">Confirm destructive actions</p>
					<p class="text-text-secondary text-sm">Show confirmation dialogs for delete/rename operations</p>
				</div>
				<label class="relative inline-flex items-center cursor-pointer">
					<input 
						type="checkbox" 
						bind:checked={settings.confirmActions}
						on:change={handleChange}
						class="sr-only peer"
					/>
					<div class="w-11 h-6 bg-dark-border peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-accent-orange"></div>
				</label>
			</div>
		</div>
	</div>

	<!-- Notifications Settings -->
	<div class="card">
		<div class="flex items-center space-x-3 mb-6">
			<Bell size={24} class="text-accent-yellow" />
			<h3 class="text-lg font-semibold text-text-primary">Notifications</h3>
		</div>
		
		<div class="flex items-center justify-between">
			<div>
				<p class="text-text-primary font-medium">Enable notifications</p>
				<p class="text-text-secondary text-sm">Show desktop notifications for completed operations</p>
			</div>
			<label class="relative inline-flex items-center cursor-pointer">
				<input 
					type="checkbox" 
					bind:checked={settings.notifications}
					on:change={handleChange}
					class="sr-only peer"
				/>
				<div class="w-11 h-6 bg-dark-border peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-accent-orange"></div>
			</label>
		</div>
	</div>

	<!-- About -->
	<div class="card bg-dark-secondary/50">
		<div class="flex items-center space-x-3 mb-4">
			<Info size={24} class="text-accent-cyan" />
			<h3 class="text-lg font-semibold text-text-primary">About Nomino</h3>
		</div>
		
		<div class="space-y-2 text-text-secondary">
			<p><strong class="text-text-primary">Version:</strong> 1.0.0</p>
			<p><strong class="text-text-primary">Built with:</strong> Tauri + SvelteKit + Rust</p>
			<p><strong class="text-text-primary">License:</strong> MIT</p>
			<p><strong class="text-text-primary">Created:</strong> 2025</p>
		</div>
		
		<div class="mt-4 pt-4 border-t border-dark-border space-y-3">
			<p class="text-text-muted text-sm">
				Nomino is a modern file management application designed to help you organize and process your files efficiently with Excel integration and Azerbaijani alphabet support.
			</p>
			
			<div class="flex items-center justify-between">
				<div>
					<p class="text-text-primary font-medium">Original Creator</p>
					<p class="text-text-secondary text-sm">ClauseBreaker</p>
				</div>
				<a 
					href="https://github.com/ClauseBreaker" 
					target="_blank" 
					rel="noopener noreferrer"
					class="btn-secondary text-xs px-3 py-1 hover:bg-accent-cyan hover:text-dark-bg transition-colors"
				>
					GitHub Profile
				</a>
			</div>
			
			<div class="flex items-center justify-between">
				<div>
					<p class="text-text-primary font-medium">Source Code</p>
					<p class="text-text-secondary text-sm">Open source on GitHub</p>
				</div>
				<a 
					href="https://github.com/ClauseBreaker/nomino" 
					target="_blank" 
					rel="noopener noreferrer"
					class="btn-secondary text-xs px-3 py-1 hover:bg-accent-orange hover:text-dark-bg transition-colors"
				>
					View Repository
				</a>
			</div>
		</div>
	</div>
</div> 