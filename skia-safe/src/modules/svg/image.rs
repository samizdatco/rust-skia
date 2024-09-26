use super::{DebugAttributes, Inherits, SvgContainer, SvgIri, SvgLength, SvgPreserveAspectRatio};
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgImage = Inherits<sb::SkSVGImage, SvgContainer>;

impl DebugAttributes for SvgImage {
    const NAME: &'static str = "Image";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("x", &self.get_x())
                .field("y", &self.get_y())
                .field("width", &self.get_width())
                .field("height", &self.get_height())
                .field("href", &self.get_href())
                .field("preserve_aspect_ratio", self.get_preserve_aspect_ratio()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGImage {
    type Base = sb::SkRefCntBase;
}

impl SvgImage {
    pub fn from_ptr(node: *mut sb::SkSVGImage) -> Option<Self> {
        let base = SvgContainer::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGImage) -> Option<Self> {
        let base = SvgContainer::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGImage[native, native_mut] => {
            x: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            y: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            width: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            height: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            href: SvgIri [get(value) => SvgIri::from_native_ref(value), set(value) => value.into_native()],
            preserve_aspect_ratio: SvgPreserveAspectRatio [get(value) => SvgPreserveAspectRatio::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
