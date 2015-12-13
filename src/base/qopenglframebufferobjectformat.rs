// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QOpenGLFramebufferObjectFormat::FreeQOpenGLFramebufferObjectFormat();
  fn _ZN30QOpenGLFramebufferObjectFormatD0Ev() -> i32;
  // proto: void QOpenGLFramebufferObjectFormat::setTextureTarget(GLenum target);
  fn _ZN30QOpenGLFramebufferObjectFormat16setTextureTargetEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFramebufferObjectFormat::setInternalTextureFormat(GLenum internalTextureFormat);
  fn _ZN30QOpenGLFramebufferObjectFormat24setInternalTextureFormatEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFramebufferObjectFormat::NewQOpenGLFramebufferObjectFormat(const QOpenGLFramebufferObjectFormat & other);
  fn _ZN30QOpenGLFramebufferObjectFormatC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QOpenGLFramebufferObjectFormat::mipmap();
  fn _ZNK30QOpenGLFramebufferObjectFormat6mipmapEv() -> i32;
  // proto: QOpenGLFramebufferObjectFormat::GLenum QOpenGLFramebufferObjectFormat::textureTarget();
  fn _ZNK30QOpenGLFramebufferObjectFormat13textureTargetEv() -> i32;
  // proto: void QOpenGLFramebufferObjectFormat::setSamples(int samples);
  fn _ZN30QOpenGLFramebufferObjectFormat10setSamplesEi(arg0: c_int) -> i32;
  // proto: void QOpenGLFramebufferObjectFormat::NewQOpenGLFramebufferObjectFormat();
  fn _ZN30QOpenGLFramebufferObjectFormatC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QOpenGLFramebufferObjectFormat::setMipmap(bool enabled);
  fn _ZN30QOpenGLFramebufferObjectFormat9setMipmapEb(arg0: int8_t) -> i32;
  // proto: QOpenGLFramebufferObjectFormat::GLenum QOpenGLFramebufferObjectFormat::internalTextureFormat();
  fn _ZNK30QOpenGLFramebufferObjectFormat21internalTextureFormatEv() -> i32;
  // proto: int QOpenGLFramebufferObjectFormat::samples();
  fn _ZNK30QOpenGLFramebufferObjectFormat7samplesEv() -> i32;
}

