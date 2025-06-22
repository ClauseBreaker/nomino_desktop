<script lang="ts">
	import { goto } from '$app/navigation';
	import { 
		FolderEdit, 
		FileText, 
		FileSpreadsheet,
		Copy,
		Calendar,
		Layers,
		FolderOpen,
		ImageIcon
	} from 'lucide-svelte';
	
	export let currentPath = '';

	// Make menuItems reactive to currentPath changes
	$: menuItems = [
		{ 
			path: '/', 
			icon: FolderEdit, 
			label: 'Qovluq Adlandırıcı',
			active: currentPath === '/'
		},
		{ 
			path: '/files', 
			icon: FileText, 
			label: 'Fayl Adlandırıcı',
			active: currentPath === '/files'
		},
		{ 
			path: '/pdf', 
			icon: ImageIcon, 
			label: 'PDF',
			active: currentPath === '/pdf'
		},
		{ 
			path: '/copy', 
			icon: Copy, 
			label: 'Fayl Kopyalama',
			active: currentPath === '/copy'
		},
		{ 
			path: '/pdf-date', 
			icon: Calendar, 
			label: 'PDF Tarix Dəyişdiricisi',
			active: currentPath === '/pdf-date'
		},
		{ 
			path: '/pdf-merge', 
			icon: Layers, 
			label: 'PDF Birləşdirici',
			active: currentPath === '/pdf-merge'
		},
		{ 
			path: '/excel-rename', 
			icon: FileSpreadsheet, 
			label: 'Excel Fayl Adı Dəyişdirici',
			active: currentPath === '/excel-rename'
		}
	];

	function navigate(path: string) {
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
</aside> 