<script>
    import { theme } from '$lib/stores/theme';
    import Chart from 'chart.js/auto';
    
    export let chartData = [];
    let canvas;
    let chart;

    // 차트 초기 생성 함수
    $: if (chartData && chartData.length > 0 && canvas) {
        
        if (chart) chart.destroy();
        
        const isDarkMode = $theme === 'dark';
        const textColor = isDarkMode ? '#fff' : '#2c3e50';
        const gridColor = isDarkMode ? 'rgba(255, 255, 255, 0.1)' : 'rgba(0, 0, 0, 0.05)';
        
        chart = new Chart(canvas, {
            type: 'line',
            data: {
                labels: chartData.map(d => d.number),
                datasets: [{
                    label: '번호별 출현 빈도',
                    data: chartData.map(d => d.frequency),
                    borderColor: '#3498db',
                    backgroundColor: 'rgba(52, 152, 219, 0.1)',
                    borderWidth: 2,
                    pointBackgroundColor: '#2980b9',
                    pointRadius: 4,
                    pointHoverRadius: 6,
                    fill: true,
                    tension: 0.4
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: {
                        display: false
                    },
                    tooltip: {
                        backgroundColor: isDarkMode ? 'rgba(44, 62, 80, 0.9)' : 'rgba(0, 0, 0, 0.8)',
                        titleColor: '#fff',
                        bodyColor: '#fff',
                        titleFont: { size: 14 },
                        bodyFont: { size: 13 },
                        callbacks: {
                            label: context => `${context.raw}회 출현`,
                            title: context => `${context[0].label}번`
                        }
                    }
                },
                scales: {
                    x: {
                        grid: { display: false },
                        title: {
                            display: true,
                            text: '로또 번호',
                            color: textColor,
                            font: { size: 14 }
                        },
                        ticks: { color: textColor }
                    },
                    y: {
                        beginAtZero: true,
                        grid: { color: gridColor },
                        title: {
                            display: true,
                            text: '출현 횟수',
                            color: textColor,
                            font: { size: 14 }
                        },
                        ticks: { color: textColor }
                    }
                }
            }
        });
    }

    // 테마 변경 시 색상만 업데이트하는 함수
    function updateChartColors() {
        if (!chart) return;

        const isDarkMode = $theme === 'dark';
        const textColor = isDarkMode ? '#fff' : '#2c3e50';
        const gridColor = isDarkMode ? 'rgba(255, 255, 255, 0.1)' : 'rgba(0, 0, 0, 0.05)';

        chart.options.scales.x.title.color = textColor;
        chart.options.scales.x.ticks.color = textColor;
        chart.options.scales.y.title.color = textColor;
        chart.options.scales.y.ticks.color = textColor;
        chart.options.scales.y.grid.color = gridColor;
        chart.options.plugins.tooltip.backgroundColor = isDarkMode ? 'rgba(44, 62, 80, 0.9)' : 'rgba(0, 0, 0, 0.8)';

        chart.update('none'); // 애니메이션 없이 즉시 업데이트
    }

    // 테마가 변경될 때 색상만 업데이트
    $: if ($theme && chart) {
        updateChartColors();
    }
</script>

<div class="section chart-container">
    <h2>번호별 1등 출현 통계</h2>
    <div class="canvas-wrapper">
        <canvas bind:this={canvas}></canvas>
    </div>
</div>

<style>
    .chart-container {
        grid-column: 1 / -1;
        padding: 2rem;
    }

    h2 {
        text-align: center;
        color: #2c3e50;
        margin-bottom: 2rem;
        font-size: 1.5rem;
    }

    .canvas-wrapper {
        height: 400px;
        width: 100%;
        position: relative;
    }

    canvas {
        width: 100% !important;
        height: 100% !important;
    }
/*
    :global(body.dark-theme) .chart-container h2 {
        color: #fff;
    }

    :global(body.dark-theme) canvas {
        filter: brightness(0.9);
    }
*/
</style>
