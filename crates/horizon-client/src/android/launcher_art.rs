//! Pixel-art decorations and theme backgrounds for the Hyper OS launcher.
//! All art is drawn with filled rects (no image assets). Animations are
//! frame-stepped at low rates and intended to pause when the game runs.

use horizon_client::ui_theme::{color, Ui};
use std::time::Instant;

/// Logo skull 9×8 (X = bone, E = accent eyes). Grid is row-major, left to right.
const LOGO_SKULL: [&str; 8] = [
    "..XXXXX..",
    ".XXXXXXX.",
    "XXXXXXXXX",
    "XXEEXEEXX",
    "XXEEXEEXX",
    "XXXXXXXXX",
    ".XXXXXXX.",
    "..X.X.X..",
];

/// Fire-skull body 15×11 (X = bone #d9cfc0, E = eye sockets).
const FIRE_SKULL: [&str; 11] = [
    "XX..XXXXXXX..XX",
    ".XXXXXXXXXXXXX.",
    "..XXXXXXXXXXX..",
    "..XXXXXXXXXXX..",
    "..XXEEXXXEEXX..",
    "..XEEEXXXEEEX..",
    "..XXEEXXXEEXX..",
    "..XXXXX.XXXXX..",
    "...XXXXXXXXX...",
    "...X.X.X.X.X...",
    "...XXXXXXXXX...",
];

/// Flame column heights (19 cols) for 3 frames, bottom-aligned.
const FLAME_FRAMES: [[u8; 19]; 3] = [
    [1, 1, 2, 3, 4, 5, 6, 6, 8, 6, 8, 6, 6, 5, 4, 3, 2, 1, 1],
    [1, 1, 3, 2, 5, 4, 7, 6, 7, 8, 7, 7, 5, 6, 3, 4, 2, 2, 1],
    [1, 2, 2, 4, 3, 6, 5, 8, 7, 6, 8, 6, 7, 4, 5, 3, 3, 1, 1],
];

/// Enderman head 10×8 (B = body, T = top row, E = eye bar).
const ENDERMAN: [&str; 8] = [
    "TTTTTTTTTT",
    "BBBBBBBBBB",
    "BBBBBBBBBB",
    "BEEBBBBEEB",
    "BEEBBBBEEB",
    "BBBBBBBBBB",
    "BBBBBBBBBB",
    "BBBBBBBBBB",
];

/// Warden head 11×9 (B = body, G = glow chest/ribs, T = tendril).
const WARDEN: [&str; 9] = [
    "..BBBBBBB..",
    ".BBBBBBBBB.",
    "BBBBBBBBBBB",
    "BBTBBBBBTBB",
    "BTBBGBBBTBB",
    "BBBGGBGBBBB",
    "BBBBGBBBBBB",
    ".BBBBBBBBB.",
    "..BBBBBBB..",
];

fn rgba(rgb: u32, a: u8) -> u32 {
    color::hex(rgb, a)
}

fn pixel(ui: &mut Ui, x: f32, y: f32, size: f32, c: u32) {
    if c >> 24 == 0 {
        return;
    }
    ui.rounded(x, y, size, size, 0.0, c);
}

/// Small logo skull next to the title (bone white + accent eyes).
pub fn draw_logo_skull(ui: &mut Ui, origin_x: f32, origin_y: f32, px: f32, accent: u32) {
    let bone = rgba(0xd9cfc0, 255);
    for (row, line) in LOGO_SKULL.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let c = match ch {
                'X' => bone,
                'E' => accent,
                _ => continue,
            };
            pixel(ui, origin_x + col as f32 * px, origin_y + row as f32 * px, px, c);
        }
    }
}

