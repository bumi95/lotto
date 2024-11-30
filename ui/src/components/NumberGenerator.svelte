<script>
    import { getColorClass } from '$lib/utils';
    import LoadingSpinner from './LoadingSpinner.svelte';
    
    export let generatedNumbers = [];
    export let onGenerate;
    
    let isGenerating = false;
    
    async function handleGenerate() {
        isGenerating = true;
        await onGenerate();
        isGenerating = false;
    }
</script>

<div class="section number-generator">
    <h1>로또 번호 추첨기</h1>
    <button on:click={handleGenerate} disabled={isGenerating}>
        {#if isGenerating}
            <LoadingSpinner size="small" />
            추첨 중...
        {:else}
            번호 추첨하기
        {/if}
    </button>
    <div id="lotto-numbers">
        {#each generatedNumbers as num}
            <div class="number {getColorClass(num)}">{num}</div>
        {/each}
    </div>
</div>

<style>
    .number-generator {
        text-align: center;
    }
    
    h1 {
        font-size: 1.5rem;
        color: #2c3e50;
        margin-bottom: 1.5rem;
        font-weight: 600;
    }
    
    button {
        background: #3498db;
        color: white;
        border: none;
        padding: 12px 24px;
        border-radius: 8px;
        font-size: 16px;
        cursor: pointer;
        transition: all 0.3s;
        margin-bottom: 1.5rem;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        display: inline-flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        min-width: 120px;
    }

    button:hover {
        background: #2980b9;
        transform: translateY(-2px);
        box-shadow: 0 6px 8px rgba(0, 0, 0, 0.15);
    }

    button:active {
        transform: translateY(0);
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }

    button:disabled {
        opacity: 0.7;
        cursor: not-allowed;
    }

    #lotto-numbers {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 0.8rem;
        margin-top: 1.5rem;
    }

    .number {
        display: inline-flex;
        justify-content: center;
        align-items: center;
        width: 28px;
        height: 28px;
        border-radius: 50%;
        font-size: 1rem;
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

    :global(.small-spinner) {
        width: 16px;
        height: 16px;
        border-width: 2px;
    }
</style> 