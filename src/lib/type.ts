export type ToastData = {
	kind: ToastKind;
	title?: string;
	message: string;
	timeout: number;
};

export enum ToastKind {
	Error,
	Simple,
	Loading,
	Success
}

export type Toast = ToastData & { id: number };

export type RawTimestamp = {
	minutes: u8;
	seconds: u8;
	millis: Option<u8>;
};

export type Vocal = {
	text: string;
	time: RawTimestamp;
	syllables: Vec<Syllable>;
};

export type Syllable = {
	text: string;
	start: RawTimestamp;
};

export type Marker =
	| {
			Named: [string, string];
	  }
	| 'Empty';

export type ProccessedMarker = {
	i: number;
	artistName: string;
	isMainVocal: boolean;
	order: number;
};

export type LyricLine = {
	marker: Marker;
	syllables: Vec<Syllable>;
	text: string;
	time: RawTimestamp;
	vocals: Vec<Vocal>;
	ln: usize;
	is_instrumental: boolean;
};

type u8 = number;
type usize = number;
type Option<T> = T | null;
type Vec<T> = T[];

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
	tracks: string[];
	genres: string[];
	disc_total: number;
	tracks_count: number;
	encoder: string;
	year?: u32;
	id: string;
};

export type SystemTime = number;

export type Track = {
	title: string;
	artists: string[];
	track: u32;
	album: string;
	album_artist?: string;
	album_id: string;
	album_year?: u32;
	lyrics: LyricLine[];
	cover_ext: string;
	mime: string;
	color?: Color;
	created_at: SystemTime;
	is_light?: boolean;
	file_path: string;
	path_base64: string;
	duration: u64;
	bitrate: u32;
	disc: number;
	encoder: string;
	embeded_lyrics: Option<string>;
	genres: string[];
};

export type QueueTrack = Track & {
	id: string;
};

export type Playlist = {
	metadata: Record<string, string>;
	tracks: Vec<string>;
	path: string;
	path_base64: string;
};

export type Media = {
	tracks: Array<[string, Track]>;
	albums: Album[];
	playlists: Playlist[];
};

export type SearchResults = {
	albums: Array<Album>;
	tracks: Array<Track>;
};

export type Line = {
	startTime: number;
	endTime: number;
	text: string;
	id: number;
	vocals: Vec<Vocal>;
	syllables: Vec<Syllable>;
	isInstrumental: boolean;
	marker: Marker;
};

export enum MessageKind {
	NeedCacheUpdate
}

export type Payload = {
	kind: MessageKind;
};

export enum ContextMenuItemType {
	Separator,
	Action
}

export type ContextMenuItem =
	| {
			type: ContextMenuItemType.Separator;
	  }
	| {
			type: ContextMenuItemType.Action;
			label: string;
			icon?: any;
			action: (data?: any) => Promise<void>;
	  };

export type ContextMenuEvent =
	| (MouseEvent & {
			currentTarget: EventTarget & HTMLDivElement;
	  })
	| (MouseEvent & {
			currentTarget: EventTarget & HTMLAnchorElement;
	  });

export enum FilterType {
	Alphabetic = 'Alphabetic',
	TimeBased = 'TimeBased',
	NoFilter = 'NoFilter'
}

export enum FilterOrder {
	Ascendant = 'Ascendant',
	Descendant = 'Descendant'
}

export type Network = {
	port?: u32;
	host?: string;
};

export type Theme = 'auto' | 'dark' | 'light';

export type Global = {
	enable_blur?: boolean;
	lang?: string;
	theme?: Theme;
};

export type Config = {
	global?: Global;
	network?: Network;
};

type DeepRequired<T> = {
	[K in keyof T]-?: NonNullable<T[K]> extends object
		? DeepRequired<NonNullable<T[K]>>
		: NonNullable<T[K]>;
};

export type DefinedConfig = DeepRequired<Config>;
export type AppInfoExternal = {
	first_run: boolean;
};

export enum QueueAddMode {
	Top,
	Bottom
}

export enum PlayingMode {
	Normal,
	Shuffle
}

export enum QueueMode {
	Continue = 'continue',
	Repeat = 'repeat',
	RepeatAll = 'repeat-all'
}

export type Lrc = {
	parsed: LyricLine[];
	raw: string;
};

export type LyricsResponse = {
	lyrics: Array<Lrc>;
};
