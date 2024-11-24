import { json } from '@sveltejs/kit';

export async function GET() {
    const numbers = generateLottoNumbers();
    return json({ numbers });
}

function generateLottoNumbers() {
    const numbers = new Set();
    while (numbers.size < 6) {
        numbers.add(Math.floor(Math.random() * 45) + 1);
    }
    return Array.from(numbers).sort((a, b) => a - b);
} 