// body block begin
// class sizeof(QOpenGLFramebufferObjectFormat)=8
pub struct QOpenGLFramebufferObjectFormat {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn FreeQOpenGLFramebufferObjectFormat<T: QOpenGLFramebufferObjectFormat_FreeQOpenGLFramebufferObjectFormat>(&mut self, value: T) -> i32 {
    value.FreeQOpenGLFramebufferObjectFormat(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObjectFormat_FreeQOpenGLFramebufferObjectFormat {
  fn FreeQOpenGLFramebufferObjectFormat(self, this: &mut QOpenGLFramebufferObjectFormat) -> i32;
}

// proto: void QOpenGLFramebufferObjectFormat::FreeQOpenGLFramebufferObjectFormat();
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_FreeQOpenGLFramebufferObjectFormat for () {
  fn FreeQOpenGLFramebufferObjectFormat(self, this: &mut QOpenGLFramebufferObjectFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QOpenGLFramebufferObjectFormatD0Ev()};
    unsafe {_ZN30QOpenGLFramebufferObjectFormatD0Ev()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn setTextureTarget<T: QOpenGLFramebufferObjectFormat_setTextureTarget>(&mut self, value: T) -> i32 {
    value.setTextureTarget(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObjectFormat_setTextureTarget {
  fn setTextureTarget(self, this: &mut QOpenGLFramebufferObjectFormat) -> i32;
}

// proto: void QOpenGLFramebufferObjectFormat::setTextureTarget(GLenum target);
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_setTextureTarget for (u32) {
  fn setTextureTarget(self, this: &mut QOpenGLFramebufferObjectFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QOpenGLFramebufferObjectFormat16setTextureTargetEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN30QOpenGLFramebufferObjectFormat16setTextureTargetEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn setInternalTextureFormat<T: QOpenGLFramebufferObjectFormat_setInternalTextureFormat>(&mut self, value: T) -> i32 {
    value.setInternalTextureFormat(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObjectFormat_setInternalTextureFormat {
  fn setInternalTextureFormat(self, this: &mut QOpenGLFramebufferObjectFormat) -> i32;
}

// proto: void QOpenGLFramebufferObjectFormat::setInternalTextureFormat(GLenum internalTextureFormat);
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_setInternalTextureFormat for (u32) {
  fn setInternalTextureFormat(self, this: &mut QOpenGLFramebufferObjectFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QOpenGLFramebufferObjectFormat24setInternalTextureFormatEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN30QOpenGLFramebufferObjectFormat24setInternalTextureFormatEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn NewQOpenGLFramebufferObjectFormat<T: QOpenGLFramebufferObjectFormat_NewQOpenGLFramebufferObjectFormat>(value: T) -> QOpenGLFramebufferObjectFormat {
    let rsthis = value.NewQOpenGLFramebufferObjectFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFramebufferObjectFormat_NewQOpenGLFramebufferObjectFormat {
  fn NewQOpenGLFramebufferObjectFormat(self) -> QOpenGLFramebufferObjectFormat;
}

// proto: void QOpenGLFramebufferObjectFormat::NewQOpenGLFramebufferObjectFormat(const QOpenGLFramebufferObjectFormat & other);
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_NewQOpenGLFramebufferObjectFormat for (&'a  QOpenGLFramebufferObjectFormat) {
  fn NewQOpenGLFramebufferObjectFormat(self) -> QOpenGLFramebufferObjectFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QOpenGLFramebufferObjectFormatC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN30QOpenGLFramebufferObjectFormatC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLFramebufferObjectFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn mipmap<T: QOpenGLFramebufferObjectFormat_mipmap>(&mut self, value: T) -> i32 {
    value.mipmap(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObjectFormat_mipmap {
  fn mipmap(self, this: &mut QOpenGLFramebufferObjectFormat) -> i32;
}

// proto: bool QOpenGLFramebufferObjectFormat::mipmap();
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_mipmap for () {
  fn mipmap(self, this: &mut QOpenGLFramebufferObjectFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK30QOpenGLFramebufferObjectFormat6mipmapEv()};
    unsafe {_ZNK30QOpenGLFramebufferObjectFormat6mipmapEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn textureTarget<T: QOpenGLFramebufferObjectFormat_textureTarget>(&mut self, value: T) -> i32 {
    value.textureTarget(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObjectFormat_textureTarget {
  fn textureTarget(self, this: &mut QOpenGLFramebufferObjectFormat) -> i32;
}

// proto: QOpenGLFramebufferObjectFormat::GLenum QOpenGLFramebufferObjectFormat::textureTarget();
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_textureTarget for () {
  fn textureTarget(self, this: &mut QOpenGLFramebufferObjectFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK30QOpenGLFramebufferObjectFormat13textureTargetEv()};
    unsafe {_ZNK30QOpenGLFramebufferObjectFormat13textureTargetEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn setSamples<T: QOpenGLFramebufferObjectFormat_setSamples>(&mut self, value: T) -> i32 {
    value.setSamples(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObjectFormat_setSamples {
  fn setSamples(self, this: &mut QOpenGLFramebufferObjectFormat) -> i32;
}

// proto: void QOpenGLFramebufferObjectFormat::setSamples(int samples);
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_setSamples for (i32) {
  fn setSamples(self, this: &mut QOpenGLFramebufferObjectFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QOpenGLFramebufferObjectFormat10setSamplesEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN30QOpenGLFramebufferObjectFormat10setSamplesEi(arg0)};
    return 1;
  }
}

// proto: void QOpenGLFramebufferObjectFormat::NewQOpenGLFramebufferObjectFormat();
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_NewQOpenGLFramebufferObjectFormat for () {
  fn NewQOpenGLFramebufferObjectFormat(self) -> QOpenGLFramebufferObjectFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QOpenGLFramebufferObjectFormatC1Ev()};
    unsafe {_ZN30QOpenGLFramebufferObjectFormatC1Ev(qthis)};
    let rsthis = QOpenGLFramebufferObjectFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn setMipmap<T: QOpenGLFramebufferObjectFormat_setMipmap>(&mut self, value: T) -> i32 {
    value.setMipmap(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObjectFormat_setMipmap {
  fn setMipmap(self, this: &mut QOpenGLFramebufferObjectFormat) -> i32;
}

// proto: void QOpenGLFramebufferObjectFormat::setMipmap(bool enabled);
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_setMipmap for (i8) {
  fn setMipmap(self, this: &mut QOpenGLFramebufferObjectFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QOpenGLFramebufferObjectFormat9setMipmapEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN30QOpenGLFramebufferObjectFormat9setMipmapEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn internalTextureFormat<T: QOpenGLFramebufferObjectFormat_internalTextureFormat>(&mut self, value: T) -> i32 {
    value.internalTextureFormat(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObjectFormat_internalTextureFormat {
  fn internalTextureFormat(self, this: &mut QOpenGLFramebufferObjectFormat) -> i32;
}

// proto: QOpenGLFramebufferObjectFormat::GLenum QOpenGLFramebufferObjectFormat::internalTextureFormat();
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_internalTextureFormat for () {
  fn internalTextureFormat(self, this: &mut QOpenGLFramebufferObjectFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK30QOpenGLFramebufferObjectFormat21internalTextureFormatEv()};
    unsafe {_ZNK30QOpenGLFramebufferObjectFormat21internalTextureFormatEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn samples<T: QOpenGLFramebufferObjectFormat_samples>(&mut self, value: T) -> i32 {
    value.samples(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObjectFormat_samples {
  fn samples(self, this: &mut QOpenGLFramebufferObjectFormat) -> i32;
}

// proto: int QOpenGLFramebufferObjectFormat::samples();
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_samples for () {
  fn samples(self, this: &mut QOpenGLFramebufferObjectFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK30QOpenGLFramebufferObjectFormat7samplesEv()};
    unsafe {_ZNK30QOpenGLFramebufferObjectFormat7samplesEv()};
    return 1;
  }
}

