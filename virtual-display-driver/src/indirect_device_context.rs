use wdf_umdf::WDF_DECLARE_CONTEXT_TYPE;
use wdf_umdf_sys::{
    IDARG_IN_ADAPTER_INIT_FINISHED, IDARG_IN_COMMITMODES, IDARG_IN_GETDEFAULTDESCRIPTIONMODES,
    IDARG_IN_PARSEMONITORDESCRIPTION, IDARG_IN_QUERYTARGETMODES, IDARG_IN_SETSWAPCHAIN,
    IDARG_OUT_GETDEFAULTDESCRIPTIONMODES, IDARG_OUT_PARSEMONITORDESCRIPTION,
    IDARG_OUT_QUERYTARGETMODES, IDDCX_ADAPTER__, IDDCX_MONITOR__, LIST_ENTRY, NTSTATUS, WDFDEVICE,
    WDFMEMORY, WDF_POWER_DEVICE_STATE,
};
use windows::Win32::Foundation::STATUS_NOT_IMPLEMENTED;

// Taken from
// https://github.com/ge9/IddSampleDriver/blob/fe98ccff703b5c1e578a0d627aeac2fa77ac58e2/IddSampleDriver/Driver.cpp#L403
static MONITOR_EDID: &[u8] = &[
    0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x31, 0xD8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x05, 0x16, 0x01, 0x03, 0x6D, 0x32, 0x1C, 0x78, 0xEA, 0x5E, 0xC0, 0xA4, 0x59, 0x4A, 0x98, 0x25,
    0x20, 0x50, 0x54, 0x00, 0x00, 0x00, 0xD1, 0xC0, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x02, 0x3A, 0x80, 0x18, 0x71, 0x38, 0x2D, 0x40, 0x58, 0x2C,
    0x45, 0x00, 0xF4, 0x19, 0x11, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x4C, 0x69, 0x6E,
    0x75, 0x78, 0x20, 0x23, 0x30, 0x0A, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00, 0xFD, 0x00, 0x3B,
    0x3D, 0x42, 0x44, 0x0F, 0x00, 0x0A, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00, 0xFC,
    0x00, 0x4C, 0x69, 0x6E, 0x75, 0x78, 0x20, 0x46, 0x48, 0x44, 0x0A, 0x20, 0x20, 0x20, 0x00, 0x05,
];

pub struct IndirectDeviceContext {
    device: WDFDEVICE,
}

impl IndirectDeviceContext {
    pub fn new(device: WDFDEVICE) -> Self {
        Self { device }
    }
}

WDF_DECLARE_CONTEXT_TYPE!(pub IndirectDeviceContext);

pub extern "C" fn adapter_init_finished(
    adapter_object: *mut IDDCX_ADAPTER__,
    p_in_args: *const IDARG_IN_ADAPTER_INIT_FINISHED,
) -> NTSTATUS {
    todo!()
}

pub extern "C" fn device_d0_entry(
    device: WDFDEVICE,
    previous_state: WDF_POWER_DEVICE_STATE,
) -> NTSTATUS {
    todo!()
}

pub extern "C" fn parse_monitor_description(
    p_in_args: *const IDARG_IN_PARSEMONITORDESCRIPTION,
    p_out_args: *mut IDARG_OUT_PARSEMONITORDESCRIPTION,
) -> NTSTATUS {
    todo!()
}

pub extern "C" fn monitor_get_default_modes(
    _monitor_object: *mut IDDCX_MONITOR__,
    _p_in_args: *const IDARG_IN_GETDEFAULTDESCRIPTIONMODES,
    _p_out_args: *mut IDARG_OUT_GETDEFAULTDESCRIPTIONMODES,
) -> NTSTATUS {
    STATUS_NOT_IMPLEMENTED.0.into()
}

pub extern "C" fn monitor_query_modes(
    monitor_object: *mut IDDCX_MONITOR__,
    p_in_args: *const IDARG_IN_QUERYTARGETMODES,
    p_out_args: *mut IDARG_OUT_QUERYTARGETMODES,
) -> NTSTATUS {
    todo!()
}

pub extern "C" fn adapter_commit_modes(
    adapter_object: *mut IDDCX_ADAPTER__,
    p_in_args: *const IDARG_IN_COMMITMODES,
) -> NTSTATUS {
    todo!()
}

pub extern "C" fn assign_swap_chain(
    monitor_object: *mut IDDCX_MONITOR__,
    p_in_args: *const IDARG_IN_SETSWAPCHAIN,
) -> NTSTATUS {
    todo!()
}

pub extern "C" fn unassign_swap_chain(monitor_object: *mut IDDCX_MONITOR__) -> NTSTATUS {
    todo!()
}
