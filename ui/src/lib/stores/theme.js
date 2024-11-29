import { writable } from 'svelte/store';

// localStorage에서 테마 설정을 가져오거나, 기본값으로 'light' 사용
const storedTheme = typeof window !== 'undefined' ? localStorage.getItem('theme') : 'light';
export const theme = writable(storedTheme || 'light');

// 테마 변경 시 localStorage에 저장
theme.subscribe(value => {
    if (typeof window !== 'undefined') {
        localStorage.setItem('theme', value);
    }
}); 