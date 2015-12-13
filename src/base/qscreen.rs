// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrect::QRect;
use super::qsizef::QSizeF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: double QScreen::logicalDotsPerInchY();
  fn _ZNK7QScreen19logicalDotsPerInchYEv() -> i32;
  // proto: QRect QScreen::geometry();
  fn _ZNK7QScreen8geometryEv() -> i32;
  // proto: QPixmap QScreen::grabWindow(WId window, int x, int y, int w, int h);
  fn _ZN7QScreen10grabWindowEiiiii(arg0: *mut c_uint, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int) -> i32;
  // proto: QSize QScreen::size();
  fn _ZNK7QScreen4sizeEv() -> i32;
  // proto: QSizeF QScreen::physicalSize();
  fn _ZNK7QScreen12physicalSizeEv() -> i32;
  // proto: QPlatformScreen * QScreen::handle();
  fn _ZNK7QScreen6handleEv() -> i32;
  // proto: QRect QScreen::availableVirtualGeometry();
  fn _ZNK7QScreen24availableVirtualGeometryEv() -> i32;
  // proto: void QScreen::FreeQScreen();
  fn _ZN7QScreenD0Ev() -> i32;
  // proto: QSize QScreen::virtualSize();
  fn _ZNK7QScreen11virtualSizeEv() -> i32;
  // proto: double QScreen::devicePixelRatio();
  fn _ZNK7QScreen16devicePixelRatioEv() -> i32;
  // proto: QList<QScreen *> QScreen::virtualSiblings();
  fn _ZNK7QScreen15virtualSiblingsEv() -> i32;
  // proto: void QScreen::NewQScreen(const QScreen & );
  fn _ZN7QScreenC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QScreen::logicalDotsPerInchChanged(qreal dpi);
  fn _ZN7QScreen25logicalDotsPerInchChangedEd(arg0: c_double) -> i32;
  // proto: void QScreen::availableGeometryChanged(const QRect & geometry);
  fn _ZN7QScreen24availableGeometryChangedERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QScreen::geometryChanged(const QRect & geometry);
  fn _ZN7QScreen15geometryChangedERK5QRect(arg0: *const c_void) -> i32;
  // proto: QRect QScreen::virtualGeometry();
  fn _ZNK7QScreen15virtualGeometryEv() -> i32;
  // proto: double QScreen::logicalDotsPerInch();
  fn _ZNK7QScreen18logicalDotsPerInchEv() -> i32;
  // proto: double QScreen::physicalDotsPerInch();
  fn _ZNK7QScreen19physicalDotsPerInchEv() -> i32;
  // proto: void QScreen::physicalDotsPerInchChanged(qreal dpi);
  fn _ZN7QScreen26physicalDotsPerInchChangedEd(arg0: c_double) -> i32;
  // proto: double QScreen::refreshRate();
  fn _ZNK7QScreen11refreshRateEv() -> i32;
  // proto: const QMetaObject * QScreen::metaObject();
  fn _ZNK7QScreen10metaObjectEv() -> i32;
  // proto: void QScreen::refreshRateChanged(qreal refreshRate);
  fn _ZN7QScreen18refreshRateChangedEd(arg0: c_double) -> i32;
  // proto: QSize QScreen::availableSize();
  fn _ZNK7QScreen13availableSizeEv() -> i32;
  // proto: QString QScreen::name();
  fn _ZNK7QScreen4nameEv() -> i32;
  // proto: QSize QScreen::availableVirtualSize();
  fn _ZNK7QScreen20availableVirtualSizeEv() -> i32;
  // proto: double QScreen::logicalDotsPerInchX();
  fn _ZNK7QScreen19logicalDotsPerInchXEv() -> i32;
  // proto: void QScreen::virtualGeometryChanged(const QRect & rect);
  fn _ZN7QScreen22virtualGeometryChangedERK5QRect(arg0: *const c_void) -> i32;
  // proto: QRect QScreen::availableGeometry();
  fn _ZNK7QScreen17availableGeometryEv() -> i32;
  // proto: double QScreen::physicalDotsPerInchX();
  fn _ZNK7QScreen20physicalDotsPerInchXEv() -> i32;
  // proto: void QScreen::physicalSizeChanged(const QSizeF & size);
  fn _ZN7QScreen19physicalSizeChangedERK6QSizeF(arg0: *const c_void) -> i32;
  // proto: double QScreen::physicalDotsPerInchY();
  fn _ZNK7QScreen20physicalDotsPerInchYEv() -> i32;
  // proto: int QScreen::depth();
  fn _ZNK7QScreen5depthEv() -> i32;
}

