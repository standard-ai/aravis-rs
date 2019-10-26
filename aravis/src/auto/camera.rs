// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use AcquisitionMode;
use Auto;
use Buffer;
use Device;
use GvStreamOption;
use PixelFormat;
use Status;
use aravis_sys;
use glib;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Camera(Object<aravis_sys::ArvCamera, aravis_sys::ArvCameraClass, CameraClass>);

    match fn {
        get_type => || aravis_sys::arv_camera_get_type(),
    }
}

impl Camera {
    pub fn new(name: Option<&str>) -> Camera {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(aravis_sys::arv_camera_new(name.to_glib_none().0))
        }
    }
}

pub const NONE_CAMERA: Option<&Camera> = None;

pub trait CameraExt: 'static {
    fn abort_acquisition(&self);

    fn acquisition(&self, timeout: u64) -> Option<Buffer>;

    fn clear_triggers(&self);

    //fn create_chunk_parser(&self) -> /*Ignored*/Option<ChunkParser>;

    //fn create_stream(&self, callback: /*Unimplemented*/FnMut(/*Ignored*/StreamCallbackType, &Buffer), user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> Option<Stream>;

    fn execute_command(&self, feature: &str);

    fn get_acquisition_mode(&self) -> AcquisitionMode;

    fn get_available_enumerations(&self, feature: &str) -> Vec<i64>;

    fn get_available_enumerations_as_display_names(&self, feature: &str) -> Vec<GString>;

    fn get_available_enumerations_as_strings(&self, feature: &str) -> Vec<GString>;

    fn get_available_pixel_formats(&self) -> Vec<i64>;

    fn get_available_pixel_formats_as_display_names(&self) -> Vec<GString>;

    fn get_available_pixel_formats_as_strings(&self) -> Vec<GString>;

    fn get_available_trigger_sources(&self) -> Vec<GString>;

    fn get_available_triggers(&self) -> Vec<GString>;

    fn get_binning(&self) -> (i32, i32);

    fn get_boolean(&self, feature: &str) -> bool;

    fn get_chunk_mode(&self) -> bool;

    fn get_chunk_state(&self, chunk: &str) -> bool;

    fn get_device(&self) -> Option<Device>;

    fn get_device_id(&self) -> Option<GString>;

    fn get_exposure_time(&self) -> f64;

    fn get_exposure_time_auto(&self) -> Auto;

    fn get_exposure_time_bounds(&self) -> (f64, f64);

    fn get_float(&self, feature: &str) -> f64;

    fn get_float_bounds(&self, feature: &str) -> (f64, f64);

    fn get_frame_count(&self) -> i64;

    fn get_frame_count_bounds(&self) -> (i64, i64);

    fn get_frame_rate(&self) -> f64;

    fn get_frame_rate_bounds(&self) -> (f64, f64);

    fn get_gain(&self) -> f64;

    fn get_gain_auto(&self) -> Auto;

    fn get_gain_bounds(&self) -> (f64, f64);

    fn get_height_bounds(&self) -> (i32, i32);

    fn get_height_increment(&self) -> i32;

    fn get_integer(&self, feature: &str) -> i64;

    fn get_integer_bounds(&self, feature: &str) -> (i64, i64);

    fn get_integer_increment(&self, feature: &str) -> i64;

    fn get_model_name(&self) -> Option<GString>;

    fn get_payload(&self) -> u32;

    fn get_pixel_format(&self) -> PixelFormat;

    fn get_pixel_format_as_string(&self) -> Option<GString>;

    fn get_region(&self) -> (i32, i32, i32, i32);

    fn get_sensor_size(&self) -> (i32, i32);

    fn get_status(&self) -> Result<Status, glib::Error>;

    fn get_string(&self, feature: &str) -> Option<GString>;

    fn get_trigger_source(&self) -> Option<GString>;

    fn get_vendor_name(&self) -> Option<GString>;

    fn get_width_bounds(&self) -> (i32, i32);

    fn get_width_increment(&self) -> i32;

    fn get_x_binning_bounds(&self) -> (i32, i32);

    fn get_x_binning_increment(&self) -> i32;

    fn get_x_offset_bounds(&self) -> (i32, i32);

    fn get_x_offset_increment(&self) -> i32;

    fn get_y_binning_bounds(&self) -> (i32, i32);

    fn get_y_binning_increment(&self) -> i32;

    fn get_y_offset_bounds(&self) -> (i32, i32);

    fn get_y_offset_increment(&self) -> i32;

    fn gv_auto_packet_size(&self) -> u32;

    fn gv_get_current_stream_channel(&self) -> i32;

    fn gv_get_n_stream_channels(&self) -> i32;

    fn gv_get_packet_delay(&self) -> i64;

    fn gv_get_packet_size(&self) -> u32;

    fn gv_select_stream_channel(&self, channel_id: i32);

    fn gv_set_packet_delay(&self, delay_ns: i64);

    fn gv_set_packet_size(&self, packet_size: i32);

    fn gv_set_stream_options(&self, options: GvStreamOption);

    fn is_binning_available(&self) -> bool;

    fn is_exposure_auto_available(&self) -> bool;

    fn is_exposure_time_available(&self) -> bool;

    fn is_feature_available(&self, feature: &str) -> bool;

    fn is_frame_rate_available(&self) -> bool;

    fn is_gain_auto_available(&self) -> bool;

    fn is_gain_available(&self) -> bool;

    fn is_gv_device(&self) -> bool;

    fn is_uv_device(&self) -> bool;

    fn set_acquisition_mode(&self, value: AcquisitionMode);

    fn set_binning(&self, dx: i32, dy: i32);

    fn set_boolean(&self, feature: &str, value: bool);

    fn set_chunk_mode(&self, is_active: bool);

    fn set_chunk_state(&self, chunk: &str, is_enabled: bool);

    fn set_chunks(&self, chunk_list: &str);

    fn set_exposure_time(&self, exposure_time_us: f64);

    fn set_exposure_time_auto(&self, auto_mode: Auto);

    fn set_float(&self, feature: &str, value: f64);

    fn set_frame_count(&self, frame_count: i64);

    fn set_frame_rate(&self, frame_rate: f64);

    fn set_gain(&self, gain: f64);

    fn set_gain_auto(&self, auto_mode: Auto);

    fn set_integer(&self, feature: &str, value: i64);

    fn set_pixel_format(&self, format: PixelFormat);

    fn set_pixel_format_from_string(&self, format: &str);

    fn set_region(&self, x: i32, y: i32, width: i32, height: i32);

    fn set_string(&self, feature: &str, value: &str);

    fn set_trigger(&self, source: &str);

    fn set_trigger_source(&self, source: &str);

    fn software_trigger(&self);

    fn start_acquisition(&self);

    fn stop_acquisition(&self);

    fn uv_get_bandwidth(&self) -> u32;

    fn uv_get_bandwidth_bounds(&self) -> (u32, u32);

    fn uv_is_bandwidth_control_available(&self) -> bool;

    fn uv_set_bandwidth(&self, bandwidth: u32);
}

