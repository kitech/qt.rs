// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
// src-file: /QtWidgets/qcheckbox.h
// dst-file: /src/widgets/qcheckbox.rs
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
use super::qabstractbutton::QAbstractButton; // 773
use std::ops::Deref;
use super::super::core::qsize::QSize; // 771
use super::qwidget::QWidget; // 773
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QCheckBox_Class_Size() -> c_int;
  // proto:  const QMetaObject * QCheckBox::metaObject();
  fn _ZNK9QCheckBox10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QCheckBox::minimumSizeHint();
  fn _ZNK9QCheckBox15minimumSizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QCheckBox::~QCheckBox();
  fn _ZN9QCheckBoxD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QCheckBox::sizeHint();
  fn _ZNK9QCheckBox8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QCheckBox::stateChanged(int );
  fn _ZN9QCheckBox12stateChangedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QCheckBox::setTristate(bool y);
  fn _ZN9QCheckBox11setTristateEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QCheckBox::QCheckBox(const QCheckBox & );
  fn dector_ZN9QCheckBoxC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QCheckBoxC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCheckBox::QCheckBox(QWidget * parent);
  fn dector_ZN9QCheckBoxC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QCheckBoxC1EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QCheckBox::isTristate();
  fn _ZNK9QCheckBox10isTristateEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QCheckBox::QCheckBox(const QString & text, QWidget * parent);
  fn dector_ZN9QCheckBoxC1ERK7QStringP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN9QCheckBoxC1ERK7QStringP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  fn QCheckBox_SlotProxy_connect__ZN9QCheckBox12stateChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QCheckBox_SlotProxy_connect_box__ZN9QCheckBox12stateChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QCheckBox)=1
#[derive(Default)]
pub struct QCheckBox {
  qbase: QAbstractButton,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _stateChanged_1: QCheckBox_stateChanged_signal,
}

