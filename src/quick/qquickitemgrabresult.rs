// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qquickitemgrabresult.h
// dst-file: /src/quick/qquickitemgrabresult.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::super::core::qurl::QUrl; // 771
use super::super::gui::qimage::QImage; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQuickItemGrabResult_Class_Size() -> c_int;
  // proto:  bool QQuickItemGrabResult::saveToFile(const QString & fileName);
  fn _ZN20QQuickItemGrabResult10saveToFileERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QUrl QQuickItemGrabResult::url();
  fn _ZNK20QQuickItemGrabResult3urlEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQuickItemGrabResult::QQuickItemGrabResult(QObject * parent);
  fn _ZN20QQuickItemGrabResultC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QImage QQuickItemGrabResult::image();
  fn _ZNK20QQuickItemGrabResult5imageEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QQuickItemGrabResult::metaObject();
  fn _ZNK20QQuickItemGrabResult10metaObjectEv(qthis: u64 /* *mut c_void*/);
  fn QQuickItemGrabResult_SlotProxy_connect__ZN20QQuickItemGrabResult5readyEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QQuickItemGrabResult)=1
#[derive(Default)]
pub struct QQuickItemGrabResult {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _ready: QQuickItemGrabResult_ready_signal,
}

impl /*struct*/ QQuickItemGrabResult {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQuickItemGrabResult {
    return QQuickItemGrabResult{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQuickItemGrabResult {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QQuickItemGrabResult {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  bool QQuickItemGrabResult::saveToFile(const QString & fileName);
impl /*struct*/ QQuickItemGrabResult {
  pub fn saveToFile<RetType, T: QQuickItemGrabResult_saveToFile<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.saveToFile(self);
    // return 1;
  }
}

pub trait QQuickItemGrabResult_saveToFile<RetType> {
  fn saveToFile(self , rsthis: & QQuickItemGrabResult) -> RetType;
}

  // proto:  bool QQuickItemGrabResult::saveToFile(const QString & fileName);
impl<'a> /*trait*/ QQuickItemGrabResult_saveToFile<i8> for (&'a QString) {
  fn saveToFile(self , rsthis: & QQuickItemGrabResult) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QQuickItemGrabResult10saveToFileERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN20QQuickItemGrabResult10saveToFileERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QUrl QQuickItemGrabResult::url();
impl /*struct*/ QQuickItemGrabResult {
  pub fn url<RetType, T: QQuickItemGrabResult_url<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.url(self);
    // return 1;
  }
}

pub trait QQuickItemGrabResult_url<RetType> {
  fn url(self , rsthis: & QQuickItemGrabResult) -> RetType;
}

  // proto:  QUrl QQuickItemGrabResult::url();
impl<'a> /*trait*/ QQuickItemGrabResult_url<QUrl> for () {
  fn url(self , rsthis: & QQuickItemGrabResult) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QQuickItemGrabResult3urlEv()};
    let mut ret = unsafe {_ZNK20QQuickItemGrabResult3urlEv(rsthis.qclsinst)};
    let mut ret1 = QUrl::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickItemGrabResult::QQuickItemGrabResult(QObject * parent);
impl /*struct*/ QQuickItemGrabResult {
  pub fn new<T: QQuickItemGrabResult_new>(value: T) -> QQuickItemGrabResult {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQuickItemGrabResult_new {
  fn new(self) -> QQuickItemGrabResult;
}

  // proto:  void QQuickItemGrabResult::QQuickItemGrabResult(QObject * parent);
impl<'a> /*trait*/ QQuickItemGrabResult_new for (&'a QObject) {
  fn new(self) -> QQuickItemGrabResult {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QQuickItemGrabResultC2EP7QObject()};
    let ctysz: c_int = unsafe{QQuickItemGrabResult_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QQuickItemGrabResultC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQuickItemGrabResult{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QImage QQuickItemGrabResult::image();
impl /*struct*/ QQuickItemGrabResult {
  pub fn image<RetType, T: QQuickItemGrabResult_image<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.image(self);
    // return 1;
  }
}

pub trait QQuickItemGrabResult_image<RetType> {
  fn image(self , rsthis: & QQuickItemGrabResult) -> RetType;
}

  // proto:  QImage QQuickItemGrabResult::image();
impl<'a> /*trait*/ QQuickItemGrabResult_image<QImage> for () {
  fn image(self , rsthis: & QQuickItemGrabResult) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QQuickItemGrabResult5imageEv()};
    let mut ret = unsafe {_ZNK20QQuickItemGrabResult5imageEv(rsthis.qclsinst)};
    let mut ret1 = QImage::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QQuickItemGrabResult::metaObject();
impl /*struct*/ QQuickItemGrabResult {
  pub fn metaObject<RetType, T: QQuickItemGrabResult_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QQuickItemGrabResult_metaObject<RetType> {
  fn metaObject(self , rsthis: & QQuickItemGrabResult) -> RetType;
}

  // proto:  const QMetaObject * QQuickItemGrabResult::metaObject();
impl<'a> /*trait*/ QQuickItemGrabResult_metaObject<()> for () {
  fn metaObject(self , rsthis: & QQuickItemGrabResult) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QQuickItemGrabResult10metaObjectEv()};
     unsafe {_ZNK20QQuickItemGrabResult10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QQuickItemGrabResult_ready
pub struct QQuickItemGrabResult_ready_signal{poi:u64}
impl /* struct */ QQuickItemGrabResult {
  pub fn ready(&self) -> QQuickItemGrabResult_ready_signal {
     return QQuickItemGrabResult_ready_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItemGrabResult_ready_signal {
  pub fn connect<T: QQuickItemGrabResult_ready_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItemGrabResult_ready_signal_connect {
  fn connect(self, sigthis: QQuickItemGrabResult_ready_signal);
}

// ready()
extern fn QQuickItemGrabResult_ready_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickItemGrabResult_ready_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickItemGrabResult_ready_signal_connect for fn() {
  fn connect(self, sigthis: QQuickItemGrabResult_ready_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItemGrabResult_ready_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItemGrabResult_SlotProxy_connect__ZN20QQuickItemGrabResult5readyEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItemGrabResult_ready_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickItemGrabResult_ready_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItemGrabResult_ready_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItemGrabResult_SlotProxy_connect__ZN20QQuickItemGrabResult5readyEv(arg0, arg1, arg2)};
  }
}
// <= body block end

