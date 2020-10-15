use crate::*;
use aftereffects_sys as ae_sys;
use c_vec::CVec;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    ffi::{CStr, CString},
    fmt::Debug,
    marker::PhantomData,
};

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Pixel {
    pub alpha: ae_sys::A_u_char,
    pub red: ae_sys::A_u_char,
    pub green: ae_sys::A_u_char,
    pub blue: ae_sys::A_u_char,
}

impl From<ae_sys::PF_Pixel> for Pixel {
    fn from(pixel: ae_sys::PF_Pixel) -> Self {
        Self {
            alpha: pixel.alpha,
            red: pixel.red,
            green: pixel.green,
            blue: pixel.blue,
        }
    }
}

impl From<Pixel> for ae_sys::PF_Pixel {
    fn from(pixel: Pixel) -> Self {
        Self {
            alpha: pixel.alpha,
            red: pixel.red,
            green: pixel.green,
            blue: pixel.blue,
        }
    }
}

pub type Pixel8 = Pixel;

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[repr(i32)]
pub enum EventType {
    None = ae_sys::PF_Event_NONE,
    NewContext = ae_sys::PF_Event_NEW_CONTEXT,
    Activate = ae_sys::PF_Event_ACTIVATE,
    DoClick = ae_sys::PF_Event_DO_CLICK,
    Drag = ae_sys::PF_Event_DRAG,
    Draw = ae_sys::PF_Event_DRAW,
    Deactivate = ae_sys::PF_Event_DEACTIVATE,
    Context = ae_sys::PF_Event_CLOSE_CONTEXT,
    Idle = ae_sys::PF_Event_IDLE,
    // New for AE 4.0, sent when mouse moves over custom UI
    AdjustCursor = ae_sys::PF_Event_ADJUST_CURSOR,
    // New for AE 7.0, replaces previous keydown event with
    // cross platform codes and unicode characters.
    Keydown = ae_sys::PF_Event_KEYDOWN,
    // New for AE 11.0, notification that the mouse is no
    // longer over a specific view (layer or comp only).
    MouseExited = ae_sys::PF_Event_MOUSE_EXITED,
    NumEvents = ae_sys::PF_Event_NUM_EVENTS,
}

bitflags! {
    pub struct EventOutFlags: u32 {
        const NONE = ae_sys::PF_EO_NONE;
        const HANDLED_EVENT = ae_sys::PF_EO_HANDLED_EVENT;
        // Rerender the comp.
        const ALWAYS_UPDATE = ae_sys::PF_EO_ALWAYS_UPDATE;
        // Do not rerender the comp.
        const NEVER_UPDATE = ae_sys::PF_EO_NEVER_UPDATE;
        // Update the view immediately after the event returns when using pf::InvalidateRect.
        const UPDATE_NOW = ae_sys::PF_EO_UPDATE_NOW;
    }
}

define_struct_wrapper!(EventExtra, PF_EventExtra);

impl EventExtra {
    pub fn get_context_handle(&self) -> ContextHandle {
        ContextHandle::from_raw(self.0.contextH)
    }

    pub fn get_event_type(&self) -> EventType {
        EventType::try_from(self.0.e_type).unwrap()
    }

