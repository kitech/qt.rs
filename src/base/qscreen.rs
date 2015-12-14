// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrect::QRect;
use super::qpixmap::QPixmap;
use super::qsize::QSize;
use super::qsizef::QSizeF;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  double QScreen::logicalDotsPerInchY();
  fn _ZNK7QScreen19logicalDotsPerInchYEv(qthis: *mut c_void) -> c_double;
  // proto:  QRect QScreen::geometry();
  fn _ZNK7QScreen8geometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPixmap QScreen::grabWindow(WId window, int x, int y, int w, int h);
  fn _ZN7QScreen10grabWindowEiiiii(qthis: *mut c_void, arg0: *mut c_uint, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int) -> *mut c_void;
  // proto:  QSize QScreen::size();
  fn _ZNK7QScreen4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSizeF QScreen::physicalSize();
  fn _ZNK7QScreen12physicalSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPlatformScreen * QScreen::handle();
  fn _ZNK7QScreen6handleEv(qthis: *mut c_void) ;
  // proto:  QRect QScreen::availableVirtualGeometry();
  fn _ZNK7QScreen24availableVirtualGeometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QScreen::FreeQScreen();
  fn _ZN7QScreenD0Ev(qthis: *mut c_void) ;
  // proto:  QSize QScreen::virtualSize();
  fn _ZNK7QScreen11virtualSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QScreen::devicePixelRatio();
  fn _ZNK7QScreen16devicePixelRatioEv(qthis: *mut c_void) -> c_double;
  // proto:  QList<QScreen *> QScreen::virtualSiblings();
  fn _ZNK7QScreen15virtualSiblingsEv(qthis: *mut c_void) ;
  // proto:  void QScreen::NewQScreen(const QScreen & );
  fn _ZN7QScreenC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QScreen::logicalDotsPerInchChanged(qreal dpi);
  fn _ZN7QScreen25logicalDotsPerInchChangedEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QScreen::availableGeometryChanged(const QRect & geometry);
  fn _ZN7QScreen24availableGeometryChangedERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QScreen::geometryChanged(const QRect & geometry);
  fn _ZN7QScreen15geometryChangedERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRect QScreen::virtualGeometry();
  fn _ZNK7QScreen15virtualGeometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QScreen::logicalDotsPerInch();
  fn _ZNK7QScreen18logicalDotsPerInchEv(qthis: *mut c_void) -> c_double;
  // proto:  double QScreen::physicalDotsPerInch();
  fn _ZNK7QScreen19physicalDotsPerInchEv(qthis: *mut c_void) -> c_double;
  // proto:  void QScreen::physicalDotsPerInchChanged(qreal dpi);
  fn _ZN7QScreen26physicalDotsPerInchChangedEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QScreen::refreshRate();
  fn _ZNK7QScreen11refreshRateEv(qthis: *mut c_void) -> c_double;
  // proto:  const QMetaObject * QScreen::metaObject();
  fn _ZNK7QScreen10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QScreen::refreshRateChanged(qreal refreshRate);
  fn _ZN7QScreen18refreshRateChangedEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QSize QScreen::availableSize();
  fn _ZNK7QScreen13availableSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QScreen::name();
  fn _ZNK7QScreen4nameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QScreen::availableVirtualSize();
  fn _ZNK7QScreen20availableVirtualSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QScreen::logicalDotsPerInchX();
  fn _ZNK7QScreen19logicalDotsPerInchXEv(qthis: *mut c_void) -> c_double;
  // proto:  void QScreen::virtualGeometryChanged(const QRect & rect);
  fn _ZN7QScreen22virtualGeometryChangedERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRect QScreen::availableGeometry();
  fn _ZNK7QScreen17availableGeometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QScreen::physicalDotsPerInchX();
  fn _ZNK7QScreen20physicalDotsPerInchXEv(qthis: *mut c_void) -> c_double;
  // proto:  void QScreen::physicalSizeChanged(const QSizeF & size);
  fn _ZN7QScreen19physicalSizeChangedERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QScreen::physicalDotsPerInchY();
  fn _ZNK7QScreen20physicalDotsPerInchYEv(qthis: *mut c_void) -> c_double;
  // proto:  int QScreen::depth();
  fn _ZNK7QScreen5depthEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QScreen)=1
