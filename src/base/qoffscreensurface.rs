// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qscreen::QScreen;
use super::qsurfaceformat::QSurfaceFormat;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QOffscreenSurface::FreeQOffscreenSurface();
  fn _ZN17QOffscreenSurfaceD0Ev() -> i32;
  // proto: void QOffscreenSurface::screenChanged(QScreen * screen);
  fn _ZN17QOffscreenSurface13screenChangedEP7QScreen(arg0: *mut c_void) -> i32;
  // proto: void QOffscreenSurface::NewQOffscreenSurface(const QOffscreenSurface & );
  fn _ZN17QOffscreenSurfaceC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QOffscreenSurface::NewQOffscreenSurface(QScreen * screen);
  fn _ZN17QOffscreenSurfaceC1EP7QScreen(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QScreen * QOffscreenSurface::screen();
  fn _ZNK17QOffscreenSurface6screenEv() -> i32;
  // proto: void QOffscreenSurface::setFormat(const QSurfaceFormat & format);
  fn _ZN17QOffscreenSurface9setFormatERK14QSurfaceFormat(arg0: *const c_void) -> i32;
  // proto: void QOffscreenSurface::setScreen(QScreen * screen);
  fn _ZN17QOffscreenSurface9setScreenEP7QScreen(arg0: *mut c_void) -> i32;
  // proto: QSurfaceFormat QOffscreenSurface::requestedFormat();
  fn _ZNK17QOffscreenSurface15requestedFormatEv() -> i32;
  // proto: QSurfaceFormat QOffscreenSurface::format();
  fn _ZNK17QOffscreenSurface6formatEv() -> i32;
  // proto: QPlatformOffscreenSurface * QOffscreenSurface::handle();
  fn _ZNK17QOffscreenSurface6handleEv() -> i32;
  // proto: const QMetaObject * QOffscreenSurface::metaObject();
  fn _ZNK17QOffscreenSurface10metaObjectEv() -> i32;
  // proto: void QOffscreenSurface::destroy();
  fn _ZN17QOffscreenSurface7destroyEv() -> i32;
  // proto: bool QOffscreenSurface::isValid();
  fn _ZNK17QOffscreenSurface7isValidEv() -> i32;
  // proto: QSize QOffscreenSurface::size();
  fn _ZNK17QOffscreenSurface4sizeEv() -> i32;
  // proto: void QOffscreenSurface::create();
  fn _ZN17QOffscreenSurface6createEv() -> i32;
}

// body block begin
// class sizeof(QOffscreenSurface)=1
pub struct QOffscreenSurface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOffscreenSurface {
  pub fn FreeQOffscreenSurface<T: QOffscreenSurface_FreeQOffscreenSurface>(&mut self, value: T) -> i32 {
    value.FreeQOffscreenSurface(self);
    return 1;
  }
}

pub trait QOffscreenSurface_FreeQOffscreenSurface {
  fn FreeQOffscreenSurface(self, this: &mut QOffscreenSurface) -> i32;
}

// proto: void QOffscreenSurface::FreeQOffscreenSurface();
impl<'a> /*trait*/ QOffscreenSurface_FreeQOffscreenSurface for () {
  fn FreeQOffscreenSurface(self, this: &mut QOffscreenSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurfaceD0Ev()};
    unsafe {_ZN17QOffscreenSurfaceD0Ev()};
    return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn screenChanged<T: QOffscreenSurface_screenChanged>(&mut self, value: T) -> i32 {
    value.screenChanged(self);
    return 1;
  }
}

pub trait QOffscreenSurface_screenChanged {
  fn screenChanged(self, this: &mut QOffscreenSurface) -> i32;
}

// proto: void QOffscreenSurface::screenChanged(QScreen * screen);
impl<'a> /*trait*/ QOffscreenSurface_screenChanged for (&'a mut QScreen) {
  fn screenChanged(self, this: &mut QOffscreenSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurface13screenChangedEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QOffscreenSurface13screenChangedEP7QScreen(arg0)};
    return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn NewQOffscreenSurface<T: QOffscreenSurface_NewQOffscreenSurface>(value: T) -> QOffscreenSurface {
    let rsthis = value.NewQOffscreenSurface();
    return rsthis;
    // return 1;
  }
}

pub trait QOffscreenSurface_NewQOffscreenSurface {
  fn NewQOffscreenSurface(self) -> QOffscreenSurface;
}

// proto: void QOffscreenSurface::NewQOffscreenSurface(const QOffscreenSurface & );
impl<'a> /*trait*/ QOffscreenSurface_NewQOffscreenSurface for (&'a  QOffscreenSurface) {
  fn NewQOffscreenSurface(self) -> QOffscreenSurface {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurfaceC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QOffscreenSurfaceC1ERKS_(qthis, arg0)};
    let rsthis = QOffscreenSurface{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QOffscreenSurface::NewQOffscreenSurface(QScreen * screen);
impl<'a> /*trait*/ QOffscreenSurface_NewQOffscreenSurface for (&'a mut QScreen) {
  fn NewQOffscreenSurface(self) -> QOffscreenSurface {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurfaceC1EP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QOffscreenSurfaceC1EP7QScreen(qthis, arg0)};
    let rsthis = QOffscreenSurface{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn screen<T: QOffscreenSurface_screen>(&mut self, value: T) -> i32 {
    value.screen(self);
    return 1;
  }
}

pub trait QOffscreenSurface_screen {
  fn screen(self, this: &mut QOffscreenSurface) -> i32;
}

// proto: QScreen * QOffscreenSurface::screen();
impl<'a> /*trait*/ QOffscreenSurface_screen for () {
  fn screen(self, this: &mut QOffscreenSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface6screenEv()};
    unsafe {_ZNK17QOffscreenSurface6screenEv()};
    return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn setFormat<T: QOffscreenSurface_setFormat>(&mut self, value: T) -> i32 {
    value.setFormat(self);
    return 1;
  }
}

pub trait QOffscreenSurface_setFormat {
  fn setFormat(self, this: &mut QOffscreenSurface) -> i32;
}

// proto: void QOffscreenSurface::setFormat(const QSurfaceFormat & format);
impl<'a> /*trait*/ QOffscreenSurface_setFormat for (&'a  QSurfaceFormat) {
  fn setFormat(self, this: &mut QOffscreenSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurface9setFormatERK14QSurfaceFormat()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QOffscreenSurface9setFormatERK14QSurfaceFormat(arg0)};
    return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn setScreen<T: QOffscreenSurface_setScreen>(&mut self, value: T) -> i32 {
    value.setScreen(self);
    return 1;
  }
}

