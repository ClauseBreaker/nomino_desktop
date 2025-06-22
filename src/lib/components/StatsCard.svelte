<script>
	import { TrendingUp, TrendingDown } from 'lucide-svelte';
	
	export let title = '';
	export let value = '';
	export let icon = null;
	export let color = 'blue';
	export let trend = null; // { value, isPositive }

	const colorClasses = {
		blue: 'bg-accent-cyan shadow-glow-cyan',
		green: 'bg-accent-green',
		orange: 'bg-accent-orange',
		red: 'bg-accent-red',
		yellow: 'bg-accent-yellow'
	};

	const iconColorClass = colorClasses[color] || colorClasses.blue;
</script>

<div class="card hover:shadow-lg transition-shadow duration-200">
	<div class="flex items-start justify-between">
		<div class="flex-1">
			<p class="text-text-secondary text-sm font-medium mb-2">{title}</p>
			<p class="text-text-primary text-2xl font-bold mb-1">{value}</p>
			
			{#if trend}
				<div class="flex items-center space-x-1">
					{#if trend.isPositive}
						<TrendingUp size={16} class="text-accent-green" />
						<span class="text-accent-green text-sm font-medium">+{trend.value}%</span>
					{:else}
						<TrendingDown size={16} class="text-accent-red" />
						<span class="text-accent-red text-sm font-medium">-{trend.value}%</span>
					{/if}
					<span class="text-text-muted text-sm">vs last month</span>
				</div>
			{/if}
		</div>
		
		{#if icon}
			<div class="w-12 h-12 rounded-xl flex items-center justify-center {iconColorClass}">
				<svelte:component this={icon} size={24} class="text-white" />
			</div>
		{/if}
	</div>
</div> 