    pub fn event_out_flags(&mut self, flags: EventOutFlags) {
        self.0.evt_out_flags = flags.bits() as _;
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Pixel16 {
    pub alpha: ae_sys::A_u_short,
    pub red: ae_sys::A_u_short,
    pub green: ae_sys::A_u_short,
    pub blue: ae_sys::A_u_short,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Pixel32 {
    pub alpha: ae_sys::PF_FpShort,
    pub red: ae_sys::PF_FpShort,
    pub green: ae_sys::PF_FpShort,
    pub blue: ae_sys::PF_FpShort,
}

#[derive(Debug, Copy, Clone, Hash)]
#[repr(i32)]
pub enum TransferMode {
    None = ae_sys::PF_Xfer_NONE,
    Copy = ae_sys::PF_Xfer_COPY,
    Behind = ae_sys::PF_Xfer_BEHIND,
    InFront = ae_sys::PF_Xfer_IN_FRONT,
    Dissolve = ae_sys::PF_Xfer_DISSOLVE,
    Add = ae_sys::PF_Xfer_ADD,
    Mulitply = ae_sys::PF_Xfer_MULTIPLY,
    Screen = ae_sys::PF_Xfer_SCREEN,
    Overlay = ae_sys::PF_Xfer_OVERLAY,
    SoftLight = ae_sys::PF_Xfer_SOFT_LIGHT,
    HardLight = ae_sys::PF_Xfer_HARD_LIGHT,
    Darken = ae_sys::PF_Xfer_DARKEN,
    Lighten = ae_sys::PF_Xfer_LIGHTEN,
    Difference = ae_sys::PF_Xfer_DIFFERENCE,
    Hue = ae_sys::PF_Xfer_HUE,
    Saturation = ae_sys::PF_Xfer_SATURATION,
    Color = ae_sys::PF_Xfer_COLOR,
    Luminosity = ae_sys::PF_Xfer_LUMINOSITY,
    MultiplyAlpha = ae_sys::PF_Xfer_MULTIPLY_ALPHA,
    MultiplyAlphaLuma = ae_sys::PF_Xfer_MULTIPLY_ALPHA_LUMA,
    MultiplyNotAlpha = ae_sys::PF_Xfer_MULTIPLY_NOT_ALPHA,
    MultiplyNotAlphaLuma = ae_sys::PF_Xfer_MULTIPLY_NOT_ALPHA_LUMA,
    AddiditivePremul = ae_sys::PF_Xfer_ADDITIVE_PREMUL,
    AlphaAdd = ae_sys::PF_Xfer_ALPHA_ADD,
    ColorDodge = ae_sys::PF_Xfer_COLOR_DODGE,
    ColorBurn = ae_sys::PF_Xfer_COLOR_BURN,
    Exclusion = ae_sys::PF_Xfer_EXCLUSION,

    Difference2 = ae_sys::PF_Xfer_DIFFERENCE2,
    ColorDodge2 = ae_sys::PF_Xfer_COLOR_DODGE2,
    ColorBurn2 = ae_sys::PF_Xfer_COLOR_BURN2,

    LinearDodge = ae_sys::PF_Xfer_LINEAR_DODGE,
    LinearBurn = ae_sys::PF_Xfer_LINEAR_BURN,
    LinearLight = ae_sys::PF_Xfer_LINEAR_LIGHT,
    VividLight = ae_sys::PF_Xfer_VIVID_LIGHT,
    PinLight = ae_sys::PF_Xfer_PIN_LIGHT,

    HardMix = ae_sys::PF_Xfer_HARD_MIX,

    LighterColor = ae_sys::PF_Xfer_LIGHTER_COLOR,
    DarkerColor = ae_sys::PF_Xfer_DARKER_COLOR,

    Subtract = ae_sys::PF_Xfer_SUBTRACT,
    Divide = ae_sys::PF_Xfer_DIVIDE,

    Reserved0 = ae_sys::PF_Xfer_RESERVED0,
    Reserved1 = ae_sys::PF_Xfer_RESERVED1,

    NumModes = ae_sys::PF_Xfer_NUM_MODES,
}

pub type XferMode = TransferMode;

#[derive(Debug, Copy, Clone, Hash)]
#[repr(C)]
pub struct CompositeMode {
    pub xfer: TransferMode,
    // For TransferMode::DissolveRandomized.
    pub rand_seed: i32,
    // 0–255.
    pub opacity: u8,
    // Ignored TransferMode::MutiplyAlpha* modes.
    pub rgb_only: u8,
    // For deep color only.
    pub opacity_su: u16,
}

#[derive(Debug, Copy, Clone, Hash)]
#[repr(C)]
pub struct Point {
    pub h: i32,
    pub v: i32,
}

#[derive(Debug, Copy, Clone, Hash)]
#[repr(C)]
pub struct RationalScale {
    pub num: ae_sys::A_long,
    pub den: ae_sys::A_u_long,
}

impl From<RationalScale> for ae_sys::PF_RationalScale {
    #[inline]
    fn from(ratio: RationalScale) -> Self {
        Self {
            num: ratio.num,
            den: ratio.den,
        }
    }
}

impl From<ae_sys::PF_RationalScale> for RationalScale {
    #[inline]
    fn from(ratio: ae_sys::PF_RationalScale) -> Self {
        Self {
            num: ratio.num,
            den: ratio.den,
        }
    }
}

impl From<RationalScale> for f64 {
    #[inline]
    fn from(ratio: RationalScale) -> Self {
        debug_assert!(ratio.den != 0);
        ratio.num as Self / ratio.den as Self
    }
}

impl From<RationalScale> for f32 {
    #[inline]
    fn from(ratio: RationalScale) -> Self {
        debug_assert!(ratio.den != 0);
        ratio.num as Self / ratio.den as Self
    }
}

pub type MaskFlags = u32;

#[derive(Debug)]
#[repr(C)]
pub struct MaskWorld {
    pub mask: EffectWorld,
    pub offset: Point,
    pub what_is_mask: MaskFlags,
}

#[derive(Copy, Clone, Debug, Hash)]
#[repr(i32)]
pub enum Quality {
    DrawingAudio = ae_sys::PF_Quality_DRAWING_AUDIO,
    Lo = ae_sys::PF_Quality_LO,
    Hi = ae_sys::PF_Quality_HI,
}

#[repr(u32)]
pub enum ModeFlags {
    AlphaPremul = ae_sys::PF_MF_Alpha_PREMUL,
    AlphaStraight = ae_sys::PF_MF_Alpha_STRAIGHT,
}

#[repr(u32)]
pub enum Field {
    Frame = ae_sys::PF_Field_FRAME,
    Upper = ae_sys::PF_Field_UPPER,
    Lower = ae_sys::PF_Field_LOWER,
}

pub type Command = ae_sys::PF_Cmd;

// FIXME: wrap this nicely
/// An EffectWorld is a view on a WorldHandle that can be used to write to.
#[derive(Debug, Copy, Clone)]
pub struct EffectWorld {
    pub effect_world: ae_sys::PF_EffectWorld,
}

pub struct EffectWorldConst {
    pub effect_world: ae_sys::PF_EffectWorld,
}

unsafe impl Send for EffectWorldConst {}
unsafe impl Sync for EffectWorldConst {}

define_handle_wrapper!(EffectBlendingTables, PF_EffectBlendingTables);

impl EffectWorld {
    #[inline]
    pub fn new(world_handle: WorldHandle) -> Result<Self, crate::Error> {
        WorldSuite::new()?.fill_out_pf_effect_world(world_handle)
    }

    pub fn from_raw(
        effect_world_ptr: *const ae_sys::PF_EffectWorld,
    ) -> Result<Self, crate::Error> {
        if std::ptr::null() == effect_world_ptr {
            Err(crate::Error::Generic)
        } else {
            Ok(EffectWorld {
                effect_world: unsafe { *effect_world_ptr },
            })
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.effect_world.width as usize
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.effect_world.height as usize
    }

    #[inline]
    pub fn row_bytes(&self) -> usize {
        self.effect_world.rowbytes as usize
    }

    #[inline]
    pub fn data_as_ptr(&self) -> *const u8 {
        self.effect_world.data as *const u8
    }

    #[inline]
    pub fn data_as_ptr_mut(&self) -> *mut u8 {
        self.effect_world.data as *mut u8
    }

    #[inline]
    pub fn data_len(&self) -> usize {
        self.height() * self.row_bytes()
    }

    pub fn row_padding_bytes(&self) -> usize {
        self.row_bytes()
            - self.width()
                * 4
                * match self.world_type() {
                    WorldType::Integer => 2,
                    WorldType::Byte => 1,
                    WorldType::Float => 4,
                    WorldType::None => panic!(),
                }
    }

    #[inline]
    pub fn as_pixel8_mut(&self, x: usize, y: usize) -> &mut Pixel8 {
        debug_assert!(x < self.width() && y < self.height());
        unsafe {
            &mut *(self.effect_world.data.add(y * self.row_bytes())
                as *mut Pixel8)
                .add(x)
        }
    }

    #[inline]
    pub fn as_pixel8(&self, x: usize, y: usize) -> &Pixel8 {
        debug_assert!(x < self.width() && y < self.height());
        self.as_pixel8_mut(x, y)
    }

    #[inline]
    pub fn as_pixel16_mut(&self, x: usize, y: usize) -> &mut Pixel16 {
        debug_assert!(x < self.width() && y < self.height());
        unsafe {
            &mut *(self.effect_world.data.add(y * self.row_bytes())
                as *mut Pixel16)
                .add(x)
        }
    }

    #[inline]
    pub fn as_pixel16(&self, x: usize, y: usize) -> &Pixel16 {
        debug_assert!(x < self.width() && y < self.height());
        self.as_pixel16_mut(x, y)
    }

    #[inline]
    pub fn as_pixel32_mut(&self, x: usize, y: usize) -> &mut Pixel32 {
        debug_assert!(x < self.width() && y < self.height());
        unsafe {
            &mut *(self.effect_world.data.add(y * self.row_bytes())
                as *mut Pixel32)
                .add(x)
        }
    }

    #[inline]
    pub fn as_pixel32(&self, x: usize, y: usize) -> &Pixel32 {
        debug_assert!(x < self.width() && y < self.height());
        unsafe {
            &*((self.effect_world.data as *const u8).add(y * self.row_bytes())
                as *const Pixel32)
                .add(x)
        }
    }

    #[inline]
    pub fn world_type(&self) -> WorldType {
        let flags = self.effect_world.world_flags;
        // Most frequent case is 16bit integer.
        if ae_sys::PF_WorldFlag_DEEP & flags as u32 != 0 {
            WorldType::Integer
        } else if ae_sys::PF_WorldFlag_RESERVED1 & flags as u32 != 0 {
            WorldType::Float
        } else {
            WorldType::Byte
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const ae_sys::PF_EffectWorld {
        &self.effect_world as *const ae_sys::PF_EffectWorld
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut ae_sys::PF_EffectWorld {
        &mut self.effect_world as *mut ae_sys::PF_EffectWorld
    }
}

#[macro_export]
macro_rules! add_param {
    (in_data: expr,
    index: expr,
    def: expr) => {
        in_data.inter.add_param.unwrap()(in_data.effect_ref, (index), &(def))
    };
}

pub fn progress(in_data: InDataHandle, count: u16, total: u16) -> i32 {
    unsafe {
        (*in_data.as_ptr()).inter.progress.unwrap()(
            (*in_data.as_ptr()).effect_ref,
            count as i32,
            total as i32,
        )
    }
}

#[derive(Debug)]
pub struct Handle<'a, T: 'a> {
    suite_ptr: *const ae_sys::PF_HandleSuite1,
    handle: ae_sys::PF_Handle,
    _marker: PhantomData<&'a T>,
}

pub struct HandleLock<'a, T> {
    parent_handle: &'a Handle<'a, T>,
    ptr: *mut T,
}

impl<'a, T> HandleLock<'a, T> {
    pub fn get(&self) -> Result<&'a T, Error> {
        if self.ptr.is_null() {
            Err(Error::InvalidIndex)
        } else {
            Ok(unsafe { &*self.ptr })
        }
    }

    pub fn get_mut(&self) -> Result<&'a mut T, Error> {
        if self.ptr.is_null() {
            Err(Error::InvalidIndex)
        } else {
            Ok(unsafe { &mut *self.ptr })
        }
    }
}

impl<'a, T> Drop for HandleLock<'a, T> {
    fn drop(&mut self) {
        ae_call_suite_fn_no_err!(
            self.parent_handle.suite_ptr,
            host_unlock_handle,
            self.parent_handle.handle
        );
    }
}

impl<'a, T: 'a> Handle<'a, T> {
    pub fn new(value: T) -> Result<Handle<'a, T>, Error> {
        match ae_acquire_suite_ptr!(
            borrow_pica_basic_as_ptr(),
            PF_HandleSuite1,
            kPFHandleSuite,
            kPFHandleSuiteVersion1
        ) {
            Ok(suite_ptr) => {
                let handle: ae_sys::PF_Handle = ae_call_suite_fn_no_err!(
                    suite_ptr,
                    host_new_handle,
                    std::mem::size_of::<T>() as u64
                );
                if handle.is_null() {
                    return Err(Error::OutOfMemory);
                }
                let ptr = ae_call_suite_fn_no_err!(
                    suite_ptr,
                    host_lock_handle,
                    handle
                ) as *mut T;
                if ptr.is_null() {
                    return Err(Error::InvalidIndex);
                }
                unsafe { ptr.write(value) };
                ae_call_suite_fn_no_err!(suite_ptr, host_unlock_handle, handle);
                Ok(Handle {
                    suite_ptr,
                    handle,
                    //dispose: true,
                    _marker: PhantomData,
                })
            }
            Err(_) => Err(Error::InvalidCallback),
        }
    }

    pub fn set(&mut self, value: T) {
        let ptr = ae_call_suite_fn_no_err!(
            self.suite_ptr,
            host_lock_handle,
            self.handle
        ) as *mut T;
        if !ptr.is_null() {
            unsafe {
                // Run destructors, if any.
                ptr.read()
            };
        }
        unsafe { ptr.write(value) };
        ae_call_suite_fn_no_err!(
            self.suite_ptr,
            host_unlock_handle,
            self.handle
        );
    }

    pub fn lock(&mut self) -> Result<HandleLock<T>, Error> {
        let ptr = ae_call_suite_fn_no_err!(
            self.suite_ptr,
            host_lock_handle,
            self.handle
        ) as *mut T;
        if ptr.is_null() {
            Err(Error::InvalidIndex)
        } else {
            Ok(HandleLock {
                parent_handle: self,
                ptr,
            })
        }
    }

    pub fn get(&self) -> Result<&'a T, Error> {
        let ptr = unsafe { *(self.handle as *const *const T) };
        if ptr.is_null() {
            Err(Error::InvalidIndex)
        } else {
            Ok(unsafe { &(*ptr) })
        }
    }

    pub fn size(&self) -> usize {
        ae_call_suite_fn_no_err!(
            self.suite_ptr,
            host_get_handle_size,
            self.handle
        ) as usize
    }

    /*
    pub fn resize(&mut self, size: usize) -> Result<(), Error> {
        ae_call_suite_fn!(self.suite_ptr, host_resize_handle, size as u64, &mut self.handle)
    }*/

    pub fn from_raw(handle: ae_sys::PF_Handle) -> Result<Handle<'a, T>, Error> {
        match ae_acquire_suite_ptr!(
            borrow_pica_basic_as_ptr(),
            PF_HandleSuite1,
            kPFHandleSuite,
            kPFHandleSuiteVersion1
        ) {
            Ok(suite_ptr) => Ok(Handle {
                suite_ptr,
                handle,
                //dispose: true,
                _marker: PhantomData,
            }),
            Err(_) => Err(Error::InvalidCallback),
        }
    }

