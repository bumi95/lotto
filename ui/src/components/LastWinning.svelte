<script>
    import { getColorClass } from '$lib/utils';
    export let lastWinInfo;
</script>

<div class="section last-winning">
    <h2 class="title">지난 회차 당첨 결과</h2>
    {#if lastWinInfo}
        <div class="winning-info">
            <p class="round">제 {lastWinInfo.drwNo}회</p>
            <p class="date">{lastWinInfo.drwNoDate}</p>
            <div class="winning-numbers">
                {#each [lastWinInfo.drwtNo1, lastWinInfo.drwtNo2, lastWinInfo.drwtNo3, 
                        lastWinInfo.drwtNo4, lastWinInfo.drwtNo5, lastWinInfo.drwtNo6] as num}
                    <div class="number small {getColorClass(num)}">{num}</div>
                {/each}
                <span class="plus">+</span>
                <div class="number small bonus {getColorClass(lastWinInfo.bnusNo)}">{lastWinInfo.bnusNo}</div>
            </div>
            <div class="prize-info">
                <p>1등 당첨금: <span class="highlight">{new Intl.NumberFormat('ko-KR').format(lastWinInfo.firstWinamnt)}원</span></p>
                <p>당첨자 수: <span class="highlight">{lastWinInfo.firstPrzwnerCo}명</span></p>
                <p>총 판매금액: <span class="highlight">{new Intl.NumberFormat('ko-KR').format(lastWinInfo.totSellamnt)}원</span></p>
            </div>
        </div>
    {/if}
</div>

<style>
    .title {
        text-align: center;
        margin-bottom: 1.5rem;
        color: #2c3e50;
    }

    .winning-info {
        text-align: center;
    }
    
    .round {
        font-size: 1.2rem;
        font-weight: bold;
        color: #2c3e50;
    }

    .date {
        color: #666;
        margin-bottom: 1rem;
    }

    .winning-numbers {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 0.5rem;
        margin: 1rem 0;
    }

    .plus {
        font-size: 1.2rem;
        font-weight: bold;
        color: #666;
    }

    .highlight {
        color: #e74c3c;
        font-weight: bold;
    }
</style> 