impl<O: IsA<Camera>> CameraExt for O {
    fn abort_acquisition(&self) {
        unsafe {
            aravis_sys::arv_camera_abort_acquisition(self.as_ref().to_glib_none().0);
        }
    }

    fn acquisition(&self, timeout: u64) -> Option<Buffer> {
        unsafe {
            from_glib_full(aravis_sys::arv_camera_acquisition(self.as_ref().to_glib_none().0, timeout))
        }
    }

    fn clear_triggers(&self) {
        unsafe {
            aravis_sys::arv_camera_clear_triggers(self.as_ref().to_glib_none().0);
        }
    }

    //fn create_chunk_parser(&self) -> /*Ignored*/Option<ChunkParser> {
    //    unsafe { TODO: call aravis_sys:arv_camera_create_chunk_parser() }
    //}

    //fn create_stream(&self, callback: /*Unimplemented*/FnMut(/*Ignored*/StreamCallbackType, &Buffer), user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> Option<Stream> {
    //    unsafe { TODO: call aravis_sys:arv_camera_create_stream() }
    //}

    fn execute_command(&self, feature: &str) {
        unsafe {
            aravis_sys::arv_camera_execute_command(self.as_ref().to_glib_none().0, feature.to_glib_none().0);
        }
    }

    fn get_acquisition_mode(&self) -> AcquisitionMode {
        unsafe {
            from_glib(aravis_sys::arv_camera_get_acquisition_mode(self.as_ref().to_glib_none().0))
        }
    }

