export async function getLastWinningNumbers() {
    const response = await fetch('http://localhost:8080/api/lotto/last-winning');
    return await response.json();
}

export async function getWinningStores() {
    const response = await fetch('http://localhost:8080/api/lotto/winning-stores');
    return await response.json();
}

export async function getChartData() {
    const response = await fetch('http://localhost:8080/api/lotto/db-chart');
    return await response.json();
}

export async function generateNumbers() {
    const response = await fetch('http://localhost:8080/api/lotto/generate');
    return await response.json();
}