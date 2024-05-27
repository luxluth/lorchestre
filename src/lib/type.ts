export type ToastEvent =
	| {
			kind: ToastKind;
			title: string;
			message: string;
			timeout: number;
	  }
	| {
			kind: ToastKind.Close;
	  };

export enum ToastKind {
	Error,
	Message,
	Loading,
	Close
}

export type LyricLine = {
	start_time: number;
	text: string;
};

export type Cover = {
	data: number[];
	mime: string;
};

export type AudioMedia = {
	title?: string;
	artists: string[];
	album?: string;
	lyrics: LyricLine[];
	cover?: string;
	color?: string;
	is_light?: boolean;
	file_path: string;
	duration: number;
};

export type Media = {
	audios: AudioMedia[];
};