    fn get_available_enumerations(&self, feature: &str) -> Vec<i64> {
        unsafe {
            let mut n_values = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_container_num(aravis_sys::arv_camera_get_available_enumerations(self.as_ref().to_glib_none().0, feature.to_glib_none().0, n_values.as_mut_ptr()), n_values.assume_init() as usize);
            ret
        }
    }

    fn get_available_enumerations_as_display_names(&self, feature: &str) -> Vec<GString> {
        unsafe {
            let mut n_values = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_container_num(aravis_sys::arv_camera_get_available_enumerations_as_display_names(self.as_ref().to_glib_none().0, feature.to_glib_none().0, n_values.as_mut_ptr()), n_values.assume_init() as usize);
            ret
        }
    }

    fn get_available_enumerations_as_strings(&self, feature: &str) -> Vec<GString> {
        unsafe {
            let mut n_values = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_container_num(aravis_sys::arv_camera_get_available_enumerations_as_strings(self.as_ref().to_glib_none().0, feature.to_glib_none().0, n_values.as_mut_ptr()), n_values.assume_init() as usize);
            ret
        }
    }

    fn get_available_pixel_formats(&self) -> Vec<i64> {
        unsafe {
            let mut n_pixel_formats = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_container_num(aravis_sys::arv_camera_get_available_pixel_formats(self.as_ref().to_glib_none().0, n_pixel_formats.as_mut_ptr()), n_pixel_formats.assume_init() as usize);
            ret
        }
    }

    fn get_available_pixel_formats_as_display_names(&self) -> Vec<GString> {
        unsafe {
            let mut n_pixel_formats = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_container_num(aravis_sys::arv_camera_get_available_pixel_formats_as_display_names(self.as_ref().to_glib_none().0, n_pixel_formats.as_mut_ptr()), n_pixel_formats.assume_init() as usize);
            ret
        }
    }

    fn get_available_pixel_formats_as_strings(&self) -> Vec<GString> {
        unsafe {
            let mut n_pixel_formats = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_container_num(aravis_sys::arv_camera_get_available_pixel_formats_as_strings(self.as_ref().to_glib_none().0, n_pixel_formats.as_mut_ptr()), n_pixel_formats.assume_init() as usize);
            ret
        }
    }

    fn get_available_trigger_sources(&self) -> Vec<GString> {
        unsafe {
            let mut n_sources = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_container_num(aravis_sys::arv_camera_get_available_trigger_sources(self.as_ref().to_glib_none().0, n_sources.as_mut_ptr()), n_sources.assume_init() as usize);
            ret
        }
    }

    fn get_available_triggers(&self) -> Vec<GString> {
        unsafe {
            let mut n_triggers = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_container_num(aravis_sys::arv_camera_get_available_triggers(self.as_ref().to_glib_none().0, n_triggers.as_mut_ptr()), n_triggers.assume_init() as usize);
            ret
        }
    }

