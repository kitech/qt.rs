// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
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
use super::qabstractbutton::*; // 773
use std::ops::Deref;
use super::super::core::qobjectdefs::*; // 771
use super::super::core::qsize::*; // 771
use super::qwidget::*; // 773
use super::super::core::qstring::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QCheckBox_Class_Size() -> c_int;
  // proto:  const QMetaObject * QCheckBox::metaObject();
  fn C_ZNK9QCheckBox10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QCheckBox::minimumSizeHint();
  fn C_ZNK9QCheckBox15minimumSizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QCheckBox::~QCheckBox();
  fn C_ZN9QCheckBoxD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QCheckBox::sizeHint();
  fn C_ZNK9QCheckBox8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QCheckBox::setTristate(bool y);
  fn C_ZN9QCheckBox11setTristateEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QCheckBox::QCheckBox(QWidget * parent);
  fn C_ZN9QCheckBoxC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  bool QCheckBox::isTristate();
  fn C_ZNK9QCheckBox10isTristateEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QCheckBox::QCheckBox(const QString & text, QWidget * parent);
  fn C_ZN9QCheckBoxC2ERK7QStringP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  fn QCheckBox_SlotProxy_connect__ZN9QCheckBox12stateChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QCheckBox)=1
#[derive(Default)]
pub struct QCheckBox {
  qbase: QAbstractButton,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _stateChanged: QCheckBox_stateChanged_signal,
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
impl<'a> /*trait*/ QCheckBox_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QCheckBox) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCheckBox10metaObjectEv()};
    let mut ret = unsafe {C_ZNK9QCheckBox10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
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
    let mut ret = unsafe {C_ZNK9QCheckBox15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QCheckBox::~QCheckBox();
impl /*struct*/ QCheckBox {
  pub fn free<RetType, T: QCheckBox_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QCheckBox_free<RetType> {
  fn free(self , rsthis: & QCheckBox) -> RetType;
}

  // proto:  void QCheckBox::~QCheckBox();
impl<'a> /*trait*/ QCheckBox_free<()> for () {
  fn free(self , rsthis: & QCheckBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBoxD2Ev()};
     unsafe {C_ZN9QCheckBoxD2Ev(rsthis.qclsinst)};
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
    let mut ret = unsafe {C_ZNK9QCheckBox8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
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
impl<'a> /*trait*/ QCheckBox_setTristate<()> for (Option<i8>) {
  fn setTristate(self , rsthis: & QCheckBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBox11setTristateEb()};
    let arg0 = (if self.is_none() {true as i8} else {self.unwrap()})  as c_char;
     unsafe {C_ZN9QCheckBox11setTristateEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCheckBox::QCheckBox(QWidget * parent);
impl /*struct*/ QCheckBox {
  pub fn new<T: QCheckBox_new>(value: T) -> QCheckBox {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QCheckBox_new {
  fn new(self) -> QCheckBox;
}

  // proto:  void QCheckBox::QCheckBox(QWidget * parent);
impl<'a> /*trait*/ QCheckBox_new for (Option<&'a QWidget>) {
  fn new(self) -> QCheckBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBoxC2EP7QWidget()};
    let ctysz: c_int = unsafe{QCheckBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QCheckBoxC2EP7QWidget(arg0)};
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
    let mut ret = unsafe {C_ZNK9QCheckBox10isTristateEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QCheckBox::QCheckBox(const QString & text, QWidget * parent);
impl<'a> /*trait*/ QCheckBox_new for (&'a QString, Option<&'a QWidget>) {
  fn new(self) -> QCheckBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBoxC2ERK7QStringP7QWidget()};
    let ctysz: c_int = unsafe{QCheckBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {0} else {self.1.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QCheckBoxC2ERK7QStringP7QWidget(arg0, arg1)};
    let rsthis = QCheckBox{qbase: QAbstractButton::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

#[derive(Default)] // for QCheckBox_stateChanged
pub struct QCheckBox_stateChanged_signal{poi:u64}
impl /* struct */ QCheckBox {
  pub fn stateChanged(&self) -> QCheckBox_stateChanged_signal {
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
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QCheckBox_stateChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QCheckBox_stateChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QCheckBox_stateChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QCheckBox_stateChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QCheckBox_SlotProxy_connect__ZN9QCheckBox12stateChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QCheckBox_stateChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QCheckBox_stateChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QCheckBox_stateChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QCheckBox_SlotProxy_connect__ZN9QCheckBox12stateChangedEi(arg0, arg1, arg2)};
  }
}
// <= body block end