pub trait QOffscreenSurface_setScreen {
  fn setScreen(self, this: &mut QOffscreenSurface) -> i32;
}

// proto: void QOffscreenSurface::setScreen(QScreen * screen);
impl<'a> /*trait*/ QOffscreenSurface_setScreen for (&'a mut QScreen) {
  fn setScreen(self, this: &mut QOffscreenSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurface9setScreenEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QOffscreenSurface9setScreenEP7QScreen(arg0)};
    return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn requestedFormat<T: QOffscreenSurface_requestedFormat>(&mut self, value: T) -> i32 {
    value.requestedFormat(self);
    return 1;
  }
}

pub trait QOffscreenSurface_requestedFormat {
  fn requestedFormat(self, this: &mut QOffscreenSurface) -> i32;
}

// proto: QSurfaceFormat QOffscreenSurface::requestedFormat();
impl<'a> /*trait*/ QOffscreenSurface_requestedFormat for () {
  fn requestedFormat(self, this: &mut QOffscreenSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface15requestedFormatEv()};
    unsafe {_ZNK17QOffscreenSurface15requestedFormatEv()};
    return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn format<T: QOffscreenSurface_format>(&mut self, value: T) -> i32 {
    value.format(self);
    return 1;
  }
}

pub trait QOffscreenSurface_format {
  fn format(self, this: &mut QOffscreenSurface) -> i32;
}

// proto: QSurfaceFormat QOffscreenSurface::format();
impl<'a> /*trait*/ QOffscreenSurface_format for () {
  fn format(self, this: &mut QOffscreenSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface6formatEv()};
    unsafe {_ZNK17QOffscreenSurface6formatEv()};
    return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn handle<T: QOffscreenSurface_handle>(&mut self, value: T) -> i32 {
    value.handle(self);
    return 1;
  }
}

pub trait QOffscreenSurface_handle {
  fn handle(self, this: &mut QOffscreenSurface) -> i32;
}

// proto: QPlatformOffscreenSurface * QOffscreenSurface::handle();
impl<'a> /*trait*/ QOffscreenSurface_handle for () {
  fn handle(self, this: &mut QOffscreenSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface6handleEv()};
    unsafe {_ZNK17QOffscreenSurface6handleEv()};
    return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn metaObject<T: QOffscreenSurface_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QOffscreenSurface_metaObject {
  fn metaObject(self, this: &mut QOffscreenSurface) -> i32;
}

// proto: const QMetaObject * QOffscreenSurface::metaObject();
impl<'a> /*trait*/ QOffscreenSurface_metaObject for () {
  fn metaObject(self, this: &mut QOffscreenSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface10metaObjectEv()};
    unsafe {_ZNK17QOffscreenSurface10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn destroy<T: QOffscreenSurface_destroy>(&mut self, value: T) -> i32 {
    value.destroy(self);
    return 1;
  }
}

pub trait QOffscreenSurface_destroy {
  fn destroy(self, this: &mut QOffscreenSurface) -> i32;
}

// proto: void QOffscreenSurface::destroy();
impl<'a> /*trait*/ QOffscreenSurface_destroy for () {
  fn destroy(self, this: &mut QOffscreenSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurface7destroyEv()};
    unsafe {_ZN17QOffscreenSurface7destroyEv()};
    return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn isValid<T: QOffscreenSurface_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QOffscreenSurface_isValid {
  fn isValid(self, this: &mut QOffscreenSurface) -> i32;
}

// proto: bool QOffscreenSurface::isValid();
impl<'a> /*trait*/ QOffscreenSurface_isValid for () {
  fn isValid(self, this: &mut QOffscreenSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface7isValidEv()};
    unsafe {_ZNK17QOffscreenSurface7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn size<T: QOffscreenSurface_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QOffscreenSurface_size {
  fn size(self, this: &mut QOffscreenSurface) -> i32;
}

// proto: QSize QOffscreenSurface::size();
impl<'a> /*trait*/ QOffscreenSurface_size for () {
  fn size(self, this: &mut QOffscreenSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface4sizeEv()};
    unsafe {_ZNK17QOffscreenSurface4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn create<T: QOffscreenSurface_create>(&mut self, value: T) -> i32 {
    value.create(self);
    return 1;
  }
}

pub trait QOffscreenSurface_create {
  fn create(self, this: &mut QOffscreenSurface) -> i32;
}

// proto: void QOffscreenSurface::create();
impl<'a> /*trait*/ QOffscreenSurface_create for () {
  fn create(self, this: &mut QOffscreenSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurface6createEv()};
    unsafe {_ZN17QOffscreenSurface6createEv()};
    return 1;
  }
}