    fn get_binning(&self) -> (i32, i32) {
        unsafe {
            let mut dx = mem::MaybeUninit::uninit();
            let mut dy = mem::MaybeUninit::uninit();
            aravis_sys::arv_camera_get_binning(self.as_ref().to_glib_none().0, dx.as_mut_ptr(), dy.as_mut_ptr());
            let dx = dx.assume_init();
            let dy = dy.assume_init();
            (dx, dy)
        }
    }

    fn get_boolean(&self, feature: &str) -> bool {
        unsafe {
            from_glib(aravis_sys::arv_camera_get_boolean(self.as_ref().to_glib_none().0, feature.to_glib_none().0))
        }
    }

    fn get_chunk_mode(&self) -> bool {
        unsafe {
            from_glib(aravis_sys::arv_camera_get_chunk_mode(self.as_ref().to_glib_none().0))
        }
    }

    fn get_chunk_state(&self, chunk: &str) -> bool {
        unsafe {
            from_glib(aravis_sys::arv_camera_get_chunk_state(self.as_ref().to_glib_none().0, chunk.to_glib_none().0))
        }
    }

    fn get_device(&self) -> Option<Device> {
        unsafe {
            from_glib_none(aravis_sys::arv_camera_get_device(self.as_ref().to_glib_none().0))
        }
    }

