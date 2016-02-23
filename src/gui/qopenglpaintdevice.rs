// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtGui/qopenglpaintdevice.h
// dst-file: /src/gui/qopenglpaintdevice.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::qpaintdevice::*; // 773
use std::ops::Deref;
use super::qpaintengine::*; // 773
use super::super::core::qsize::*; // 771
use super::qopenglcontext::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QOpenGLPaintDevice_Class_Size() -> c_int;
  // proto:  QPaintEngine * QOpenGLPaintDevice::paintEngine();
  fn C_ZNK18QOpenGLPaintDevice11paintEngineEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QOpenGLPaintDevice::size();
  fn C_ZNK18QOpenGLPaintDevice4sizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QOpenGLPaintDevice::setPaintFlipped(bool flipped);
  fn C_ZN18QOpenGLPaintDevice15setPaintFlippedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QOpenGLPaintDevice::~QOpenGLPaintDevice();
  fn C_ZN18QOpenGLPaintDeviceD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QOpenGLPaintDevice::QOpenGLPaintDevice(int width, int height);
  fn C_ZN18QOpenGLPaintDeviceC2Eii(arg0: c_int, arg1: c_int) -> u64;
  // proto:  QOpenGLContext * QOpenGLPaintDevice::context();
  fn C_ZNK18QOpenGLPaintDevice7contextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QOpenGLPaintDevice::setDevicePixelRatio(qreal devicePixelRatio);
  fn C_ZN18QOpenGLPaintDevice19setDevicePixelRatioEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QOpenGLPaintDevice::QOpenGLPaintDevice();
  fn C_ZN18QOpenGLPaintDeviceC2Ev() -> u64;
  // proto:  int QOpenGLPaintDevice::devType();
  fn C_ZNK18QOpenGLPaintDevice7devTypeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  qreal QOpenGLPaintDevice::dotsPerMeterX();
  fn C_ZNK18QOpenGLPaintDevice13dotsPerMeterXEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QOpenGLPaintDevice::setDotsPerMeterX(qreal );
  fn C_ZN18QOpenGLPaintDevice16setDotsPerMeterXEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  qreal QOpenGLPaintDevice::dotsPerMeterY();
  fn C_ZNK18QOpenGLPaintDevice13dotsPerMeterYEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QOpenGLPaintDevice::setDotsPerMeterY(qreal );
  fn C_ZN18QOpenGLPaintDevice16setDotsPerMeterYEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  bool QOpenGLPaintDevice::paintFlipped();
  fn C_ZNK18QOpenGLPaintDevice12paintFlippedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QOpenGLPaintDevice::setSize(const QSize & size);
  fn C_ZN18QOpenGLPaintDevice7setSizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QOpenGLPaintDevice::ensureActiveTarget();
  fn C_ZN18QOpenGLPaintDevice18ensureActiveTargetEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QOpenGLPaintDevice::QOpenGLPaintDevice(const QSize & size);
  fn C_ZN18QOpenGLPaintDeviceC2ERK5QSize(arg0: *mut c_void) -> u64;
} // <= ext block end