// body block begin
// class sizeof(QScreen)=1
pub struct QScreen {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QScreen {
  pub fn logicalDotsPerInchY<T: QScreen_logicalDotsPerInchY>(&mut self, value: T) -> i32 {
    value.logicalDotsPerInchY(self);
    return 1;
  }
}

pub trait QScreen_logicalDotsPerInchY {
  fn logicalDotsPerInchY(self, this: &mut QScreen) -> i32;
}

// proto: double QScreen::logicalDotsPerInchY();
impl<'a> /*trait*/ QScreen_logicalDotsPerInchY for () {
  fn logicalDotsPerInchY(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen19logicalDotsPerInchYEv()};
    unsafe {_ZNK7QScreen19logicalDotsPerInchYEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn geometry<T: QScreen_geometry>(&mut self, value: T) -> i32 {
    value.geometry(self);
    return 1;
  }
}

pub trait QScreen_geometry {
  fn geometry(self, this: &mut QScreen) -> i32;
}

// proto: QRect QScreen::geometry();
impl<'a> /*trait*/ QScreen_geometry for () {
  fn geometry(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen8geometryEv()};
    unsafe {_ZNK7QScreen8geometryEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn grabWindow<T: QScreen_grabWindow>(&mut self, value: T) -> i32 {
    value.grabWindow(self);
    return 1;
  }
}

pub trait QScreen_grabWindow {
  fn grabWindow(self, this: &mut QScreen) -> i32;
}

// proto: QPixmap QScreen::grabWindow(WId window, int x, int y, int w, int h);
impl<'a> /*trait*/ QScreen_grabWindow for (*mut i32, i32, i32, i32, i32) {
  fn grabWindow(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen10grabWindowEiiiii()};
    let arg0 = self.0  as *mut c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    unsafe {_ZN7QScreen10grabWindowEiiiii(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn size<T: QScreen_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QScreen_size {
  fn size(self, this: &mut QScreen) -> i32;
}

// proto: QSize QScreen::size();
impl<'a> /*trait*/ QScreen_size for () {
  fn size(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen4sizeEv()};
    unsafe {_ZNK7QScreen4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn physicalSize<T: QScreen_physicalSize>(&mut self, value: T) -> i32 {
    value.physicalSize(self);
    return 1;
  }
}

pub trait QScreen_physicalSize {
  fn physicalSize(self, this: &mut QScreen) -> i32;
}

// proto: QSizeF QScreen::physicalSize();
impl<'a> /*trait*/ QScreen_physicalSize for () {
  fn physicalSize(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen12physicalSizeEv()};
    unsafe {_ZNK7QScreen12physicalSizeEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn handle<T: QScreen_handle>(&mut self, value: T) -> i32 {
    value.handle(self);
    return 1;
  }
}

pub trait QScreen_handle {
  fn handle(self, this: &mut QScreen) -> i32;
}

// proto: QPlatformScreen * QScreen::handle();
impl<'a> /*trait*/ QScreen_handle for () {
  fn handle(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen6handleEv()};
    unsafe {_ZNK7QScreen6handleEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn availableVirtualGeometry<T: QScreen_availableVirtualGeometry>(&mut self, value: T) -> i32 {
    value.availableVirtualGeometry(self);
    return 1;
  }
}

pub trait QScreen_availableVirtualGeometry {
  fn availableVirtualGeometry(self, this: &mut QScreen) -> i32;
}

// proto: QRect QScreen::availableVirtualGeometry();
impl<'a> /*trait*/ QScreen_availableVirtualGeometry for () {
  fn availableVirtualGeometry(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen24availableVirtualGeometryEv()};
    unsafe {_ZNK7QScreen24availableVirtualGeometryEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn FreeQScreen<T: QScreen_FreeQScreen>(&mut self, value: T) -> i32 {
    value.FreeQScreen(self);
    return 1;
  }
}

pub trait QScreen_FreeQScreen {
  fn FreeQScreen(self, this: &mut QScreen) -> i32;
}

// proto: void QScreen::FreeQScreen();
impl<'a> /*trait*/ QScreen_FreeQScreen for () {
  fn FreeQScreen(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreenD0Ev()};
    unsafe {_ZN7QScreenD0Ev()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn virtualSize<T: QScreen_virtualSize>(&mut self, value: T) -> i32 {
    value.virtualSize(self);
    return 1;
  }
}

pub trait QScreen_virtualSize {
  fn virtualSize(self, this: &mut QScreen) -> i32;
}

// proto: QSize QScreen::virtualSize();
impl<'a> /*trait*/ QScreen_virtualSize for () {
  fn virtualSize(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen11virtualSizeEv()};
    unsafe {_ZNK7QScreen11virtualSizeEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn devicePixelRatio<T: QScreen_devicePixelRatio>(&mut self, value: T) -> i32 {
    value.devicePixelRatio(self);
    return 1;
  }
}

pub trait QScreen_devicePixelRatio {
  fn devicePixelRatio(self, this: &mut QScreen) -> i32;
}

// proto: double QScreen::devicePixelRatio();
impl<'a> /*trait*/ QScreen_devicePixelRatio for () {
  fn devicePixelRatio(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen16devicePixelRatioEv()};
    unsafe {_ZNK7QScreen16devicePixelRatioEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn virtualSiblings<T: QScreen_virtualSiblings>(&mut self, value: T) -> i32 {
    value.virtualSiblings(self);
    return 1;
  }
}

pub trait QScreen_virtualSiblings {
  fn virtualSiblings(self, this: &mut QScreen) -> i32;
}

// proto: QList<QScreen *> QScreen::virtualSiblings();
impl<'a> /*trait*/ QScreen_virtualSiblings for () {
  fn virtualSiblings(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen15virtualSiblingsEv()};
    unsafe {_ZNK7QScreen15virtualSiblingsEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn NewQScreen<T: QScreen_NewQScreen>(value: T) -> QScreen {
    let rsthis = value.NewQScreen();
    return rsthis;
    // return 1;
  }
}

pub trait QScreen_NewQScreen {
  fn NewQScreen(self) -> QScreen;
}

// proto: void QScreen::NewQScreen(const QScreen & );
impl<'a> /*trait*/ QScreen_NewQScreen for (&'a  QScreen) {
  fn NewQScreen(self) -> QScreen {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreenC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QScreenC1ERKS_(qthis, arg0)};
    let rsthis = QScreen{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn logicalDotsPerInchChanged<T: QScreen_logicalDotsPerInchChanged>(&mut self, value: T) -> i32 {
    value.logicalDotsPerInchChanged(self);
    return 1;
  }
}

pub trait QScreen_logicalDotsPerInchChanged {
  fn logicalDotsPerInchChanged(self, this: &mut QScreen) -> i32;
}

// proto: void QScreen::logicalDotsPerInchChanged(qreal dpi);
impl<'a> /*trait*/ QScreen_logicalDotsPerInchChanged for (f64) {
  fn logicalDotsPerInchChanged(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen25logicalDotsPerInchChangedEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN7QScreen25logicalDotsPerInchChangedEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn availableGeometryChanged<T: QScreen_availableGeometryChanged>(&mut self, value: T) -> i32 {
    value.availableGeometryChanged(self);
    return 1;
  }
}

pub trait QScreen_availableGeometryChanged {
  fn availableGeometryChanged(self, this: &mut QScreen) -> i32;
}

// proto: void QScreen::availableGeometryChanged(const QRect & geometry);
impl<'a> /*trait*/ QScreen_availableGeometryChanged for (&'a  QRect) {
  fn availableGeometryChanged(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen24availableGeometryChangedERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QScreen24availableGeometryChangedERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn geometryChanged<T: QScreen_geometryChanged>(&mut self, value: T) -> i32 {
    value.geometryChanged(self);
    return 1;
  }
}

pub trait QScreen_geometryChanged {
  fn geometryChanged(self, this: &mut QScreen) -> i32;
}

// proto: void QScreen::geometryChanged(const QRect & geometry);
impl<'a> /*trait*/ QScreen_geometryChanged for (&'a  QRect) {
  fn geometryChanged(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen15geometryChangedERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QScreen15geometryChangedERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn virtualGeometry<T: QScreen_virtualGeometry>(&mut self, value: T) -> i32 {
    value.virtualGeometry(self);
    return 1;
  }
}

pub trait QScreen_virtualGeometry {
  fn virtualGeometry(self, this: &mut QScreen) -> i32;
}

// proto: QRect QScreen::virtualGeometry();
impl<'a> /*trait*/ QScreen_virtualGeometry for () {
  fn virtualGeometry(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen15virtualGeometryEv()};
    unsafe {_ZNK7QScreen15virtualGeometryEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn logicalDotsPerInch<T: QScreen_logicalDotsPerInch>(&mut self, value: T) -> i32 {
    value.logicalDotsPerInch(self);
    return 1;
  }
}

pub trait QScreen_logicalDotsPerInch {
  fn logicalDotsPerInch(self, this: &mut QScreen) -> i32;
}

// proto: double QScreen::logicalDotsPerInch();
impl<'a> /*trait*/ QScreen_logicalDotsPerInch for () {
  fn logicalDotsPerInch(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen18logicalDotsPerInchEv()};
    unsafe {_ZNK7QScreen18logicalDotsPerInchEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn physicalDotsPerInch<T: QScreen_physicalDotsPerInch>(&mut self, value: T) -> i32 {
    value.physicalDotsPerInch(self);
    return 1;
  }
}

pub trait QScreen_physicalDotsPerInch {
  fn physicalDotsPerInch(self, this: &mut QScreen) -> i32;
}

// proto: double QScreen::physicalDotsPerInch();
impl<'a> /*trait*/ QScreen_physicalDotsPerInch for () {
  fn physicalDotsPerInch(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen19physicalDotsPerInchEv()};
    unsafe {_ZNK7QScreen19physicalDotsPerInchEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn physicalDotsPerInchChanged<T: QScreen_physicalDotsPerInchChanged>(&mut self, value: T) -> i32 {
    value.physicalDotsPerInchChanged(self);
    return 1;
  }
}

pub trait QScreen_physicalDotsPerInchChanged {
  fn physicalDotsPerInchChanged(self, this: &mut QScreen) -> i32;
}

// proto: void QScreen::physicalDotsPerInchChanged(qreal dpi);
impl<'a> /*trait*/ QScreen_physicalDotsPerInchChanged for (f64) {
  fn physicalDotsPerInchChanged(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen26physicalDotsPerInchChangedEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN7QScreen26physicalDotsPerInchChangedEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn refreshRate<T: QScreen_refreshRate>(&mut self, value: T) -> i32 {
    value.refreshRate(self);
    return 1;
  }
}

pub trait QScreen_refreshRate {
  fn refreshRate(self, this: &mut QScreen) -> i32;
}

// proto: double QScreen::refreshRate();
impl<'a> /*trait*/ QScreen_refreshRate for () {
  fn refreshRate(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen11refreshRateEv()};
    unsafe {_ZNK7QScreen11refreshRateEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn metaObject<T: QScreen_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QScreen_metaObject {
  fn metaObject(self, this: &mut QScreen) -> i32;
}

// proto: const QMetaObject * QScreen::metaObject();
impl<'a> /*trait*/ QScreen_metaObject for () {
  fn metaObject(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen10metaObjectEv()};
    unsafe {_ZNK7QScreen10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn refreshRateChanged<T: QScreen_refreshRateChanged>(&mut self, value: T) -> i32 {
    value.refreshRateChanged(self);
    return 1;
  }
}

pub trait QScreen_refreshRateChanged {
  fn refreshRateChanged(self, this: &mut QScreen) -> i32;
}

// proto: void QScreen::refreshRateChanged(qreal refreshRate);
impl<'a> /*trait*/ QScreen_refreshRateChanged for (f64) {
  fn refreshRateChanged(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen18refreshRateChangedEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN7QScreen18refreshRateChangedEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn availableSize<T: QScreen_availableSize>(&mut self, value: T) -> i32 {
    value.availableSize(self);
    return 1;
  }
}

pub trait QScreen_availableSize {
  fn availableSize(self, this: &mut QScreen) -> i32;
}

// proto: QSize QScreen::availableSize();
impl<'a> /*trait*/ QScreen_availableSize for () {
  fn availableSize(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen13availableSizeEv()};
    unsafe {_ZNK7QScreen13availableSizeEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn name<T: QScreen_name>(&mut self, value: T) -> i32 {
    value.name(self);
    return 1;
  }
}

pub trait QScreen_name {
  fn name(self, this: &mut QScreen) -> i32;
}

// proto: QString QScreen::name();
impl<'a> /*trait*/ QScreen_name for () {
  fn name(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen4nameEv()};
    unsafe {_ZNK7QScreen4nameEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn availableVirtualSize<T: QScreen_availableVirtualSize>(&mut self, value: T) -> i32 {
    value.availableVirtualSize(self);
    return 1;
  }
}

pub trait QScreen_availableVirtualSize {
  fn availableVirtualSize(self, this: &mut QScreen) -> i32;
}

// proto: QSize QScreen::availableVirtualSize();
impl<'a> /*trait*/ QScreen_availableVirtualSize for () {
  fn availableVirtualSize(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen20availableVirtualSizeEv()};
    unsafe {_ZNK7QScreen20availableVirtualSizeEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn logicalDotsPerInchX<T: QScreen_logicalDotsPerInchX>(&mut self, value: T) -> i32 {
    value.logicalDotsPerInchX(self);
    return 1;
  }
}

pub trait QScreen_logicalDotsPerInchX {
  fn logicalDotsPerInchX(self, this: &mut QScreen) -> i32;
}

// proto: double QScreen::logicalDotsPerInchX();
impl<'a> /*trait*/ QScreen_logicalDotsPerInchX for () {
  fn logicalDotsPerInchX(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen19logicalDotsPerInchXEv()};
    unsafe {_ZNK7QScreen19logicalDotsPerInchXEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn virtualGeometryChanged<T: QScreen_virtualGeometryChanged>(&mut self, value: T) -> i32 {
    value.virtualGeometryChanged(self);
    return 1;
  }
}

pub trait QScreen_virtualGeometryChanged {
  fn virtualGeometryChanged(self, this: &mut QScreen) -> i32;
}

// proto: void QScreen::virtualGeometryChanged(const QRect & rect);
impl<'a> /*trait*/ QScreen_virtualGeometryChanged for (&'a  QRect) {
  fn virtualGeometryChanged(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen22virtualGeometryChangedERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QScreen22virtualGeometryChangedERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn availableGeometry<T: QScreen_availableGeometry>(&mut self, value: T) -> i32 {
    value.availableGeometry(self);
    return 1;
  }
}

pub trait QScreen_availableGeometry {
  fn availableGeometry(self, this: &mut QScreen) -> i32;
}

// proto: QRect QScreen::availableGeometry();
impl<'a> /*trait*/ QScreen_availableGeometry for () {
  fn availableGeometry(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen17availableGeometryEv()};
    unsafe {_ZNK7QScreen17availableGeometryEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn physicalDotsPerInchX<T: QScreen_physicalDotsPerInchX>(&mut self, value: T) -> i32 {
    value.physicalDotsPerInchX(self);
    return 1;
  }
}

pub trait QScreen_physicalDotsPerInchX {
  fn physicalDotsPerInchX(self, this: &mut QScreen) -> i32;
}

// proto: double QScreen::physicalDotsPerInchX();
impl<'a> /*trait*/ QScreen_physicalDotsPerInchX for () {
  fn physicalDotsPerInchX(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen20physicalDotsPerInchXEv()};
    unsafe {_ZNK7QScreen20physicalDotsPerInchXEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn physicalSizeChanged<T: QScreen_physicalSizeChanged>(&mut self, value: T) -> i32 {
    value.physicalSizeChanged(self);
    return 1;
  }
}

pub trait QScreen_physicalSizeChanged {
  fn physicalSizeChanged(self, this: &mut QScreen) -> i32;
}

// proto: void QScreen::physicalSizeChanged(const QSizeF & size);
impl<'a> /*trait*/ QScreen_physicalSizeChanged for (&'a  QSizeF) {
  fn physicalSizeChanged(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen19physicalSizeChangedERK6QSizeF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QScreen19physicalSizeChangedERK6QSizeF(arg0)};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn physicalDotsPerInchY<T: QScreen_physicalDotsPerInchY>(&mut self, value: T) -> i32 {
    value.physicalDotsPerInchY(self);
    return 1;
  }
}

pub trait QScreen_physicalDotsPerInchY {
  fn physicalDotsPerInchY(self, this: &mut QScreen) -> i32;
}

// proto: double QScreen::physicalDotsPerInchY();
impl<'a> /*trait*/ QScreen_physicalDotsPerInchY for () {
  fn physicalDotsPerInchY(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen20physicalDotsPerInchYEv()};
    unsafe {_ZNK7QScreen20physicalDotsPerInchYEv()};
    return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn depth<T: QScreen_depth>(&mut self, value: T) -> i32 {
    value.depth(self);
    return 1;
  }
}

pub trait QScreen_depth {
  fn depth(self, this: &mut QScreen) -> i32;
}

// proto: int QScreen::depth();
impl<'a> /*trait*/ QScreen_depth for () {
  fn depth(self, this: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen5depthEv()};
    unsafe {_ZNK7QScreen5depthEv()};
    return 1;
  }
}

