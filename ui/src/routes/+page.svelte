<script>
    import { onMount } from 'svelte';
    import NumberGenerator from '../components/NumberGenerator.svelte';
    import LastWinning from '../components/LastWinning.svelte';
    import WinningStores from '../components/WinningStores.svelte';
    
    let lastWinInfo = null;
    let winningStores = [];
    
    onMount(async () => {
        try {
            const [lastWinResponse, storesResponse] = await Promise.all([
                fetch('/api/lotto/last-winning'),
                fetch('/api/lotto/winning-stores')
            ]);
            
            if (lastWinResponse.ok) {
                lastWinInfo = await lastWinResponse.json();
            }
            
            if (storesResponse.ok) {
                winningStores = await storesResponse.json();
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
        }
    });
</script>

<div class="container">
    <NumberGenerator />
    <LastWinning {lastWinInfo} />
    <WinningStores {winningStores} />
</div>

<style>
    :global(body) {
        background: #f0f2f5;
        margin: 0;
        padding: 0;
    }

    .container {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        gap: 2rem;
        max-width: 1200px;
        margin: 0 auto;
        padding: 2rem;
    }

    :global(.section) {
        background: white;
        border-radius: 15px;
        padding: 1.5rem;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    }

    :global(.number) {
        display: inline-block;
        color: #fff;
        width: 40px;
        height: 40px;
        line-height: 40px;
        border-radius: 50%;
        margin: 5px;
        text-align: center;
        box-shadow: 
            inset -4px -4px 8px rgba(0,0,0,0.4),
            inset 4px 4px 8px rgba(255,255,255,0.4),
            5px 5px 10px rgba(0,0,0,0.4),
            -2px -2px 6px rgba(255,255,255,0.1);
        text-shadow: 2px 2px 3px rgba(0,0,0,0.5);
        position: relative;
        overflow: hidden;
    }

    :global(.number.small) {
        width: 30px;
        height: 30px;
        line-height: 30px;
        font-size: 16px;
    }

    :global(.yellow) { background: linear-gradient(145deg, #ffd324 0%, #e6b600 50%, #cc9900 100%); }
    :global(.blue) { background: linear-gradient(145deg, #3eacf0 0%, #2980b9 50%, #1a5981 100%); }
    :global(.red) { background: linear-gradient(145deg, #fc6255 0%, #c0392b 50%, #962b21 100%); }
    :global(.gray) { background: linear-gradient(145deg, #95a5a6 0%, #6a7778 50%, #4a5354 100%); }
    :global(.green) { background: linear-gradient(145deg, #40e086 0%, #27ae60 50%, #1e8449 100%); }
</style>