    /// Consumes the handle.
    pub fn into_raw(handle: Handle<T>) -> ae_sys::PF_Handle {
        //let us = crate::aegp::UtilitySuite::new().unwrap();
        //us.write_to_os_console("Handle::into_raw()").unwrap();

        let return_handle = handle.handle;
        // Handle is just on the stack so
        // we're not leaking anything here
        std::mem::forget(handle);
        // Make sure drop(Handle) does *not*
        // actually drop anything since we're
        // passing ownership.
        //handle.dispose = false;
        return_handle
        // drop(handle) gets called.
    }

    /// Returns the raw handle.
    pub fn as_raw(&self) -> ae_sys::PF_Handle {
        self.handle
    }
}

impl<'a, T: 'a> Drop for Handle<'a, T> {
    fn drop(&mut self) {
        let ptr = unsafe { *(self.handle as *const *const T) };
        if !ptr.is_null() {
            unsafe { ptr.read() };
        }

        ae_call_suite_fn_no_err!(
            self.suite_ptr,
            host_dispose_handle,
            self.handle
        );
    }
}

/// A flat handle takes a [Vec<u8>] as data. This is useful when data it passed
/// to Ae permanently or between runs of your plug-in.
/// You can use something like [serde_cbor::to_vec()] to seriealize your data
/// structure into a flat [Vec<u8>].
#[derive(Debug)]
pub struct FlatHandle<'a> {
    suite_ptr: *const ae_sys::PF_HandleSuite1,
    handle: ae_sys::PF_Handle,
    _marker: PhantomData<&'a ()>,
}

impl<'a> FlatHandle<'a> {
    pub fn new(slice: impl Into<Vec<u8>>) -> Result<FlatHandle<'a>, Error> {
        let pica_basic_suite_ptr = borrow_pica_basic_as_ptr();

        match ae_acquire_suite_ptr!(
            pica_basic_suite_ptr,
            PF_HandleSuite1,
            kPFHandleSuite,
            kPFHandleSuiteVersion1
        ) {
            Ok(suite_ptr) => {
                let vector = slice.into();

                let handle: ae_sys::PF_Handle = ae_call_suite_fn_no_err!(
                    suite_ptr,
                    host_new_handle,
                    vector.len() as u64
                );

                let ptr = ae_call_suite_fn_no_err!(
                    suite_ptr,
                    host_lock_handle,
                    handle
                ) as *mut u8;
                debug_assert!(!ptr.is_null());

                let dest =
                    std::ptr::slice_from_raw_parts_mut(ptr, vector.len());
                unsafe {
                    (*dest).copy_from_slice(vector.as_slice());
                }
                ae_call_suite_fn_no_err!(suite_ptr, host_unlock_handle, handle);
                Ok(FlatHandle {
                    suite_ptr,
                    handle,
                    _marker: PhantomData,
                })
            }
            Err(e) => Err(e),
        }
    }

    pub fn as_slice(&'a self) -> &'a [u8] {
        let ptr = unsafe { *(self.handle as *const *const u8) };
        if ptr.is_null() {
            &[]
        } else {
            unsafe { &*std::ptr::slice_from_raw_parts(ptr, self.size()) }
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        unsafe { *(self.handle as *const *const u8) }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let ptr = unsafe { *(self.handle as *const *const u8) };
        if ptr.is_null() {
            Vec::new()
        } else {
            unsafe {
                &*std::ptr::slice_from_raw_parts(
                    *(self.handle as *const *const u8),
                    self.size(),
                )
            }
            .to_vec()
        }
    }

    pub fn size(&self) -> usize {
        ae_call_suite_fn_no_err!(
            self.suite_ptr,
            host_get_handle_size,
            self.handle
        ) as usize
    }

    pub fn from_raw(
        handle: ae_sys::PF_Handle,
    ) -> Result<FlatHandle<'a>, Error> {
        match ae_acquire_suite_ptr!(
            borrow_pica_basic_as_ptr(),
            PF_HandleSuite1,
            kPFHandleSuite,
            kPFHandleSuiteVersion1
        ) {
            Ok(suite_ptr) => {
                if handle.is_null() {
                    Err(Error::Generic)
                } else {
                    let ptr = unsafe { *(handle as *const *const u8) };
                    if ptr.is_null() {
                        Err(Error::InternalStructDamaged)
                    } else {
                        Ok(FlatHandle {
                            suite_ptr,
                            handle,
                            //dispose: true,
                            _marker: PhantomData,
                        })
                    }
                }
            }
            Err(_) => Err(Error::InvalidCallback),
        }
    }

    /// Consumes the handle.
    pub fn into_raw(handle: FlatHandle) -> ae_sys::PF_Handle {
        let return_handle = handle.handle;
        // Handle is just on the stack so
        // we're not leaking anything here.
        // We need to call forget() or else
        // drop() will be called on handle
        // which will dispose the memory.
        std::mem::forget(handle);

        return_handle
    }
}
/*
impl<'a> Clone for FlatHandle<'a> {
    fn clone(&self) -> FlatHandle<'a> {
        Self::new(self.as_slice()).unwrap()
    }
}*/

impl<'a> Drop for FlatHandle<'a> {
    fn drop(&mut self) {
        ae_call_suite_fn_no_err!(
            self.suite_ptr,
            host_dispose_handle,
            self.handle
        );
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl From<ae_sys::PF_LRect> for Rect {
    fn from(rect: ae_sys::PF_LRect) -> Self {
        Rect {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        }
    }
}

impl From<Rect> for ae_sys::PF_LRect {
    fn from(rect: Rect) -> Self {
        ae_sys::PF_LRect {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        }
    }
}

impl Rect {
    pub fn is_empty(&self) -> bool {
        (self.left >= self.right) || (self.top >= self.bottom)
    }

    pub fn union<'a>(&'a mut self, other: &Rect) -> &'a mut Rect {
        if other.is_empty() {
            *self = *other;
        } else if !other.is_empty() {
            self.left = std::cmp::min(self.left, other.left);
            self.top = std::cmp::min(self.top, other.top);
            self.right = std::cmp::max(self.right, other.right);
            self.bottom = std::cmp::max(self.bottom, other.bottom);
        }
        self
    }

