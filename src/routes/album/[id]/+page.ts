import type { PageLoad } from './$types';
import type { Album } from '$lib/type';
import { MUD_ENDPOINT } from '$lib/config';

export const csr = true;
export const prerender = false;

export const load: PageLoad = async ({ params, fetch }) => {
	let req = await fetch(`${MUD_ENDPOINT}/album/${params.id}`);
	if (req.ok) {
		return {
			album: (await req.json()) as Album
		};
	}
	return {
		error: await req.text()
	};
};