pub struct QScreen {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QScreen {
  pub fn logicalDotsPerInchY<T: QScreen_logicalDotsPerInchY>(&mut self, value: T) -> f64 {
    return value.logicalDotsPerInchY(self);
    // return 1;
  }
}

pub trait QScreen_logicalDotsPerInchY {
  fn logicalDotsPerInchY(self, rsthis: &mut QScreen) -> f64;
}

// proto:  double QScreen::logicalDotsPerInchY();
impl<'a> /*trait*/ QScreen_logicalDotsPerInchY for () {
  fn logicalDotsPerInchY(self, rsthis: &mut QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen19logicalDotsPerInchYEv()};
    let mut ret = unsafe {_ZNK7QScreen19logicalDotsPerInchYEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn geometry<T: QScreen_geometry>(&mut self, value: T) -> QRect {
    return value.geometry(self);
    // return 1;
  }
}

pub trait QScreen_geometry {
  fn geometry(self, rsthis: &mut QScreen) -> QRect;
}

// proto:  QRect QScreen::geometry();
impl<'a> /*trait*/ QScreen_geometry for () {
  fn geometry(self, rsthis: &mut QScreen) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen8geometryEv()};
    let mut ret = unsafe {_ZNK7QScreen8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn grabWindow<T: QScreen_grabWindow>(&mut self, value: T) -> QPixmap {
    return value.grabWindow(self);
    // return 1;
  }
}

pub trait QScreen_grabWindow {
  fn grabWindow(self, rsthis: &mut QScreen) -> QPixmap;
}

// proto:  QPixmap QScreen::grabWindow(WId window, int x, int y, int w, int h);
impl<'a> /*trait*/ QScreen_grabWindow for (*mut i32, i32, i32, i32, i32) {
  fn grabWindow(self, rsthis: &mut QScreen) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen10grabWindowEiiiii()};
    let arg0 = self.0  as *mut c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let mut ret = unsafe {_ZN7QScreen10grabWindowEiiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QPixmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn size<T: QScreen_size>(&mut self, value: T) -> QSize {
    return value.size(self);
    // return 1;
  }
}

pub trait QScreen_size {
  fn size(self, rsthis: &mut QScreen) -> QSize;
}

// proto:  QSize QScreen::size();
impl<'a> /*trait*/ QScreen_size for () {
  fn size(self, rsthis: &mut QScreen) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen4sizeEv()};
    let mut ret = unsafe {_ZNK7QScreen4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn physicalSize<T: QScreen_physicalSize>(&mut self, value: T) -> QSizeF {
    return value.physicalSize(self);
    // return 1;
  }
}

pub trait QScreen_physicalSize {
  fn physicalSize(self, rsthis: &mut QScreen) -> QSizeF;
}

// proto:  QSizeF QScreen::physicalSize();
impl<'a> /*trait*/ QScreen_physicalSize for () {
  fn physicalSize(self, rsthis: &mut QScreen) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen12physicalSizeEv()};
    let mut ret = unsafe {_ZNK7QScreen12physicalSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn handle<T: QScreen_handle>(&mut self, value: T)  {
     value.handle(self);
    // return 1;
  }
}

pub trait QScreen_handle {
  fn handle(self, rsthis: &mut QScreen) ;
}

// proto:  QPlatformScreen * QScreen::handle();
impl<'a> /*trait*/ QScreen_handle for () {
  fn handle(self, rsthis: &mut QScreen)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen6handleEv()};
     unsafe {_ZNK7QScreen6handleEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn availableVirtualGeometry<T: QScreen_availableVirtualGeometry>(&mut self, value: T) -> QRect {
    return value.availableVirtualGeometry(self);
    // return 1;
  }
}

pub trait QScreen_availableVirtualGeometry {
  fn availableVirtualGeometry(self, rsthis: &mut QScreen) -> QRect;
}

// proto:  QRect QScreen::availableVirtualGeometry();
impl<'a> /*trait*/ QScreen_availableVirtualGeometry for () {
  fn availableVirtualGeometry(self, rsthis: &mut QScreen) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen24availableVirtualGeometryEv()};
    let mut ret = unsafe {_ZNK7QScreen24availableVirtualGeometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn FreeQScreen<T: QScreen_FreeQScreen>(&mut self, value: T)  {
     value.FreeQScreen(self);
    // return 1;
  }
}

pub trait QScreen_FreeQScreen {
  fn FreeQScreen(self, rsthis: &mut QScreen) ;
}

// proto:  void QScreen::FreeQScreen();
impl<'a> /*trait*/ QScreen_FreeQScreen for () {
  fn FreeQScreen(self, rsthis: &mut QScreen)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreenD0Ev()};
     unsafe {_ZN7QScreenD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn virtualSize<T: QScreen_virtualSize>(&mut self, value: T) -> QSize {
    return value.virtualSize(self);
    // return 1;
  }
}

pub trait QScreen_virtualSize {
  fn virtualSize(self, rsthis: &mut QScreen) -> QSize;
}