/// Animated fire skull + flames centered behind the Play button.
/// `opacity` ~0.35–0.40. Eyes flicker ~0.5s, flames ~0.24s.
pub fn draw_fire_skull(
    ui: &mut Ui,
    cx: f32,
    cy: f32,
    px: f32,
    t0: Instant,
    anim: bool,
) {
    let elapsed = t0.elapsed().as_secs_f32();
    let eye_hot = if anim {
        ((elapsed / 0.5) as u32 % 2) == 0
    } else {
        true
    };
    let eye = if eye_hot {
        rgba(0xd8b4fe, 230)
    } else {
        rgba(0x9333ea, 220)
    };
    let bone = rgba(0xd9cfc0, 100); // ~40% overall feel via alpha
    let cols = 15.0;
    let rows = 11.0;
    let ox = cx - (cols * px) * 0.5;
    let oy = cy - (rows * px) * 0.35;

    // Flames first (behind skull), 19 cols × up to 8 rows, overlap top 2 skull rows.
    {
        let frame = if anim {
            ((elapsed / 0.24) as usize) % 3
        } else {
            0
        };
        let heights = &FLAME_FRAMES[frame];
        let flame_ox = cx - (19.0 * px) * 0.5;
        let flame_base = oy + 2.0 * px; // overlap top of skull
        for (i, &h) in heights.iter().enumerate() {
            let hx = flame_ox + i as f32 * px;
            for level in 0..h {
                let ly = flame_base - (level as f32 + 1.0) * px;
                let frac = (level as f32 + 1.0) / h as f32;
                let c = if frac > 0.7 {
                    rgba(0x6d28d9, 150)
                } else if frac > 0.4 {
                    rgba(0xa855f7, 160)
                } else {
                    rgba(0xe9d5ff, 170)
                };
                pixel(ui, hx, ly, px, c);
            }
        }
    }

    for (row, line) in FIRE_SKULL.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let c = match ch {
                'X' => bone,
                'E' => eye,
                _ => continue,
            };
            pixel(ui, ox + col as f32 * px, oy + row as f32 * px, px, c);
        }
    }
}

/// The End theme decorations: Enderman head, pillars, crystals, floor, particles.
pub fn draw_end_theme(ui: &mut Ui, w: f32, h: f32, t0: Instant, anim: bool) {
    let elapsed = t0.elapsed().as_secs_f32();
    let px = (h / 28.0).clamp(3.0, 6.0);
    let cx = w * 0.42;
    let cy = h * 0.48;

    // Enderman head
    let body = rgba(0x15101f, 255);
    let top = rgba(0x241b44, 255);
    let eye = rgba(0xd070ff, 255);
    let eye_white = rgba(0xffffff, 255);
    let pulse = if anim {
        0.7 + 0.3 * ((elapsed * 2.2).sin() * 0.5 + 0.5)
    } else {
        1.0
    };
    let eye_a = (220.0 * pulse) as u8;
    let ox = cx - 5.0 * px;
    let oy = cy - 4.0 * px;
    for (row, line) in ENDERMAN.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let c = match ch {
                'T' => top,
                'B' => body,
                'E' => rgba(0xd070ff, eye_a),
                _ => continue,
            };
            pixel(ui, ox + col as f32 * px, oy + row as f32 * px, px, c);
        }
    }
    // White tips on outer ends of eye bars (row 3 and 4, cols 1 and 8)
    pixel(ui, ox + 1.0 * px, oy + 3.0 * px, px, eye_white);
    pixel(ui, ox + 8.0 * px, oy + 3.0 * px, px, eye_white);
    pixel(ui, ox + 1.0 * px, oy + 4.0 * px, px, eye_white);
    pixel(ui, ox + 8.0 * px, oy + 4.0 * px, px, eye_white);

    // Obsidian pillars
    let pillar_w = px * 2.5;
    let pillar_h = h * 0.55;
    let left_x = w * 0.08;
    let right_x = w * 0.72;
    let pillar_y = h - pillar_h - 18.0;
    let obs = rgba(0x1b1628, 255);
    let obs_edge = rgba(0x2a2240, 255);
    for &(px0, _) in &[(left_x, true), (right_x, false)] {
        ui.rounded(px0, pillar_y, pillar_w, pillar_h, 1.0, obs);
        ui.rounded(px0, pillar_y, px * 0.6, pillar_h, 0.0, obs_edge);
    }

    // Floating end crystals (bob 3px)
    let bob = if anim { (elapsed * 1.8).sin() * 3.0 } else { 0.0 };
    for &cx0 in &[left_x + pillar_w * 0.5, right_x + pillar_w * 0.5] {
        let cy0 = pillar_y - 10.0 + bob;
        // diamond
        let s = px * 2.2;
        ui.rounded(cx0 - s * 0.5, cy0 - s * 0.5, s, s, 1.0, rgba(0xe04fff, 230));
        ui.rounded(cx0 - s * 0.25, cy0 - s * 0.25, s * 0.5, s * 0.5, 0.5, rgba(0xffd0ff, 255));
    }

    // End stone floor strip
    ui.rounded(0.0, h - 14.0, w, 14.0, 0.0, rgba(0xa8a26a, 255));
    ui.rounded(0.0, h - 14.0, w, 5.0, 0.0, rgba(0xcfc98a, 200));

    // Small purple particles
    if anim {
        for i in 0..6 {
            let seed = i as f32 * 17.3;
            let pxp = ((elapsed * 0.4 + seed).sin() * 0.5 + 0.5) * (w * 0.7) + w * 0.05;
            let pyp = h * 0.35 + ((elapsed * 0.7 + seed * 1.3).sin() * 0.5 + 0.5) * h * 0.4;
            let on = ((elapsed * 3.0 + seed) as u32 % 3) != 0;
            if on {
                pixel(ui, pxp, pyp, px * 0.7, rgba(0xa24bff, 180));
            }
        }
    }
}