    pub fn is_edge_pixel(&self, x: i32, y: i32) -> bool {
        let mut x_hit = (x == self.left) || (x == self.right);
        let mut y_hit = (y == self.top) || (y == self.bottom);

        if x_hit {
            y_hit = (y >= self.top) && (y <= self.bottom);
        } else {
            if y_hit {
                x_hit = (x >= self.left) && (x <= self.right);
            }
        }
        x_hit && y_hit
    }
}

define_handle_wrapper!(ProgressInfo, PF_ProgPtr);

#[derive(Copy, Clone, Debug)]
pub struct SmartRenderCallbacks {
    pub(crate) rc_ptr: *const ae_sys::PF_SmartRenderCallbacks,
}

impl SmartRenderCallbacks {
    pub fn from_raw(rc_ptr: *const ae_sys::PF_SmartRenderCallbacks) -> Self {
        Self { rc_ptr }
    }

    pub fn as_ptr(&self) -> *const ae_sys::PF_SmartRenderCallbacks {
        self.rc_ptr
    }

    pub fn checkout_layer_pixels(
        &self,
        effect_ref: ProgressInfo,
        checkout_id: u32,
    ) -> Result<EffectWorld, Error> {
        if let Some(checkout_layer_pixels) =
            unsafe { *self.rc_ptr }.checkout_layer_pixels
        {
            let mut effect_world_ptr =
                std::mem::MaybeUninit::<*mut ae_sys::PF_EffectWorld>::uninit();

            match unsafe {
                checkout_layer_pixels(
                    effect_ref.as_ptr(),
                    checkout_id as i32,
                    effect_world_ptr.as_mut_ptr(),
                )
            } as u32
            {
                ae_sys::PF_Err_NONE => Ok(EffectWorld {
                    effect_world: unsafe { *effect_world_ptr.assume_init() },
                }),
                e => Err(unsafe { Error::from_unchecked(e as i32) }),
            }
        } else {
            Err(Error::InvalidCallback)
        }
    }

    pub fn checkin_layer_pixels(
        &self,
        effect_ref: ProgressInfo,
        checkout_id: u32,
    ) -> Result<(), Error> {
        if let Some(checkin_layer_pixels) =
            unsafe { *self.rc_ptr }.checkin_layer_pixels
        {
            match unsafe {
                checkin_layer_pixels(effect_ref.as_ptr(), checkout_id as i32)
            } as u32
            {
                ae_sys::PF_Err_NONE => Ok(()),
                e => Err(unsafe { Error::from_unchecked(e as i32) }),
            }
        } else {
            Err(Error::InvalidCallback)
        }
    }

    pub fn checkout_output(
        &self,
        effect_ref: ProgressInfo,
    ) -> Result<EffectWorld, Error> {
        if let Some(checkout_output) = unsafe { *self.rc_ptr }.checkout_output {
            let mut effect_world_ptr =
                std::mem::MaybeUninit::<*mut ae_sys::PF_EffectWorld>::uninit();

            match unsafe {
                checkout_output(
                    effect_ref.as_ptr(),
                    effect_world_ptr.as_mut_ptr(),
                )
            } as u32
            {
                ae_sys::PF_Err_NONE => Ok(EffectWorld {
                    effect_world: unsafe { *effect_world_ptr.assume_init() },
                }),
                e => Err(unsafe { Error::from_unchecked(e as i32) }),
            }
        } else {
            Err(Error::InvalidCallback)
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum ParamIndex {
    None = ae_sys::PF_ParamIndex_NONE,
    CheckAll = ae_sys::PF_ParamIndex_CHECK_ALL,
    CheckAllExceptLayerParams =
        ae_sys::PF_ParamIndex_CHECK_ALL_EXCEPT_LAYER_PARAMS,
    CheckAllHonorExclude = ae_sys::PF_ParamIndex_CHECK_ALL_HONOR_EXCLUDE,
}

#[derive(Copy, Clone, Debug)]
pub struct PreRenderCallbacks {
    pub(crate) rc_ptr: *const ae_sys::PF_PreRenderCallbacks,
}

impl PreRenderCallbacks {
    pub fn from_raw(rc_ptr: *const ae_sys::PF_PreRenderCallbacks) -> Self {
        Self { rc_ptr }
    }

    pub fn as_ptr(&self) -> *const ae_sys::PF_PreRenderCallbacks {
        self.rc_ptr
    }

    pub fn checkout_layer(
        &self,
        effect_ref: ProgressInfo,
        index: i32,
        checkout_id: i32,
        // FIXME: warp this struct
        req: &ae_sys::PF_RenderRequest,
        what_time: i32,
        time_step: i32,
        time_scale: u32,
    ) -> Result<ae_sys::PF_CheckoutResult, Error> {
        if let Some(checkout_layer) = unsafe { *self.rc_ptr }.checkout_layer {
            let mut checkout_result =
                std::mem::MaybeUninit::<ae_sys::PF_CheckoutResult>::uninit();

            match unsafe {
                checkout_layer(
                    effect_ref.as_ptr(),
                    index as i32,
                    checkout_id as i32,
                    req,
                    what_time,
                    time_step,
                    time_scale,
                    checkout_result.as_mut_ptr(),
                )
            } as u32
            {
                ae_sys::PF_Err_NONE => {
                    Ok(unsafe { checkout_result.assume_init() })
                }
                e => Err(unsafe { Error::from_unchecked(e as i32) }),
            }
        } else {
            Err(Error::InvalidCallback)
        }
    }

    /* FIXME
    pub fn guid_mix_in_ptr(
            effect_ref: ProgressInfo,
            buf: [u8],
        ) -> PF_Err,
    >,*/
}
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct InDataHandle {
    pub(crate) in_data_ptr: *const ae_sys::PF_InData,
}

impl InDataHandle {
    pub fn from_raw(in_data_ptr: *const ae_sys::PF_InData) -> Self {
        Self { in_data_ptr }
    }

    pub fn as_ptr(&self) -> *const ae_sys::PF_InData {
        self.in_data_ptr
    }

    pub fn is_null(&self) -> bool {
        self.in_data_ptr.is_null()
    }
}

pub type ProgPtr = ae_sys::PF_ProgPtr;

bitflags! {
    pub struct CustomEventFlags: u32 {
        const NONE = ae_sys::PF_CustomEFlag_NONE;
        const COMP = ae_sys::PF_CustomEFlag_COMP;
        const LAYER = ae_sys::PF_CustomEFlag_LAYER;
        const EFFECT = ae_sys::PF_CustomEFlag_EFFECT;
        const PREVIEW = ae_sys::PF_CustomEFlag_PREVIEW;
    }
}

bitflags! {
    struct _UIAlignment: u32 {
        // No values other than PF_UIAlignment_NONE are honored, in Ae or PPro.
        const NONE = ae_sys::PF_UIAlignment_NONE;
        const TOP = ae_sys::PF_UIAlignment_TOP;
        const LEFT = ae_sys::PF_UIAlignment_LEFT;
        const BOTTOM = ae_sys::PF_UIAlignment_BOTTOM;
        const RIGHT = ae_sys::PF_UIAlignment_RIGHT;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CustomUIInfo(ae_sys::PF_CustomUIInfo);

impl CustomUIInfo {
    pub fn new() -> Self {
        Self(unsafe { std::mem::MaybeUninit::zeroed().assume_init() })
    }

    pub fn as_ptr(&self) -> *const ae_sys::PF_CustomUIInfo {
        &self.0
    }

    pub fn events<'a>(&'a mut self, events: CustomEventFlags) -> &'a mut Self {
        self.0.events = events.bits() as _;
        self
    }

    pub fn comp_ui_width<'a>(&'a mut self, width: u16) -> &'a mut Self {
        self.0.comp_ui_width = width as _;
        self
    }

    pub fn comp_ui_height<'a>(&'a mut self, height: u16) -> &'a mut Self {
        self.0.comp_ui_height = height as _;
        self
    }

    pub fn layer_ui_width<'a>(&'a mut self, width: u16) -> &'a mut Self {
        self.0.layer_ui_width = width as _;
        self
    }

    pub fn layer_ui_height<'a>(&'a mut self, height: u16) -> &'a mut Self {
        self.0.layer_ui_height = height as _;
        self
    }

    pub fn preview_ui_width<'a>(&'a mut self, width: u16) -> &'a mut Self {
        self.0.preview_ui_width = width as _;
        self
    }

    pub fn preview_ui_height<'a>(&'a mut self, height: u16) -> &'a mut Self {
        self.0.preview_ui_height = height as _;
        self
    }

    pub fn finalize(self) -> Self {
        self
    }
}

