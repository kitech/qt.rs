// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpaintengine::QPaintEngine;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QPaintEngine * QOpenGLPaintDevice::paintEngine();
  fn _ZNK18QOpenGLPaintDevice11paintEngineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QOpenGLPaintDevice::size();
  fn _ZNK18QOpenGLPaintDevice4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QOpenGLPaintDevice::setPaintFlipped(bool flipped);
  fn _ZN18QOpenGLPaintDevice15setPaintFlippedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QOpenGLPaintDevice::FreeQOpenGLPaintDevice();
  fn _ZN18QOpenGLPaintDeviceD0Ev(qthis: *mut c_void) ;
  // proto:  void QOpenGLPaintDevice::NewQOpenGLPaintDevice(int width, int height);
  fn _ZN18QOpenGLPaintDeviceC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QOpenGLPaintDevice::NewQOpenGLPaintDevice(const QOpenGLPaintDevice & );
  fn _ZN18QOpenGLPaintDeviceC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QOpenGLContext * QOpenGLPaintDevice::context();
  fn _ZNK18QOpenGLPaintDevice7contextEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLPaintDevice::setDevicePixelRatio(qreal devicePixelRatio);
  fn _ZN18QOpenGLPaintDevice19setDevicePixelRatioEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QOpenGLPaintDevice::NewQOpenGLPaintDevice();
  fn _ZN18QOpenGLPaintDeviceC1Ev(qthis: *mut c_void) ;
  // proto:  int QOpenGLPaintDevice::devType();
  fn _ZNK18QOpenGLPaintDevice7devTypeEv(qthis: *mut c_void) -> c_int;
  // proto:  double QOpenGLPaintDevice::dotsPerMeterX();
  fn _ZNK18QOpenGLPaintDevice13dotsPerMeterXEv(qthis: *mut c_void) -> c_double;
  // proto:  void QOpenGLPaintDevice::setDotsPerMeterX(qreal );
  fn _ZN18QOpenGLPaintDevice16setDotsPerMeterXEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QOpenGLPaintDevice::dotsPerMeterY();
  fn _ZNK18QOpenGLPaintDevice13dotsPerMeterYEv(qthis: *mut c_void) -> c_double;
  // proto:  void QOpenGLPaintDevice::setDotsPerMeterY(qreal );
  fn _ZN18QOpenGLPaintDevice16setDotsPerMeterYEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  bool QOpenGLPaintDevice::paintFlipped();
  fn _ZNK18QOpenGLPaintDevice12paintFlippedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLPaintDevice::setSize(const QSize & size);
  fn _ZN18QOpenGLPaintDevice7setSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOpenGLPaintDevice::ensureActiveTarget();
  fn _ZN18QOpenGLPaintDevice18ensureActiveTargetEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLPaintDevice::NewQOpenGLPaintDevice(const QSize & size);
  fn _ZN18QOpenGLPaintDeviceC1ERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QOpenGLPaintDevice)=1
