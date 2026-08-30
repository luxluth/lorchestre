use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use mtk::bytemuck;
use mtk::clr;
use mtk::style::{AlignItems, JustifyContent, Size, Style, TextStyle};
use mtk::ui::{
    ViewStyleExt,
    widgets::{PaintContext, WgpuPainter, column, text, wgpu_canvas},
};
use mtk::wgpu;
use mtk::windowing::{Window, WindowAttributes};

use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::formats::TrackType;
use symphonia::core::formats::probe::Hint;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;

pub const POINTS_PER_FRAME: usize = 1024;
pub const NUM_TRAIL_FRAMES: usize = 5;
pub const TARGET_FPS: u32 = 60;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[bytemuck(crate = "mtk::bytemuck")]
pub struct SoundBlobUniforms {
    pub time: f32,
    pub aspect: f32,
    pub base_radius: f32,
    pub amp_scale: f32,
    pub point_count: u32,
    pub trail_count: u32,
    pub _pad1: f32,
    pub _pad2: f32,
    pub flair_color: [f32; 4],
    pub secondary_color: [f32; 4],
}

#[derive(Clone, Debug)]
pub struct VisualFrame {
    pub displacements: [f32; POINTS_PER_FRAME],
}

#[derive(Clone, Debug)]
pub struct CachedSoundShape {
    pub sample_rate: u32,
    pub total_frames: usize,
    pub frames: Vec<VisualFrame>,
}

impl CachedSoundShape {
    pub fn from_audio_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let src = File::open(path)?;
        let mss = MediaSourceStream::new(Box::new(src), Default::default());

        let hint = Hint::new();
        let mut format = symphonia::default::get_probe().probe(
            &hint,
            mss,
            FormatOptions::default(),
            MetadataOptions::default(),
        )?;

        let track = format
            .default_track(TrackType::Audio)
            .or_else(|| format.first_track_known_codec(TrackType::Audio))
            .or_else(|| format.first_track(TrackType::Audio))
            .ok_or("No audio track found")?;

        let audio_params = track
            .codec_params
            .as_ref()
            .and_then(|cp| cp.audio())
            .ok_or("No audio codec params found")?;

        let sample_rate = audio_params.sample_rate.unwrap_or(44100);
        let mut decoder = symphonia::default::get_codecs()
            .make_audio_decoder(audio_params, &AudioDecoderOptions::default())?;

        let track_id = track.id;
        let mut raw_mono_samples: Vec<f32> = Vec::new();

        while let Ok(Some(packet)) = format.next_packet() {
            if packet.track_id != track_id {
                continue;
            }

            match decoder.decode(&packet) {
                Ok(decoded) => {
                    let channels = decoded.spec().channels().count().max(1);
                    let mut interleaved = Vec::new();
                    decoded.copy_to_vec_interleaved(&mut interleaved);
                    for chunk in interleaved.chunks(channels) {
                        let sum: f32 = chunk.iter().sum();
                        raw_mono_samples.push(sum / channels as f32);
                    }
                }
                Err(symphonia::core::errors::Error::DecodeError(_)) => (),
                Err(e) => return Err(Box::new(e)),
            }
        }

        if raw_mono_samples.is_empty() {
            return Ok(CachedSoundShape {
                sample_rate,
                total_frames: 0,
                frames: Vec::new(),
            });
        }

        let samples_per_frame = (sample_rate / TARGET_FPS) as usize;
        let total_frames = raw_mono_samples.len() / samples_per_frame.max(1);
        let mut frames = Vec::with_capacity(total_frames);

        for f in 0..total_frames {
            let start_idx = f * samples_per_frame;
            let mut frame = VisualFrame {
                displacements: [0.0; POINTS_PER_FRAME],
            };

            let available = raw_mono_samples.len() - start_idx;
            let window_len = POINTS_PER_FRAME.min(available);

            for i in 0..window_len {
                frame.displacements[i] = raw_mono_samples[start_idx + i];
            }

            let blend_len = 32.min(POINTS_PER_FRAME / 2);
            for i in 0..blend_len {
                let t = i as f32 / blend_len as f32;
                let s_start = frame.displacements[i];
                let s_end = frame.displacements[POINTS_PER_FRAME - blend_len + i];
                frame.displacements[i] = s_start * t + s_end * (1.0 - t);
            }

            frames.push(frame);
        }

