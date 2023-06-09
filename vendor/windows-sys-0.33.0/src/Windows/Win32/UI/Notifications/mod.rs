#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub type INotificationActivationCallback = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Notifications'*"]
pub struct NOTIFICATION_USER_INPUT_DATA {
    pub Key: ::windows_sys::core::PCWSTR,
    pub Value: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for NOTIFICATION_USER_INPUT_DATA {}
impl ::core::clone::Clone for NOTIFICATION_USER_INPUT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
