<script>
	export let type = 'line'; // 'line' | 'donut'
	export let data = [];

	// Simple SVG chart implementation
	function getLineChartPath(data, width = 300, height = 200) {
		if (!data.length) return '';
		
		const maxFiles = Math.max(...data.map(d => d.files || 0));
		const maxFolders = Math.max(...data.map(d => d.folders || 0));
		const maxValue = Math.max(maxFiles, maxFolders);
		
		const stepX = width / (data.length - 1);
		
		const filesPath = data.map((d, i) => {
			const x = i * stepX;
			const y = height - (d.files / maxValue) * height;
			return `${i === 0 ? 'M' : 'L'} ${x} ${y}`;
		}).join(' ');
		
		return filesPath;
	}

	function getDonutSegments(data, radius = 80, innerRadius = 50) {
		const total = data.reduce((sum, d) => sum + d.value, 0);
		let currentAngle = 0;
		
		return data.map(item => {
			const angle = (item.value / total) * 360;
			const startAngle = currentAngle;
			const endAngle = currentAngle + angle;
			currentAngle = endAngle;
			
			const startX = radius + Math.cos((startAngle - 90) * Math.PI / 180) * radius;
			const startY = radius + Math.sin((startAngle - 90) * Math.PI / 180) * radius;
			const endX = radius + Math.cos((endAngle - 90) * Math.PI / 180) * radius;
			const endY = radius + Math.sin((endAngle - 90) * Math.PI / 180) * radius;
			
			const largeArcFlag = angle > 180 ? 1 : 0;
			
			const pathData = [
				`M ${radius} ${radius}`,
				`L ${startX} ${startY}`,
				`A ${radius} ${radius} 0 ${largeArcFlag} 1 ${endX} ${endY}`,
				'Z'
			].join(' ');
			
			return {
				path: pathData,
				color: item.color,
				name: item.name,
				value: item.value,
				percentage: Math.round((item.value / total) * 100)
			};
		});
	}

	$: lineData = type === 'line' ? data : [];
	$: donutSegments = type === 'donut' ? getDonutSegments(data) : [];
</script>

<div class="h-64 flex items-center justify-center">
	{#if type === 'line'}
		<svg width="100%" height="200" viewBox="0 0 300 200" class="overflow-visible">
			<!-- Grid lines -->
			{#each [0, 1, 2, 3, 4] as line}
				<line 
					x1="0" 
					y1={line * 50} 
					x2="300" 
					y2={line * 50} 
					stroke="#2A2A3E" 
					stroke-width="1"
				/>
			{/each}
			
			<!-- Files line -->
			<path 
				d={getLineChartPath(lineData)} 
				fill="none" 
				stroke="url(#gradient1)" 
				stroke-width="3"
			/>
			
			<!-- Data points -->
			{#each lineData as point, i}
				<circle 
					cx={i * (300 / (lineData.length - 1))} 
					cy={200 - (point.files / Math.max(...lineData.map(d => d.files)) * 200)}
					r="4" 
					fill="#00F7FF"
				/>
			{/each}
			
			<!-- Gradient definition -->
			<defs>
				<linearGradient id="gradient1" x1="0%" y1="0%" x2="100%" y2="0%">
					<stop offset="0%" style="stop-color:#00F7FF;stop-opacity:1" />
					<stop offset="100%" style="stop-color:#00B2FF;stop-opacity:1" />
				</linearGradient>
			</defs>
		</svg>
		
	{:else if type === 'donut'}
		<div class="flex items-center space-x-8">
			<svg width="160" height="160" viewBox="0 0 160 160">
				{#each donutSegments as segment}
					<path 
						d={segment.path} 
						fill={segment.color}
						class="hover:opacity-80 transition-opacity cursor-pointer"
					/>
				{/each}
				
				<!-- Center hole -->
				<circle cx="80" cy="80" r="50" fill="#1E1E2F" />
				
				<!-- Center text -->
				<text x="80" y="75" text-anchor="middle" class="fill-text-primary text-xs font-semibold">
					Total
				</text>
				<text x="80" y="90" text-anchor="middle" class="fill-text-primary text-lg font-bold">
					100%
				</text>
			</svg>
			
			<!-- Legend -->
			<div class="space-y-3">
				{#each donutSegments as segment}
					<div class="flex items-center space-x-3">
						<div class="w-3 h-3 rounded-full" style="background-color: {segment.color}"></div>
						<div class="flex-1">
							<p class="text-text-primary text-sm font-medium">{segment.name}</p>
							<p class="text-text-secondary text-xs">{segment.percentage}%</p>
						</div>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div> 