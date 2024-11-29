<script>
    import { getColorClass } from '$lib/utils';
    export let lastWinInfo;
</script>

<div class="section last-winning">
    <h1>지난 회차 당첨 결과</h1>
    {#if lastWinInfo}
        <div class="winning-info">
            <h2>제 {lastWinInfo.drwNo}회</h2>
            <h3>{lastWinInfo.drwNoDate}</h3>
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
    h1 {
        text-align: center;
        margin-bottom: 1.5rem;
        color: #2c3e50;
        font-size: 1.5rem;
        font-weight: 600;
    }

    .winning-info {
        text-align: center;
    }
    
    h2 {
        font-size: 1.3rem;
        font-weight: bold;
        color: #2c3e50;
    }

    h3 {
        color: #666;
        margin-bottom: 1rem;
        font-size: 1.1rem;
    }

    .winning-numbers {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 0.8rem;
        margin: 1.5rem 0;
    }

    .number {
        display: inline-flex;
        justify-content: center;
        align-items: center;
        width: 45px;
        height: 45px;
        border-radius: 50%;
        font-size: 1.4rem;
        font-weight: 600;
        color: white;
        box-shadow: 
            inset -4px -4px 8px rgba(0, 0, 0, 0.5),
            inset 4px 4px 8px rgba(255, 255, 255, 0.5),
            5px 5px 10px rgba(0, 0, 0, 0.2),
            -2px -2px 6px rgba(255, 255, 255, 0.1);
        text-shadow: 2px 2px 3px rgba(0, 0, 0, 0.5);
        position: relative;
        transition: transform 0.2s;
    }

    .number:hover {
        transform: scale(1.1);
    }

    .number::after {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        border-radius: 50%;
        background: radial-gradient(circle at 30% 30%, rgba(255,255,255,0.4) 0%, rgba(255,255,255,0) 70%);
        pointer-events: none;
    }

    .plus {
        font-size: 1.5rem;
        font-weight: bold;
        color: #666;
        margin: 0 0.5rem;
    }

    .highlight {
        color: #e74c3c;
        font-weight: bold;
        font-size: 1.1rem;
    }

    .prize-info {
        margin-top: 1.5rem;
        line-height: 1.8;
        font-size: 1.1rem;
    }
</style> 