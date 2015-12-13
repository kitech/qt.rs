// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qcolor::QColor;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: unsigned int QColormap::pixel(const QColor & color);
  fn _ZNK9QColormap5pixelERK6QColor(arg0: *const c_void) -> i32;
  // proto: const QVector<QColor> QColormap::colormap();
  fn _ZNK9QColormap8colormapEv() -> i32;
  // proto: const QColor QColormap::colorAt(uint pixel);
  fn _ZNK9QColormap7colorAtEj(arg0: c_uint) -> i32;
  // proto: void QColormap::FreeQColormap();
  fn _ZN9QColormapD0Ev() -> i32;
  // proto: void QColormap::NewQColormap();
  fn _ZN9QColormapC1Ev(qthis: *mut c_void) -> i32;
  // proto: QColormap QColormap::instance(int screen);
  fn _ZN9QColormap8instanceEi(arg0: c_int) -> i32;
  // proto: int QColormap::size();
  fn _ZNK9QColormap4sizeEv() -> i32;
  // proto: void QColormap::NewQColormap(const QColormap & colormap);
  fn _ZN9QColormapC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QColormap::initialize();
  fn _ZN9QColormap10initializeEv() -> i32;
  // proto: int QColormap::depth();
  fn _ZNK9QColormap5depthEv() -> i32;
  // proto: void QColormap::cleanup();
  fn _ZN9QColormap7cleanupEv() -> i32;
}

// body block begin
// class sizeof(QColormap)=8
pub struct QColormap {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QColormap {
  pub fn pixel<T: QColormap_pixel>(&mut self, value: T) -> i32 {
    value.pixel(self);
    return 1;
  }
}

pub trait QColormap_pixel {
  fn pixel(self, this: &mut QColormap) -> i32;
}

// proto: unsigned int QColormap::pixel(const QColor & color);
impl<'a> /*trait*/ QColormap_pixel for (&'a  QColor) {
  fn pixel(self, this: &mut QColormap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QColormap5pixelERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QColormap5pixelERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QColormap {
  pub fn colormap<T: QColormap_colormap>(&mut self, value: T) -> i32 {
    value.colormap(self);
    return 1;
  }
}

pub trait QColormap_colormap {
  fn colormap(self, this: &mut QColormap) -> i32;
}

// proto: const QVector<QColor> QColormap::colormap();
impl<'a> /*trait*/ QColormap_colormap for () {
  fn colormap(self, this: &mut QColormap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QColormap8colormapEv()};
    unsafe {_ZNK9QColormap8colormapEv()};
    return 1;
  }
}

impl /*struct*/ QColormap {
  pub fn colorAt<T: QColormap_colorAt>(&mut self, value: T) -> i32 {
    value.colorAt(self);
    return 1;
  }
}

pub trait QColormap_colorAt {
  fn colorAt(self, this: &mut QColormap) -> i32;
}

// proto: const QColor QColormap::colorAt(uint pixel);
impl<'a> /*trait*/ QColormap_colorAt for (u32) {
  fn colorAt(self, this: &mut QColormap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QColormap7colorAtEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZNK9QColormap7colorAtEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QColormap {
  pub fn FreeQColormap<T: QColormap_FreeQColormap>(&mut self, value: T) -> i32 {
    value.FreeQColormap(self);
    return 1;
  }
}

pub trait QColormap_FreeQColormap {
  fn FreeQColormap(self, this: &mut QColormap) -> i32;
}

// proto: void QColormap::FreeQColormap();
impl<'a> /*trait*/ QColormap_FreeQColormap for () {
  fn FreeQColormap(self, this: &mut QColormap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QColormapD0Ev()};
    unsafe {_ZN9QColormapD0Ev()};
    return 1;
  }
}

impl /*struct*/ QColormap {
  pub fn NewQColormap<T: QColormap_NewQColormap>(value: T) -> QColormap {
    let rsthis = value.NewQColormap();
    return rsthis;
    // return 1;
  }
}

pub trait QColormap_NewQColormap {
  fn NewQColormap(self) -> QColormap;
}

// proto: void QColormap::NewQColormap();
impl<'a> /*trait*/ QColormap_NewQColormap for () {
  fn NewQColormap(self) -> QColormap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QColormapC1Ev()};
    unsafe {_ZN9QColormapC1Ev(qthis)};
    let rsthis = QColormap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QColormap {
  pub fn instance<T: QColormap_instance>(&mut self, value: T) -> i32 {
    value.instance(self);
    return 1;
  }
}

pub trait QColormap_instance {
  fn instance(self, this: &mut QColormap) -> i32;
}

// proto: QColormap QColormap::instance(int screen);
impl<'a> /*trait*/ QColormap_instance for (i32) {
  fn instance(self, this: &mut QColormap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QColormap8instanceEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QColormap8instanceEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QColormap {
  pub fn size<T: QColormap_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QColormap_size {
  fn size(self, this: &mut QColormap) -> i32;
}

// proto: int QColormap::size();
impl<'a> /*trait*/ QColormap_size for () {
  fn size(self, this: &mut QColormap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QColormap4sizeEv()};
    unsafe {_ZNK9QColormap4sizeEv()};
    return 1;
  }
}

// proto: void QColormap::NewQColormap(const QColormap & colormap);
impl<'a> /*trait*/ QColormap_NewQColormap for (&'a  QColormap) {
  fn NewQColormap(self) -> QColormap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QColormapC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QColormapC1ERKS_(qthis, arg0)};
    let rsthis = QColormap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QColormap {
  pub fn initialize<T: QColormap_initialize>(&mut self, value: T) -> i32 {
    value.initialize(self);
    return 1;
  }
}

pub trait QColormap_initialize {
  fn initialize(self, this: &mut QColormap) -> i32;
}

// proto: void QColormap::initialize();
impl<'a> /*trait*/ QColormap_initialize for () {
  fn initialize(self, this: &mut QColormap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QColormap10initializeEv()};
    unsafe {_ZN9QColormap10initializeEv()};
    return 1;
  }
}

impl /*struct*/ QColormap {
  pub fn depth<T: QColormap_depth>(&mut self, value: T) -> i32 {
    value.depth(self);
    return 1;
  }
}

pub trait QColormap_depth {
  fn depth(self, this: &mut QColormap) -> i32;
}

// proto: int QColormap::depth();
impl<'a> /*trait*/ QColormap_depth for () {
  fn depth(self, this: &mut QColormap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QColormap5depthEv()};
    unsafe {_ZNK9QColormap5depthEv()};
    return 1;
  }
}

impl /*struct*/ QColormap {
  pub fn cleanup<T: QColormap_cleanup>(&mut self, value: T) -> i32 {
    value.cleanup(self);
    return 1;
  }
}

pub trait QColormap_cleanup {
  fn cleanup(self, this: &mut QColormap) -> i32;
}

// proto: void QColormap::cleanup();
impl<'a> /*trait*/ QColormap_cleanup for () {
  fn cleanup(self, this: &mut QColormap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QColormap7cleanupEv()};
    unsafe {_ZN9QColormap7cleanupEv()};
    return 1;
  }
}

