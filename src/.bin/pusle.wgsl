struct Uniforms {
    time: f32,
    aspect: f32,
    base_radius: f32,
    amp_scale: f32,
    point_count: u32,
    trail_count: u32,
    _pad1: f32,
    _pad2: f32,
    flair_color: vec4<f32>,
    secondary_color: vec4<f32>,
};

@group(0) @binding(0) var<uniform> u: Uniforms;
@group(0) @binding(1) var<storage, read> audio_data: array<f32>;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

const PI: f32 = 3.14159265359;
const TAU: f32 = 6.28318530718;

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(i32(in_vertex_index & 1u) * 4 - 1);
    let y = f32(i32(in_vertex_index & 2u) * 2 - 1);
    out.position = vec4<f32>(x, y, 0.0, 1.0);
    out.uv = vec2<f32>(x, y);
    return out;
}

fn sample_audio_frame(angle: f32, frame_idx: u32) -> f32 {
    let norm = fract((angle + PI * 0.5) / TAU);
    let total_pts = f32(u.point_count);
    let raw_idx = norm * total_pts;
    let base_offset = frame_idx * u.point_count;

    let i0 = u32(floor(raw_idx)) % u.point_count;
    let i1 = (i0 + 1u) % u.point_count;
    let f = fract(raw_idx);
    let smooth_f = f * f * (3.0 - 2.0 * f);

    let s0 = audio_data[base_offset + i0];
    let s1 = audio_data[base_offset + i1];
    return mix(s0, s1, smooth_f);
}

fn hash12(p: vec2<f32>) -> f32 {
    let p3 = fract(vec3<f32>(p.xyx) * 0.1031);
    let dot_val = dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * dot_val);
}

fn calc_particles(p: vec2<f32>, target_radius: f32) -> vec4<f32> {
    let dist = length(p);
    let dr = dist - target_radius;
    if dr < -0.02 || dr > 0.32 {
        return vec4<f32>(0.0);
    }

    let angle = atan2(p.y, p.x);
    let sectors = 72.0;
    let sector_f = (angle + PI) / TAU * sectors;
    let sector_i = floor(sector_f);

    var spark_color = vec3<f32>(0.0);
    var spark_alpha = 0.0;

    for (var s = -1; s <= 1; s = s + 1) {
        let sec = (sector_i + f32(s) + sectors) % sectors;
        for (var k = 0; k < 3; k = k + 1) {
            let seed = sec * 3.17 + f32(k) * 13.53;
            let speed = 0.15 + hash12(vec2<f32>(seed, 1.0)) * 0.35;
            let life = fract(u.time * speed + hash12(vec2<f32>(seed, 2.0)));
            let spark_r = target_radius + life * 0.24;

            let spark_ang = (sec + 0.5 + (hash12(vec2<f32>(seed, 3.0)) - 0.5) * 0.75) / sectors * TAU - PI;
            let spark_pos = vec2<f32>(cos(spark_ang), sin(spark_ang)) * spark_r;

            let p_dist = length(p - spark_pos);
            let size = 0.0026 + hash12(vec2<f32>(seed, 4.0)) * 0.0020;
            let core_spark = smoothstep(size, 0.0, p_dist);
            let halo_spark = 0.0012 / (p_dist + 0.0016);
            let fade = (1.0 - life) * smoothstep(0.0, 0.10, life);

            let base_col = mix(u.flair_color.rgb, u.secondary_color.rgb, hash12(vec2<f32>(seed, 5.0)));
            let intensity = (core_spark * 1.8 + halo_spark * 0.7) * fade;

            spark_color += base_col * intensity;
            spark_alpha += (core_spark * 0.95 + halo_spark * 0.45) * fade;
        }
    }

    let final_alpha = clamp(spark_alpha, 0.0, 1.0);
    return vec4<f32>(spark_color, final_alpha);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var p = in.uv;
    if u.aspect > 1.0 {
        p.x *= u.aspect;
    } else {
        p.y /= u.aspect;
    }

    let angle = atan2(p.y, p.x);
    let dist = length(p);

    let disp0 = sample_audio_frame(angle, 0u) * u.amp_scale;
    let r0 = u.base_radius + disp0;
    let d0 = abs(dist - r0);

    let max_radius = 0.28;
    let edge_fade = clamp(1.0 - d0 / max_radius, 0.0, 1.0);
    let falloff = edge_fade * edge_fade * (3.0 - 2.0 * edge_fade);

    // 1. Organic Interior Fill (tinted with flair_color)
    let fill_mask = smoothstep(r0, r0 - 0.04, dist);
    let fill_color = u.flair_color.rgb * fill_mask * 0.14;
    let fill_alpha = fill_mask * 0.14;

    // 2. Main Glow & Stroke
    let bloom0 = (0.0065 / (d0 + 0.0025)) * falloff;
    let core0 = smoothstep(0.005, 0.001, d0) * falloff;
    let stroke0_color = u.flair_color.rgb * bloom0 + mix(u.flair_color.rgb, vec3<f32>(1.0), 0.35) * core0 * 1.6;
    let stroke0_alpha = clamp(bloom0 * 0.35 + core0 * 0.85, 0.0, 1.0);

    // 3. Multi-Frame Historical Echo Trails
    var trail_rgb = vec3<f32>(0.0);
    var trail_alpha = 0.0;
    let trail_weights = array<f32, 4>(0.32, 0.20, 0.12, 0.06);

    for (var t = 1u; t < 5u; t = t + 1u) {
        let disp_t = sample_audio_frame(angle, t) * u.amp_scale;
        let r_t = u.base_radius + disp_t;
        let dt = abs(dist - r_t);
        let tfade = clamp(1.0 - dt / max_radius, 0.0, 1.0);
        let tfalloff = tfade * tfade * (3.0 - 2.0 * tfade);
        let weight = trail_weights[t - 1u];

        let t_factor = f32(t) / 5.0;
        let tint = mix(u.flair_color.rgb, u.secondary_color.rgb, t_factor);

        let t_bloom = (0.0035 / (dt + 0.003)) * tfalloff * weight;
        let t_core = smoothstep(0.004, 0.001, dt) * tfalloff * weight;

        trail_rgb += tint * (t_bloom + t_core * 0.8);
        trail_alpha += (t_bloom * 0.3 + t_core * 0.5);
    }

    // 4. Fine Irradiating Stardust Particles
    let particles = calc_particles(p, r0);

    // 5. Radiating Flare Rays on Peak Moments
    let dr = dist - r0;
    let ray_noise = sin(angle * 24.0 + u.time * 2.0) * sin(angle * 12.0 - u.time * 1.5);
    let ray = max(0.0, ray_noise) * smoothstep(0.0, 0.03, dr) * smoothstep(0.18, 0.02, dr) * 0.14;
    let ray_color = u.flair_color.rgb * ray;
    let ray_alpha = ray * 0.5;

    let final_rgb = stroke0_color + fill_color + trail_rgb + particles.rgb + ray_color;
    let final_alpha = clamp(stroke0_alpha + fill_alpha + trail_alpha + particles.a + ray_alpha, 0.0, 1.0);

    return vec4<f32>(final_rgb, final_alpha);
}
