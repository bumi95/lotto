import { json } from '@sveltejs/kit';
import { getWinningStores } from '$lib/lottoAPI';

export async function GET() {
    try {
        //const { round } = await getLastWinningNumbers();
        const stores = await getWinningStores();
        return json(stores);
    } catch (error) {
        console.error('당첨 판매점 정보 조회 중 오류:', error);
        return new Response(
            JSON.stringify({ error: '당첨 판매점 정보를 가져오는데 실패했습니다.' }), 
            { status: 500 }
        );
    }
}