pub struct InteractCallbacks(InDataHandle);

impl InteractCallbacks {
    pub fn new(in_data: InDataHandle) -> Self {
        Self(in_data)
    }

    pub fn register_ui(
        &self,
        custom_ui_info: CustomUIInfo,
    ) -> Result<(), Error> {
        match unsafe {
            (*self.0.as_ptr()).inter.register_ui.unwrap()(
                (*self.0.as_ptr()).effect_ref,
                custom_ui_info.as_ptr() as _,
            )
        } as u32
        {
            ae_sys::PF_Err_NONE => Ok(()),
            e => Err(unsafe { Error::from_unchecked(e as i32) }),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum ParamType {
    Reserved = ae_sys::PF_Param_RESERVED,
    Layer = ae_sys::PF_Param_LAYER,
    Slider = ae_sys::PF_Param_SLIDER,
    FixSlider = ae_sys::PF_Param_FIX_SLIDER,
    Angle = ae_sys::PF_Param_ANGLE,
    CheckBox = ae_sys::PF_Param_CHECKBOX,
    Color = ae_sys::PF_Param_COLOR,
    Point = ae_sys::PF_Param_POINT,
    PopUp = ae_sys::PF_Param_POPUP,
    Custom = ae_sys::PF_Param_CUSTOM,
    NoData = ae_sys::PF_Param_NO_DATA,
    FloatSlider = ae_sys::PF_Param_FLOAT_SLIDER,
    ArbitraryData = ae_sys::PF_Param_ARBITRARY_DATA,
    Path = ae_sys::PF_Param_PATH,
    GroupStart = ae_sys::PF_Param_GROUP_START,
    GroupEnd = ae_sys::PF_Param_GROUP_END,
    Button = ae_sys::PF_Param_BUTTON,
    Reserved2 = ae_sys::PF_Param_RESERVED2,
    Reserved3 = ae_sys::PF_Param_RESERVED3,
    Point3D = ae_sys::PF_Param_POINT_3D,
}

bitflags! {
    pub struct ParamUIFlags: u32 {
        const NONE = ae_sys::PF_PUI_NONE;
        /// Effect has custom UI and wants events for this params' title (portion visible when twirled up).
        const TOPIC = ae_sys::PF_PUI_TOPIC;
        /// Effect has custom UI and wants events for this params' control (portion invisible when twirled up).
        const CONTROL = ae_sys::PF_PUI_CONTROL;
        // Param will be used as UI only, no data (new in AE 4.0).
        const CONTROL_ONLY = ae_sys::PF_PUI_STD_CONTROL_ONLY;
        // stop param from appearing in Effect Controls (which in PPro also means you won't see a keyframe track there)
        const NO_ECW_UI = ae_sys::PF_PUI_NO_ECW_UI;
        // draw a thick separating line above this param; not used by AE
        const ECW_SEPARATOR = ae_sys::PF_PUI_ECW_SEPARATOR;
        // disable (gray-out) UI for this param
        const DISABLED = ae_sys::PF_PUI_DISABLED;
        // AE will not erase the ECW topic, it's up to the FX to erase/draw every pixel.
        // Handy if FX author implements an offscreen, prevents flashing.
        const DONT_ERASE_TOPIC = ae_sys::PF_PUI_DONT_ERASE_TOPIC;
        const DONT_ERASE_CONTROL = ae_sys::PF_PUI_DONT_ERASE_CONTROL;
        /// Display as a radio-button group; only valid for PF_Param_POPUP; ignored by Ae.
        const RADIO_BUTTON = ae_sys::PF_PUI_RADIO_BUTTON;
        /// In Ae as of CS6, this hides the parameter UI in both the Effect Controls and Timeline.
        /// in Premiere since earlier than that, this hides the parameter UI in the Effect Controls,
        ///	which includes the keyframe track; for PPro only, the flag is dynamic and can be cleared
        ///	to make the parameter visible again.
        const INVISIBLE = ae_sys::PF_PUI_INVISIBLE;
    }
}

bitflags! {
    pub struct ParamFlag: u32 {
        const RESERVED1 = ae_sys::PF_ParamFlag_RESERVED1;
        const CANNOT_TIME_VARY = ae_sys::PF_ParamFlag_CANNOT_TIME_VARY;
        const CANNOT_INTERP= ae_sys::PF_ParamFlag_CANNOT_INTERP;
        const RESERVED2= ae_sys::PF_ParamFlag_RESERVED2;
        const RESERVED3 = ae_sys::PF_ParamFlag_RESERVED3;
        const TWIRLY = ae_sys::PF_ParamFlag_COLLAPSE_TWIRLY;
        const SUPERVISE = ae_sys::PF_ParamFlag_SUPERVISE;
        const START_COLLAPSED = ae_sys::PF_ParamFlag_START_COLLAPSED;
        const USE_VALUE_FOR_OLD_PROJECTS = ae_sys::PF_ParamFlag_USE_VALUE_FOR_OLD_PROJECTS;
        const LAYER_PARAM_IS_TRACKMATTE = ae_sys::PF_ParamFlag_LAYER_PARAM_IS_TRACKMATTE;
        const EXCLUDE_FROM_HAVE_INPUTS_CHANGED = ae_sys::PF_ParamFlag_EXCLUDE_FROM_HAVE_INPUTS_CHANGED;
        const SKIP_REVEAL_WHEN_UNHIDDEN = ae_sys::PF_ParamFlag_SKIP_REVEAL_WHEN_UNHIDDEN;
    }
}

bitflags! {
    pub struct ValueDisplayFlag: u16 {
        const NONE = ae_sys::PF_ValueDisplayFlag_NONE as u16;
        const PERCENT = ae_sys::PF_ValueDisplayFlag_PERCENT as u16;
        const PIXEL = ae_sys::PF_ValueDisplayFlag_PIXEL as u16;
        const RESERVED = ae_sys::PF_ValueDisplayFlag_RESERVED1 as u16;
        const REVERSE = ae_sys::PF_ValueDisplayFlag_REVERSE as u16;
    }
}

//define_param_wrapper!(ButtonDef, PF_ButtonDef, button_def);

#[repr(C)]
#[derive(Clone)]
pub struct ButtonDef {
    button_def: ae_sys::PF_ButtonDef,
    label: CString,
}

//define_param_value_str_wrapper!(ButtonDef, button_def);
//define_param_value_desc_wrapper!(ButtonDef, button_def);

impl ButtonDef {
    pub fn new() -> Self {
        Self {
            button_def: unsafe {
                std::mem::MaybeUninit::zeroed().assume_init()
            },
            label: CString::new("").unwrap(),
        }
    }

    pub fn label<'a>(&'a mut self, label: &str) -> &'a mut Self {
        self.label = CString::new(label).unwrap();
        self.button_def.u.namesptr = self.label.as_ptr();
        self
    }

    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_BUTTON == param.param_def_boxed.param_type {
            Some(Self {
                button_def: unsafe { param.param_def_boxed.u.button_d },
                label: unsafe {
                    CString::from_raw(
                        param.param_def_boxed.u.button_d.u.namesptr as _,
                    )
                },
            })
        } else {
            None
        }
    }

    pub fn into_raw(def: ButtonDef) -> ae_sys::PF_ButtonDef {
        def.button_def
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct PopupDef(ae_sys::PF_PopupDef, CString);

use crate::ae_sys::PF_PopupDef;
define_param_basic_wrapper!(PopupDef, PF_PopupDef, i32, u16);
//define_param_value_str_wrapper!(PopupDef, popup_def);
//define_param_value_desc_wrapper!(PopupDef, popup_def);

impl PopupDef {
    pub fn new() -> Self {
        Self(
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
            CString::new("").unwrap(),
        )
    }

    pub fn names<'a>(&'a mut self, names: Vec<&str>) -> &'a mut Self {
        // Build a string in the format "list|of|choices|", the
        // format Ae expects. Ae ignores the trailing '|'.
        let mut names_tmp = String::new();
        names
            .iter()
            .for_each(|s| names_tmp += format!("{}|", *s).as_str());
        self.1 = CString::new(names_tmp).unwrap();
        self.0.u.namesptr = self.1.as_ptr();
        self.0.num_choices = names.len().try_into().unwrap();
        self
    }

    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_POPUP == param.param_def_boxed.param_type {
            Some(Self(
                unsafe { param.param_def_boxed.u.pd },
                CString::new("").unwrap(),
            ))
        } else {
            None
        }
    }

    //pub fn check_out()

    pub fn get(&self) -> u16 {
        self.0.value as u16
    }
}