        Ok(CachedSoundShape {
            sample_rate,
            total_frames: frames.len(),
            frames,
        })
    }

    pub fn get_interpolated_frame_at_time(&self, seconds: f32) -> Option<VisualFrame> {
        if self.frames.is_empty() {
            return None;
        }
        let total_time_frames = seconds * TARGET_FPS as f32;
        let idx0 = (total_time_frames.floor() as usize).min(self.frames.len() - 1);
        let idx1 = (idx0 + 1).min(self.frames.len() - 1);
        let fract = total_time_frames.fract();

        let f0 = &self.frames[idx0];
        let f1 = &self.frames[idx1];

        let mut result = VisualFrame {
            displacements: [0.0; POINTS_PER_FRAME],
        };

        for i in 0..POINTS_PER_FRAME {
            result.displacements[i] =
                f0.displacements[i] * (1.0 - fract) + f1.displacements[i] * fract;
        }

        Some(result)
    }
}

#[derive(Clone)]
pub struct SoundBlobPainter {
    playback_start: Instant,
    cached_shape: Option<Arc<CachedSoundShape>>,
    pub flair_color: [f32; 4],
    pub secondary_color: [f32; 4],
    _audio_player: Option<Arc<rodio::Player>>,
    _audio_sink: Option<Arc<rodio::stream::MixerDeviceSink>>,
    pipeline: Option<Arc<wgpu::RenderPipeline>>,
    bind_group: Option<Arc<wgpu::BindGroup>>,
    uniform_buffer: Option<Arc<wgpu::Buffer>>,
    audio_storage_buffer: Option<Arc<wgpu::Buffer>>,
}

impl SoundBlobPainter {
    pub fn new(audio_path: Option<PathBuf>) -> Self {
        Self::with_colors(
            audio_path,
            [0.35, 0.55, 1.0, 1.0],  // Electric cyan / royal blue flair
            [0.65, 0.35, 0.95, 1.0], // Neon violet secondary
        )
    }

    pub fn with_colors(
        audio_path: Option<PathBuf>,
        flair_color: [f32; 4],
        secondary_color: [f32; 4],
    ) -> Self {
        let mut cached_shape = None;
        let mut audio_player = None;
        let mut audio_sink = None;

        if let Some(path) = audio_path {
            match CachedSoundShape::from_audio_file(&path) {
                Ok(shape) => {
                    cached_shape = Some(Arc::new(shape));
                }
                Err(e) => eprintln!("Failed to parse audio file: {e}"),
            }

            if let Ok(sink_handle) = rodio::DeviceSinkBuilder::open_default_sink() {
                if let Ok(file) = File::open(&path) {
                    if let Ok(decoder) = rodio::Decoder::try_from(file) {
                        let player = rodio::Player::connect_new(sink_handle.mixer());
                        player.append(decoder);
                        player.set_speed(432.0 / 440.0);
                        player.play();
                        audio_player = Some(Arc::new(player));
                        audio_sink = Some(Arc::new(sink_handle));
                    }
                }
            }
        }

        Self {
            playback_start: Instant::now(),
            cached_shape,
            flair_color,
            secondary_color,
            _audio_player: audio_player,
            _audio_sink: audio_sink,
            pipeline: None,
            bind_group: None,
            uniform_buffer: None,
            audio_storage_buffer: None,
        }
    }

    pub fn with_flair_color(mut self, flair_color: [f32; 4]) -> Self {
        self.flair_color = flair_color;
        self
    }

    pub fn with_secondary_color(mut self, secondary_color: [f32; 4]) -> Self {
        self.secondary_color = secondary_color;
        self
    }
}

const BLOB_SHADER_SOURCE: &str = include_str!("./pusle.wgsl");

