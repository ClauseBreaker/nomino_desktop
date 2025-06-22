<script>
	import '../app.css';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import { Minus, Square, X } from 'lucide-svelte';

	let currentPath = '';
	let isTauriApp = false;
	let isMaximized = false;

	$: currentPath = $page.url.pathname;

	onMount(async () => {
		// Check if running in Tauri
		isTauriApp = typeof window !== 'undefined' && typeof window['__TAURI_IPC__'] === 'function';
		
		if (isTauriApp) {
			try {
				const { appWindow } = await import('@tauri-apps/api/window');
				// Listen for window resize events
				isMaximized = await appWindow.isMaximized();
				
				appWindow.listen('tauri://resize', async () => {
					isMaximized = await appWindow.isMaximized();
				});
			} catch (error) {
				console.error('Failed to setup window listeners:', error);
			}
		}
	});

	async function minimizeWindow() {
		if (!isTauriApp) return;
		
		try {
			const { appWindow } = await import('@tauri-apps/api/window');
			await appWindow.minimize();
		} catch (error) {
			console.error('Failed to minimize window:', error);
		}
	}

	async function toggleMaximize() {
		if (!isTauriApp) return;
		
		try {
			const { appWindow } = await import('@tauri-apps/api/window');
			await appWindow.toggleMaximize();
		} catch (error) {
			console.error('Failed to toggle maximize window:', error);
		}
	}

	async function closeWindow() {
		if (!isTauriApp) return;
		
		try {
			const { appWindow } = await import('@tauri-apps/api/window');
			await appWindow.close();
		} catch (error) {
			console.error('Failed to close window:', error);
		}
	}
</script>

<div class="flex h-screen bg-dark-bg overflow-hidden">
	<!-- Sidebar -->
	<Sidebar {currentPath} />
	
	<!-- Main Content Area -->
	<div class="flex-1 flex flex-col min-w-0">
		<!-- Title Bar with Window Controls -->
		{#if isTauriApp}
			<div class="h-10 bg-dark-bg border-b border-dark-border flex items-center justify-end px-4 drag-region">
				<div class="flex space-x-1 no-drag">
					<!-- Minimize Button -->
					<button
						on:click={minimizeWindow}
						class="w-12 h-8 rounded flex items-center justify-center text-gray-400 hover:text-white hover:bg-gray-600 transition-colors duration-200"
						title="Свернуть"
					>
						<Minus size={16} />
					</button>
					
					<!-- Maximize/Restore Button -->
					<button
						on:click={toggleMaximize}
						class="w-12 h-8 rounded flex items-center justify-center text-gray-400 hover:text-white hover:bg-gray-600 transition-colors duration-200"
						title={isMaximized ? "Восстановить" : "Развернуть"}
					>
						{#if isMaximized}
							<div class="relative w-4 h-4 flex items-center justify-center">
								<!-- Restore icon: two overlapping squares -->
								<Square size={10} class="absolute -top-0.5 -left-0.5" />
								<Square size={10} class="absolute top-0.5 left-0.5" />
							</div>
						{:else}
							<Square size={14} />
						{/if}
					</button>
					
					<!-- Close Button -->
					<button
						on:click={closeWindow}
						class="w-12 h-8 rounded flex items-center justify-center text-gray-400 hover:text-white hover:bg-red-600 transition-colors duration-200"
						title="Закрыть"
					>
						<X size={16} />
					</button>
				</div>
			</div>
		{/if}
		
		<!-- Page Content -->
		<main class="flex-1 overflow-auto p-6">
			<slot />
		</main>
	</div>
</div> 