// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtGui/qclipboard.h
// dst-file: /src/gui/qclipboard.rs
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
use super::super::core::qobject::*; // 771
use std::ops::Deref;
use super::super::core::qstring::*; // 771
use super::qpixmap::*; // 773
use super::super::core::qmimedata::*; // 771
use super::super::core::qobjectdefs::*; // 771
use super::qimage::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QClipboard_Class_Size() -> c_int;
  // proto:  bool QClipboard::supportsFindBuffer();
  fn C_ZNK10QClipboard18supportsFindBufferEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QClipboard::ownsFindBuffer();
  fn C_ZNK10QClipboard14ownsFindBufferEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QClipboard::ownsClipboard();
  fn C_ZNK10QClipboard13ownsClipboardEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QMetaObject * QClipboard::metaObject();
  fn C_ZNK10QClipboard10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QClipboard::supportsSelection();
  fn C_ZNK10QClipboard17supportsSelectionEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QClipboard::ownsSelection();
  fn C_ZNK10QClipboard13ownsSelectionEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QClipboard_SlotProxy_connect__ZN10QClipboard16selectionChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QClipboard_SlotProxy_connect__ZN10QClipboard11dataChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QClipboard_SlotProxy_connect__ZN10QClipboard17findBufferChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QClipboard)=1
#[derive(Default)]
pub struct QClipboard {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _changed: QClipboard_changed_signal,
  pub _findBufferChanged: QClipboard_findBufferChanged_signal,
  pub _selectionChanged: QClipboard_selectionChanged_signal,
  pub _dataChanged: QClipboard_dataChanged_signal,
}

impl /*struct*/ QClipboard {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QClipboard {
    return QClipboard{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QClipboard {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QClipboard {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  bool QClipboard::supportsFindBuffer();
impl /*struct*/ QClipboard {
  pub fn supportsFindBuffer<RetType, T: QClipboard_supportsFindBuffer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.supportsFindBuffer(self);
    // return 1;
  }
}

pub trait QClipboard_supportsFindBuffer<RetType> {
  fn supportsFindBuffer(self , rsthis: & QClipboard) -> RetType;
}

  // proto:  bool QClipboard::supportsFindBuffer();
impl<'a> /*trait*/ QClipboard_supportsFindBuffer<i8> for () {
  fn supportsFindBuffer(self , rsthis: & QClipboard) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QClipboard18supportsFindBufferEv()};
    let mut ret = unsafe {C_ZNK10QClipboard18supportsFindBufferEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QClipboard::ownsFindBuffer();
impl /*struct*/ QClipboard {
  pub fn ownsFindBuffer<RetType, T: QClipboard_ownsFindBuffer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ownsFindBuffer(self);
    // return 1;
  }
}

pub trait QClipboard_ownsFindBuffer<RetType> {
  fn ownsFindBuffer(self , rsthis: & QClipboard) -> RetType;
}

  // proto:  bool QClipboard::ownsFindBuffer();
impl<'a> /*trait*/ QClipboard_ownsFindBuffer<i8> for () {
  fn ownsFindBuffer(self , rsthis: & QClipboard) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QClipboard14ownsFindBufferEv()};
    let mut ret = unsafe {C_ZNK10QClipboard14ownsFindBufferEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QClipboard::ownsClipboard();
impl /*struct*/ QClipboard {
  pub fn ownsClipboard<RetType, T: QClipboard_ownsClipboard<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ownsClipboard(self);
    // return 1;
  }
}

pub trait QClipboard_ownsClipboard<RetType> {
  fn ownsClipboard(self , rsthis: & QClipboard) -> RetType;
}

