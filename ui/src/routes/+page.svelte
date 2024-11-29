<script>
    import { onMount } from 'svelte';
    import { theme } from '$lib/stores/theme';
    import ThemeToggle from '../components/ThemeToggle.svelte';
    import NumberGenerator from '../components/NumberGenerator.svelte';
    import LastWinning from '../components/LastWinning.svelte';
    import WinningStores from '../components/WinningStores.svelte';
    import DbChart from '../components/DbChart.svelte';
    import LoadingSpinner from '../components/LoadingSpinner.svelte';
    
    let lastWinInfo = null;
    let winningStores = [];
    let generatedNumbers = [];
    let chartData = [];
    let isLoading = true;
    
    async function handleGenerateNumbers() {
        try {
            const response = await fetch('/api/lotto/generate');
            if (response.ok) {
                const data = await response.json();
                generatedNumbers = data.numbers;
            }
        } catch (error) {
            console.error('번호 생성 중 오류:', error);
        }
    }
    
    onMount(async () => {
        try {
            const [lastWinResponse, storesResponse, chartResponse] = await Promise.all([
                fetch('/api/lotto/last-winning'),
                fetch('/api/lotto/winning-stores'),
                fetch('/api/lotto/db-chart')
            ]);
            
            if (lastWinResponse.ok) {
                lastWinInfo = await lastWinResponse.json();
            }
            
            if (storesResponse.ok) {
                winningStores = await storesResponse.json();
            }

            if (chartResponse.ok) {
                const rawData = await chartResponse.json();
                //console.log('차트 데이터 응답:', rawData);  // 디버깅용 로그 추가
                
                // 번호별 빈도수 계산
                const frequencyMap = new Array(45).fill(0);
                rawData.forEach(round => {
                    [round.num1, round.num2, round.num3, round.num4, round.num5, round.num6].forEach(num => {
                        frequencyMap[num - 1]++;
                    });
                });
                
                // 차트 데이터 형식으로 변환
                chartData = frequencyMap.map((frequency, index) => ({
                    number: index + 1,
                    frequency: frequency
                }));
                
                //console.log('처리된 차트 데이터:', chartData);  // 디버깅용 로그 추가
            }
        } catch (error) {
            console.error('데이터 로딩 중 오류:', error);
            lastWinInfo = {
                drw_no: 1234,
                drw_no_date: "2024-03-23",
                drwt_no1: 1,
                drwt_no2: 2,
                drwt_no3: 3,
                drwt_no4: 4,
                drwt_no5: 5,
                drwt_no6: 6,
                bnus_no: 7,
                first_winamnt: 2000000000,
                first_przwner_co: 10,
                tot_sellamnt: 100000000000
            };
            
            winningStores = [
                { 
                    name: "행복복권방",
                    method: "자동", 
                    address: "서울 강남구 역삼동"
                },
                { 
                    name: "대박복권",
                    method: "수동", 
                    address: "경기도 성남시 분당구"
                }
            ];
            
            // 기본 차트 데이터 설정
            chartData = Array(45).fill(0).map((_, i) => ({
                number: i + 1,
                frequency: 0
            }));
        } finally {
            isLoading = false;
        }
    });

    // 테마 변경 시 body 클래스 업데이트
    $: if (typeof document !== 'undefined') {
        document.body.classList.toggle('dark-theme', $theme === 'dark');
    }
</script>

<LoadingSpinner visible={isLoading} />

<div class="container" class:loading={isLoading}>
    {#if !isLoading}
        <NumberGenerator 
            {generatedNumbers} 
            onGenerate={handleGenerateNumbers}
        />
        <LastWinning {lastWinInfo} />
        <WinningStores {winningStores} />
        <DbChart {chartData} />
    {/if}
</div>

<ThemeToggle />

<style>
    :global(body) {
        background: #f0f2f5;
        margin: 0;
        padding: 0;
        transition: all 0.3s ease;
    }

    :global(body.dark-theme) {
        background: #1a1a1a;
        color: #fff;
    }

    :global(body.dark-theme .section) {
        background: #2c3e50;
        color: #fff;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
    }

    :global(body.dark-theme .number) {
        box-shadow: 
            inset -4px -4px 8px rgba(0,0,0,0.6),
            inset 4px 4px 8px rgba(255,255,255,0.2),
            5px 5px 10px rgba(0,0,0,0.6),
            -2px -2px 6px rgba(255,255,255,0.05);
    }

    :global(body.dark-theme) :global(button) {
        background-color: #1a2733;
        color: #fff;
        border: 1px solid #34495e;
    }

    :global(body.dark-theme) :global(button:hover) {
        background-color: #111a22;
    }

    :global(body.dark-theme) :global(input),
    :global(body.dark-theme) :global(select) {
        background-color: #2c3e50;
        color: #fff;
        border: 1px solid #34495e;
    }

    :global(body.dark-theme) :global(table) {
        color: #fff;
    }

    :global(body.dark-theme) :global(th),
    :global(body.dark-theme) :global(td) {
        border-color: #34495e;
    }

    :global(body.dark-theme) :global(h1),
    :global(body.dark-theme) :global(h2),
    :global(body.dark-theme) :global(h3) {
        color: #fff;
    }

    .container {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 1rem;
        max-width: 1200px;
        margin: 0 auto;
        padding: 1rem;
    }

    @media (max-width: 768px) {
        .container {
            grid-template-columns: 1fr;
            padding: 0.5rem;
            gap: 0.5rem;
        }
    }

    :global(.section) {
        background: white;
        border-radius: 15px;
        padding: 1rem;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    }

    :global(.number) {
        width: clamp(25px, 5vw, 40px);
        height: clamp(25px, 5vw, 40px);
        line-height: clamp(25px, 5vw, 40px);
        font-size: clamp(12px, 3vw, 16px);
    }

    :global(.number.small) {
        width: clamp(20px, 4vw, 30px);
        height: clamp(20px, 4vw, 30px);
        line-height: clamp(20px, 4vw, 30px);
        font-size: clamp(10px, 2.5vw, 14px);
    }

    @media (max-width: 480px) {
        :global(.section) {
            padding: 0.75rem;
        }
    }

    :global(.yellow) { background: linear-gradient(145deg, #ffd324 0%, #e6b600 50%, #cc9900 100%); }
    :global(.blue) { background: linear-gradient(145deg, #3eacf0 0%, #2980b9 50%, #1a5981 100%); }
    :global(.red) { background: linear-gradient(145deg, #fc6255 0%, #c0392b 50%, #962b21 100%); }
    :global(.gray) { background: linear-gradient(145deg, #95a5a6 0%, #6a7778 50%, #4a5354 100%); }
    :global(.green) { background: linear-gradient(145deg, #40e086 0%, #27ae60 50%, #1e8449 100%); }

    .container.loading {
        opacity: 0.5;
        pointer-events: none;
    }
</style>
