
use std::collections::BTreeMap;

use nalgebra::Vector4;

#[derive(Clone, Copy)] 
pub enum BackgroundMode {
    SolidColor = 0,
    RepeatPixels = 1,
    MirrorPixels = 2,
}
impl Default for BackgroundMode {
    fn default() -> Self { Self::SolidColor }
}
impl From<i32> for BackgroundMode {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::RepeatPixels,
            2 => Self::MirrorPixels,
            _ => Self::SolidColor
        }
    }
}

#[derive(Clone)]
pub struct StabilizationParams {
    pub size: (usize, usize), // Processing input size
    pub output_size: (usize, usize), // Processing output size
    pub video_size: (usize, usize), // Full resolution input size
    pub video_output_size: (usize, usize), // Full resoution output size

    pub background: Vector4<f32>,

    pub frame_readout_time: f64,
    pub adaptive_zoom_window: f64,
    pub fov: f64,
    pub fovs: Vec<f64>,
    pub min_fov: f64,
    pub fps: f64,
    pub fps_scale: Option<f64>,
    pub frame_count: usize,
    pub duration_ms: f64,

    pub trim_start: f64,
    pub trim_end: f64,

    pub video_rotation: f64,

    pub lens_correction_amount: f64,
    pub background_mode: BackgroundMode,

    pub framebuffer_inverted: bool,
    pub is_calibrator: bool,
    
    pub stab_enabled: bool,
    pub show_detected_features: bool,
    pub show_optical_flow: bool,

    pub zooming_debug_points: std::collections::BTreeMap<i64, Vec<(f64, f64)>>
}
impl Default for StabilizationParams {
    fn default() -> Self {
        Self {
            fov: 1.0,
            min_fov: 1.0,
            fovs: vec![],
            stab_enabled: true,
            show_detected_features: true,
            show_optical_flow: true,
            frame_readout_time: 0.0, 
            adaptive_zoom_window: 0.0, 

            size: (0, 0),
            output_size: (0, 0),
            video_size: (0, 0),
            video_output_size: (0, 0),

            video_rotation: 0.0,

            lens_correction_amount: 1.0,
            background_mode: BackgroundMode::SolidColor,
            
            framebuffer_inverted: false,
            is_calibrator: false,

            trim_start: 0.0,
            trim_end: 1.0,

            zooming_debug_points: BTreeMap::new(),
        
            background: Vector4::new(0.0, 0.0, 0.0, 0.0),
    
            fps: 0.0,
            fps_scale: None,
            frame_count: 0,
            duration_ms: 0.0,
        }
    }
}

impl StabilizationParams {
    pub fn get_scaled_duration_ms(&self) -> f64 {
        match self.fps_scale {
            Some(scale) => self.duration_ms / scale,
            None            => self.duration_ms
        }
    }
    pub fn get_scaled_fps(&self) -> f64 {
        match self.fps_scale {
            Some(scale) => self.fps * scale,
            None            => self.fps
        }
    }

    pub fn set_fovs(&mut self, fovs: Vec<f64>, mut lens_fov_adjustment: f64) {
        if let Some(mut min_fov) = fovs.iter().copied().reduce(f64::min) {
            min_fov *= self.video_size.0 as f64 / self.video_output_size.0.max(1) as f64;
            if lens_fov_adjustment <= 0.0001 { lens_fov_adjustment = 1.0 };
            self.min_fov = min_fov / lens_fov_adjustment;
        }
        if fovs.is_empty() {
            self.min_fov = 1.0;
        }
        self.fovs = fovs;
    }
}