    fn get_device_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(aravis_sys::arv_camera_get_device_id(self.as_ref().to_glib_none().0))
        }
    }

    fn get_exposure_time(&self) -> f64 {
        unsafe {
            aravis_sys::arv_camera_get_exposure_time(self.as_ref().to_glib_none().0)
        }
    }

    fn get_exposure_time_auto(&self) -> Auto {
        unsafe {
            from_glib(aravis_sys::arv_camera_get_exposure_time_auto(self.as_ref().to_glib_none().0))
        }
    }

    fn get_exposure_time_bounds(&self) -> (f64, f64) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            aravis_sys::arv_camera_get_exposure_time_bounds(self.as_ref().to_glib_none().0, min.as_mut_ptr(), max.as_mut_ptr());
            let min = min.assume_init();
            let max = max.assume_init();
            (min, max)
        }
    }

    fn get_float(&self, feature: &str) -> f64 {
        unsafe {
            aravis_sys::arv_camera_get_float(self.as_ref().to_glib_none().0, feature.to_glib_none().0)
        }
    }

    fn get_float_bounds(&self, feature: &str) -> (f64, f64) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            aravis_sys::arv_camera_get_float_bounds(self.as_ref().to_glib_none().0, feature.to_glib_none().0, min.as_mut_ptr(), max.as_mut_ptr());
            let min = min.assume_init();
            let max = max.assume_init();
            (min, max)
        }
    }

    fn get_frame_count(&self) -> i64 {
        unsafe {
            aravis_sys::arv_camera_get_frame_count(self.as_ref().to_glib_none().0)
        }
    }

    fn get_frame_count_bounds(&self) -> (i64, i64) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            aravis_sys::arv_camera_get_frame_count_bounds(self.as_ref().to_glib_none().0, min.as_mut_ptr(), max.as_mut_ptr());
            let min = min.assume_init();
            let max = max.assume_init();
            (min, max)
        }
    }

    fn get_frame_rate(&self) -> f64 {
        unsafe {
            aravis_sys::arv_camera_get_frame_rate(self.as_ref().to_glib_none().0)
        }
    }

    fn get_frame_rate_bounds(&self) -> (f64, f64) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            aravis_sys::arv_camera_get_frame_rate_bounds(self.as_ref().to_glib_none().0, min.as_mut_ptr(), max.as_mut_ptr());
            let min = min.assume_init();
            let max = max.assume_init();
            (min, max)
        }
    }

    fn get_gain(&self) -> f64 {
        unsafe {
            aravis_sys::arv_camera_get_gain(self.as_ref().to_glib_none().0)
        }
    }

    fn get_gain_auto(&self) -> Auto {
        unsafe {
            from_glib(aravis_sys::arv_camera_get_gain_auto(self.as_ref().to_glib_none().0))
        }
    }

    fn get_gain_bounds(&self) -> (f64, f64) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            aravis_sys::arv_camera_get_gain_bounds(self.as_ref().to_glib_none().0, min.as_mut_ptr(), max.as_mut_ptr());
            let min = min.assume_init();
            let max = max.assume_init();
            (min, max)
        }
    }

    fn get_height_bounds(&self) -> (i32, i32) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            aravis_sys::arv_camera_get_height_bounds(self.as_ref().to_glib_none().0, min.as_mut_ptr(), max.as_mut_ptr());
            let min = min.assume_init();
            let max = max.assume_init();
            (min, max)
        }
    }

    fn get_height_increment(&self) -> i32 {
        unsafe {
            aravis_sys::arv_camera_get_height_increment(self.as_ref().to_glib_none().0)
        }
    }

    fn get_integer(&self, feature: &str) -> i64 {
        unsafe {
            aravis_sys::arv_camera_get_integer(self.as_ref().to_glib_none().0, feature.to_glib_none().0)
        }
    }

    fn get_integer_bounds(&self, feature: &str) -> (i64, i64) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            aravis_sys::arv_camera_get_integer_bounds(self.as_ref().to_glib_none().0, feature.to_glib_none().0, min.as_mut_ptr(), max.as_mut_ptr());
            let min = min.assume_init();
            let max = max.assume_init();
            (min, max)
        }
    }

    fn get_integer_increment(&self, feature: &str) -> i64 {
        unsafe {
            aravis_sys::arv_camera_get_integer_increment(self.as_ref().to_glib_none().0, feature.to_glib_none().0)
        }
    }

    fn get_model_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(aravis_sys::arv_camera_get_model_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_payload(&self) -> u32 {
        unsafe {
            aravis_sys::arv_camera_get_payload(self.as_ref().to_glib_none().0)
        }
    }

    fn get_pixel_format(&self) -> PixelFormat {
        unsafe {
            aravis_sys::arv_camera_get_pixel_format(self.as_ref().to_glib_none().0)
        }
    }

    fn get_pixel_format_as_string(&self) -> Option<GString> {
        unsafe {
            from_glib_none(aravis_sys::arv_camera_get_pixel_format_as_string(self.as_ref().to_glib_none().0))
        }
    }

    fn get_region(&self) -> (i32, i32, i32, i32) {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            aravis_sys::arv_camera_get_region(self.as_ref().to_glib_none().0, x.as_mut_ptr(), y.as_mut_ptr(), width.as_mut_ptr(), height.as_mut_ptr());
            let x = x.assume_init();
            let y = y.assume_init();
            let width = width.assume_init();
            let height = height.assume_init();
            (x, y, width, height)
        }
    }

    fn get_sensor_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            aravis_sys::arv_camera_get_sensor_size(self.as_ref().to_glib_none().0, width.as_mut_ptr(), height.as_mut_ptr());
            let width = width.assume_init();
            let height = height.assume_init();
            (width, height)
        }
    }

    fn get_status(&self) -> Result<Status, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = aravis_sys::arv_camera_get_status(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_string(&self, feature: &str) -> Option<GString> {
        unsafe {
            from_glib_none(aravis_sys::arv_camera_get_string(self.as_ref().to_glib_none().0, feature.to_glib_none().0))
        }
    }

    fn get_trigger_source(&self) -> Option<GString> {
        unsafe {
            from_glib_none(aravis_sys::arv_camera_get_trigger_source(self.as_ref().to_glib_none().0))
        }
    }

    fn get_vendor_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(aravis_sys::arv_camera_get_vendor_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_width_bounds(&self) -> (i32, i32) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            aravis_sys::arv_camera_get_width_bounds(self.as_ref().to_glib_none().0, min.as_mut_ptr(), max.as_mut_ptr());
            let min = min.assume_init();
            let max = max.assume_init();
            (min, max)
        }
    }

    fn get_width_increment(&self) -> i32 {
        unsafe {
            aravis_sys::arv_camera_get_width_increment(self.as_ref().to_glib_none().0)
        }
    }

    fn get_x_binning_bounds(&self) -> (i32, i32) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            aravis_sys::arv_camera_get_x_binning_bounds(self.as_ref().to_glib_none().0, min.as_mut_ptr(), max.as_mut_ptr());
            let min = min.assume_init();
            let max = max.assume_init();
            (min, max)
        }
    }

    fn get_x_binning_increment(&self) -> i32 {
        unsafe {
            aravis_sys::arv_camera_get_x_binning_increment(self.as_ref().to_glib_none().0)
        }
    }

    fn get_x_offset_bounds(&self) -> (i32, i32) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            aravis_sys::arv_camera_get_x_offset_bounds(self.as_ref().to_glib_none().0, min.as_mut_ptr(), max.as_mut_ptr());
            let min = min.assume_init();
            let max = max.assume_init();
            (min, max)
        }
    }

    fn get_x_offset_increment(&self) -> i32 {
        unsafe {
            aravis_sys::arv_camera_get_x_offset_increment(self.as_ref().to_glib_none().0)
        }
    }

    fn get_y_binning_bounds(&self) -> (i32, i32) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            aravis_sys::arv_camera_get_y_binning_bounds(self.as_ref().to_glib_none().0, min.as_mut_ptr(), max.as_mut_ptr());
            let min = min.assume_init();
            let max = max.assume_init();
            (min, max)
        }
    }

    fn get_y_binning_increment(&self) -> i32 {
        unsafe {
            aravis_sys::arv_camera_get_y_binning_increment(self.as_ref().to_glib_none().0)
        }
    }

    fn get_y_offset_bounds(&self) -> (i32, i32) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            aravis_sys::arv_camera_get_y_offset_bounds(self.as_ref().to_glib_none().0, min.as_mut_ptr(), max.as_mut_ptr());
            let min = min.assume_init();
            let max = max.assume_init();
            (min, max)
        }
    }

    fn get_y_offset_increment(&self) -> i32 {
        unsafe {
            aravis_sys::arv_camera_get_y_offset_increment(self.as_ref().to_glib_none().0)
        }
    }

    fn gv_auto_packet_size(&self) -> u32 {
        unsafe {
            aravis_sys::arv_camera_gv_auto_packet_size(self.as_ref().to_glib_none().0)
        }
    }

    fn gv_get_current_stream_channel(&self) -> i32 {
        unsafe {
            aravis_sys::arv_camera_gv_get_current_stream_channel(self.as_ref().to_glib_none().0)
        }
    }

    fn gv_get_n_stream_channels(&self) -> i32 {
        unsafe {
            aravis_sys::arv_camera_gv_get_n_stream_channels(self.as_ref().to_glib_none().0)
        }
    }

    fn gv_get_packet_delay(&self) -> i64 {
        unsafe {
            aravis_sys::arv_camera_gv_get_packet_delay(self.as_ref().to_glib_none().0)
        }
    }

    fn gv_get_packet_size(&self) -> u32 {
        unsafe {
            aravis_sys::arv_camera_gv_get_packet_size(self.as_ref().to_glib_none().0)
        }
    }

    fn gv_select_stream_channel(&self, channel_id: i32) {
        unsafe {
            aravis_sys::arv_camera_gv_select_stream_channel(self.as_ref().to_glib_none().0, channel_id);
        }
    }

    fn gv_set_packet_delay(&self, delay_ns: i64) {
        unsafe {
            aravis_sys::arv_camera_gv_set_packet_delay(self.as_ref().to_glib_none().0, delay_ns);
        }
    }

    fn gv_set_packet_size(&self, packet_size: i32) {
        unsafe {
            aravis_sys::arv_camera_gv_set_packet_size(self.as_ref().to_glib_none().0, packet_size);
        }
    }

    fn gv_set_stream_options(&self, options: GvStreamOption) {
        unsafe {
            aravis_sys::arv_camera_gv_set_stream_options(self.as_ref().to_glib_none().0, options.to_glib());
        }
    }

    fn is_binning_available(&self) -> bool {
        unsafe {
            from_glib(aravis_sys::arv_camera_is_binning_available(self.as_ref().to_glib_none().0))
        }
    }

    fn is_exposure_auto_available(&self) -> bool {
        unsafe {
            from_glib(aravis_sys::arv_camera_is_exposure_auto_available(self.as_ref().to_glib_none().0))
        }
    }

    fn is_exposure_time_available(&self) -> bool {
        unsafe {
            from_glib(aravis_sys::arv_camera_is_exposure_time_available(self.as_ref().to_glib_none().0))
        }
    }

    fn is_feature_available(&self, feature: &str) -> bool {
        unsafe {
            from_glib(aravis_sys::arv_camera_is_feature_available(self.as_ref().to_glib_none().0, feature.to_glib_none().0))
        }
    }

    fn is_frame_rate_available(&self) -> bool {
        unsafe {
            from_glib(aravis_sys::arv_camera_is_frame_rate_available(self.as_ref().to_glib_none().0))
        }
    }

    fn is_gain_auto_available(&self) -> bool {
        unsafe {
            from_glib(aravis_sys::arv_camera_is_gain_auto_available(self.as_ref().to_glib_none().0))
        }
    }

    fn is_gain_available(&self) -> bool {
        unsafe {
            from_glib(aravis_sys::arv_camera_is_gain_available(self.as_ref().to_glib_none().0))
        }
    }

    fn is_gv_device(&self) -> bool {
        unsafe {
            from_glib(aravis_sys::arv_camera_is_gv_device(self.as_ref().to_glib_none().0))
        }
    }

    fn is_uv_device(&self) -> bool {
        unsafe {
            from_glib(aravis_sys::arv_camera_is_uv_device(self.as_ref().to_glib_none().0))
        }
    }

    fn set_acquisition_mode(&self, value: AcquisitionMode) {
        unsafe {
            aravis_sys::arv_camera_set_acquisition_mode(self.as_ref().to_glib_none().0, value.to_glib());
        }
    }

    fn set_binning(&self, dx: i32, dy: i32) {
        unsafe {
            aravis_sys::arv_camera_set_binning(self.as_ref().to_glib_none().0, dx, dy);
        }
    }

    fn set_boolean(&self, feature: &str, value: bool) {
        unsafe {
            aravis_sys::arv_camera_set_boolean(self.as_ref().to_glib_none().0, feature.to_glib_none().0, value.to_glib());
        }
    }

    fn set_chunk_mode(&self, is_active: bool) {
        unsafe {
            aravis_sys::arv_camera_set_chunk_mode(self.as_ref().to_glib_none().0, is_active.to_glib());
        }
    }

    fn set_chunk_state(&self, chunk: &str, is_enabled: bool) {
        unsafe {
            aravis_sys::arv_camera_set_chunk_state(self.as_ref().to_glib_none().0, chunk.to_glib_none().0, is_enabled.to_glib());
        }
    }

    fn set_chunks(&self, chunk_list: &str) {
        unsafe {
            aravis_sys::arv_camera_set_chunks(self.as_ref().to_glib_none().0, chunk_list.to_glib_none().0);
        }
    }

    fn set_exposure_time(&self, exposure_time_us: f64) {
        unsafe {
            aravis_sys::arv_camera_set_exposure_time(self.as_ref().to_glib_none().0, exposure_time_us);
        }
    }

    fn set_exposure_time_auto(&self, auto_mode: Auto) {
        unsafe {
            aravis_sys::arv_camera_set_exposure_time_auto(self.as_ref().to_glib_none().0, auto_mode.to_glib());
        }
    }

    fn set_float(&self, feature: &str, value: f64) {
        unsafe {
            aravis_sys::arv_camera_set_float(self.as_ref().to_glib_none().0, feature.to_glib_none().0, value);
        }
    }

    fn set_frame_count(&self, frame_count: i64) {
        unsafe {
            aravis_sys::arv_camera_set_frame_count(self.as_ref().to_glib_none().0, frame_count);
        }
    }

    fn set_frame_rate(&self, frame_rate: f64) {
        unsafe {
            aravis_sys::arv_camera_set_frame_rate(self.as_ref().to_glib_none().0, frame_rate);
        }
    }

    fn set_gain(&self, gain: f64) {
        unsafe {
            aravis_sys::arv_camera_set_gain(self.as_ref().to_glib_none().0, gain);
        }
    }

    fn set_gain_auto(&self, auto_mode: Auto) {
        unsafe {
            aravis_sys::arv_camera_set_gain_auto(self.as_ref().to_glib_none().0, auto_mode.to_glib());
        }
    }

    fn set_integer(&self, feature: &str, value: i64) {
        unsafe {
            aravis_sys::arv_camera_set_integer(self.as_ref().to_glib_none().0, feature.to_glib_none().0, value);
        }
    }

    fn set_pixel_format(&self, format: PixelFormat) {
        unsafe {
            aravis_sys::arv_camera_set_pixel_format(self.as_ref().to_glib_none().0, format);
        }
    }

    fn set_pixel_format_from_string(&self, format: &str) {
        unsafe {
            aravis_sys::arv_camera_set_pixel_format_from_string(self.as_ref().to_glib_none().0, format.to_glib_none().0);
        }
    }

    fn set_region(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            aravis_sys::arv_camera_set_region(self.as_ref().to_glib_none().0, x, y, width, height);
        }
    }

    fn set_string(&self, feature: &str, value: &str) {
        unsafe {
            aravis_sys::arv_camera_set_string(self.as_ref().to_glib_none().0, feature.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_trigger(&self, source: &str) {
        unsafe {
            aravis_sys::arv_camera_set_trigger(self.as_ref().to_glib_none().0, source.to_glib_none().0);
        }
    }

    fn set_trigger_source(&self, source: &str) {
        unsafe {
            aravis_sys::arv_camera_set_trigger_source(self.as_ref().to_glib_none().0, source.to_glib_none().0);
        }
    }

    fn software_trigger(&self) {
        unsafe {
            aravis_sys::arv_camera_software_trigger(self.as_ref().to_glib_none().0);
        }
    }

    fn start_acquisition(&self) {
        unsafe {
            aravis_sys::arv_camera_start_acquisition(self.as_ref().to_glib_none().0);
        }
    }

    fn stop_acquisition(&self) {
        unsafe {
            aravis_sys::arv_camera_stop_acquisition(self.as_ref().to_glib_none().0);
        }
    }

    fn uv_get_bandwidth(&self) -> u32 {
        unsafe {
            aravis_sys::arv_camera_uv_get_bandwidth(self.as_ref().to_glib_none().0)
        }
    }

    fn uv_get_bandwidth_bounds(&self) -> (u32, u32) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            aravis_sys::arv_camera_uv_get_bandwidth_bounds(self.as_ref().to_glib_none().0, min.as_mut_ptr(), max.as_mut_ptr());
            let min = min.assume_init();
            let max = max.assume_init();
            (min, max)
        }
    }

    fn uv_is_bandwidth_control_available(&self) -> bool {
        unsafe {
            from_glib(aravis_sys::arv_camera_uv_is_bandwidth_control_available(self.as_ref().to_glib_none().0))
        }
    }

    fn uv_set_bandwidth(&self, bandwidth: u32) {
        unsafe {
            aravis_sys::arv_camera_uv_set_bandwidth(self.as_ref().to_glib_none().0, bandwidth);
        }
    }
}

impl fmt::Display for Camera {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Camera")
    }
}