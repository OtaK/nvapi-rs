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

nvenum! {
    pub enum NV_FORMAT / DisplayFormat {
        NV_FORMAT_UNKNOWN / Default = 0,
        NV_FORMAT_P8 / P8 = 1,
        NV_FORMAT_R5G6B5 / R5G6B5 = 2,
        NV_FORMAT_A8R8G8B8/ A8R8G8B8 = 3,
        NV_FORMAT_A16B16G16R16F/ A16B16G16R16F = 4,
    }
}

nvenum! {
    pub enum NV_ROTATE / DisplayRotation {
        NV_ROTATE_0 / Default = 0,
        NV_ROTATE_90 / Rotate90 = 1,
        NV_ROTATE_180 / Rotate180 = 2,
        NV_ROTATE_270 / Rotate270 = 3,
        NV_ROTATE_IGNORED / Ignored = 4,
    }
}

nvenum! {
    pub enum NV_SCALING / DisplayScaling {
        NV_SCALING_DEFAULT / Default = 0,// No change.
        NV_SCALING_GPU_SCALING_TO_CLOSEST / GpuScalingToClosest = 1, // Balanced - Full Screen.
        NV_SCALING_GPU_SCALING_TO_NATIVE / GpuScalingToNative = 2, // Force GPU - Full Screen.
        NV_SCALING_GPU_SCANOUT_TO_NATIVE / GpuScanoutToNative = 3, // Force GPU - Centered Scaling.
        NV_SCALING_GPU_SCALING_TO_ASPECT_SCANOUT_TO_NATIVE / GpuScalingToAspectScanoutToNative = 4, // Force GPU - Aspect Ratio.
        NV_SCALING_GPU_SCALING_TO_ASPECT_SCANOUT_TO_CLOSEST / GpuScalingToAspectScanoutToClosest = 5, // Balanced - Aspect Ratio.
        NV_SCALING_GPU_SCANOUT_TO_CLOSEST / GpuScanoutToClosest = 6, // Balanced - Centered Scaling.
        NV_SCALING_MONITOR_SCALING / MonitorScaling = 7,
        NV_SCALING_ADAPTER_SCALING / AdapterScaling = 8,
        NV_SCALING_CENTERED / Centered = 9,
        NV_SCALING_ASPECT_SCALING / AspectScaling = 10,
        NV_SCALING_CUSTOMIZED / Customized = 11, //For future use.
    }
}

nvenum! {
    pub enum NV_DISPLAY_TV_FORMAT / DisplayTVFormat {
        NV_DISPLAY_TV_FORMAT_NONE / None = 0,
        NV_DISPLAY_TV_FORMAT_SD_NTSCM / SDNTSCM = 1,
        NV_DISPLAY_TV_FORMAT_SD_NTSCJ / SDNTSCJ = 2,
        NV_DISPLAY_TV_FORMAT_SD_PALM / SDPALM = 3,
        NV_DISPLAY_TV_FORMAT_SD_PALBDGH / SDPALBDGH = 4,
        NV_DISPLAY_TV_FORMAT_SD_PALN / SDPALN = 5,
        NV_DISPLAY_TV_FORMAT_SD_PALNC / SDPALNC = 6,
        NV_DISPLAY_TV_FORMAT_SD_576i / SD576i = 7,
        NV_DISPLAY_TV_FORMAT_SD_480i / SD480i  = 8,
        NV_DISPLAY_TV_FORMAT_ED_480p / ED480p = 9,
        NV_DISPLAY_TV_FORMAT_ED_576p / ED576p = 10,
        NV_DISPLAY_TV_FORMAT_HD_720p / HD720p = 11,
        NV_DISPLAY_TV_FORMAT_HD_1080i / HD1080i = 12,
        NV_DISPLAY_TV_FORMAT_HD_1080p / HD1080p = 13,
        NV_DISPLAY_TV_FORMAT_HD_720p50 / HD720p50 = 14,
        NV_DISPLAY_TV_FORMAT_HD_1080p24 / HD1080p24 = 15,
        NV_DISPLAY_TV_FORMAT_HD_1080i50 / HD1080i50 = 16,
        NV_DISPLAY_TV_FORMAT_HD_1080p50 / HD1080p50 = 17,
        NV_DISPLAY_TV_FORMAT_UHD_4Kp30 / UHD4Kp30 = 18,
        NV_DISPLAY_TV_FORMAT_UHD_4Kp30_3840 / UHD4Kp303840 = 19,
        NV_DISPLAY_TV_FORMAT_UHD_4Kp25 / UHD4Kp25 = 20,
        NV_DISPLAY_TV_FORMAT_UHD_4Kp25_3840 / UHD4Kp253840 = 21,
        NV_DISPLAY_TV_FORMAT_UHD_4Kp24 / UHD4Kp24 = 22,
        NV_DISPLAY_TV_FORMAT_UHD_4Kp24_3840 / UHD4Kp243840 = 23,
        NV_DISPLAY_TV_FORMAT_UHD_4Kp24_SMPTE / UHD4Kp24SMPTE = 24,
        NV_DISPLAY_TV_FORMAT_UHD_4Kp50_3840 / UHD4Kp503840 = 25,
        NV_DISPLAY_TV_FORMAT_UHD_4Kp60_3840 / UHD4Kp603840 = 26,
        NV_DISPLAY_TV_FORMAT_UHD_4Kp30_4096 / UHD4Kp304096 = 27,
        NV_DISPLAY_TV_FORMAT_UHD_4Kp25_4096 / UHD4Kp254096 = 28,
        NV_DISPLAY_TV_FORMAT_UHD_4Kp24_4096 / UHD4Kp244096 = 29,
        NV_DISPLAY_TV_FORMAT_UHD_4Kp50_4096 / UHD4Kp504096 = 30,
        NV_DISPLAY_TV_FORMAT_UHD_4Kp60_4096 / UHD4Kp604096 = 31,
        NV_DISPLAY_TV_FORMAT_SD_OTHER / SDOther = 32,
        NV_DISPLAY_TV_FORMAT_ED_OTHER / EDOther = 33,
        NV_DISPLAY_TV_FORMAT_HD_OTHER / HDOther = 34,
        NV_DISPLAY_TV_FORMAT_ANY / Any = 35,
    }
}

