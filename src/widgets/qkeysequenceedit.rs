// auto generated, do not modify.
// created: Sun Dec 27 22:52:03 2015
// src-file: /QtWidgets/qkeysequenceedit.h
// dst-file: /src/widgets/qkeysequenceedit.rs
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
use super::qwidget::QWidget; // 773
use std::ops::Deref;
use super::super::gui::qkeysequence::QKeySequence; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QKeySequenceEdit_Class_Size() -> c_int;
  // proto:  void QKeySequenceEdit::QKeySequenceEdit(const QKeySequenceEdit & );
  fn dector_ZN16QKeySequenceEditC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN16QKeySequenceEditC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QKeySequenceEdit::QKeySequenceEdit(const QKeySequence & keySequence, QWidget * parent);
  fn dector_ZN16QKeySequenceEditC1ERK12QKeySequenceP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN16QKeySequenceEditC1ERK12QKeySequenceP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QKeySequenceEdit::clear();
  fn _ZN16QKeySequenceEdit5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QKeySequenceEdit::setKeySequence(const QKeySequence & keySequence);
  fn _ZN16QKeySequenceEdit14setKeySequenceERK12QKeySequence(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QKeySequence QKeySequenceEdit::keySequence();
  fn _ZNK16QKeySequenceEdit11keySequenceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QKeySequenceEdit::~QKeySequenceEdit();
  fn _ZN16QKeySequenceEditD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QKeySequenceEdit::keySequenceChanged(const QKeySequence & keySequence);
  fn _ZN16QKeySequenceEdit18keySequenceChangedERK12QKeySequence(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QKeySequenceEdit::editingFinished();
  fn _ZN16QKeySequenceEdit15editingFinishedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QKeySequenceEdit::QKeySequenceEdit(QWidget * parent);
  fn dector_ZN16QKeySequenceEditC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN16QKeySequenceEditC1EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QKeySequenceEdit::metaObject();
  fn _ZNK16QKeySequenceEdit10metaObjectEv(qthis: u64 /* *mut c_void*/);
  fn QKeySequenceEdit_SlotProxy_connect__ZN16QKeySequenceEdit18keySequenceChangedERK12QKeySequence(qthis: *mut c_void, fptr: *mut c_void);
  fn QKeySequenceEdit_SlotProxy_connect__ZN16QKeySequenceEdit15editingFinishedEv(qthis: *mut c_void, fptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QKeySequenceEdit)=1
#[derive(Default)]
pub struct QKeySequenceEdit {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _editingFinished_1: QKeySequenceEdit_editingFinished_signal,
  pub _keySequenceChanged_1: QKeySequenceEdit_keySequenceChanged_signal,
}

impl /*struct*/ QKeySequenceEdit {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QKeySequenceEdit {
    return QKeySequenceEdit{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QKeySequenceEdit {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QKeySequenceEdit {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  void QKeySequenceEdit::QKeySequenceEdit(const QKeySequenceEdit & );
impl /*struct*/ QKeySequenceEdit {
  pub fn New<T: QKeySequenceEdit_New>(value: T) -> QKeySequenceEdit {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QKeySequenceEdit_New {
  fn New(self) -> QKeySequenceEdit;
}

  // proto:  void QKeySequenceEdit::QKeySequenceEdit(const QKeySequenceEdit & );
impl<'a> /*trait*/ QKeySequenceEdit_New for (&'a QKeySequenceEdit) {
  fn New(self) -> QKeySequenceEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEditC1ERKS_()};
    let ctysz: c_int = unsafe{QKeySequenceEdit_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN16QKeySequenceEditC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN16QKeySequenceEditC1ERKS_(arg0)} as u64;
    let rsthis = QKeySequenceEdit{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QKeySequenceEdit::QKeySequenceEdit(const QKeySequence & keySequence, QWidget * parent);
impl<'a> /*trait*/ QKeySequenceEdit_New for (&'a QKeySequence, &'a QWidget) {
  fn New(self) -> QKeySequenceEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEditC1ERK12QKeySequenceP7QWidget()};
    let ctysz: c_int = unsafe{QKeySequenceEdit_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN16QKeySequenceEditC1ERK12QKeySequenceP7QWidget(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN16QKeySequenceEditC1ERK12QKeySequenceP7QWidget(arg0, arg1)} as u64;
    let rsthis = QKeySequenceEdit{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QKeySequenceEdit::clear();
impl /*struct*/ QKeySequenceEdit {
  pub fn clear<RetType, T: QKeySequenceEdit_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QKeySequenceEdit_clear<RetType> {
  fn clear(self , rsthis: & QKeySequenceEdit) -> RetType;
}

  // proto:  void QKeySequenceEdit::clear();
impl<'a> /*trait*/ QKeySequenceEdit_clear<()> for () {
  fn clear(self , rsthis: & QKeySequenceEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEdit5clearEv()};
     unsafe {_ZN16QKeySequenceEdit5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QKeySequenceEdit::setKeySequence(const QKeySequence & keySequence);
impl /*struct*/ QKeySequenceEdit {
  pub fn setKeySequence<RetType, T: QKeySequenceEdit_setKeySequence<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setKeySequence(self);
    // return 1;
  }
}

pub trait QKeySequenceEdit_setKeySequence<RetType> {
  fn setKeySequence(self , rsthis: & QKeySequenceEdit) -> RetType;
}

  // proto:  void QKeySequenceEdit::setKeySequence(const QKeySequence & keySequence);
impl<'a> /*trait*/ QKeySequenceEdit_setKeySequence<()> for (&'a QKeySequence) {
  fn setKeySequence(self , rsthis: & QKeySequenceEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEdit14setKeySequenceERK12QKeySequence()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QKeySequenceEdit14setKeySequenceERK12QKeySequence(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QKeySequence QKeySequenceEdit::keySequence();
impl /*struct*/ QKeySequenceEdit {
  pub fn keySequence<RetType, T: QKeySequenceEdit_keySequence<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keySequence(self);
    // return 1;
  }
}

pub trait QKeySequenceEdit_keySequence<RetType> {
  fn keySequence(self , rsthis: & QKeySequenceEdit) -> RetType;
}

  // proto:  QKeySequence QKeySequenceEdit::keySequence();
impl<'a> /*trait*/ QKeySequenceEdit_keySequence<QKeySequence> for () {
  fn keySequence(self , rsthis: & QKeySequenceEdit) -> QKeySequence {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QKeySequenceEdit11keySequenceEv()};
    let mut ret = unsafe {_ZNK16QKeySequenceEdit11keySequenceEv(rsthis.qclsinst)};
    let mut ret1 = QKeySequence::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QKeySequenceEdit::~QKeySequenceEdit();
impl /*struct*/ QKeySequenceEdit {
  pub fn Free<RetType, T: QKeySequenceEdit_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QKeySequenceEdit_Free<RetType> {
  fn Free(self , rsthis: & QKeySequenceEdit) -> RetType;
}

  // proto:  void QKeySequenceEdit::~QKeySequenceEdit();
impl<'a> /*trait*/ QKeySequenceEdit_Free<()> for () {
  fn Free(self , rsthis: & QKeySequenceEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEditD0Ev()};
     unsafe {_ZN16QKeySequenceEditD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QKeySequenceEdit::keySequenceChanged(const QKeySequence & keySequence);
impl /*struct*/ QKeySequenceEdit {
  pub fn keySequenceChanged<RetType, T: QKeySequenceEdit_keySequenceChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keySequenceChanged(self);
    // return 1;
  }
}

pub trait QKeySequenceEdit_keySequenceChanged<RetType> {
  fn keySequenceChanged(self , rsthis: & QKeySequenceEdit) -> RetType;
}

  // proto:  void QKeySequenceEdit::keySequenceChanged(const QKeySequence & keySequence);
impl<'a> /*trait*/ QKeySequenceEdit_keySequenceChanged<()> for (&'a QKeySequence) {
  fn keySequenceChanged(self , rsthis: & QKeySequenceEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEdit18keySequenceChangedERK12QKeySequence()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QKeySequenceEdit18keySequenceChangedERK12QKeySequence(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QKeySequenceEdit::editingFinished();
impl /*struct*/ QKeySequenceEdit {
  pub fn editingFinished<RetType, T: QKeySequenceEdit_editingFinished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.editingFinished(self);
    // return 1;
  }
}

pub trait QKeySequenceEdit_editingFinished<RetType> {
  fn editingFinished(self , rsthis: & QKeySequenceEdit) -> RetType;
}

  // proto:  void QKeySequenceEdit::editingFinished();
impl<'a> /*trait*/ QKeySequenceEdit_editingFinished<()> for () {
  fn editingFinished(self , rsthis: & QKeySequenceEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEdit15editingFinishedEv()};
     unsafe {_ZN16QKeySequenceEdit15editingFinishedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QKeySequenceEdit::QKeySequenceEdit(QWidget * parent);
impl<'a> /*trait*/ QKeySequenceEdit_New for (&'a QWidget) {
  fn New(self) -> QKeySequenceEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEditC1EP7QWidget()};
    let ctysz: c_int = unsafe{QKeySequenceEdit_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN16QKeySequenceEditC1EP7QWidget(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN16QKeySequenceEditC1EP7QWidget(arg0)} as u64;
    let rsthis = QKeySequenceEdit{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QKeySequenceEdit::metaObject();
impl /*struct*/ QKeySequenceEdit {
  pub fn metaObject<RetType, T: QKeySequenceEdit_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QKeySequenceEdit_metaObject<RetType> {
  fn metaObject(self , rsthis: & QKeySequenceEdit) -> RetType;
}

  // proto:  const QMetaObject * QKeySequenceEdit::metaObject();
impl<'a> /*trait*/ QKeySequenceEdit_metaObject<()> for () {
  fn metaObject(self , rsthis: & QKeySequenceEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QKeySequenceEdit10metaObjectEv()};
     unsafe {_ZNK16QKeySequenceEdit10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QKeySequenceEdit_editingFinished
pub struct QKeySequenceEdit_editingFinished_signal{poi:u64}
impl /* struct */ QKeySequenceEdit {
  pub fn editingFinished_1(self) -> QKeySequenceEdit_editingFinished_signal {
     return QKeySequenceEdit_editingFinished_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QKeySequenceEdit_editingFinished_signal {
  pub fn connect<T: QKeySequenceEdit_editingFinished_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QKeySequenceEdit_editingFinished_signal_connect {
  fn connect(self, sigthis: QKeySequenceEdit_editingFinished_signal);
}

#[derive(Default)] // for QKeySequenceEdit_keySequenceChanged
pub struct QKeySequenceEdit_keySequenceChanged_signal{poi:u64}
impl /* struct */ QKeySequenceEdit {
  pub fn keySequenceChanged_1(self) -> QKeySequenceEdit_keySequenceChanged_signal {
     return QKeySequenceEdit_keySequenceChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QKeySequenceEdit_keySequenceChanged_signal {
  pub fn connect<T: QKeySequenceEdit_keySequenceChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QKeySequenceEdit_keySequenceChanged_signal_connect {
  fn connect(self, sigthis: QKeySequenceEdit_keySequenceChanged_signal);
}

// keySequenceChanged(const class QKeySequence &)
extern fn QKeySequenceEdit_keySequenceChanged_signal_connect_cb_0(arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QKeySequenceEdit_keySequenceChanged_signal_connect for (extern fn(QKeySequence)) {
  fn connect(self, sigthis: QKeySequenceEdit_keySequenceChanged_signal) {
    // do smth...
    unsafe {QKeySequenceEdit_SlotProxy_connect__ZN16QKeySequenceEdit18keySequenceChangedERK12QKeySequence(sigthis.poi as *mut c_void, QKeySequenceEdit_keySequenceChanged_signal_connect_cb_0 as *mut c_void)};
  }
}
// editingFinished()
extern fn QKeySequenceEdit_editingFinished_signal_connect_cb_1() {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QKeySequenceEdit_editingFinished_signal_connect for (extern fn()) {
  fn connect(self, sigthis: QKeySequenceEdit_editingFinished_signal) {
    // do smth...
    unsafe {QKeySequenceEdit_SlotProxy_connect__ZN16QKeySequenceEdit15editingFinishedEv(sigthis.poi as *mut c_void, QKeySequenceEdit_editingFinished_signal_connect_cb_1 as *mut c_void)};
  }
}
// <= body block end

