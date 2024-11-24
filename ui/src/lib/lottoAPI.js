export async function getLastWinningNumbers() {
    const response = await fetch('http://localhost:8080/api/lotto/last-winning');
    return await response.json();
}

export async function getWinningStores() {
    const response = await fetch('http://localhost:8080/api/lotto/winning-stores');
    return await response.json();
}

function formatPrize(amount) {
    const billion = Math.floor(amount / 100000000);
    const million = Math.floor((amount % 100000000) / 10000);
    
    let result = '';
    if (billion > 0) result += `${billion}억 `;
    if (million > 0) result += `${million}만`;
    
    return result + '원';
} 