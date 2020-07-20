use std::os::raw::c_char;
use status::NvAPI_Status;
use handles;

nvapi! {
    pub type EnumNvidiaDisplayHandleFn = extern "C" fn(thisEnum: u32, pNvDispHandle: *mut handles::NvDisplayHandle) -> NvAPI_Status;

    /// This function returns the handle of the NVIDIA display specified by the enum
    /// index (thisEnum). The client should keep enumerating until it
    /// returns NVAPI_END_ENUMERATION.
    ///
    /// Note: Display handles can get invalidated on a modeset, so the calling applications need to
    /// renum the handles after every modeset.
    pub unsafe fn NvAPI_EnumNvidiaDisplayHandle;
}

nvapi! {
    pub type EnumNvidiaUnAttachedDisplayHandleFn = extern "C" fn(thisEnum: u32, pNvUnAttachedDispHandle: *mut handles::NvUnAttachedDisplayHandle) -> NvAPI_Status;

    /// This function returns the handle of the NVIDIA unattached display specified by the enum
    /// index (thisEnum). The client should keep enumerating until it
    /// returns error.
    ///
    /// Note: Display handles can get invalidated on a modeset, so the calling applications need to
    /// renum the handles after every modeset.
    pub unsafe fn NvAPI_EnumNvidiaUnAttachedDisplayHandle;
}

nvapi! {
    pub type GetAssociatedNvidiaDisplayHandleFn = extern "C" fn(szDisplayName: *const c_char, pNvDispHandle: *mut handles::NvDisplayHandle) -> NvAPI_Status;

    /// This function returns the handle of the NVIDIA display that is associated
    /// with the given display "name" (such as "\\.\DISPLAY1").
    pub unsafe fn NvAPI_GetAssociatedNvidiaDisplayHandle;
}


nvapi! {
    pub type GetAssociatedNvidiaDisplayNameFn = extern "C" fn(pNvDispHandle: *const handles::NvDisplayHandle, szDisplayName: *mut crate::types::NvAPI_ShortString) -> NvAPI_Status;

    /// For a given NVIDIA display handle, this function returns a string (such as "\\.\DISPLAY1") to identify the display.
    /// SUPPORTED OS: Windows 7 and higher
    pub unsafe fn NvAPI_GetAssociatedNvidiaDisplayName;
}

nvapi! {
    pub type GetDisplayIdByDisplayNameFn = extern "C" fn(displayName: *const crate::types::NvAPI_ShortString, displayId: *mut u32) -> NvAPI_Status;
    /// This API retrieves the Display Id of a given display by display name.
    /// The display must be active to retrieve the displayId.
    /// In the case of clone mode or Surround gaming, the primary or top-left display will be returned.
    /// SUPPORTED OS: Windows 7 and higher
    pub unsafe fn NvAPI_DISP_GetDisplayIdByDisplayName;
}

nvapi! {
    pub type DISP_GetAssociatedUnAttachedNvidiaDisplayHandleFn = extern "C" fn(szDisplayName: *const c_char, pNvUnAttachedDispHandle: *mut handles::NvDisplayHandle) -> NvAPI_Status;

    /// This function returns the handle of an unattached NVIDIA display that is
    /// associated with the given display name (such as "\\DISPLAY1").
    pub unsafe fn NvAPI_DISP_GetAssociatedUnAttachedNvidiaDisplayHandle;
}

nvstruct! {
    /// Used in NvAPI_GetDVCInfo().
    pub struct NV_DISPLAY_DVC_INFO {
        pub version: u32,
        pub currentLevel: i32,
        pub minLevel: i32,
        pub maxLevel: i32,
    }
}

nvapi! {
    pub type GetDVCInfoFn = extern "C" fn(pNvDispHandle: *const handles::NvDisplayHandle, nvDispId: u32, info: *mut NV_DISPLAY_DVC_INFO) -> NvAPI_Status;

    pub unsafe fn NvAPI_GetDVCInfo;
}

nvapi! {
    pub type GetDVCInfoExFn = extern "C" fn(pNvDispHandle: *const handles::NvDisplayHandle, nvDispId: u32, info: *mut NV_DISPLAY_DVC_INFO) -> NvAPI_Status;

    pub unsafe fn NvAPI_GetDVCInfoEx;
}

nvapi! {
    pub type SetDVCLevelFn = extern "C" fn(pNvDispHandle: *const handles::NvDisplayHandle, nvDispId: u32, level: i32) -> NvAPI_Status;

    pub unsafe fn NvAPI_SetDVCLevel;
}