// body block begin =>
// class sizeof(QOpenGLPaintDevice)=1
#[derive(Default)]
pub struct QOpenGLPaintDevice {
  qbase: QPaintDevice,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QOpenGLPaintDevice {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLPaintDevice {
    return QOpenGLPaintDevice{qbase: QPaintDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLPaintDevice {
  type Target = QPaintDevice;

  fn deref(&self) -> &QPaintDevice {
    return & self.qbase;
  }
}
impl AsRef<QPaintDevice> for QOpenGLPaintDevice {
  fn as_ref(& self) -> & QPaintDevice {
    return & self.qbase;
  }
}
  // proto:  QPaintEngine * QOpenGLPaintDevice::paintEngine();
impl /*struct*/ QOpenGLPaintDevice {
  pub fn paintEngine<RetType, T: QOpenGLPaintDevice_paintEngine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paintEngine(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_paintEngine<RetType> {
  fn paintEngine(self , rsthis: & QOpenGLPaintDevice) -> RetType;
}

  // proto:  QPaintEngine * QOpenGLPaintDevice::paintEngine();
impl<'a> /*trait*/ QOpenGLPaintDevice_paintEngine<QPaintEngine> for () {
  fn paintEngine(self , rsthis: & QOpenGLPaintDevice) -> QPaintEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice11paintEngineEv()};
    let mut ret = unsafe {C_ZNK18QOpenGLPaintDevice11paintEngineEv(rsthis.qclsinst)};
    let mut ret1 = QPaintEngine::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QOpenGLPaintDevice::size();
impl /*struct*/ QOpenGLPaintDevice {
  pub fn size<RetType, T: QOpenGLPaintDevice_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_size<RetType> {
  fn size(self , rsthis: & QOpenGLPaintDevice) -> RetType;
}

  // proto:  QSize QOpenGLPaintDevice::size();
impl<'a> /*trait*/ QOpenGLPaintDevice_size<QSize> for () {
  fn size(self , rsthis: & QOpenGLPaintDevice) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice4sizeEv()};
    let mut ret = unsafe {C_ZNK18QOpenGLPaintDevice4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QOpenGLPaintDevice::setPaintFlipped(bool flipped);
impl /*struct*/ QOpenGLPaintDevice {
  pub fn setPaintFlipped<RetType, T: QOpenGLPaintDevice_setPaintFlipped<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPaintFlipped(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_setPaintFlipped<RetType> {
  fn setPaintFlipped(self , rsthis: & QOpenGLPaintDevice) -> RetType;
}

  // proto:  void QOpenGLPaintDevice::setPaintFlipped(bool flipped);
impl<'a> /*trait*/ QOpenGLPaintDevice_setPaintFlipped<()> for (i8) {
  fn setPaintFlipped(self , rsthis: & QOpenGLPaintDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDevice15setPaintFlippedEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN18QOpenGLPaintDevice15setPaintFlippedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLPaintDevice::~QOpenGLPaintDevice();
impl /*struct*/ QOpenGLPaintDevice {
  pub fn free<RetType, T: QOpenGLPaintDevice_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_free<RetType> {
  fn free(self , rsthis: & QOpenGLPaintDevice) -> RetType;
}

  // proto:  void QOpenGLPaintDevice::~QOpenGLPaintDevice();
impl<'a> /*trait*/ QOpenGLPaintDevice_free<()> for () {
  fn free(self , rsthis: & QOpenGLPaintDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDeviceD2Ev()};
     unsafe {C_ZN18QOpenGLPaintDeviceD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLPaintDevice::QOpenGLPaintDevice(int width, int height);
impl /*struct*/ QOpenGLPaintDevice {
  pub fn new<T: QOpenGLPaintDevice_new>(value: T) -> QOpenGLPaintDevice {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_new {
  fn new(self) -> QOpenGLPaintDevice;
}

  // proto:  void QOpenGLPaintDevice::QOpenGLPaintDevice(int width, int height);
impl<'a> /*trait*/ QOpenGLPaintDevice_new for (i32, i32) {
  fn new(self) -> QOpenGLPaintDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDeviceC2Eii()};
    let ctysz: c_int = unsafe{QOpenGLPaintDevice_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let qthis: u64 = unsafe {C_ZN18QOpenGLPaintDeviceC2Eii(arg0, arg1)};
    let rsthis = QOpenGLPaintDevice{qbase: QPaintDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QOpenGLContext * QOpenGLPaintDevice::context();
impl /*struct*/ QOpenGLPaintDevice {
  pub fn context<RetType, T: QOpenGLPaintDevice_context<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.context(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_context<RetType> {
  fn context(self , rsthis: & QOpenGLPaintDevice) -> RetType;
}

  // proto:  QOpenGLContext * QOpenGLPaintDevice::context();
impl<'a> /*trait*/ QOpenGLPaintDevice_context<QOpenGLContext> for () {
  fn context(self , rsthis: & QOpenGLPaintDevice) -> QOpenGLContext {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice7contextEv()};
    let mut ret = unsafe {C_ZNK18QOpenGLPaintDevice7contextEv(rsthis.qclsinst)};
    let mut ret1 = QOpenGLContext::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QOpenGLPaintDevice::setDevicePixelRatio(qreal devicePixelRatio);
impl /*struct*/ QOpenGLPaintDevice {
  pub fn setDevicePixelRatio<RetType, T: QOpenGLPaintDevice_setDevicePixelRatio<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDevicePixelRatio(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_setDevicePixelRatio<RetType> {
  fn setDevicePixelRatio(self , rsthis: & QOpenGLPaintDevice) -> RetType;
}

  // proto:  void QOpenGLPaintDevice::setDevicePixelRatio(qreal devicePixelRatio);
impl<'a> /*trait*/ QOpenGLPaintDevice_setDevicePixelRatio<()> for (f64) {
  fn setDevicePixelRatio(self , rsthis: & QOpenGLPaintDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDevice19setDevicePixelRatioEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN18QOpenGLPaintDevice19setDevicePixelRatioEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLPaintDevice::QOpenGLPaintDevice();
impl<'a> /*trait*/ QOpenGLPaintDevice_new for () {
  fn new(self) -> QOpenGLPaintDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDeviceC2Ev()};
    let ctysz: c_int = unsafe{QOpenGLPaintDevice_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN18QOpenGLPaintDeviceC2Ev()};
    let rsthis = QOpenGLPaintDevice{qbase: QPaintDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QOpenGLPaintDevice::devType();
impl /*struct*/ QOpenGLPaintDevice {
  pub fn devType<RetType, T: QOpenGLPaintDevice_devType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.devType(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_devType<RetType> {
  fn devType(self , rsthis: & QOpenGLPaintDevice) -> RetType;
}

  // proto:  int QOpenGLPaintDevice::devType();
impl<'a> /*trait*/ QOpenGLPaintDevice_devType<i32> for () {
  fn devType(self , rsthis: & QOpenGLPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice7devTypeEv()};
    let mut ret = unsafe {C_ZNK18QOpenGLPaintDevice7devTypeEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  qreal QOpenGLPaintDevice::dotsPerMeterX();
impl /*struct*/ QOpenGLPaintDevice {
  pub fn dotsPerMeterX<RetType, T: QOpenGLPaintDevice_dotsPerMeterX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dotsPerMeterX(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_dotsPerMeterX<RetType> {
  fn dotsPerMeterX(self , rsthis: & QOpenGLPaintDevice) -> RetType;
}

  // proto:  qreal QOpenGLPaintDevice::dotsPerMeterX();
impl<'a> /*trait*/ QOpenGLPaintDevice_dotsPerMeterX<f64> for () {
  fn dotsPerMeterX(self , rsthis: & QOpenGLPaintDevice) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice13dotsPerMeterXEv()};
    let mut ret = unsafe {C_ZNK18QOpenGLPaintDevice13dotsPerMeterXEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QOpenGLPaintDevice::setDotsPerMeterX(qreal );
impl /*struct*/ QOpenGLPaintDevice {
  pub fn setDotsPerMeterX<RetType, T: QOpenGLPaintDevice_setDotsPerMeterX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDotsPerMeterX(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_setDotsPerMeterX<RetType> {
  fn setDotsPerMeterX(self , rsthis: & QOpenGLPaintDevice) -> RetType;
}

  // proto:  void QOpenGLPaintDevice::setDotsPerMeterX(qreal );
impl<'a> /*trait*/ QOpenGLPaintDevice_setDotsPerMeterX<()> for (f64) {
  fn setDotsPerMeterX(self , rsthis: & QOpenGLPaintDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDevice16setDotsPerMeterXEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN18QOpenGLPaintDevice16setDotsPerMeterXEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QOpenGLPaintDevice::dotsPerMeterY();
impl /*struct*/ QOpenGLPaintDevice {
  pub fn dotsPerMeterY<RetType, T: QOpenGLPaintDevice_dotsPerMeterY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dotsPerMeterY(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_dotsPerMeterY<RetType> {
  fn dotsPerMeterY(self , rsthis: & QOpenGLPaintDevice) -> RetType;
}

  // proto:  qreal QOpenGLPaintDevice::dotsPerMeterY();
impl<'a> /*trait*/ QOpenGLPaintDevice_dotsPerMeterY<f64> for () {
  fn dotsPerMeterY(self , rsthis: & QOpenGLPaintDevice) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice13dotsPerMeterYEv()};
    let mut ret = unsafe {C_ZNK18QOpenGLPaintDevice13dotsPerMeterYEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QOpenGLPaintDevice::setDotsPerMeterY(qreal );
impl /*struct*/ QOpenGLPaintDevice {
  pub fn setDotsPerMeterY<RetType, T: QOpenGLPaintDevice_setDotsPerMeterY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDotsPerMeterY(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_setDotsPerMeterY<RetType> {
  fn setDotsPerMeterY(self , rsthis: & QOpenGLPaintDevice) -> RetType;
}

  // proto:  void QOpenGLPaintDevice::setDotsPerMeterY(qreal );
impl<'a> /*trait*/ QOpenGLPaintDevice_setDotsPerMeterY<()> for (f64) {
  fn setDotsPerMeterY(self , rsthis: & QOpenGLPaintDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDevice16setDotsPerMeterYEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN18QOpenGLPaintDevice16setDotsPerMeterYEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QOpenGLPaintDevice::paintFlipped();
impl /*struct*/ QOpenGLPaintDevice {
  pub fn paintFlipped<RetType, T: QOpenGLPaintDevice_paintFlipped<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paintFlipped(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_paintFlipped<RetType> {
  fn paintFlipped(self , rsthis: & QOpenGLPaintDevice) -> RetType;
}

  // proto:  bool QOpenGLPaintDevice::paintFlipped();
impl<'a> /*trait*/ QOpenGLPaintDevice_paintFlipped<i8> for () {
  fn paintFlipped(self , rsthis: & QOpenGLPaintDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLPaintDevice12paintFlippedEv()};
    let mut ret = unsafe {C_ZNK18QOpenGLPaintDevice12paintFlippedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QOpenGLPaintDevice::setSize(const QSize & size);
impl /*struct*/ QOpenGLPaintDevice {
  pub fn setSize<RetType, T: QOpenGLPaintDevice_setSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSize(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_setSize<RetType> {
  fn setSize(self , rsthis: & QOpenGLPaintDevice) -> RetType;
}

  // proto:  void QOpenGLPaintDevice::setSize(const QSize & size);
impl<'a> /*trait*/ QOpenGLPaintDevice_setSize<()> for (&'a QSize) {
  fn setSize(self , rsthis: & QOpenGLPaintDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDevice7setSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QOpenGLPaintDevice7setSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLPaintDevice::ensureActiveTarget();
impl /*struct*/ QOpenGLPaintDevice {
  pub fn ensureActiveTarget<RetType, T: QOpenGLPaintDevice_ensureActiveTarget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ensureActiveTarget(self);
    // return 1;
  }
}

pub trait QOpenGLPaintDevice_ensureActiveTarget<RetType> {
  fn ensureActiveTarget(self , rsthis: & QOpenGLPaintDevice) -> RetType;
}

  // proto:  void QOpenGLPaintDevice::ensureActiveTarget();
impl<'a> /*trait*/ QOpenGLPaintDevice_ensureActiveTarget<()> for () {
  fn ensureActiveTarget(self , rsthis: & QOpenGLPaintDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDevice18ensureActiveTargetEv()};
     unsafe {C_ZN18QOpenGLPaintDevice18ensureActiveTargetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLPaintDevice::QOpenGLPaintDevice(const QSize & size);
impl<'a> /*trait*/ QOpenGLPaintDevice_new for (&'a QSize) {
  fn new(self) -> QOpenGLPaintDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLPaintDeviceC2ERK5QSize()};
    let ctysz: c_int = unsafe{QOpenGLPaintDevice_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN18QOpenGLPaintDeviceC2ERK5QSize(arg0)};
    let rsthis = QOpenGLPaintDevice{qbase: QPaintDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

