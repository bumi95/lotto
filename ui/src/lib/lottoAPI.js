export async function getLastWinningNumbers() {
    const response = await fetch('http://change-my-life.fly.dev/api/lotto/last-winning');
    return await response.json();
}

export async function getWinningStores() {
    const response = await fetch('http://change-my-life.fly.dev/api/lotto/winning-stores');
    return await response.json();
}

export async function getChartData() {
    const response = await fetch('http://change-my-life.fly.dev/api/lotto/db-chart');
    return await response.json();
}

export async function generateNumbers() {
    const response = await fetch('http://change-my-life.fly.dev/api/lotto/generate');
    return await response.json();
}