impl WgpuPainter for SoundBlobPainter {
    fn init(&mut self, device: &wgpu::Device, _queue: &wgpu::Queue, format: wgpu::TextureFormat) {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Sound Blob Shader"),
            source: wgpu::ShaderSource::Wgsl(BLOB_SHADER_SOURCE.into()),
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Sound Blob Uniform Buffer"),
            size: std::mem::size_of::<SoundBlobUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let audio_storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Sound Blob Storage Buffer"),
            size: (POINTS_PER_FRAME * NUM_TRAIL_FRAMES * std::mem::size_of::<f32>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Sound Blob Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Sound Blob Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: audio_storage_buffer.as_entire_binding(),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Sound Blob Pipeline Layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Sound Blob Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        self.pipeline = Some(Arc::new(pipeline));
        self.bind_group = Some(Arc::new(bind_group));
        self.uniform_buffer = Some(Arc::new(uniform_buffer));
        self.audio_storage_buffer = Some(Arc::new(audio_storage_buffer));
    }

    fn prepare(&mut self, _device: &wgpu::Device, queue: &wgpu::Queue) {
        let elapsed = self.playback_start.elapsed().as_secs_f32();

        if let Some(buf) = &self.uniform_buffer {
            let uniforms = SoundBlobUniforms {
                time: elapsed,
                aspect: 600.0 / 400.0,
                base_radius: 0.38,
                amp_scale: 0.28,
                point_count: POINTS_PER_FRAME as u32,
                trail_count: NUM_TRAIL_FRAMES as u32,
                _pad1: 0.0,
                _pad2: 0.0,
                flair_color: self.flair_color,
                secondary_color: self.secondary_color,
            };
            queue.write_buffer(buf, 0, bytemuck::bytes_of(&uniforms));
        }

        if let Some(storage_buf) = &self.audio_storage_buffer {
            let mut sample_data = vec![0.0f32; POINTS_PER_FRAME * NUM_TRAIL_FRAMES];

            if let Some(cached) = &self.cached_shape {
                for t in 0..NUM_TRAIL_FRAMES {
                    let trail_time = (elapsed - (t as f32) * (1.0 / TARGET_FPS as f32)).max(0.0);
                    if let Some(frame) = cached.get_interpolated_frame_at_time(trail_time) {
                        let offset = t * POINTS_PER_FRAME;
                        sample_data[offset..offset + POINTS_PER_FRAME]
                            .copy_from_slice(&frame.displacements);
                    }
                }
            } else {
                for t in 0..NUM_TRAIL_FRAMES {
                    let trail_elapsed = elapsed - (t as f32) * 0.04;
                    let offset = t * POINTS_PER_FRAME;
                    let decay = (1.0 - (t as f32) * 0.18).max(0.0);
                    for i in 0..POINTS_PER_FRAME {
                        let theta = (i as f32 / POINTS_PER_FRAME as f32) * std::f32::consts::TAU
                            - std::f32::consts::FRAC_PI_2;
                        let wave1 = (theta * 3.0 + trail_elapsed * 1.5).sin() * 0.12;
                        let wave2 = (theta * 6.0 - trail_elapsed * 2.0).cos() * 0.06;
                        sample_data[offset + i] = (wave1 + wave2) * decay;
                    }
                }
            }

            queue.write_buffer(storage_buf, 0, bytemuck::cast_slice(&sample_data));
        }
    }

    fn paint(&mut self, ctx: &mut PaintContext) {
        {
            let mut rpass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Sound Blob Canvas Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: ctx.target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            if let (Some(pipeline), Some(bind_group)) = (&self.pipeline, &self.bind_group) {
                rpass.set_pipeline(pipeline);
                rpass.set_bind_group(0, bind_group.as_ref(), &[]);
                rpass.draw(0..3, 0..1);
            }
        }

        ctx.request_frame();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let audio_file_path = args.get(1).map(PathBuf::from);

    let display_title = audio_file_path
        .as_ref()
        .and_then(|p| p.file_name())
        .and_then(|f| f.to_str())
        .unwrap_or("Demo Waveform (No File Provided)")
        .to_string();

    let painter = SoundBlobPainter::new(audio_file_path)
        .with_flair_color([0.25, 0.45, 0.95, 1.0]) //
        .with_secondary_color([0.70, 0.25, 0.85, 1.0]); // 

    let title_string = format!("Playing: {display_title}");

    let mut window = Window::with(
        (),
        |_state, _msg: ()| {},
        move |_state| {
            column((
                text("Sound Shape Visualizer").style(Style::new().padding(10.0).set_text_style(
                    TextStyle {
                        font_size: 22.0,
                        color: clr!(white),
                        ..Default::default()
                    },
                )),
                text(&title_string).style(Style::new().padding(5.0).set_text_style(TextStyle {
                    font_size: 13.0,
                    color: clr!(white).with_alpha(220), // rgba!(18, 18, 18, 220),
                    ..Default::default()
                })),
                wgpu_canvas(painter.clone()).style(
                    Style::new()
                        .width(Size::Fixed(600))
                        .height(Size::Fixed(400)),
                ),
            ))
            .style(
                Style::new()
                    .bg_color(clr!(0x181818FF))
                    .padding(24.0)
                    .width(Size::Percent(1.0))
                    .height(Size::Percent(1.0))
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Center),
            )
        },
    );

    let attrs = WindowAttributes::new()
        .with_title("MTK Sound Shape Visualizer")
        .with_size((800, 600).into());

    window.present_with(attrs);
}