impl /*struct*/ QCheckBox {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QCheckBox {
    return QCheckBox{qbase: QAbstractButton::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QCheckBox {
  type Target = QAbstractButton;

  fn deref(&self) -> &QAbstractButton {
    return & self.qbase;
  }
}
impl AsRef<QAbstractButton> for QCheckBox {
  fn as_ref(& self) -> & QAbstractButton {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QCheckBox::metaObject();
impl /*struct*/ QCheckBox {
  pub fn metaObject<RetType, T: QCheckBox_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QCheckBox_metaObject<RetType> {
  fn metaObject(self , rsthis: & QCheckBox) -> RetType;
}

  // proto:  const QMetaObject * QCheckBox::metaObject();
impl<'a> /*trait*/ QCheckBox_metaObject<()> for () {
  fn metaObject(self , rsthis: & QCheckBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCheckBox10metaObjectEv()};
     unsafe {_ZNK9QCheckBox10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QCheckBox::minimumSizeHint();
impl /*struct*/ QCheckBox {
  pub fn minimumSizeHint<RetType, T: QCheckBox_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QCheckBox_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QCheckBox) -> RetType;
}

  // proto:  QSize QCheckBox::minimumSizeHint();
impl<'a> /*trait*/ QCheckBox_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QCheckBox) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCheckBox15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK9QCheckBox15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QCheckBox::~QCheckBox();
impl /*struct*/ QCheckBox {
  pub fn Free<RetType, T: QCheckBox_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QCheckBox_Free<RetType> {
  fn Free(self , rsthis: & QCheckBox) -> RetType;
}

  // proto:  void QCheckBox::~QCheckBox();
impl<'a> /*trait*/ QCheckBox_Free<()> for () {
  fn Free(self , rsthis: & QCheckBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBoxD0Ev()};
     unsafe {_ZN9QCheckBoxD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QCheckBox::sizeHint();
impl /*struct*/ QCheckBox {
  pub fn sizeHint<RetType, T: QCheckBox_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QCheckBox_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QCheckBox) -> RetType;
}

  // proto:  QSize QCheckBox::sizeHint();
impl<'a> /*trait*/ QCheckBox_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QCheckBox) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCheckBox8sizeHintEv()};
    let mut ret = unsafe {_ZNK9QCheckBox8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QCheckBox::stateChanged(int );
impl /*struct*/ QCheckBox {
  pub fn stateChanged<RetType, T: QCheckBox_stateChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stateChanged(self);
    // return 1;
  }
}

pub trait QCheckBox_stateChanged<RetType> {
  fn stateChanged(self , rsthis: & QCheckBox) -> RetType;
}

  // proto:  void QCheckBox::stateChanged(int );
impl<'a> /*trait*/ QCheckBox_stateChanged<()> for (i32) {
  fn stateChanged(self , rsthis: & QCheckBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBox12stateChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QCheckBox12stateChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCheckBox::setTristate(bool y);
impl /*struct*/ QCheckBox {
  pub fn setTristate<RetType, T: QCheckBox_setTristate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTristate(self);
    // return 1;
  }
}

pub trait QCheckBox_setTristate<RetType> {
  fn setTristate(self , rsthis: & QCheckBox) -> RetType;
}

  // proto:  void QCheckBox::setTristate(bool y);
impl<'a> /*trait*/ QCheckBox_setTristate<()> for (i8) {
  fn setTristate(self , rsthis: & QCheckBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBox11setTristateEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QCheckBox11setTristateEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCheckBox::QCheckBox(const QCheckBox & );
impl /*struct*/ QCheckBox {
  pub fn New<T: QCheckBox_New>(value: T) -> QCheckBox {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QCheckBox_New {
  fn New(self) -> QCheckBox;
}

  // proto:  void QCheckBox::QCheckBox(const QCheckBox & );
impl<'a> /*trait*/ QCheckBox_New for (&'a QCheckBox) {
  fn New(self) -> QCheckBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBoxC1ERKS_()};
    let ctysz: c_int = unsafe{QCheckBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QCheckBoxC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QCheckBoxC1ERKS_(arg0)} as u64;
    let rsthis = QCheckBox{qbase: QAbstractButton::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QCheckBox::QCheckBox(QWidget * parent);
impl<'a> /*trait*/ QCheckBox_New for (&'a QWidget) {
  fn New(self) -> QCheckBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBoxC1EP7QWidget()};
    let ctysz: c_int = unsafe{QCheckBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QCheckBoxC1EP7QWidget(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QCheckBoxC1EP7QWidget(arg0)} as u64;
    let rsthis = QCheckBox{qbase: QAbstractButton::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QCheckBox::isTristate();
impl /*struct*/ QCheckBox {
  pub fn isTristate<RetType, T: QCheckBox_isTristate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isTristate(self);
    // return 1;
  }
}

pub trait QCheckBox_isTristate<RetType> {
  fn isTristate(self , rsthis: & QCheckBox) -> RetType;
}

  // proto:  bool QCheckBox::isTristate();
impl<'a> /*trait*/ QCheckBox_isTristate<i8> for () {
  fn isTristate(self , rsthis: & QCheckBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCheckBox10isTristateEv()};
    let mut ret = unsafe {_ZNK9QCheckBox10isTristateEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QCheckBox::QCheckBox(const QString & text, QWidget * parent);
impl<'a> /*trait*/ QCheckBox_New for (&'a QString, &'a QWidget) {
  fn New(self) -> QCheckBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBoxC1ERK7QStringP7QWidget()};
    let ctysz: c_int = unsafe{QCheckBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN9QCheckBoxC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN9QCheckBoxC1ERK7QStringP7QWidget(arg0, arg1)} as u64;
    let rsthis = QCheckBox{qbase: QAbstractButton::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

#[derive(Default)] // for QCheckBox_stateChanged
pub struct QCheckBox_stateChanged_signal{poi:u64}
impl /* struct */ QCheckBox {
  pub fn stateChanged_1(&self) -> QCheckBox_stateChanged_signal {
     return QCheckBox_stateChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QCheckBox_stateChanged_signal {
  pub fn connect<T: QCheckBox_stateChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QCheckBox_stateChanged_signal_connect {
  fn connect(self, sigthis: QCheckBox_stateChanged_signal);
}

// stateChanged(int)
extern fn QCheckBox_stateChanged_signal_connect_cb_0(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
}
extern fn QCheckBox_stateChanged_signal_connect_cb_box_0(rsfptr_raw:*mut c_void, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QCheckBox_stateChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QCheckBox_stateChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QCheckBox_stateChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QCheckBox_SlotProxy_connect__ZN9QCheckBox12stateChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QCheckBox_stateChanged_signal_connect for Box<fn(i32)> {
  fn connect(self, sigthis: QCheckBox_stateChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QCheckBox_stateChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QCheckBox_SlotProxy_connect__ZN9QCheckBox12stateChangedEi(arg0, arg1, arg2)};
  }
}
// <= body block end

