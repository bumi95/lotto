<script>
    export let winningStores;
    
    // method 값 정규화 함수
    function normalizeMethod(method) {
        return method?.trim() || '';
    }
</script>

<div class="section winning-stores">
    <h2 class="title">1등 당첨 판매점</h2>
    <div class="store-list">
        {#each winningStores as store}
            <div class="store-item">
                <h3>{store.name}</h3>
                <p class="method" 
                   class:auto={normalizeMethod(store.method) === '자동'} 
                   class:manual={normalizeMethod(store.method) === '수동'}>
                    {store.method}
                </p>
                <p class="address">{store.address}</p>
            </div>
        {/each}
    </div>
</div>

<style>
    .title {
        text-align: center;
        margin-bottom: 1.5rem;
        color: #2c3e50;
    }

    .winning-stores {
        text-align: center;
    }
    
    .store-list {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        max-height: 400px;
        overflow-y: auto;
    }

    .store-item {
        padding: 1rem;
        border-radius: 8px;
        background: #f8f9fa;
        transition: transform 0.2s;
    }

    .store-item:hover {
        transform: translateY(-2px);
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }

    .store-item h3 {
        color: #2c3e50;
        margin: 0 0 0.5rem 0;
    }

    .store-item p {
        margin: 0;
        color: #666;
    }

    .method {
        font-weight: bold;
        margin-top: 0.5rem;
        color: #666; /* 기본 색상 */
    }

    .auto {
        color: #e74c3c !important;  /* 자동: 빨간색 */
    }

    .manual {
        color: #3498db !important;  /* 수동: 파란색 */
    }

    .store-item .address {
        margin-top: 0.5rem;
    }
</style> 