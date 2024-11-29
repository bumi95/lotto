import { json } from '@sveltejs/kit';
import { generateNumbers } from '$lib/lottoAPI';

export async function GET() {
    const numbers = await generateNumbers();
    return json({ numbers });
}
