#![doc = r" Auto-generated glue file (well, it will be in the future)"]
#[allow(unused)]
use crate::com_stubs::{ComStubClassParams, ComStubParams, ComStubVtableParams};
use lazy_static::lazy_static;
lazy_static! {
    pub(crate) static ref COM_STUB_PARAMS: ComStubParams = ComStubParams {
        classes: vec![ComStubClassParams {
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
        }],
    };
}
