<script>
    export let winningStores;
    
    // method 값 정규화 함수
    function normalizeMethod(method) {
        return method?.trim() || '';
    }
</script>

<div class="section winning-stores">
    <h1>1등 당첨 판매점</h1>
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
    h1 {
        text-align: center;
        margin-bottom: 1.5rem;
        color: #2c3e50;
        font-size: clamp(1.2rem, 4vw, 1.5rem);
    }

    .winning-stores {
        text-align: center;
    }
    
    .store-list {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        max-height: clamp(300px, 60vh, 400px);
        overflow-y: auto;
    }

    .store-item {
        padding: 1rem;
        border-radius: 8px;
        background: #f8f9fa;
        transition: all 0.2s ease;
    }

    .store-item:hover {
        transform: translateY(-2px);
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }

    :global(body.dark-theme) .store-item {
        background-color: #1a2733;
        color: #fff;
    }

    :global(body.dark-theme) .store-item:hover {
        background-color: #111a22;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    }

    :global(body.dark-theme) .store-item h3 {
        color: #fff;
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

    .store-item h3 {
        font-size: clamp(1rem, 3.5vw, 1.2rem);
    }

    .store-item p {
        font-size: clamp(0.875rem, 3vw, 1rem);
    }

    @media (max-width: 480px) {
        .store-item {
            padding: 0.75rem;
        }
    }
</style> 