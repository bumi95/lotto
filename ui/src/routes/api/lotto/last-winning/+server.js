import { json } from '@sveltejs/kit';
import { getLastWinningNumbers } from '$lib/lottoAPI';

export async function GET() {
    try {
        const lastWinning = await getLastWinningNumbers();
        return json(lastWinning);
    } catch (error) {
        console.error('로또 정보 조회 중 오류:', error);
        return new Response(
            JSON.stringify({ error: '당첨 정보를 가져오는데 실패했습니다.' }), 
            { status: 500 }
        );
    }
} 