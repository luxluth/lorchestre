import { readFile } from '@tauri-apps/plugin-fs';

export async function createUrlFrom(u: string) {
	let content = await readFile(new URL(u));
	const blob = new Blob([content]);
	return URL.createObjectURL(blob);
}
