#![doc = r" do not edit! File auto-generated with win32-bindgen"]
#[allow(unused)]
use crate::com_stubs::{ComStubClassParams, ComStubParams, ComStubVtableParams};
use lazy_static::lazy_static;
lazy_static! {
    pub(crate) static ref COM_STUB_PARAMS: ComStubParams = ComStubParams {
        classes: vec![
            ComStubClassParams {
                namespace: "Windows.Win32.Graphics.DirectDraw".to_string(),
                name: "DirectDraw".to_string(),
                vtables: vec![ComStubVtableParams {
                    function_names: vec![
                        ("IDirectDraw".to_string(), "QueryInterface".to_string()),
                        ("IDirectDraw".to_string(), "AddRef".to_string()),
                        ("IDirectDraw".to_string(), "Release".to_string()),
                        ("IDirectDraw".to_string(), "Compact".to_string()),
                        ("IDirectDraw".to_string(), "CreateClipper".to_string()),
                        ("IDirectDraw".to_string(), "CreatePalette".to_string()),
                        ("IDirectDraw".to_string(), "CreateSurface".to_string()),
                        ("IDirectDraw".to_string(), "DuplicateSurface".to_string()),
                        ("IDirectDraw".to_string(), "EnumDisplayModes".to_string()),
                        ("IDirectDraw".to_string(), "EnumSurfaces".to_string()),
                        ("IDirectDraw".to_string(), "FlipToGDISurface".to_string()),
                        ("IDirectDraw".to_string(), "GetCaps".to_string()),
                        ("IDirectDraw".to_string(), "GetDisplayMode".to_string()),
                        ("IDirectDraw".to_string(), "GetFourCCCodes".to_string()),
                        ("IDirectDraw".to_string(), "GetGDISurface".to_string()),
                        ("IDirectDraw".to_string(), "GetMonitorFrequency".to_string()),
                        ("IDirectDraw".to_string(), "GetScanLine".to_string()),
                        (
                            "IDirectDraw".to_string(),
                            "GetVerticalBlankStatus".to_string()
                        ),
                        ("IDirectDraw".to_string(), "Initialize".to_string()),
                        ("IDirectDraw".to_string(), "RestoreDisplayMode".to_string()),
                        ("IDirectDraw".to_string(), "SetCooperativeLevel".to_string()),
                        ("IDirectDraw".to_string(), "SetDisplayMode".to_string()),
                        (
                            "IDirectDraw".to_string(),
                            "WaitForVerticalBlank".to_string()
                        ),
                    ]
                }],
            },
            ComStubClassParams {
                namespace: "Windows.Win32.Graphics.DirectDraw".to_string(),
                name: "DirectDrawSurface".to_string(),
                vtables: vec![ComStubVtableParams {
                    function_names: vec![
                        (
                            "IDirectDrawSurface".to_string(),
                            "QueryInterface".to_string()
                        ),
                        ("IDirectDrawSurface".to_string(), "AddRef".to_string()),
                        ("IDirectDrawSurface".to_string(), "Release".to_string()),
                        (
                            "IDirectDrawSurface".to_string(),
                            "AddAttachedSurface".to_string()
                        ),
                        (
                            "IDirectDrawSurface".to_string(),
                            "AddOverlayDirtyRect".to_string()
                        ),
                        ("IDirectDrawSurface".to_string(), "Blt".to_string()),
                        ("IDirectDrawSurface".to_string(), "BltBatch".to_string()),
                        ("IDirectDrawSurface".to_string(), "BltFast".to_string()),
                        (
                            "IDirectDrawSurface".to_string(),
                            "DeleteAttachedSurface".to_string()
                        ),
                        (
                            "IDirectDrawSurface".to_string(),
                            "EnumAttachedSurfaces".to_string()
                        ),
                        (
                            "IDirectDrawSurface".to_string(),
                            "EnumOverlayZOrders".to_string()
                        ),
                        ("IDirectDrawSurface".to_string(), "Flip".to_string()),
                        (
                            "IDirectDrawSurface".to_string(),
                            "GetAttachedSurface".to_string()
                        ),
                        ("IDirectDrawSurface".to_string(), "GetBltStatus".to_string()),
                        ("IDirectDrawSurface".to_string(), "GetCaps".to_string()),
                        ("IDirectDrawSurface".to_string(), "GetClipper".to_string()),
                        ("IDirectDrawSurface".to_string(), "GetColorKey".to_string()),
                        ("IDirectDrawSurface".to_string(), "GetDC".to_string()),
                        (
                            "IDirectDrawSurface".to_string(),
                            "GetFlipStatus".to_string()
                        ),
                        (
                            "IDirectDrawSurface".to_string(),
                            "GetOverlayPosition".to_string()
                        ),
                        ("IDirectDrawSurface".to_string(), "GetPalette".to_string()),
                        (
                            "IDirectDrawSurface".to_string(),
                            "GetPixelFormat".to_string()
                        ),
                        (
                            "IDirectDrawSurface".to_string(),
                            "GetSurfaceDesc".to_string()
                        ),
                        ("IDirectDrawSurface".to_string(), "Initialize".to_string()),
                        ("IDirectDrawSurface".to_string(), "IsLost".to_string()),
                        ("IDirectDrawSurface".to_string(), "Lock".to_string()),
                        ("IDirectDrawSurface".to_string(), "ReleaseDC".to_string()),
                        ("IDirectDrawSurface".to_string(), "Restore".to_string()),
                        ("IDirectDrawSurface".to_string(), "SetClipper".to_string()),
                        ("IDirectDrawSurface".to_string(), "SetColorKey".to_string()),
                        (
                            "IDirectDrawSurface".to_string(),
                            "SetOverlayPosition".to_string()
                        ),
                        ("IDirectDrawSurface".to_string(), "SetPalette".to_string()),
                        ("IDirectDrawSurface".to_string(), "Unlock".to_string()),
                        (
                            "IDirectDrawSurface".to_string(),
                            "UpdateOverlay".to_string()
                        ),
                        (
                            "IDirectDrawSurface".to_string(),
                            "UpdateOverlayDisplay".to_string()
                        ),
                        (
                            "IDirectDrawSurface".to_string(),
                            "UpdateOverlayZOrder".to_string()
                        ),
                    ]
                }],
            },
        ],
    };
}