// proto:  QSize QScreen::virtualSize();
impl<'a> /*trait*/ QScreen_virtualSize for () {
  fn virtualSize(self, rsthis: &mut QScreen) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen11virtualSizeEv()};
    let mut ret = unsafe {_ZNK7QScreen11virtualSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn devicePixelRatio<T: QScreen_devicePixelRatio>(&mut self, value: T) -> f64 {
    return value.devicePixelRatio(self);
    // return 1;
  }
}

pub trait QScreen_devicePixelRatio {
  fn devicePixelRatio(self, rsthis: &mut QScreen) -> f64;
}

// proto:  double QScreen::devicePixelRatio();
impl<'a> /*trait*/ QScreen_devicePixelRatio for () {
  fn devicePixelRatio(self, rsthis: &mut QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen16devicePixelRatioEv()};
    let mut ret = unsafe {_ZNK7QScreen16devicePixelRatioEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn virtualSiblings<T: QScreen_virtualSiblings>(&mut self, value: T)  {
     value.virtualSiblings(self);
    // return 1;
  }
}

pub trait QScreen_virtualSiblings {
  fn virtualSiblings(self, rsthis: &mut QScreen) ;
}

// proto:  QList<QScreen *> QScreen::virtualSiblings();
impl<'a> /*trait*/ QScreen_virtualSiblings for () {
  fn virtualSiblings(self, rsthis: &mut QScreen)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen15virtualSiblingsEv()};
     unsafe {_ZNK7QScreen15virtualSiblingsEv(rsthis.qclsinst)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QScreenC1ERKS_(qthis, arg0)};
    let rsthis = QScreen{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn logicalDotsPerInchChanged<T: QScreen_logicalDotsPerInchChanged>(&mut self, value: T)  {
     value.logicalDotsPerInchChanged(self);
    // return 1;
  }
}

pub trait QScreen_logicalDotsPerInchChanged {
  fn logicalDotsPerInchChanged(self, rsthis: &mut QScreen) ;
}

// proto:  void QScreen::logicalDotsPerInchChanged(qreal dpi);
impl<'a> /*trait*/ QScreen_logicalDotsPerInchChanged for (f64) {
  fn logicalDotsPerInchChanged(self, rsthis: &mut QScreen)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen25logicalDotsPerInchChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN7QScreen25logicalDotsPerInchChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn availableGeometryChanged<T: QScreen_availableGeometryChanged>(&mut self, value: T)  {
     value.availableGeometryChanged(self);
    // return 1;
  }
}

pub trait QScreen_availableGeometryChanged {
  fn availableGeometryChanged(self, rsthis: &mut QScreen) ;
}

// proto:  void QScreen::availableGeometryChanged(const QRect & geometry);
impl<'a> /*trait*/ QScreen_availableGeometryChanged for (&'a  QRect) {
  fn availableGeometryChanged(self, rsthis: &mut QScreen)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen24availableGeometryChangedERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QScreen24availableGeometryChangedERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn geometryChanged<T: QScreen_geometryChanged>(&mut self, value: T)  {
     value.geometryChanged(self);
    // return 1;
  }
}

pub trait QScreen_geometryChanged {
  fn geometryChanged(self, rsthis: &mut QScreen) ;
}

// proto:  void QScreen::geometryChanged(const QRect & geometry);
impl<'a> /*trait*/ QScreen_geometryChanged for (&'a  QRect) {
  fn geometryChanged(self, rsthis: &mut QScreen)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen15geometryChangedERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QScreen15geometryChangedERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn virtualGeometry<T: QScreen_virtualGeometry>(&mut self, value: T) -> QRect {
    return value.virtualGeometry(self);
    // return 1;
  }
}

pub trait QScreen_virtualGeometry {
  fn virtualGeometry(self, rsthis: &mut QScreen) -> QRect;
}

// proto:  QRect QScreen::virtualGeometry();
impl<'a> /*trait*/ QScreen_virtualGeometry for () {
  fn virtualGeometry(self, rsthis: &mut QScreen) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen15virtualGeometryEv()};
    let mut ret = unsafe {_ZNK7QScreen15virtualGeometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn logicalDotsPerInch<T: QScreen_logicalDotsPerInch>(&mut self, value: T) -> f64 {
    return value.logicalDotsPerInch(self);
    // return 1;
  }
}

pub trait QScreen_logicalDotsPerInch {
  fn logicalDotsPerInch(self, rsthis: &mut QScreen) -> f64;
}

