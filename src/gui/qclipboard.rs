// auto generated, do not modify.
// created: Sun Dec 27 22:52:02 2015
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::qpixmap::QPixmap; // 773
use super::super::core::qmimedata::QMimeData; // 771
use super::qimage::QImage; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QClipboard_Class_Size() -> c_int;
  // proto:  void QClipboard::~QClipboard();
  fn _ZN10QClipboardD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QClipboard::QClipboard(QObject * parent);
  fn dector_ZN10QClipboardC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN10QClipboardC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QClipboard::QClipboard(const QClipboard & );
  fn dector_ZN10QClipboardC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN10QClipboardC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QClipboard::supportsFindBuffer();
  fn _ZNK10QClipboard18supportsFindBufferEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QClipboard::selectionChanged();
  fn _ZN10QClipboard16selectionChangedEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QClipboard::ownsFindBuffer();
  fn _ZNK10QClipboard14ownsFindBufferEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QClipboard::dataChanged();
  fn _ZN10QClipboard11dataChangedEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QClipboard::ownsClipboard();
  fn _ZNK10QClipboard13ownsClipboardEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QMetaObject * QClipboard::metaObject();
  fn _ZNK10QClipboard10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QClipboard::supportsSelection();
  fn _ZNK10QClipboard17supportsSelectionEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QClipboard::ownsSelection();
  fn _ZNK10QClipboard13ownsSelectionEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QClipboard::findBufferChanged();
  fn _ZN10QClipboard17findBufferChangedEv(qthis: u64 /* *mut c_void*/);
  fn QClipboard_SlotProxy_connect__ZN10QClipboard7changedENS_4ModeE(qthis: *mut c_void, fptr: *mut c_void);
  fn QClipboard_SlotProxy_connect__ZN10QClipboard16selectionChangedEv(qthis: *mut c_void, fptr: *mut c_void);
  fn QClipboard_SlotProxy_connect__ZN10QClipboard11dataChangedEv(qthis: *mut c_void, fptr: *mut c_void);
  fn QClipboard_SlotProxy_connect__ZN10QClipboard17findBufferChangedEv(qthis: *mut c_void, fptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QClipboard)=1
#[derive(Default)]
pub struct QClipboard {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _changed_1: QClipboard_changed_signal,
  pub _findBufferChanged_1: QClipboard_findBufferChanged_signal,
  pub _selectionChanged_1: QClipboard_selectionChanged_signal,
  pub _dataChanged_1: QClipboard_dataChanged_signal,
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
  // proto:  void QClipboard::~QClipboard();
impl /*struct*/ QClipboard {
  pub fn Free<RetType, T: QClipboard_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QClipboard_Free<RetType> {
  fn Free(self , rsthis: & QClipboard) -> RetType;
}

