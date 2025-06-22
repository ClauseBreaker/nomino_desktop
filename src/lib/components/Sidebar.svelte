<script>
	import { goto } from '$app/navigation';
	import { 
		FolderEdit, 
		FileText, 
		Folder, 
		FileSpreadsheet,
		Settings 
	} from 'lucide-svelte';
	
	export let currentPath = '';

	const menuItems = [
		{ 
			path: '/', 
			icon: FolderEdit, 
			label: 'Qovluqların adını dəyişdirmək',
			active: currentPath === '/' 
		},
		{ 
			path: '/files', 
			icon: FileText, 
			label: 'Files',
			active: currentPath.startsWith('/files') 
		},
		{ 
			path: '/folders', 
			icon: Folder, 
			label: 'Folders',
			active: currentPath.startsWith('/folders') 
		},
		{ 
			path: '/pdf', 
			icon: FileSpreadsheet, 
			label: 'PDF',
			active: currentPath.startsWith('/pdf') 
		},
		{ 
			path: '/settings', 
			icon: Settings, 
			label: 'Settings',
			active: currentPath.startsWith('/settings') 
		}
	];

	function navigate(path) {
		goto(path);
	}
</script>

<aside class="w-28 bg-dark-sidebar border-r border-dark-border flex flex-col items-center py-6">
	<!-- Logo -->
	<div class="mb-8">
		<div class="w-14 h-14 rounded-2xl flex items-center justify-center bg-dark-card border border-dark-border overflow-hidden">
			<img 
				src="/logo_new.png" 
				alt="Nomino Logo" 
				class="w-full h-full object-contain p-1"
			/>
		</div>
	</div>

	<!-- Navigation Items -->
	<nav class="flex flex-col space-y-4 flex-1">
		{#each menuItems as item}
			<button
				on:click={() => navigate(item.path)}
				class="group relative w-14 h-14 rounded-2xl flex items-center justify-center transition-all duration-300 hover:scale-105 {item.active 
					? 'bg-accent-orange shadow-lg shadow-accent-orange/40' 
					: 'bg-dark-card border border-transparent hover:border-accent-orange hover:bg-accent-orange/10'}"
				title={item.label}
			>
				<svelte:component 
					this={item.icon} 
					size={22} 
					class={item.active ? 'text-white' : 'text-gray-400 group-hover:text-accent-orange'} 
				/>
				
				<!-- Tooltip -->
				<div class="absolute left-20 top-1/2 transform -translate-y-1/2 bg-dark-card text-text-primary px-3 py-2 rounded-lg text-sm font-medium opacity-0 group-hover:opacity-100 transition-opacity duration-200 pointer-events-none whitespace-nowrap z-50 shadow-lg border border-dark-border">
					{item.label}
				</div>
			</button>
		{/each}
	</nav>

	<!-- User Profile -->
	<div class="mt-auto">
		<div class="w-12 h-12 bg-gradient-to-br from-accent-green to-accent-blue rounded-full flex items-center justify-center">
			<span class="text-white font-semibold text-sm">U</span>
		</div>
	</div>
</aside> 