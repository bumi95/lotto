<script>
    import { getColorClass } from '$lib/utils';
    
    let numbers = [];
    
    function drawNumbers() {
        const nums = new Set();
        while(nums.size < 6) {
            nums.add(Math.floor(Math.random() * 45) + 1);
        }
        numbers = Array.from(nums).sort((a, b) => a - b);
    }
</script>

<div class="section number-generator">
    <h1>로또 번호 추첨기</h1>
    <button on:click={drawNumbers}>번호 추첨하기</button>
    <div id="lotto-numbers">
        {#each numbers as num}
            <div class="number {getColorClass(num)}">{num}</div>
        {/each}
    </div>
</div>

<style>
    .number-generator {
        text-align: center;
    }
    
    button {
        background: #3498db;
        color: white;
        border: none;
        padding: 12px 24px;
        border-radius: 8px;
        font-size: 16px;
        cursor: pointer;
        transition: background 0.3s;
        margin-bottom: 1rem;
    }

    button:hover {
        background: #2980b9;
    }
</style> 