use crate::ae_sys::PF_AngleDef;
define_param_wrapper!(AngleDef, PF_AngleDef);
define_param_basic_wrapper!(AngleDef, PF_AngleDef, i32, i32);
//define_param_value_str_wrapper!(AngleDef, angle_def);
//define_param_value_desc_wrapper!(AngleDef, angle_def);

impl AngleDef {
    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_ANGLE == param.param_def_boxed.param_type {
            Some(Self(unsafe { param.param_def_boxed.u.ad }))
        } else {
            None
        }
    }

    pub fn get(&self) -> i32 {
        self.0.value
    }
}

define_param_wrapper!(ColorDef, PF_ColorDef);

impl ColorDef {
    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_COLOR == param.param_def_boxed.param_type {
            Some(Self(unsafe { param.param_def_boxed.u.cd }))
        } else {
            None
        }
    }

    pub fn get(&self) -> Pixel {
        Pixel::from(self.0.value)
    }

    pub fn value<'a>(&'a mut self, value: Pixel) -> &'a mut Self {
        self.0.value = ae_sys::PF_Pixel::from(value);
        self
    }

    pub fn default<'a>(&'a mut self, default: Pixel) -> &'a mut Self {
        self.0.dephault = ae_sys::PF_Pixel::from(default);
        self
    }

    pub fn into_raw(def: ColorDef) -> ae_sys::PF_ColorDef {
        def.0
    }
}

use crate::ae_sys::PF_SliderDef;
define_param_wrapper!(SliderDef, PF_SliderDef);
define_param_basic_wrapper!(SliderDef, PF_SliderDef, i32, i32);
define_param_valid_min_max_wrapper!(SliderDef, i32);
define_param_slider_min_max_wrapper!(SliderDef, i32);
define_param_value_str_wrapper!(SliderDef);
define_param_value_desc_wrapper!(SliderDef);

impl SliderDef {
    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_SLIDER == param.param_def_boxed.param_type {
            Some(Self(unsafe { param.param_def_boxed.u.sd }))
        } else {
            None
        }
    }

    pub fn get(&self) -> i32 {
        self.0.value
    }
}
/* Adobe recommends not useing fixed (point) sliders any more and
 * instead to use float sliders instead.

    define_param_wrapper!(FixedSliderDef, PF_FixedSliderDef, slider_def);
    define_param_basic_wrapper!(FixedSliderDef, PF_FixedSliderDef, slider_def, i32, i32);
    define_param_slider_min_max_wrapper!(FixedSliderDef, PF_FixedSliderDef, slider_def, i32);
    define_param_value_str_wrapper!(FixedSliderDef, slider_def);
    define_param_value_desc_wrapper!(FixedSliderDef, slider_def);

    impl FixedSliderDef {
        pub fn precision<'a>(&'a mut self, precision: u16) -> &'a mut FixedSliderDef {
            self.slider_def.precision = precision as i16;
            self
        }

        pub fn display_flags<'a>(&'a mut self, display_flags: ValueDisplayFlag) -> &'a mut FixedSliderDef {
            self.slider_def.display_flags = display_flags.bits() as i16;
            self
        }

 *
}*/

// Float Slider
use crate::ae_sys::PF_FloatSliderDef;
define_param_wrapper!(FloatSliderDef, PF_FloatSliderDef);
define_param_basic_wrapper!(FloatSliderDef, PF_FloatSliderDef, f64, f32);
define_param_valid_min_max_wrapper!(FloatSliderDef, f32);
define_param_slider_min_max_wrapper!(FloatSliderDef, f32);
define_param_value_desc_wrapper!(FloatSliderDef);

impl FloatSliderDef {
    pub fn display_flags<'a>(
        &'a mut self,
        display_flags: ValueDisplayFlag,
    ) -> &'a mut Self {
        self.0.display_flags = display_flags.bits() as i16;
        self
    }

    pub fn precision<'a>(&'a mut self, precision: u8) -> &'a mut Self {
        self.0.precision = precision as i16;
        self
    }

    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_FLOAT_SLIDER == param.param_def_boxed.param_type {
            Some(Self(unsafe { param.param_def_boxed.u.fs_d }))
        } else {
            None
        }
    }

    pub fn get(&self) -> f64 {
        self.0.value
    }
}

// Checkbox

// PF_CheckBoxDef does not implement Debug trait so we can't use
// the define_param_basic_wrapper!() macro.
#[repr(C)]
#[derive(Clone)]
pub struct CheckBoxDef(ae_sys::PF_CheckBoxDef, CString);

impl CheckBoxDef {
    pub fn new() -> Self {
        Self(
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
            CString::new("").unwrap(),
        )
    }

    pub fn label<'a>(&'a mut self, label: &str) -> &'a mut Self {
        self.1 = CString::new(label).unwrap();
        self.0.u.nameptr = self.1.as_ptr();
        self
    }

    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_CHECKBOX == param.param_def_boxed.param_type {
            Some(Self(unsafe { param.param_def_boxed.u.bd }, unsafe {
                CString::from_raw(param.param_def_boxed.u.bd.u.nameptr as _)
            }))
        } else {
            None
        }
    }

    pub fn get(&self) -> bool {
        self.0.value != 0
    }
}

use crate::ae_sys::PF_CheckBoxDef;
define_param_basic_wrapper!(CheckBoxDef, PF_CheckBoxDef, i32, bool);

pub struct ArbitraryDef(ae_sys::PF_ArbitraryDef);

impl ArbitraryDef {
    pub fn new() -> Self {
        Self(unsafe { std::mem::MaybeUninit::zeroed().assume_init() })
    }

    pub fn into_raw(def: ArbitraryDef) -> ae_sys::PF_ArbitraryDef {
        def.0
    }

    pub fn finalize(self) -> Self {
        self
    }

    /*pub fn value<T: Any>(mut self, data: T) -> Self {
        self.1 = Box::new(data);
        self
    }*/

    pub fn default(mut self, handle: FlatHandle) -> Self {
        self.0.dephault = FlatHandle::into_raw(handle);
        self
    }

    pub fn refcon(mut self, refcon: usize) -> Self {
        self.0.refconPV = refcon as _;
        self
    }

    pub fn has_refcon(&self, refcon: usize) -> bool {
        self.0.refconPV == refcon as _
    }
}

pub trait ArbitraryData<T> {
    fn default() -> T;
    fn interpolate(&self, other: &T, value: f64) -> T;
}

define_struct_wrapper!(ArbParamsExtra, PF_ArbParamsExtra);

impl ArbParamsExtra {
    pub fn id(&self) -> isize {
        self.0.id as _
    }

    pub fn refcon(&self) -> usize {
        unsafe { std::mem::transmute(self.0.u.new_func_params.refconPV) }
    }

    pub fn which_function(&self) -> u32 {
        self.0.which_function as _
    }

