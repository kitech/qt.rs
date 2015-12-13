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
  // proto:  unsigned int QColormap::pixel(const QColor & color);
  fn _ZNK9QColormap5pixelERK6QColor(qthis: *mut c_void, arg0: *mut c_void) -> c_uint;
  // proto:  const QVector<QColor> QColormap::colormap();
  fn _ZNK9QColormap8colormapEv(qthis: *mut c_void) ;
  // proto:  const QColor QColormap::colorAt(uint pixel);
  fn _ZNK9QColormap7colorAtEj(qthis: *mut c_void, arg0: c_uint) -> *mut c_void;
  // proto:  void QColormap::FreeQColormap();
  fn _ZN9QColormapD0Ev(qthis: *mut c_void) ;
  // proto:  void QColormap::NewQColormap();
  fn _ZN9QColormapC1Ev(qthis: *mut c_void) ;
  // proto: static QColormap QColormap::instance(int screen);
  fn _ZN9QColormap8instanceEi(arg0: c_int) -> *mut c_void;
  // proto:  int QColormap::size();
  fn _ZNK9QColormap4sizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QColormap::NewQColormap(const QColormap & colormap);
  fn _ZN9QColormapC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static void QColormap::initialize();
  fn _ZN9QColormap10initializeEv() ;
  // proto:  int QColormap::depth();
  fn _ZNK9QColormap5depthEv(qthis: *mut c_void) -> c_int;
  // proto: static void QColormap::cleanup();
  fn _ZN9QColormap7cleanupEv() ;
}

// body block begin
// class sizeof(QColormap)=8
pub struct QColormap {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QColormap {
  pub fn pixel<T: QColormap_pixel>(&mut self, value: T) -> u32 {
    return value.pixel(self);
    // return 1;
  }
}

pub trait QColormap_pixel {
  fn pixel(self, rsthis: &mut QColormap) -> u32;
}

// proto:  unsigned int QColormap::pixel(const QColor & color);
impl<'a> /*trait*/ QColormap_pixel for (&'a  QColor) {
  fn pixel(self, rsthis: &mut QColormap) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QColormap5pixelERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QColormap5pixelERK6QColor(rsthis.qclsinst, arg0)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QColormap {
  pub fn colormap<T: QColormap_colormap>(&mut self, value: T)  {
     value.colormap(self);
    // return 1;
  }
}

pub trait QColormap_colormap {
  fn colormap(self, rsthis: &mut QColormap) ;
}

// proto:  const QVector<QColor> QColormap::colormap();
impl<'a> /*trait*/ QColormap_colormap for () {
  fn colormap(self, rsthis: &mut QColormap)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QColormap8colormapEv()};
     unsafe {_ZNK9QColormap8colormapEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QColormap {
  pub fn colorAt<T: QColormap_colorAt>(&mut self, value: T) -> QColor {
    return value.colorAt(self);
    // return 1;
  }
}

pub trait QColormap_colorAt {
  fn colorAt(self, rsthis: &mut QColormap) -> QColor;
}

// proto:  const QColor QColormap::colorAt(uint pixel);
impl<'a> /*trait*/ QColormap_colorAt for (u32) {
  fn colorAt(self, rsthis: &mut QColormap) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QColormap7colorAtEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZNK9QColormap7colorAtEj(rsthis.qclsinst, arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColormap {
  pub fn FreeQColormap<T: QColormap_FreeQColormap>(&mut self, value: T)  {
     value.FreeQColormap(self);
    // return 1;
  }
}

pub trait QColormap_FreeQColormap {
  fn FreeQColormap(self, rsthis: &mut QColormap) ;
}

// proto:  void QColormap::FreeQColormap();
impl<'a> /*trait*/ QColormap_FreeQColormap for () {
  fn FreeQColormap(self, rsthis: &mut QColormap)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QColormapD0Ev()};
     unsafe {_ZN9QColormapD0Ev(rsthis.qclsinst)};
    // return 1;
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
  pub fn instance<T: QColormap_instance>(&mut self, value: T) -> QColormap {
    return value.instance(self);
    // return 1;
  }
}

pub trait QColormap_instance {
  fn instance(self, rsthis: &mut QColormap) -> QColormap;
}

// proto: static QColormap QColormap::instance(int screen);
impl<'a> /*trait*/ QColormap_instance for (i32) {
  fn instance(self, rsthis: &mut QColormap) -> QColormap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QColormap8instanceEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN9QColormap8instanceEi(arg0)};
    let mut ret1 = QColormap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColormap {
  pub fn size<T: QColormap_size>(&mut self, value: T) -> i32 {
    return value.size(self);
    // return 1;
  }
}

pub trait QColormap_size {
  fn size(self, rsthis: &mut QColormap) -> i32;
}

// proto:  int QColormap::size();
impl<'a> /*trait*/ QColormap_size for () {
  fn size(self, rsthis: &mut QColormap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QColormap4sizeEv()};
    let mut ret = unsafe {_ZNK9QColormap4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QColormap::NewQColormap(const QColormap & colormap);
impl<'a> /*trait*/ QColormap_NewQColormap for (&'a  QColormap) {
  fn NewQColormap(self) -> QColormap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QColormapC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QColormapC1ERKS_(qthis, arg0)};
    let rsthis = QColormap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QColormap {
  pub fn initialize<T: QColormap_initialize>(&mut self, value: T)  {
     value.initialize(self);
    // return 1;
  }
}

pub trait QColormap_initialize {
  fn initialize(self, rsthis: &mut QColormap) ;
}

// proto: static void QColormap::initialize();
impl<'a> /*trait*/ QColormap_initialize for () {
  fn initialize(self, rsthis: &mut QColormap)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QColormap10initializeEv()};
     unsafe {_ZN9QColormap10initializeEv()};
    // return 1;
  }
}

impl /*struct*/ QColormap {
  pub fn depth<T: QColormap_depth>(&mut self, value: T) -> i32 {
    return value.depth(self);
    // return 1;
  }
}

pub trait QColormap_depth {
  fn depth(self, rsthis: &mut QColormap) -> i32;
}

// proto:  int QColormap::depth();
impl<'a> /*trait*/ QColormap_depth for () {
  fn depth(self, rsthis: &mut QColormap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QColormap5depthEv()};
    let mut ret = unsafe {_ZNK9QColormap5depthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QColormap {
  pub fn cleanup<T: QColormap_cleanup>(&mut self, value: T)  {
     value.cleanup(self);
    // return 1;
  }
}

pub trait QColormap_cleanup {
  fn cleanup(self, rsthis: &mut QColormap) ;
}

// proto: static void QColormap::cleanup();
impl<'a> /*trait*/ QColormap_cleanup for () {
  fn cleanup(self, rsthis: &mut QColormap)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QColormap7cleanupEv()};
     unsafe {_ZN9QColormap7cleanupEv()};
    // return 1;
  }
}

