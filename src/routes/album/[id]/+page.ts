import type { PageLoad } from './$types';

export const csr = true;
export const prerender = false;

export const load: PageLoad = async ({ params }) => {
	return {
		id: params.id
	};
};