pub struct QOpenGLPaintDevice {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLPaintDevice {
  pub fn paintEngine<T: QOpenGLPaintDevice_paintEngine>(&mut self, value: T) -> QPaintEngine {
    return value.paintEngine(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_paintEngine {
  fn paintEngine(self, rsthis: &mut QOpenGLPaintDevice) -> QPaintEngine;
}

// proto:  QPaintEngine * QOpenGLPaintDevice::paintEngine();
impl<'a> /*trait*/ QOpenGLPaintDevice_paintEngine for () {
  fn paintEngine(self, rsthis: &mut QOpenGLPaintDevice) -> QPaintEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice11paintEngineEv()};
    let mut ret = unsafe {_ZNK18QOpenGLPaintDevice11paintEngineEv(rsthis.qclsinst)};
    let mut ret1 = QPaintEngine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QOpenGLPaintDevice {
  pub fn size<T: QOpenGLPaintDevice_size>(&mut self, value: T) -> QSize {
    return value.size(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_size {
  fn size(self, rsthis: &mut QOpenGLPaintDevice) -> QSize;
}

// proto:  QSize QOpenGLPaintDevice::size();
impl<'a> /*trait*/ QOpenGLPaintDevice_size for () {
  fn size(self, rsthis: &mut QOpenGLPaintDevice) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice4sizeEv()};
    let mut ret = unsafe {_ZNK18QOpenGLPaintDevice4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QOpenGLPaintDevice {
  pub fn setPaintFlipped<T: QOpenGLPaintDevice_setPaintFlipped>(&mut self, value: T)  {
     value.setPaintFlipped(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_setPaintFlipped {
  fn setPaintFlipped(self, rsthis: &mut QOpenGLPaintDevice) ;
}

// proto:  void QOpenGLPaintDevice::setPaintFlipped(bool flipped);
impl<'a> /*trait*/ QOpenGLPaintDevice_setPaintFlipped for (i8) {
  fn setPaintFlipped(self, rsthis: &mut QOpenGLPaintDevice)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDevice15setPaintFlippedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN18QOpenGLPaintDevice15setPaintFlippedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLPaintDevice {
  pub fn FreeQOpenGLPaintDevice<T: QOpenGLPaintDevice_FreeQOpenGLPaintDevice>(&mut self, value: T)  {
     value.FreeQOpenGLPaintDevice(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_FreeQOpenGLPaintDevice {
  fn FreeQOpenGLPaintDevice(self, rsthis: &mut QOpenGLPaintDevice) ;
}

// proto:  void QOpenGLPaintDevice::FreeQOpenGLPaintDevice();
impl<'a> /*trait*/ QOpenGLPaintDevice_FreeQOpenGLPaintDevice for () {
  fn FreeQOpenGLPaintDevice(self, rsthis: &mut QOpenGLPaintDevice)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDeviceD0Ev()};
     unsafe {_ZN18QOpenGLPaintDeviceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLPaintDevice {
  pub fn NewQOpenGLPaintDevice<T: QOpenGLPaintDevice_NewQOpenGLPaintDevice>(value: T) -> QOpenGLPaintDevice {
    let rsthis = value.NewQOpenGLPaintDevice();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_NewQOpenGLPaintDevice {
  fn NewQOpenGLPaintDevice(self) -> QOpenGLPaintDevice;
}

// proto: void QOpenGLPaintDevice::NewQOpenGLPaintDevice(int width, int height);
impl<'a> /*trait*/ QOpenGLPaintDevice_NewQOpenGLPaintDevice for (i32, i32) {
  fn NewQOpenGLPaintDevice(self) -> QOpenGLPaintDevice {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDeviceC1Eii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN18QOpenGLPaintDeviceC1Eii(qthis, arg0, arg1)};
    let rsthis = QOpenGLPaintDevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QOpenGLPaintDevice::NewQOpenGLPaintDevice(const QOpenGLPaintDevice & );
impl<'a> /*trait*/ QOpenGLPaintDevice_NewQOpenGLPaintDevice for (&'a  QOpenGLPaintDevice) {
  fn NewQOpenGLPaintDevice(self) -> QOpenGLPaintDevice {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDeviceC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QOpenGLPaintDeviceC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLPaintDevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLPaintDevice {
  pub fn context<T: QOpenGLPaintDevice_context>(&mut self, value: T)  {
     value.context(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_context {
  fn context(self, rsthis: &mut QOpenGLPaintDevice) ;
}

// proto:  QOpenGLContext * QOpenGLPaintDevice::context();
impl<'a> /*trait*/ QOpenGLPaintDevice_context for () {
  fn context(self, rsthis: &mut QOpenGLPaintDevice)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice7contextEv()};
     unsafe {_ZNK18QOpenGLPaintDevice7contextEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLPaintDevice {
  pub fn setDevicePixelRatio<T: QOpenGLPaintDevice_setDevicePixelRatio>(&mut self, value: T)  {
     value.setDevicePixelRatio(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_setDevicePixelRatio {
  fn setDevicePixelRatio(self, rsthis: &mut QOpenGLPaintDevice) ;
}

// proto:  void QOpenGLPaintDevice::setDevicePixelRatio(qreal devicePixelRatio);
impl<'a> /*trait*/ QOpenGLPaintDevice_setDevicePixelRatio for (f64) {
  fn setDevicePixelRatio(self, rsthis: &mut QOpenGLPaintDevice)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDevice19setDevicePixelRatioEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN18QOpenGLPaintDevice19setDevicePixelRatioEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QOpenGLPaintDevice::NewQOpenGLPaintDevice();
impl<'a> /*trait*/ QOpenGLPaintDevice_NewQOpenGLPaintDevice for () {
  fn NewQOpenGLPaintDevice(self) -> QOpenGLPaintDevice {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDeviceC1Ev()};
    unsafe {_ZN18QOpenGLPaintDeviceC1Ev(qthis)};
    let rsthis = QOpenGLPaintDevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLPaintDevice {
  pub fn devType<T: QOpenGLPaintDevice_devType>(&mut self, value: T) -> i32 {
    return value.devType(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_devType {
  fn devType(self, rsthis: &mut QOpenGLPaintDevice) -> i32;
}

// proto:  int QOpenGLPaintDevice::devType();
impl<'a> /*trait*/ QOpenGLPaintDevice_devType for () {
  fn devType(self, rsthis: &mut QOpenGLPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice7devTypeEv()};
    let mut ret = unsafe {_ZNK18QOpenGLPaintDevice7devTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QOpenGLPaintDevice {
  pub fn dotsPerMeterX<T: QOpenGLPaintDevice_dotsPerMeterX>(&mut self, value: T) -> f64 {
    return value.dotsPerMeterX(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_dotsPerMeterX {
  fn dotsPerMeterX(self, rsthis: &mut QOpenGLPaintDevice) -> f64;
}

// proto:  double QOpenGLPaintDevice::dotsPerMeterX();
impl<'a> /*trait*/ QOpenGLPaintDevice_dotsPerMeterX for () {
  fn dotsPerMeterX(self, rsthis: &mut QOpenGLPaintDevice) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice13dotsPerMeterXEv()};
    let mut ret = unsafe {_ZNK18QOpenGLPaintDevice13dotsPerMeterXEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QOpenGLPaintDevice {
  pub fn setDotsPerMeterX<T: QOpenGLPaintDevice_setDotsPerMeterX>(&mut self, value: T)  {
     value.setDotsPerMeterX(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_setDotsPerMeterX {
  fn setDotsPerMeterX(self, rsthis: &mut QOpenGLPaintDevice) ;
}

// proto:  void QOpenGLPaintDevice::setDotsPerMeterX(qreal );
impl<'a> /*trait*/ QOpenGLPaintDevice_setDotsPerMeterX for (f64) {
  fn setDotsPerMeterX(self, rsthis: &mut QOpenGLPaintDevice)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDevice16setDotsPerMeterXEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN18QOpenGLPaintDevice16setDotsPerMeterXEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLPaintDevice {
  pub fn dotsPerMeterY<T: QOpenGLPaintDevice_dotsPerMeterY>(&mut self, value: T) -> f64 {
    return value.dotsPerMeterY(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_dotsPerMeterY {
  fn dotsPerMeterY(self, rsthis: &mut QOpenGLPaintDevice) -> f64;
}

// proto:  double QOpenGLPaintDevice::dotsPerMeterY();
impl<'a> /*trait*/ QOpenGLPaintDevice_dotsPerMeterY for () {
  fn dotsPerMeterY(self, rsthis: &mut QOpenGLPaintDevice) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice13dotsPerMeterYEv()};
    let mut ret = unsafe {_ZNK18QOpenGLPaintDevice13dotsPerMeterYEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QOpenGLPaintDevice {
  pub fn setDotsPerMeterY<T: QOpenGLPaintDevice_setDotsPerMeterY>(&mut self, value: T)  {
     value.setDotsPerMeterY(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_setDotsPerMeterY {
  fn setDotsPerMeterY(self, rsthis: &mut QOpenGLPaintDevice) ;
}

// proto:  void QOpenGLPaintDevice::setDotsPerMeterY(qreal );
impl<'a> /*trait*/ QOpenGLPaintDevice_setDotsPerMeterY for (f64) {
  fn setDotsPerMeterY(self, rsthis: &mut QOpenGLPaintDevice)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDevice16setDotsPerMeterYEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN18QOpenGLPaintDevice16setDotsPerMeterYEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLPaintDevice {
  pub fn paintFlipped<T: QOpenGLPaintDevice_paintFlipped>(&mut self, value: T) -> i8 {
    return value.paintFlipped(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_paintFlipped {
  fn paintFlipped(self, rsthis: &mut QOpenGLPaintDevice) -> i8;
}

// proto:  bool QOpenGLPaintDevice::paintFlipped();
impl<'a> /*trait*/ QOpenGLPaintDevice_paintFlipped for () {
  fn paintFlipped(self, rsthis: &mut QOpenGLPaintDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice12paintFlippedEv()};
    let mut ret = unsafe {_ZNK18QOpenGLPaintDevice12paintFlippedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLPaintDevice {
  pub fn setSize<T: QOpenGLPaintDevice_setSize>(&mut self, value: T)  {
     value.setSize(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_setSize {
  fn setSize(self, rsthis: &mut QOpenGLPaintDevice) ;
}

// proto:  void QOpenGLPaintDevice::setSize(const QSize & size);
impl<'a> /*trait*/ QOpenGLPaintDevice_setSize for (&'a  QSize) {
  fn setSize(self, rsthis: &mut QOpenGLPaintDevice)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDevice7setSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QOpenGLPaintDevice7setSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLPaintDevice {
  pub fn ensureActiveTarget<T: QOpenGLPaintDevice_ensureActiveTarget>(&mut self, value: T)  {
     value.ensureActiveTarget(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_ensureActiveTarget {
  fn ensureActiveTarget(self, rsthis: &mut QOpenGLPaintDevice) ;
}

// proto:  void QOpenGLPaintDevice::ensureActiveTarget();
impl<'a> /*trait*/ QOpenGLPaintDevice_ensureActiveTarget for () {
  fn ensureActiveTarget(self, rsthis: &mut QOpenGLPaintDevice)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDevice18ensureActiveTargetEv()};
     unsafe {_ZN18QOpenGLPaintDevice18ensureActiveTargetEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QOpenGLPaintDevice::NewQOpenGLPaintDevice(const QSize & size);
impl<'a> /*trait*/ QOpenGLPaintDevice_NewQOpenGLPaintDevice for (&'a  QSize) {
  fn NewQOpenGLPaintDevice(self) -> QOpenGLPaintDevice {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDeviceC1ERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QOpenGLPaintDeviceC1ERK5QSize(qthis, arg0)};
    let rsthis = QOpenGLPaintDevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

