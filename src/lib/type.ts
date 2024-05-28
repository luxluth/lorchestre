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

type u8 = number;

export type Color = {
	r: u8;
	g: u8;
	b: u8;
};

export type Cover = {
	data: number[];
	mime: string;
};

type u32 = number;
type u64 = number;

export type Album = {
	name: string;
	artist: string;
	tracks: Track[];
	year?: u32;
	id: string;
};

export type Track = {
	title: string;
	artists: string[];
	track: u32;
	album: string;
	album_artist?: string;
	album_id: string;
	album_year?: u32;
	lyrics: LyricLine[];
	cover?: string;
	color?: Color;
	is_light?: boolean;
	file_path: string;
	duration: u64;
};

export type Media = {
	albums: Album[];
};

export type Line = {
	startTime: number;
	endTime: number;
	text: string;
	id: number;
};

export enum PlayerDispatchKind {
	TimeUpdate,
	NewMedia,
	PlayPause
}

export type PlayerDispatch =
	| {
			kind: PlayerDispatchKind.TimeUpdate;
			data: number;
	  }
	| {
			kind: PlayerDispatchKind.NewMedia;
			data: Track;
	  }
	| {
			kind: PlayerDispatchKind.PlayPause;
			data: 'play' | 'paused';
	  };