/// Warden theme decorations.
pub fn draw_warden_theme(ui: &mut Ui, w: f32, h: f32, t0: Instant, anim: bool) {
    let elapsed = t0.elapsed().as_secs_f32();
    let px = (h / 26.0).clamp(3.0, 6.5);
    let cx = w * 0.42;
    let cy = h * 0.48;
    let body = rgba(0x143a40, 255);
    // Heartbeat pulse ~1.1s
    let beat = if anim {
        let phase = (elapsed % 1.1) / 1.1;
        if phase < 0.12 || (phase > 0.22 && phase < 0.32) {
            1.0
        } else {
            0.55
        }
    } else {
        0.8
    };
    let glow = rgba(0x3de0d0, (200.0 * beat) as u8);
    let tendril = rgba(0x2fd4c4, (180.0 * beat) as u8);

    let ox = cx - 5.5 * px;
    let oy = cy - 4.5 * px;
    for (row, line) in WARDEN.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let c = match ch {
                'B' => body,
                'G' => glow,
                'T' => tendril,
                _ => continue,
            };
            pixel(ui, ox + col as f32 * px, oy + row as f32 * px, px, c);
        }
    }

    // Sculk patches along bottom
    let sculk = rgba(0x1b8f9c, 220);
    for i in 0..8 {
        let sx = (i as f32 / 8.0) * w + ((elapsed * 0.3 + i as f32).sin() * 6.0);
        let sw = 18.0 + (i % 3) as f32 * 8.0;
        let sh = 6.0 + (i % 2) as f32 * 4.0;
        ui.rounded(sx, h - sh - 4.0, sw, sh, 2.0, sculk);
    }

    // Soul particles
    if anim {
        for i in 0..5 {
            let seed = i as f32 * 11.0;
            let pxp = ((elapsed * 0.25 + seed).sin() * 0.5 + 0.5) * w * 0.75 + w * 0.05;
            let pyp = h * 0.55 - ((elapsed * 0.5 + seed).rem_euclid(3.0)) * h * 0.12;
            pixel(ui, pxp, pyp, px * 0.6, rgba(0x3de0d0, 200));
        }
    }
}