    pub fn dispatch<
        T: ArbitraryData<T>
            + DeserializeOwned
            + Serialize
            + PartialEq
            + PartialOrd,
    >(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self.0.which_function as _ {
            ae_sys::PF_Arbitrary_NEW_FUNC => unsafe {
                //println!("NEW_FUNC");
                // Create a new instance, serialize it to a Vec<u8>
                // pass it to a FlatHandle and turn that into a raw
                // ae handle that we stash in the PF_ArbParamsExtra
                // struct wrapper.
                self.0.u.new_func_params.arbPH.write(FlatHandle::into_raw(
                    FlatHandle::new(serde_cbor::to_vec(&T::default())?)?,
                ));
            },

            ae_sys::PF_Arbitrary_DISPOSE_FUNC => {
                //println!("DISPOSE_FUNC");
                // Create a new handle from the raw Ae handle. This
                // disposes then handle when it goes out of scope
                // and is dropped just after.
                FlatHandle::from_raw(unsafe {
                    self.0.u.dispose_func_params.arbH
                })?;
            }

            ae_sys::PF_Arbitrary_COPY_FUNC => unsafe {
                //println!("COPY_FUNC");
                // Create a new handle wraper from the sources,
                // get a referece to that as a slice create a new
                // handle from that and write that to the
                // destination pointer.
                let src =
                    FlatHandle::from_raw(self.0.u.copy_func_params.src_arbH)?;

                self.0.u.copy_func_params.dst_arbPH.write(
                    FlatHandle::into_raw(FlatHandle::new(src.as_slice())?),
                );

                // Make sure we do not drop the source handle.
                FlatHandle::into_raw(src);
            },

            ae_sys::PF_Arbitrary_FLAT_SIZE_FUNC => unsafe {
                let handle =
                    FlatHandle::from_raw(self.0.u.flat_size_func_params.arbH)?;

                self.0
                    .u
                    .flat_size_func_params
                    .flat_data_sizePLu
                    .write(handle.size() as _);

                // Make sure we do not drop the source handle.
                FlatHandle::into_raw(handle);
            },

            ae_sys::PF_Arbitrary_FLATTEN_FUNC => {
                let handle = FlatHandle::from_raw(unsafe {
                    self.0.u.flatten_func_params.arbH
                })?;

                debug_assert!(
                    handle.size()
                        <= unsafe { self.0.u.flatten_func_params.buf_sizeLu }
                            as _
                );

                unsafe {
                    std::ptr::copy_nonoverlapping(
                        handle.as_ptr(),
                        self.0.u.flatten_func_params.flat_dataPV as _,
                        handle.size(),
                    );
                }

                // Make sure we do not drop the handle.
                FlatHandle::into_raw(handle);
            }

            ae_sys::PF_Arbitrary_UNFLATTEN_FUNC => unsafe {
                self.0.u.unflatten_func_params.arbPH.write(
                    FlatHandle::into_raw(FlatHandle::new(CVec::<u8>::new(
                        self.0.u.unflatten_func_params.flat_dataPV as *mut u8,
                        self.0.u.unflatten_func_params.buf_sizeLu as _,
                    ))?),
                );
            },

            ae_sys::PF_Arbitrary_INTERP_FUNC => unsafe {
                let left = FlatHandle::from_raw(
                    self.0.u.interp_func_params.left_arbH,
                )?;
                let right = FlatHandle::from_raw(
                    self.0.u.interp_func_params.right_arbH,
                )?;

                self.0.u.interp_func_params.interpPH.write(
                    FlatHandle::into_raw(FlatHandle::new(serde_cbor::to_vec(
                        &serde_cbor::from_slice::<T>(&left.as_slice())?
                            .interpolate(
                                &serde_cbor::from_slice::<T>(
                                    &right.as_slice(),
                                )?,
                                self.0.u.interp_func_params.tF,
                            ),
                    )?)?),
                );

                // Make sure we do not drop the handles.
                FlatHandle::into_raw(left);
                FlatHandle::into_raw(right);
            },

            ae_sys::PF_Arbitrary_COMPARE_FUNC => {
                let handle_a = FlatHandle::from_raw(unsafe {
                    self.0.u.compare_func_params.a_arbH
                })?;
                let a = serde_cbor::from_slice::<T>(&handle_a.as_slice())?;
                // Make sure we do not drop the handle.
                FlatHandle::into_raw(handle_a);

                let handle_b = FlatHandle::from_raw(unsafe {
                    self.0.u.compare_func_params.b_arbH
                })?;
                let b = serde_cbor::from_slice::<T>(&handle_b.as_slice())?;
                // Make sure we do not drop the handle.
                FlatHandle::into_raw(handle_b);

                if a < b {
                    unsafe {
                        self.0
                            .u
                            .compare_func_params
                            .compareP
                            .write(ae_sys::PF_ArbCompare_LESS as _);
                    }
                } else if a > b {
                    unsafe {
                        self.0
                            .u
                            .compare_func_params
                            .compareP
                            .write(ae_sys::PF_ArbCompare_MORE as _);
                    }
                } else if a == b {
                    unsafe {
                        self.0
                            .u
                            .compare_func_params
                            .compareP
                            .write(ae_sys::PF_ArbCompare_EQUAL as _);
                    }
                } else {
                    unsafe {
                        self.0
                            .u
                            .compare_func_params
                            .compareP
                            .write(ae_sys::PF_ArbCompare_NOT_EQUAL as _);
                    }
                }
            }

            ae_sys::PF_Arbitrary_PRINT_SIZE_FUNC => unsafe {
                let handle =
                    FlatHandle::from_raw(self.0.u.print_size_func_params.arbH)?;

                self.0.u.print_size_func_params.print_sizePLu.write(
                    (serde_json::to_string(&serde_cbor::from_slice::<T>(
                        &handle.as_slice(),
                    )?)?
                    // The len + terminating nul byte.
                    .len()
                        + 1) as _,
                );

                // Make sure we do not drop the handle.
                FlatHandle::into_raw(handle);
            },

            // Print arbitrary data into a string as JSON.
            // Note that we could use any text-based serializer here.
            ae_sys::PF_Arbitrary_PRINT_FUNC => {
                let handle = FlatHandle::from_raw(unsafe {
                    self.0.u.print_func_params.arbH
                })?;
                let string =
                    serde_json::to_string(&serde_cbor::from_slice::<T>(
                        &handle.as_slice(),
                    )?)?;

                if string.len() + 1
                    <= unsafe { self.0.u.print_func_params.print_sizeLu } as _
                    && unsafe { self.0.u.print_func_params.print_flags } == 0
                {
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            string.as_ptr(),
                            self.0.u.print_func_params.print_bufferPC as _,
                            string.len(),
                        );
                        // Nul-terminate the C string.
                        self.0
                            .u
                            .print_func_params
                            .print_bufferPC
                            .offset(string.len() as _)
                            .write(0);
                    }
                }

                // Make sure we do not drop the handle.
                FlatHandle::into_raw(handle);
            }
            ae_sys::PF_Arbitrary_SCAN_FUNC => unsafe {
                self.0.u.scan_func_params.arbPH.write(FlatHandle::into_raw(
                    FlatHandle::new(serde_cbor::to_vec::<T>(
                        &serde_json::from_str(
                            CStr::from_ptr(self.0.u.scan_func_params.bufPC)
                                .to_str()?,
                        )?,
                    )?)?,
                ));
            },
            _ => {
                return Err(Box::new(Error::Generic));
            }
        }
        Ok(())
    }
}

#[repr(C)]
//#[derive(Clone)]
pub enum ParamDefUnion {
    PopupDef(PopupDef),
    AngleDef(AngleDef),
    CheckBoxDef(CheckBoxDef),
    SliderDef(SliderDef),
    FloatSliderDef(FloatSliderDef),
    ColorDef(ColorDef),
    ButtonDef(ButtonDef),
    //FixedSliderDef(FixedSliderDef),
    ArbitraryDef(ArbitraryDef),
}

#[derive(Clone)]
#[repr(C)]
pub struct ParamDef {
    param_def_boxed: std::mem::ManuallyDrop<Box<ae_sys::PF_ParamDef>>,
    drop: bool,
    in_data_ptr: *const ae_sys::PF_InData,
}

impl ParamDef {
    pub fn new(in_data_handle: InDataHandle) -> Self {
        Self {
            param_def_boxed: std::mem::ManuallyDrop::new(unsafe {
                Box::new_zeroed().assume_init()
            }),
            drop: true,
            in_data_ptr: in_data_handle.as_ptr(),
        }
    }

    pub fn from_raw(
        in_data_ptr: *const ae_sys::PF_InData,
        param_def: *mut ae_sys::PF_ParamDef,
    ) -> Self {
        Self {
            param_def_boxed: unsafe {
                std::mem::ManuallyDrop::new(Box::from_raw(param_def))
            },
            drop: false,
            in_data_ptr,
        }
    }

    pub fn add(&mut self, index: i32) {
        unsafe {
            (*self.in_data_ptr).inter.add_param.unwrap()(
                (*self.in_data_ptr).effect_ref,
                index,
                &mut **self.param_def_boxed,
            );
        }
        // Parameters we just added are not checked out
        // so they do not need to be checked in.
        self.drop = false;
    }

    pub fn checkout(
        in_data_handle: InDataHandle,
        index: i32,
        what_time: i32,
        time_step: i32,
        time_scale: u32,
    ) -> ParamDef {
        let mut param_def_boxed = std::mem::ManuallyDrop::new(unsafe {
            Box::<ae_sys::PF_ParamDef>::new_zeroed().assume_init()
        });
        let in_data_ptr = in_data_handle.as_ptr();
        unsafe {
            (*in_data_ptr).inter.checkout_param.unwrap()(
                (*in_data_ptr).effect_ref,
                index,
                what_time,
                time_step,
                time_scale,
                &mut **param_def_boxed,
            );
        }
        ParamDef {
            param_def_boxed,
            drop: true,
            in_data_ptr,
        }
    }

