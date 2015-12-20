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
  fn _ZN18QOpenGLPaintDevice15setPaintFlippedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QOpenGLPaintDevice::~QOpenGLPaintDevice();
  fn _ZN18QOpenGLPaintDeviceD0Ev(qthis: *mut c_void);
  // proto:  void QOpenGLPaintDevice::QOpenGLPaintDevice(int width, int height);
  fn _ZN18QOpenGLPaintDeviceC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QOpenGLPaintDevice::QOpenGLPaintDevice(const QOpenGLPaintDevice & );
  fn _ZN18QOpenGLPaintDeviceC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QOpenGLContext * QOpenGLPaintDevice::context();
  fn _ZNK18QOpenGLPaintDevice7contextEv(qthis: *mut c_void);
  // proto:  void QOpenGLPaintDevice::setDevicePixelRatio(qreal devicePixelRatio);
  fn _ZN18QOpenGLPaintDevice19setDevicePixelRatioEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QOpenGLPaintDevice::QOpenGLPaintDevice();
  fn _ZN18QOpenGLPaintDeviceC1Ev(qthis: *mut c_void);
  // proto:  int QOpenGLPaintDevice::devType();
  fn _ZNK18QOpenGLPaintDevice7devTypeEv(qthis: *mut c_void) -> c_int;
  // proto:  qreal QOpenGLPaintDevice::dotsPerMeterX();
  fn _ZNK18QOpenGLPaintDevice13dotsPerMeterXEv(qthis: *mut c_void) -> c_double;
  // proto:  void QOpenGLPaintDevice::setDotsPerMeterX(qreal );
  fn _ZN18QOpenGLPaintDevice16setDotsPerMeterXEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QOpenGLPaintDevice::dotsPerMeterY();
  fn _ZNK18QOpenGLPaintDevice13dotsPerMeterYEv(qthis: *mut c_void) -> c_double;
  // proto:  void QOpenGLPaintDevice::setDotsPerMeterY(qreal );
  fn _ZN18QOpenGLPaintDevice16setDotsPerMeterYEd(qthis: *mut c_void, arg0: c_double);
  // proto:  bool QOpenGLPaintDevice::paintFlipped();
  fn _ZNK18QOpenGLPaintDevice12paintFlippedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QOpenGLPaintDevice::setSize(const QSize & size);
  fn _ZN18QOpenGLPaintDevice7setSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOpenGLPaintDevice::ensureActiveTarget();
  fn _ZN18QOpenGLPaintDevice18ensureActiveTargetEv(qthis: *mut c_void);
  // proto:  void QOpenGLPaintDevice::QOpenGLPaintDevice(const QSize & size);
  fn _ZN18QOpenGLPaintDeviceC1ERK5QSize(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QOpenGLPaintDevice)=1
pub struct QOpenGLPaintDevice {
  pub qclsinst: *mut c_void,
}

  // proto:  QPaintEngine * QOpenGLPaintDevice::paintEngine();
impl /*struct*/ QOpenGLPaintDevice {
  pub fn paintEngine<RetType, T: QOpenGLPaintDevice_paintEngine<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.paintEngine(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_paintEngine<RetType> {
  fn paintEngine(self , rsthis: &mut QOpenGLPaintDevice) -> RetType;
}

  // proto:  QPaintEngine * QOpenGLPaintDevice::paintEngine();
impl<'a> /*trait*/ QOpenGLPaintDevice_paintEngine<QPaintEngine> for () {
  fn paintEngine(self , rsthis: &mut QOpenGLPaintDevice) -> QPaintEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice11paintEngineEv()};
    let mut ret = unsafe {_ZNK18QOpenGLPaintDevice11paintEngineEv(rsthis.qclsinst)};
    let mut ret1 = QPaintEngine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QOpenGLPaintDevice::size();
impl /*struct*/ QOpenGLPaintDevice {
  pub fn size<RetType, T: QOpenGLPaintDevice_size<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_size<RetType> {
  fn size(self , rsthis: &mut QOpenGLPaintDevice) -> RetType;
}