// proto:  double QScreen::logicalDotsPerInch();
impl<'a> /*trait*/ QScreen_logicalDotsPerInch for () {
  fn logicalDotsPerInch(self, rsthis: &mut QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen18logicalDotsPerInchEv()};
    let mut ret = unsafe {_ZNK7QScreen18logicalDotsPerInchEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn physicalDotsPerInch<T: QScreen_physicalDotsPerInch>(&mut self, value: T) -> f64 {
    return value.physicalDotsPerInch(self);
    // return 1;
  }
}

pub trait QScreen_physicalDotsPerInch {
  fn physicalDotsPerInch(self, rsthis: &mut QScreen) -> f64;
}

// proto:  double QScreen::physicalDotsPerInch();
impl<'a> /*trait*/ QScreen_physicalDotsPerInch for () {
  fn physicalDotsPerInch(self, rsthis: &mut QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen19physicalDotsPerInchEv()};
    let mut ret = unsafe {_ZNK7QScreen19physicalDotsPerInchEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn physicalDotsPerInchChanged<T: QScreen_physicalDotsPerInchChanged>(&mut self, value: T)  {
     value.physicalDotsPerInchChanged(self);
    // return 1;
  }
}

pub trait QScreen_physicalDotsPerInchChanged {
  fn physicalDotsPerInchChanged(self, rsthis: &mut QScreen) ;
}

// proto:  void QScreen::physicalDotsPerInchChanged(qreal dpi);
impl<'a> /*trait*/ QScreen_physicalDotsPerInchChanged for (f64) {
  fn physicalDotsPerInchChanged(self, rsthis: &mut QScreen)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen26physicalDotsPerInchChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN7QScreen26physicalDotsPerInchChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn refreshRate<T: QScreen_refreshRate>(&mut self, value: T) -> f64 {
    return value.refreshRate(self);
    // return 1;
  }
}

pub trait QScreen_refreshRate {
  fn refreshRate(self, rsthis: &mut QScreen) -> f64;
}

// proto:  double QScreen::refreshRate();
impl<'a> /*trait*/ QScreen_refreshRate for () {
  fn refreshRate(self, rsthis: &mut QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen11refreshRateEv()};
    let mut ret = unsafe {_ZNK7QScreen11refreshRateEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn metaObject<T: QScreen_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QScreen_metaObject {
  fn metaObject(self, rsthis: &mut QScreen) ;
}

// proto:  const QMetaObject * QScreen::metaObject();
impl<'a> /*trait*/ QScreen_metaObject for () {
  fn metaObject(self, rsthis: &mut QScreen)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen10metaObjectEv()};
     unsafe {_ZNK7QScreen10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn refreshRateChanged<T: QScreen_refreshRateChanged>(&mut self, value: T)  {
     value.refreshRateChanged(self);
    // return 1;
  }
}

pub trait QScreen_refreshRateChanged {
  fn refreshRateChanged(self, rsthis: &mut QScreen) ;
}

// proto:  void QScreen::refreshRateChanged(qreal refreshRate);
impl<'a> /*trait*/ QScreen_refreshRateChanged for (f64) {
  fn refreshRateChanged(self, rsthis: &mut QScreen)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen18refreshRateChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN7QScreen18refreshRateChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn availableSize<T: QScreen_availableSize>(&mut self, value: T) -> QSize {
    return value.availableSize(self);
    // return 1;
  }
}

pub trait QScreen_availableSize {
  fn availableSize(self, rsthis: &mut QScreen) -> QSize;
}

// proto:  QSize QScreen::availableSize();
impl<'a> /*trait*/ QScreen_availableSize for () {
  fn availableSize(self, rsthis: &mut QScreen) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen13availableSizeEv()};
    let mut ret = unsafe {_ZNK7QScreen13availableSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn name<T: QScreen_name>(&mut self, value: T) -> QString {
    return value.name(self);
    // return 1;
  }
}

pub trait QScreen_name {
  fn name(self, rsthis: &mut QScreen) -> QString;
}

// proto:  QString QScreen::name();
impl<'a> /*trait*/ QScreen_name for () {
  fn name(self, rsthis: &mut QScreen) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen4nameEv()};
    let mut ret = unsafe {_ZNK7QScreen4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn availableVirtualSize<T: QScreen_availableVirtualSize>(&mut self, value: T) -> QSize {
    return value.availableVirtualSize(self);
    // return 1;
  }
}

pub trait QScreen_availableVirtualSize {
  fn availableVirtualSize(self, rsthis: &mut QScreen) -> QSize;
}

