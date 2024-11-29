import { json } from '@sveltejs/kit';
import { getChartData } from '$lib/lottoAPI';

export async function GET() {
    try {
        // DB에서 모든 당첨 번호 데이터 가져오기
        const response = await getChartData();
        const chartData = processChartData(response);
        return json(chartData);
    } catch (error) {
        console.error('차트 데이터 생성 중 오류:', error);
        return json({ error: '차트 데이터를 생성하는데 실패했습니다' }, { status: 500 });
    }
}

function processChartData(data) {
    const numberFrequency = Array(45).fill(0);

    data.forEach(round => {
        [round.num1, round.num2, round.num3, round.num4, round.num5, round.num6].forEach(num => {
            numberFrequency[num - 1]++;
        });
    });

    return numberFrequency.map((count, index) => ({
        number: index + 1,
        frequency: count
    }));
}