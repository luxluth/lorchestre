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
	tracks: string[];
	year?: u32;
	id: string;
};

export type SystemTime = {
	nanos_since_epoch: number;
	secs_since_epoch: number;
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
	cover_ext: string;
	mime: string;
	color?: Color;
	created_at: SystemTime;
	is_light?: boolean;
	file_path: string;
	duration: u64;
	bitrate: u32;
};

export type QueueTrack = Track & {
	id: string;
};

export type Playlist = {
	name: string;
	tracks: string[];
	path: string;
	id: string;
};

export type Media = {
	tracks: Record<string, Track>;
	albums: Album[];
	playlists: Playlist[];
};

export type SearchResults = {
	albums: Array<Album>;
	playlists: Array<Playlist>;
	tracks: Array<Track>;
};

export type Line = {
	startTime: number;
	endTime: number;
	text: string;
	id: number;
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
	Alphabetic,
	TimeBased
}

export enum FilterOrder {
	Ascendant,
	Descendant
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
	Continue,
	Repeat,
	RepeatAll
}