nvenum! {
    pub enum NV_GPU_CONNECTOR_TYPE / GpuConnectorType {
        NVAPI_GPU_CONNECTOR_VGA_15_PIN / VGA15Pin = 0,
        NVAPI_GPU_CONNECTOR_TV_COMPOSITE / TvComposite = 1,
        NVAPI_GPU_CONNECTOR_TV_SVIDEO / SVideo = 2,
        NVAPI_GPU_CONNECTOR_TV_HDTV_COMPONENT / HdtvComponent = 3,
        NVAPI_GPU_CONNECTOR_TV_SCART / Scart = 4,
        NVAPI_GPU_CONNECTOR_TV_COMPOSITE_SCART_ON_EIAJ4120 / CompositeScartOnEIAJ4120 = 5,
        NVAPI_GPU_CONNECTOR_TV_HDTV_EIAJ4120 / HdtvEIAJ4120 = 6,
        NVAPI_GPU_CONNECTOR_PC_POD_HDTV_YPRPB / PodHdtvYprpb = 7,
        NVAPI_GPU_CONNECTOR_PC_POD_SVIDEO / PodSVideo = 8,
        NVAPI_GPU_CONNECTOR_PC_POD_COMPOSITE / PodComposite = 9,
        NVAPI_GPU_CONNECTOR_DVI_I_TV_SVIDEO / DVIITVSvideo = 10,
        NVAPI_GPU_CONNECTOR_DVI_I_TV_COMPOSITE / DVIITVComposite = 11,
        NVAPI_GPU_CONNECTOR_DVI_I / DVII = 12,
        NVAPI_GPU_CONNECTOR_DVI_D / DVID = 13,
        NVAPI_GPU_CONNECTOR_ADC / ADC = 14,
        NVAPI_GPU_CONNECTOR_LFH_DVI_I_1 / LfhDVII1 = 15,
        NVAPI_GPU_CONNECTOR_LFH_DVI_I_2 / LfhDVII2 = 16,
        NVAPI_GPU_CONNECTOR_SPWG / SPWG = 17,
        NVAPI_GPU_CONNECTOR_OEM / OEM = 18,
        NVAPI_GPU_CONNECTOR_DISPLAYPORT_EXTERNAL / DisplayPortExternal = 19,
        NVAPI_GPU_CONNECTOR_DISPLAYPORT_INTERNAL / DisplayPortInternal = 20,
        NVAPI_GPU_CONNECTOR_DISPLAYPORT_MINI_EXT / DisplayPortMiniExt = 21,
        NVAPI_GPU_CONNECTOR_HDMI_A / HDMIa = 22,
        NVAPI_GPU_CONNECTOR_HDMI_C_MINI / HDMIcMini = 23,
        NVAPI_GPU_CONNECTOR_LFH_DISPLAYPORT_1 / LfhDisplayPort1 = 24,
        NVAPI_GPU_CONNECTOR_LFH_DISPLAYPORT_2, / LfhDisplayPort2 = 25,
        NVAPI_GPU_CONNECTOR_VIRTUAL_WFD / VirtualWfd = 26,
        NVAPI_GPU_CONNECTOR_USB_C / UsbC = 27,
        NVAPI_GPU_CONNECTOR_UNKNOWN / Unknown = 28,
    }
}

nvstruct! {
    pub struct NV_DISPLAY_PATH {
        pub deviceMask: u32,
        pub sourceId: u32,
        pub bPrimary: crate::types::BoolU32,
        pub connector: NV_GPU_CONNECTOR_TYPE,
        pub width: u32,
        pub height: u32,
        pub depth: u32,
        pub colorFormat: NV_FORMAT,
        pub roation: NV_ROTATE,
        pub scaling: NV_SCALING,
        pub refreshRate: u32,
        pub interlaced: crate::types::BoolU32,
        pub tvFormat: NV_DISPLAY_TV_FORMAT,
        pub posx: u32,
        pub posy: u32,
        pub bGDIPrimary: crate::types::BoolU32,
        pub bForceModeSet: crate::types::BoolU32,
        pub bFocusDisplay: crate::types::BoolU32,
        pub gpuId: u32,
    }
}

// nvstruct! {
//     pub struct NV_DISPLAY_PATH_INFO {
//         pub version: u32;
//         pub count: u32;

//     }
// }

nvapi! {
    pub type NvAPI_DISP_GetDisplayConfig = extern "C" fn(pathInfoCount: *mut u32, pathInfo: *mut ) -> NvAPI_Status;

    /// This function returns the handle of an unattached NVIDIA display that is
    /// associated with the given display name (such as "\\DISPLAY1").
    pub unsafe fn NvAPI_DISP_GetDisplayConfig;
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