  // proto:  void QClipboard::~QClipboard();
impl<'a> /*trait*/ QClipboard_Free<()> for () {
  fn Free(self , rsthis: & QClipboard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QClipboardD0Ev()};
     unsafe {_ZN10QClipboardD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QClipboard::QClipboard(QObject * parent);
impl /*struct*/ QClipboard {
  pub fn New<T: QClipboard_New>(value: T) -> QClipboard {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QClipboard_New {
  fn New(self) -> QClipboard;
}

  // proto:  void QClipboard::QClipboard(QObject * parent);
impl<'a> /*trait*/ QClipboard_New for (&'a QObject) {
  fn New(self) -> QClipboard {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QClipboardC1EP7QObject()};
    let ctysz: c_int = unsafe{QClipboard_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN10QClipboardC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN10QClipboardC1EP7QObject(arg0)} as u64;
    let rsthis = QClipboard{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QClipboard::QClipboard(const QClipboard & );
impl<'a> /*trait*/ QClipboard_New for (&'a QClipboard) {
  fn New(self) -> QClipboard {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QClipboardC1ERKS_()};
    let ctysz: c_int = unsafe{QClipboard_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN10QClipboardC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN10QClipboardC1ERKS_(arg0)} as u64;
    let rsthis = QClipboard{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
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
    let mut ret = unsafe {_ZNK10QClipboard18supportsFindBufferEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QClipboard::selectionChanged();
impl /*struct*/ QClipboard {
  pub fn selectionChanged<RetType, T: QClipboard_selectionChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectionChanged(self);
    // return 1;
  }
}

pub trait QClipboard_selectionChanged<RetType> {
  fn selectionChanged(self , rsthis: & QClipboard) -> RetType;
}

  // proto:  void QClipboard::selectionChanged();
impl<'a> /*trait*/ QClipboard_selectionChanged<()> for () {
  fn selectionChanged(self , rsthis: & QClipboard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QClipboard16selectionChangedEv()};
     unsafe {_ZN10QClipboard16selectionChangedEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {_ZNK10QClipboard14ownsFindBufferEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QClipboard::dataChanged();
impl /*struct*/ QClipboard {
  pub fn dataChanged<RetType, T: QClipboard_dataChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dataChanged(self);
    // return 1;
  }
}

pub trait QClipboard_dataChanged<RetType> {
  fn dataChanged(self , rsthis: & QClipboard) -> RetType;
}

  // proto:  void QClipboard::dataChanged();
impl<'a> /*trait*/ QClipboard_dataChanged<()> for () {
  fn dataChanged(self , rsthis: & QClipboard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QClipboard11dataChangedEv()};
     unsafe {_ZN10QClipboard11dataChangedEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {_ZNK10QClipboard13ownsClipboardEv(rsthis.qclsinst)};
    return ret as i8;
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
impl<'a> /*trait*/ QClipboard_metaObject<()> for () {
  fn metaObject(self , rsthis: & QClipboard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QClipboard10metaObjectEv()};
     unsafe {_ZNK10QClipboard10metaObjectEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {_ZNK10QClipboard17supportsSelectionEv(rsthis.qclsinst)};
    return ret as i8;
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
    let mut ret = unsafe {_ZNK10QClipboard13ownsSelectionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QClipboard::findBufferChanged();
impl /*struct*/ QClipboard {
  pub fn findBufferChanged<RetType, T: QClipboard_findBufferChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.findBufferChanged(self);
    // return 1;
  }
}

pub trait QClipboard_findBufferChanged<RetType> {
  fn findBufferChanged(self , rsthis: & QClipboard) -> RetType;
}

  // proto:  void QClipboard::findBufferChanged();
impl<'a> /*trait*/ QClipboard_findBufferChanged<()> for () {
  fn findBufferChanged(self , rsthis: & QClipboard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QClipboard17findBufferChangedEv()};
     unsafe {_ZN10QClipboard17findBufferChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QClipboard_changed
pub struct QClipboard_changed_signal{poi:u64}
impl /* struct */ QClipboard {
  pub fn changed_1(self) -> QClipboard_changed_signal {
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
  pub fn findBufferChanged_1(self) -> QClipboard_findBufferChanged_signal {
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
  pub fn selectionChanged_1(self) -> QClipboard_selectionChanged_signal {
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
  pub fn dataChanged_1(self) -> QClipboard_dataChanged_signal {
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

// changed(class QClipboard::Mode)
extern fn QClipboard_changed_signal_connect_cb_0(arg0: c_int) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QClipboard_changed_signal_connect for (extern fn(i32)) {
  fn connect(self, sigthis: QClipboard_changed_signal) {
    // do smth...
    unsafe {QClipboard_SlotProxy_connect__ZN10QClipboard7changedENS_4ModeE(sigthis.poi as *mut c_void, QClipboard_changed_signal_connect_cb_0 as *mut c_void)};
  }
}
// selectionChanged()
extern fn QClipboard_selectionChanged_signal_connect_cb_1() {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QClipboard_selectionChanged_signal_connect for (extern fn()) {
  fn connect(self, sigthis: QClipboard_selectionChanged_signal) {
    // do smth...
    unsafe {QClipboard_SlotProxy_connect__ZN10QClipboard16selectionChangedEv(sigthis.poi as *mut c_void, QClipboard_selectionChanged_signal_connect_cb_1 as *mut c_void)};
  }
}
// dataChanged()
extern fn QClipboard_dataChanged_signal_connect_cb_2() {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QClipboard_dataChanged_signal_connect for (extern fn()) {
  fn connect(self, sigthis: QClipboard_dataChanged_signal) {
    // do smth...
    unsafe {QClipboard_SlotProxy_connect__ZN10QClipboard11dataChangedEv(sigthis.poi as *mut c_void, QClipboard_dataChanged_signal_connect_cb_2 as *mut c_void)};
  }
}
// findBufferChanged()
extern fn QClipboard_findBufferChanged_signal_connect_cb_3() {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QClipboard_findBufferChanged_signal_connect for (extern fn()) {
  fn connect(self, sigthis: QClipboard_findBufferChanged_signal) {
    // do smth...
    unsafe {QClipboard_SlotProxy_connect__ZN10QClipboard17findBufferChangedEv(sigthis.poi as *mut c_void, QClipboard_findBufferChanged_signal_connect_cb_3 as *mut c_void)};
  }
}
// <= body block end