  // proto:  QSize QOpenGLPaintDevice::size();
impl<'a> /*trait*/ QOpenGLPaintDevice_size<QSize> for () {
  fn size(self , rsthis: &mut QOpenGLPaintDevice) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice4sizeEv()};
    let mut ret = unsafe {_ZNK18QOpenGLPaintDevice4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QOpenGLPaintDevice::setPaintFlipped(bool flipped);
impl /*struct*/ QOpenGLPaintDevice {
  pub fn setPaintFlipped<RetType, T: QOpenGLPaintDevice_setPaintFlipped<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPaintFlipped(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_setPaintFlipped<RetType> {
  fn setPaintFlipped(self , rsthis: &mut QOpenGLPaintDevice) -> RetType;
}

  // proto:  void QOpenGLPaintDevice::setPaintFlipped(bool flipped);
impl<'a> /*trait*/ QOpenGLPaintDevice_setPaintFlipped<()> for (i8) {
  fn setPaintFlipped(self , rsthis: &mut QOpenGLPaintDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDevice15setPaintFlippedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN18QOpenGLPaintDevice15setPaintFlippedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLPaintDevice::~QOpenGLPaintDevice();
impl /*struct*/ QOpenGLPaintDevice {
  pub fn FreeQOpenGLPaintDevice<RetType, T: QOpenGLPaintDevice_FreeQOpenGLPaintDevice<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQOpenGLPaintDevice(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_FreeQOpenGLPaintDevice<RetType> {
  fn FreeQOpenGLPaintDevice(self , rsthis: &mut QOpenGLPaintDevice) -> RetType;
}

  // proto:  void QOpenGLPaintDevice::~QOpenGLPaintDevice();
impl<'a> /*trait*/ QOpenGLPaintDevice_FreeQOpenGLPaintDevice<()> for () {
  fn FreeQOpenGLPaintDevice(self , rsthis: &mut QOpenGLPaintDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDeviceD0Ev()};
     unsafe {_ZN18QOpenGLPaintDeviceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLPaintDevice::QOpenGLPaintDevice(int width, int height);
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

  // proto:  void QOpenGLPaintDevice::QOpenGLPaintDevice(int width, int height);
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

  // proto:  void QOpenGLPaintDevice::QOpenGLPaintDevice(const QOpenGLPaintDevice & );
impl<'a> /*trait*/ QOpenGLPaintDevice_NewQOpenGLPaintDevice for (QOpenGLPaintDevice) {
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

  // proto:  QOpenGLContext * QOpenGLPaintDevice::context();
impl /*struct*/ QOpenGLPaintDevice {
  pub fn context<RetType, T: QOpenGLPaintDevice_context<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.context(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_context<RetType> {
  fn context(self , rsthis: &mut QOpenGLPaintDevice) -> RetType;
}

  // proto:  QOpenGLContext * QOpenGLPaintDevice::context();
impl<'a> /*trait*/ QOpenGLPaintDevice_context<()> for () {
  fn context(self , rsthis: &mut QOpenGLPaintDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice7contextEv()};
     unsafe {_ZNK18QOpenGLPaintDevice7contextEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLPaintDevice::setDevicePixelRatio(qreal devicePixelRatio);
impl /*struct*/ QOpenGLPaintDevice {
  pub fn setDevicePixelRatio<RetType, T: QOpenGLPaintDevice_setDevicePixelRatio<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDevicePixelRatio(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_setDevicePixelRatio<RetType> {
  fn setDevicePixelRatio(self , rsthis: &mut QOpenGLPaintDevice) -> RetType;
}

  // proto:  void QOpenGLPaintDevice::setDevicePixelRatio(qreal devicePixelRatio);
impl<'a> /*trait*/ QOpenGLPaintDevice_setDevicePixelRatio<()> for (f64) {
  fn setDevicePixelRatio(self , rsthis: &mut QOpenGLPaintDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDevice19setDevicePixelRatioEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN18QOpenGLPaintDevice19setDevicePixelRatioEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLPaintDevice::QOpenGLPaintDevice();
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

  // proto:  int QOpenGLPaintDevice::devType();
impl /*struct*/ QOpenGLPaintDevice {
  pub fn devType<RetType, T: QOpenGLPaintDevice_devType<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.devType(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_devType<RetType> {
  fn devType(self , rsthis: &mut QOpenGLPaintDevice) -> RetType;
}

  // proto:  int QOpenGLPaintDevice::devType();
impl<'a> /*trait*/ QOpenGLPaintDevice_devType<i32> for () {
  fn devType(self , rsthis: &mut QOpenGLPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice7devTypeEv()};
    let mut ret = unsafe {_ZNK18QOpenGLPaintDevice7devTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  qreal QOpenGLPaintDevice::dotsPerMeterX();
impl /*struct*/ QOpenGLPaintDevice {
  pub fn dotsPerMeterX<RetType, T: QOpenGLPaintDevice_dotsPerMeterX<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.dotsPerMeterX(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_dotsPerMeterX<RetType> {
  fn dotsPerMeterX(self , rsthis: &mut QOpenGLPaintDevice) -> RetType;
}

  // proto:  qreal QOpenGLPaintDevice::dotsPerMeterX();
impl<'a> /*trait*/ QOpenGLPaintDevice_dotsPerMeterX<f64> for () {
  fn dotsPerMeterX(self , rsthis: &mut QOpenGLPaintDevice) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice13dotsPerMeterXEv()};
    let mut ret = unsafe {_ZNK18QOpenGLPaintDevice13dotsPerMeterXEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QOpenGLPaintDevice::setDotsPerMeterX(qreal );
impl /*struct*/ QOpenGLPaintDevice {
  pub fn setDotsPerMeterX<RetType, T: QOpenGLPaintDevice_setDotsPerMeterX<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDotsPerMeterX(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_setDotsPerMeterX<RetType> {
  fn setDotsPerMeterX(self , rsthis: &mut QOpenGLPaintDevice) -> RetType;
}

  // proto:  void QOpenGLPaintDevice::setDotsPerMeterX(qreal );
impl<'a> /*trait*/ QOpenGLPaintDevice_setDotsPerMeterX<()> for (f64) {
  fn setDotsPerMeterX(self , rsthis: &mut QOpenGLPaintDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDevice16setDotsPerMeterXEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN18QOpenGLPaintDevice16setDotsPerMeterXEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QOpenGLPaintDevice::dotsPerMeterY();
impl /*struct*/ QOpenGLPaintDevice {
  pub fn dotsPerMeterY<RetType, T: QOpenGLPaintDevice_dotsPerMeterY<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.dotsPerMeterY(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_dotsPerMeterY<RetType> {
  fn dotsPerMeterY(self , rsthis: &mut QOpenGLPaintDevice) -> RetType;
}

  // proto:  qreal QOpenGLPaintDevice::dotsPerMeterY();
impl<'a> /*trait*/ QOpenGLPaintDevice_dotsPerMeterY<f64> for () {
  fn dotsPerMeterY(self , rsthis: &mut QOpenGLPaintDevice) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice13dotsPerMeterYEv()};
    let mut ret = unsafe {_ZNK18QOpenGLPaintDevice13dotsPerMeterYEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QOpenGLPaintDevice::setDotsPerMeterY(qreal );
impl /*struct*/ QOpenGLPaintDevice {
  pub fn setDotsPerMeterY<RetType, T: QOpenGLPaintDevice_setDotsPerMeterY<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDotsPerMeterY(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_setDotsPerMeterY<RetType> {
  fn setDotsPerMeterY(self , rsthis: &mut QOpenGLPaintDevice) -> RetType;
}

  // proto:  void QOpenGLPaintDevice::setDotsPerMeterY(qreal );
impl<'a> /*trait*/ QOpenGLPaintDevice_setDotsPerMeterY<()> for (f64) {
  fn setDotsPerMeterY(self , rsthis: &mut QOpenGLPaintDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDevice16setDotsPerMeterYEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN18QOpenGLPaintDevice16setDotsPerMeterYEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QOpenGLPaintDevice::paintFlipped();
impl /*struct*/ QOpenGLPaintDevice {
  pub fn paintFlipped<RetType, T: QOpenGLPaintDevice_paintFlipped<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.paintFlipped(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_paintFlipped<RetType> {
  fn paintFlipped(self , rsthis: &mut QOpenGLPaintDevice) -> RetType;
}

  // proto:  bool QOpenGLPaintDevice::paintFlipped();
impl<'a> /*trait*/ QOpenGLPaintDevice_paintFlipped<i8> for () {
  fn paintFlipped(self , rsthis: &mut QOpenGLPaintDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice12paintFlippedEv()};
    let mut ret = unsafe {_ZNK18QOpenGLPaintDevice12paintFlippedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLPaintDevice::setSize(const QSize & size);
impl /*struct*/ QOpenGLPaintDevice {
  pub fn setSize<RetType, T: QOpenGLPaintDevice_setSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSize(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_setSize<RetType> {
  fn setSize(self , rsthis: &mut QOpenGLPaintDevice) -> RetType;
}

  // proto:  void QOpenGLPaintDevice::setSize(const QSize & size);
impl<'a> /*trait*/ QOpenGLPaintDevice_setSize<()> for (QSize) {
  fn setSize(self , rsthis: &mut QOpenGLPaintDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDevice7setSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QOpenGLPaintDevice7setSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLPaintDevice::ensureActiveTarget();
impl /*struct*/ QOpenGLPaintDevice {
  pub fn ensureActiveTarget<RetType, T: QOpenGLPaintDevice_ensureActiveTarget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.ensureActiveTarget(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_ensureActiveTarget<RetType> {
  fn ensureActiveTarget(self , rsthis: &mut QOpenGLPaintDevice) -> RetType;
}

  // proto:  void QOpenGLPaintDevice::ensureActiveTarget();
impl<'a> /*trait*/ QOpenGLPaintDevice_ensureActiveTarget<()> for () {
  fn ensureActiveTarget(self , rsthis: &mut QOpenGLPaintDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDevice18ensureActiveTargetEv()};
     unsafe {_ZN18QOpenGLPaintDevice18ensureActiveTargetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLPaintDevice::QOpenGLPaintDevice(const QSize & size);
impl<'a> /*trait*/ QOpenGLPaintDevice_NewQOpenGLPaintDevice for (QSize) {
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