/// Soft purple glass-like background used by the Hyper OS launcher.
/// Built from low-alpha shapes so it stays lightweight on mobile GPUs.
pub fn draw_hyperos_background(
    ui: &mut Ui,
    w: f32,
    h: f32,
    accent: u32,
    t0: Instant,
    anim: bool,
) {
    let elapsed = t0.elapsed().as_secs_f32();
    let glow = rgba(accent, 18);
    let glow_soft = rgba(accent, 10);
    let edge = rgba(accent, 34);

    // Broad ambient light pools; deliberately subtle behind readable cards.
    ui.rounded(-w * 0.08, h * 0.08, w * 0.58, h * 0.72, h * 0.20, glow_soft);
    ui.rounded(w * 0.48, h * 0.38, w * 0.60, h * 0.70, h * 0.22, glow_soft);
    ui.rounded(w * 0.16, h * 0.12, w * 0.66, h * 0.76, h * 0.24, rgba(0x211044, 26));

    // Thin inset frame gives the dark background a layered, glass-panel feel.
    ui.border(10.0, 10.0, (w - 20.0).max(1.0), (h - 20.0).max(1.0), 18.0, edge);
    ui.rounded(18.0, h * 0.17, w * 0.36, 2.0, 1.0, rgba(accent, 42));
    ui.rounded(w * 0.66, h * 0.82, w * 0.25, 2.0, 1.0, rgba(accent, 32));

    // Small drifting pixels add depth without a heavy image texture.
    for i in 0..14 {
        let seed = i as f32 * 19.17;
        let drift = if anim { elapsed } else { 0.0 };
        let x = ((seed * 1.71 + drift * (0.7 + (i % 3) as f32 * 0.13)).sin() * 0.5 + 0.5) * w;
        let y = ((seed * 0.83 + drift * 0.24).cos() * 0.5 + 0.5) * h;
        let size = if i % 4 == 0 { 4.0 } else { 2.0 };
        let alpha = if i % 3 == 0 { 62 } else { 34 };
        pixel(ui, x, y, size, rgba(accent, alpha));
    }
}

/// Theme identifiers (stored in config).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum LauncherTheme {
    #[default]
    Night,
    FireSkull,
    Midnight,
    Crimson,
    TheEnd,
    Warden,
}

impl LauncherTheme {
    pub const ALL: [Self; 6] = [
        Self::Night,
        Self::FireSkull,
        Self::Midnight,
        Self::Crimson,
        Self::TheEnd,
        Self::Warden,
    ];
    pub fn id(self) -> &'static str {
        match self {
            Self::Night => "night",
            Self::FireSkull => "fire-skull",
            Self::Midnight => "midnight",
            Self::Crimson => "crimson",
            Self::TheEnd => "the-end",
            Self::Warden => "warden",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Night => "Noturno",
            Self::FireSkull => "Chama roxa",
            Self::Midnight => "Meia-noite",
            Self::Crimson => "Carmesim",
            Self::TheEnd => "O End",
            Self::Warden => "Guardião",
        }
    }
    pub fn from_id(s: &str) -> Self {
        match s {
            "fire-skull" => Self::FireSkull,
            "midnight" => Self::Midnight,
            "crimson" => Self::Crimson,
            "the-end" => Self::TheEnd,
            "warden" => Self::Warden,
            _ => Self::Night,
        }
    }
    /// Default accent RGB for the theme (user can still override).
    pub fn default_accent(self) -> u32 {
        match self {
            Self::Night | Self::FireSkull => 0x8b6cf0,
            Self::Midnight => 0x2f7be0,
            Self::Crimson => 0xe0482f,
            Self::TheEnd => 0xa24bff,
            Self::Warden => 0x1fb3a6,
        }
    }
    pub fn bg(self) -> u32 {
        match self {
            Self::Night | Self::FireSkull => color::BG,
            Self::Midnight => rgba(0x0a1220, 255),
            Self::Crimson => rgba(0x14080a, 255),
            Self::TheEnd => rgba(0x08060f, 255),
            Self::Warden => rgba(0x041012, 255),
        }
    }
    pub fn panel(self) -> u32 {
        match self {
            Self::Night | Self::FireSkull => color::RAISED,
            Self::Midnight => rgba(0x121a2c, 220),
            Self::Crimson => rgba(0x1c0c10, 220),
            Self::TheEnd => rgba(0x17122a, 230),
            Self::Warden => rgba(0x0c2326, 230),
        }
    }
    pub fn play_shadow(self) -> u32 {
        match self {
            Self::TheEnd => rgba(0x6a2cb0, 180),
            Self::Warden => rgba(0x0f736b, 180),
            _ => rgba(0x000000, 140),
        }
    }
}

/// Accent swatches the Customize page offers.
pub const ACCENT_SWATCHES: [(u32, &str); 7] = [
    (0x8b6cf0, "Purple"),
    (0x2f7be0, "Blue"),
    (0x1fa37a, "Green"),
    (0xe0a82f, "Yellow"),
    (0xe0482f, "Red"),
    (0xa24bff, "End"),
    (0x1fb3a6, "Warden"),
];

pub fn accent_color(rgb: u32) -> u32 {
    rgba(rgb, 255)
}
