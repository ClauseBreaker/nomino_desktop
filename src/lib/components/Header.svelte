<script>
	import { Search, ChevronDown, User } from 'lucide-svelte';
	
	let searchQuery = '';
	let filterValue = 'Last 7 Days';
	let showDropdown = false;
	
	const filterOptions = [
		'Last 7 Days',
		'Last 30 Days',
		'Last 3 Months',
		'Last Year',
		'All Time'
	];

	function handleSearch() {
		// Implement search functionality
		console.log('Search:', searchQuery);
	}

	function selectFilter(option) {
		filterValue = option;
		showDropdown = false;
	}

	function getGreeting() {
		const hour = new Date().getHours();
		if (hour < 12) return 'Good Morning';
		if (hour < 18) return 'Good Afternoon';
		return 'Good Evening';
	}
</script>

<header class="h-16 bg-dark-bg border-b border-dark-border flex items-center justify-between px-6">
	<!-- Greeting -->
	<div class="flex items-center">
		<h1 class="text-xl font-semibold text-text-primary">
			{getGreeting()}, <span class="text-accent-orange">User</span>
		</h1>
	</div>

	<!-- Search Bar -->
	<div class="flex-1 max-w-md mx-8">
		<div class="relative">
			<input
				bind:value={searchQuery}
				type="text"
				placeholder="Search here..."
				class="w-full input-field pl-10 pr-4"
				on:keydown={(e) => e.key === 'Enter' && handleSearch()}
			/>
			<Search 
				size={20} 
				class="absolute left-3 top-1/2 transform -translate-y-1/2 text-text-muted" 
			/>
		</div>
	</div>

	<!-- Right Section -->
	<div class="flex items-center space-x-4">
		<!-- Filter Dropdown -->
		<div class="relative">
			<button
				on:click={() => showDropdown = !showDropdown}
				class="flex items-center space-x-2 bg-dark-card hover:bg-gray-700 text-text-primary px-4 py-2 rounded-button border border-dark-border transition-colors"
			>
				<span class="text-sm font-medium">{filterValue}</span>
				<ChevronDown size={16} class="text-text-secondary" />
			</button>
			
			{#if showDropdown}
				<div class="absolute right-0 top-full mt-2 w-48 bg-dark-card border border-dark-border rounded-lg shadow-lg z-50">
					{#each filterOptions as option}
						<button
							on:click={() => selectFilter(option)}
							class="w-full text-left px-4 py-2 text-sm text-text-primary hover:bg-gray-700 first:rounded-t-lg last:rounded-b-lg transition-colors"
						>
							{option}
						</button>
					{/each}
				</div>
			{/if}
		</div>

		<!-- User Avatar -->
		<div class="w-10 h-10 bg-gradient-to-br from-accent-cyan to-accent-blue rounded-full flex items-center justify-center">
			<User size={20} class="text-white" />
		</div>
	</div>
</header>

<!-- Click outside to close dropdown -->
{#if showDropdown}
	<div 
		class="fixed inset-0 z-40" 
		on:click={() => showDropdown = false}
		on:keydown={(e) => e.key === 'Escape' && (showDropdown = false)}
		role="button"
		tabindex="0"
	></div>
{/if} 