// proto:  QSize QScreen::availableVirtualSize();
impl<'a> /*trait*/ QScreen_availableVirtualSize for () {
  fn availableVirtualSize(self, rsthis: &mut QScreen) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen20availableVirtualSizeEv()};
    let mut ret = unsafe {_ZNK7QScreen20availableVirtualSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn logicalDotsPerInchX<T: QScreen_logicalDotsPerInchX>(&mut self, value: T) -> f64 {
    return value.logicalDotsPerInchX(self);
    // return 1;
  }
}

pub trait QScreen_logicalDotsPerInchX {
  fn logicalDotsPerInchX(self, rsthis: &mut QScreen) -> f64;
}

// proto:  double QScreen::logicalDotsPerInchX();
impl<'a> /*trait*/ QScreen_logicalDotsPerInchX for () {
  fn logicalDotsPerInchX(self, rsthis: &mut QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen19logicalDotsPerInchXEv()};
    let mut ret = unsafe {_ZNK7QScreen19logicalDotsPerInchXEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn virtualGeometryChanged<T: QScreen_virtualGeometryChanged>(&mut self, value: T)  {
     value.virtualGeometryChanged(self);
    // return 1;
  }
}

pub trait QScreen_virtualGeometryChanged {
  fn virtualGeometryChanged(self, rsthis: &mut QScreen) ;
}

// proto:  void QScreen::virtualGeometryChanged(const QRect & rect);
impl<'a> /*trait*/ QScreen_virtualGeometryChanged for (&'a  QRect) {
  fn virtualGeometryChanged(self, rsthis: &mut QScreen)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen22virtualGeometryChangedERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QScreen22virtualGeometryChangedERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn availableGeometry<T: QScreen_availableGeometry>(&mut self, value: T) -> QRect {
    return value.availableGeometry(self);
    // return 1;
  }
}

pub trait QScreen_availableGeometry {
  fn availableGeometry(self, rsthis: &mut QScreen) -> QRect;
}

// proto:  QRect QScreen::availableGeometry();
impl<'a> /*trait*/ QScreen_availableGeometry for () {
  fn availableGeometry(self, rsthis: &mut QScreen) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen17availableGeometryEv()};
    let mut ret = unsafe {_ZNK7QScreen17availableGeometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn physicalDotsPerInchX<T: QScreen_physicalDotsPerInchX>(&mut self, value: T) -> f64 {
    return value.physicalDotsPerInchX(self);
    // return 1;
  }
}

pub trait QScreen_physicalDotsPerInchX {
  fn physicalDotsPerInchX(self, rsthis: &mut QScreen) -> f64;
}

// proto:  double QScreen::physicalDotsPerInchX();
impl<'a> /*trait*/ QScreen_physicalDotsPerInchX for () {
  fn physicalDotsPerInchX(self, rsthis: &mut QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen20physicalDotsPerInchXEv()};
    let mut ret = unsafe {_ZNK7QScreen20physicalDotsPerInchXEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn physicalSizeChanged<T: QScreen_physicalSizeChanged>(&mut self, value: T)  {
     value.physicalSizeChanged(self);
    // return 1;
  }
}

pub trait QScreen_physicalSizeChanged {
  fn physicalSizeChanged(self, rsthis: &mut QScreen) ;
}

// proto:  void QScreen::physicalSizeChanged(const QSizeF & size);
impl<'a> /*trait*/ QScreen_physicalSizeChanged for (&'a  QSizeF) {
  fn physicalSizeChanged(self, rsthis: &mut QScreen)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen19physicalSizeChangedERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QScreen19physicalSizeChangedERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn physicalDotsPerInchY<T: QScreen_physicalDotsPerInchY>(&mut self, value: T) -> f64 {
    return value.physicalDotsPerInchY(self);
    // return 1;
  }
}

pub trait QScreen_physicalDotsPerInchY {
  fn physicalDotsPerInchY(self, rsthis: &mut QScreen) -> f64;
}

// proto:  double QScreen::physicalDotsPerInchY();
impl<'a> /*trait*/ QScreen_physicalDotsPerInchY for () {
  fn physicalDotsPerInchY(self, rsthis: &mut QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen20physicalDotsPerInchYEv()};
    let mut ret = unsafe {_ZNK7QScreen20physicalDotsPerInchYEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QScreen {
  pub fn depth<T: QScreen_depth>(&mut self, value: T) -> i32 {
    return value.depth(self);
    // return 1;
  }
}

pub trait QScreen_depth {
  fn depth(self, rsthis: &mut QScreen) -> i32;
}

// proto:  int QScreen::depth();
impl<'a> /*trait*/ QScreen_depth for () {
  fn depth(self, rsthis: &mut QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen5depthEv()};
    let mut ret = unsafe {_ZNK7QScreen5depthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