  // proto:  bool QClipboard::ownsClipboard();
impl<'a> /*trait*/ QClipboard_ownsClipboard<i8> for () {
  fn ownsClipboard(self , rsthis: & QClipboard) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QClipboard13ownsClipboardEv()};
    let mut ret = unsafe {C_ZNK10QClipboard13ownsClipboardEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  const QMetaObject * QClipboard::metaObject();
impl /*struct*/ QClipboard {
  pub fn metaObject<RetType, T: QClipboard_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QClipboard_metaObject<RetType> {
  fn metaObject(self , rsthis: & QClipboard) -> RetType;
}

  // proto:  const QMetaObject * QClipboard::metaObject();
impl<'a> /*trait*/ QClipboard_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QClipboard) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QClipboard10metaObjectEv()};
    let mut ret = unsafe {C_ZNK10QClipboard10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QClipboard::supportsSelection();
impl /*struct*/ QClipboard {
  pub fn supportsSelection<RetType, T: QClipboard_supportsSelection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.supportsSelection(self);
    // return 1;
  }
}

pub trait QClipboard_supportsSelection<RetType> {
  fn supportsSelection(self , rsthis: & QClipboard) -> RetType;
}

  // proto:  bool QClipboard::supportsSelection();
impl<'a> /*trait*/ QClipboard_supportsSelection<i8> for () {
  fn supportsSelection(self , rsthis: & QClipboard) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QClipboard17supportsSelectionEv()};
    let mut ret = unsafe {C_ZNK10QClipboard17supportsSelectionEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QClipboard::ownsSelection();
impl /*struct*/ QClipboard {
  pub fn ownsSelection<RetType, T: QClipboard_ownsSelection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ownsSelection(self);
    // return 1;
  }
}

pub trait QClipboard_ownsSelection<RetType> {
  fn ownsSelection(self , rsthis: & QClipboard) -> RetType;
}

  // proto:  bool QClipboard::ownsSelection();
impl<'a> /*trait*/ QClipboard_ownsSelection<i8> for () {
  fn ownsSelection(self , rsthis: & QClipboard) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QClipboard13ownsSelectionEv()};
    let mut ret = unsafe {C_ZNK10QClipboard13ownsSelectionEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

#[derive(Default)] // for QClipboard_changed
pub struct QClipboard_changed_signal{poi:u64}
impl /* struct */ QClipboard {
  pub fn changed(&self) -> QClipboard_changed_signal {
     return QClipboard_changed_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QClipboard_changed_signal {
  pub fn connect<T: QClipboard_changed_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QClipboard_changed_signal_connect {
  fn connect(self, sigthis: QClipboard_changed_signal);
}

#[derive(Default)] // for QClipboard_findBufferChanged
pub struct QClipboard_findBufferChanged_signal{poi:u64}
impl /* struct */ QClipboard {
  pub fn findBufferChanged(&self) -> QClipboard_findBufferChanged_signal {
     return QClipboard_findBufferChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QClipboard_findBufferChanged_signal {
  pub fn connect<T: QClipboard_findBufferChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QClipboard_findBufferChanged_signal_connect {
  fn connect(self, sigthis: QClipboard_findBufferChanged_signal);
}

#[derive(Default)] // for QClipboard_selectionChanged
pub struct QClipboard_selectionChanged_signal{poi:u64}
impl /* struct */ QClipboard {
  pub fn selectionChanged(&self) -> QClipboard_selectionChanged_signal {
     return QClipboard_selectionChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QClipboard_selectionChanged_signal {
  pub fn connect<T: QClipboard_selectionChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QClipboard_selectionChanged_signal_connect {
  fn connect(self, sigthis: QClipboard_selectionChanged_signal);
}

#[derive(Default)] // for QClipboard_dataChanged
pub struct QClipboard_dataChanged_signal{poi:u64}
impl /* struct */ QClipboard {
  pub fn dataChanged(&self) -> QClipboard_dataChanged_signal {
     return QClipboard_dataChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QClipboard_dataChanged_signal {
  pub fn connect<T: QClipboard_dataChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QClipboard_dataChanged_signal_connect {
  fn connect(self, sigthis: QClipboard_dataChanged_signal);
}

// selectionChanged()
extern fn QClipboard_selectionChanged_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QClipboard_selectionChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QClipboard_selectionChanged_signal_connect for fn() {
  fn connect(self, sigthis: QClipboard_selectionChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QClipboard_selectionChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QClipboard_SlotProxy_connect__ZN10QClipboard16selectionChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QClipboard_selectionChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QClipboard_selectionChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QClipboard_selectionChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QClipboard_SlotProxy_connect__ZN10QClipboard16selectionChangedEv(arg0, arg1, arg2)};
  }
}
// dataChanged()
extern fn QClipboard_dataChanged_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QClipboard_dataChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QClipboard_dataChanged_signal_connect for fn() {
  fn connect(self, sigthis: QClipboard_dataChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QClipboard_dataChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QClipboard_SlotProxy_connect__ZN10QClipboard11dataChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QClipboard_dataChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QClipboard_dataChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QClipboard_dataChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QClipboard_SlotProxy_connect__ZN10QClipboard11dataChangedEv(arg0, arg1, arg2)};
  }
}
// findBufferChanged()
extern fn QClipboard_findBufferChanged_signal_connect_cb_2(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QClipboard_findBufferChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QClipboard_findBufferChanged_signal_connect for fn() {
  fn connect(self, sigthis: QClipboard_findBufferChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QClipboard_findBufferChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QClipboard_SlotProxy_connect__ZN10QClipboard17findBufferChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QClipboard_findBufferChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QClipboard_findBufferChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QClipboard_findBufferChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QClipboard_SlotProxy_connect__ZN10QClipboard17findBufferChangedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