    pub fn do_not_checkin(&mut self) {
        self.drop = false;
    }

    pub fn param<'a>(&'a mut self, param: ParamDefUnion) -> &'a mut ParamDef {
        match param {
            ParamDefUnion::PopupDef(pd) => {
                self.param_def_boxed.u.pd = PopupDef::into_raw(pd);
                self.param_def_boxed.param_type = ae_sys::PF_Param_POPUP;
            }
            ParamDefUnion::AngleDef(ad) => {
                self.param_def_boxed.u.ad = AngleDef::into_raw(ad);
                self.param_def_boxed.param_type = ae_sys::PF_Param_ANGLE;
            }
            ParamDefUnion::CheckBoxDef(bd) => {
                self.param_def_boxed.u.bd = CheckBoxDef::into_raw(bd);
                self.param_def_boxed.param_type = ae_sys::PF_Param_BUTTON;
            }
            ParamDefUnion::ColorDef(cd) => {
                self.param_def_boxed.u.cd = ColorDef::into_raw(cd);
                self.param_def_boxed.param_type = ae_sys::PF_Param_COLOR;
            }
            ParamDefUnion::SliderDef(sd) => {
                self.param_def_boxed.u.sd = SliderDef::into_raw(sd);
                self.param_def_boxed.param_type = ae_sys::PF_Param_SLIDER;
            }
            ParamDefUnion::FloatSliderDef(fs_d) => {
                self.param_def_boxed.u.fs_d = FloatSliderDef::into_raw(fs_d);
                self.param_def_boxed.param_type = ae_sys::PF_Param_FLOAT_SLIDER;
            } /*ParamDefUnion::FixedSliderDef(sd) => {
            self.param_def_boxed.u.fd = FixedSliderDef::into_raw(sd);
            self.param_def_boxed.param_type = ae_sys::PF_Param_FIX_SLIDER;
            }*/
            ParamDefUnion::ButtonDef(button_d) => {
                self.param_def_boxed.u.button_d = ButtonDef::into_raw(button_d);
                self.param_def_boxed.param_type = ae_sys::PF_Param_BUTTON;
            }
            ParamDefUnion::ArbitraryDef(arb_d) => {
                self.param_def_boxed.u.arb_d = ArbitraryDef::into_raw(arb_d);
                self.param_def_boxed.param_type =
                    ae_sys::PF_Param_ARBITRARY_DATA;
            }
        }
        self
    }

    pub fn param_type<'a>(
        &'a mut self,
        param_type: ParamType,
    ) -> &'a mut ParamDef {
        self.param_def_boxed.param_type = param_type as i32;
        self
    }

    pub fn name<'a>(&'a mut self, name: &str) -> &'a mut ParamDef {
        assert!(name.len() < 32);
        let name_cstr = CString::new(name).unwrap();
        let name_slice = name_cstr.to_bytes_with_nul();
        self.param_def_boxed.name[0..name_slice.len()]
            .copy_from_slice(unsafe { std::mem::transmute(name_slice) });
        self
    }

    pub fn ui_flags<'a>(&'a mut self, flags: ParamUIFlags) -> &'a mut ParamDef {
        self.param_def_boxed.ui_flags = flags.bits() as _;
        self
    }

    pub fn ui_width<'a>(&'a mut self, width: u16) -> &'a mut ParamDef {
        self.param_def_boxed.ui_width = width as _;
        self
    }

    pub fn ui_height<'a>(&'a mut self, height: u16) -> &'a mut ParamDef {
        self.param_def_boxed.ui_height = height as _;
        self
    }

    pub fn flags<'a>(&'a mut self, flags: ParamFlag) -> &'a mut ParamDef {
        self.param_def_boxed.flags = flags.bits() as _;
        self
    }
}

impl Drop for ParamDef {
    fn drop(&mut self) {
        if self.drop {
            unsafe {
                (*self.in_data_ptr).inter.checkin_param.unwrap()(
                    (*self.in_data_ptr).effect_ref,
                    // ManuallyDrop ensures we do not double free the memory
                    // after passing the pointer to the box contents to the FFI.
                    &mut **self.param_def_boxed,
                );
            }
        }
        unsafe {
            std::mem::ManuallyDrop::drop(&mut self.param_def_boxed);
        }
    }
}

define_handle_wrapper!(ContextHandle, PF_ContextH);

define_suite!(
    EffectCustomUISuite,
    PF_EffectCustomUISuite1,
    kPFEffectCustomUISuite,
    kPFEffectCustomUISuiteVersion1
);

impl EffectCustomUISuite {
    pub fn get_drawing_reference(
        &self,
        context_handle: &ContextHandle,
    ) -> Result<drawbot::DrawRef, Error> {
        let mut draw_reference =
            std::mem::MaybeUninit::<ae_sys::DRAWBOT_DrawRef>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            PF_GetDrawingReference,
            context_handle.as_ptr(),
            draw_reference.as_mut_ptr()
        ) {
            Ok(()) => Ok(drawbot::DrawRef::from_raw(unsafe {
                draw_reference.assume_init()
            })),
            Err(e) => Err(e),
        }
    }
}

define_suite!(
    EffectCustomUIOverlayThemeSuite,
    PF_EffectCustomUIOverlayThemeSuite1,
    kPFEffectCustomUIOverlayThemeSuite,
    kPFEffectCustomUIOverlayThemeSuiteVersion1
);

impl EffectCustomUIOverlayThemeSuite {
    pub fn get_preferred_foreground_color(
        &self,
    ) -> Result<drawbot::ColorRGBA, Error> {
        let mut color = std::mem::MaybeUninit::<drawbot::ColorRGBA>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            PF_GetPreferredForegroundColor,
            color.as_mut_ptr() as _,
        ) {
            Ok(()) => Ok(unsafe { color.assume_init() }),
            Err(e) => Err(e),
        }
    }

    //PF_GetPreferredShadowColor
    //PF_GetPreferredShadowOffset

    pub fn get_preferred_stroke_width(&self) -> Result<f32, Error> {
        let mut width = std::mem::MaybeUninit::<f32>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            PF_GetPreferredStrokeWidth,
            width.as_mut_ptr(),
        ) {
            Ok(()) => Ok(unsafe { width.assume_init() }),
            Err(e) => Err(e),
        }
    }

    pub fn get_preferred_vertex_size(&self) -> Result<f32, Error> {
        let mut size = std::mem::MaybeUninit::<f32>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            PF_GetPreferredVertexSize,
            size.as_mut_ptr(),
        ) {
            Ok(()) => Ok(unsafe { size.assume_init() }),
            Err(e) => Err(e),
        }
    }
}

/*
pub enum GameState {
    FreePlacement(FreePlacement),
    Play(PlayState),
    Scoring(ScoringState),
    Done(ScoringState),
}

assume!(GameState);
assume!(GameState, Play(x) => x, PlayState);
assume!(GameState, Scoring(x) => x, ScoringState);
assume!(GameState, FreePlacement(x) => x, FreePlacement);

// and then I can:

state.assume::<PlayState>()
*/

pub trait AssumeFrom<T> {
    fn assume(x: &T) -> &Self;
    fn assume_mut(x: &mut T) -> &mut Self;
}

#[macro_export]
macro_rules! assume {
    ($owner:ident, $var:pat => $out:expr, $ty:ty) => {
        impl AssumeFrom<$owner> for $ty {
            fn assume(x: &$owner) -> &$ty {
                use $owner::*;
                match x {
                    $var => $out,
                    _ => panic!(
                        concat!(
                            "Assumed ",
                            stringify!($var),
                            " but was in {:?}"
                        ),
                        x
                    ),
                }
            }

            fn assume_mut(x: &mut $owner) -> &mut $ty {
                use $owner::*;
                match x {
                    $var => $out,
                    _ => panic!(
                        concat!(
                            "Assumed ",
                            stringify!($var),
                            " but was in {:?}"
                        ),
                        x
                    ),
                }
            }
        }
    };
    ($owner:ident) => {
        impl $owner {
            pub fn assume<T: AssumeFrom<Self>>(&self) -> &T {
                T::assume(self)
            }

            pub fn assume_mut<T: AssumeFrom<Self>>(&mut self) -> &mut T {
                T::assume_mut(self)
            }
        }
    };
}
