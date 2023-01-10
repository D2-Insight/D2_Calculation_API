#[derive(Debug, Clone, Copy)]
pub struct ReloadData {
    a: f64,
    b: f64,
    c: f64,
}
impl ReloadData {
    fn calc_reload_time(&self, _reload_stat: i32, _duration_scalar: f64) -> f64 {
        let reload_stat = _reload_stat as f64;
        (self.a * (reload_stat.powi(2))) + (self.b * reload_stat) + self.c
    }
}

#[derive(Debug, Clone, Copy)]
struct RangeResponse {
    hip_falloff_start: f64,
    hip_falloff_end: f64,
    falloff_start: f64,
    falloff_end: f64,
}
#[derive(Debug, Clone, Copy)]
pub struct RangeData {
    zoom: f64,
    zoom_tier: f64,
    vpp: f64,
    base_min: f64,
    base_max: f64,
    scale_max: bool,
}
impl RangeData {
    fn calc_range_falloff(
        &self,
        _range_stat: i32,
        _zoom_stat: i32,
        _hipfire_mult: f64,
        _global_mult: f64,
    ) -> RangeResponse {
        let range_stat = _range_stat as f64;
        let zoom_stat = _zoom_stat as f64;

        let new_zoom = (zoom_stat - self.zoom_tier) / 10.0 + self.zoom;

        let hip_falloff_start =
            (range_stat * self.vpp + self.base_min) * _hipfire_mult * _global_mult;
        let hip_falloff_end =
            (range_stat * self.vpp + self.base_max) * _hipfire_mult * _global_mult;

        let falloff_start = (range_stat * self.vpp + self.base_min) * new_zoom * _global_mult;
        let falloff_end = (range_stat * self.vpp + self.base_max) * new_zoom * _global_mult;

        RangeResponse {
            hip_falloff_start,
            hip_falloff_end,
            falloff_start,
            falloff_end,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct HandlingResponse {
    ready_time: f64,
    stow_time: f64,
}
#[derive(Debug, Clone, Copy)]
pub struct HandlingData {
    ready_vpp: f64,
    ready_base: f64,
    stow_vpp: f64,
    stow_base: f64,
}
impl HandlingData {
    fn calc_handling_times(&self, _handling_stat: i32, _duraction_scalar: f64) -> HandlingResponse {
        let handling_stat = _handling_stat as f64;

        let ready_time = (handling_stat * self.ready_vpp + self.ready_base) * _duraction_scalar;
        let stow_time = (handling_stat * self.stow_vpp + self.stow_base) * _duraction_scalar;

        HandlingResponse {
            ready_time,
            stow_time,
        }
    }
}
