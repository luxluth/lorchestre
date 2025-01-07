type Handler = (value: any) => void;

export default class Ws {
	#ws: WebSocket;
	#listeners: Map<string, Set<Handler>> = new Map();
	constructor(endpoint: string) {
		this.#ws = new WebSocket(endpoint);
		this.#ws.onmessage = (event) => {
			const data = new String(event.data);
			let spans = data.split('\n');
			let [ev, value] = [spans.shift(), JSON.parse(spans.join('\n'))];
			this.#listeners.get(ev as string)?.forEach((f) => {
				f(value);
			});
		};
	}

	on(event: string, handler: Handler) {
		if (this.#listeners.has(event)) {
			this.#listeners.set(event, this.#listeners.get(event)!.add(handler));
		} else {
			this.#listeners.set(event, new Set<Handler>().add(handler));
		}
	}

	emit(event: string, value: any) {
		this.#ws.send(`${event}\n${JSON.stringify(value)}`);
	}
}
