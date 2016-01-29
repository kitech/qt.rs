// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtWidgets/qshortcut.h
// dst-file: /src/widgets/qshortcut.rs
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
use super::super::gui::qkeysequence::*; // 771
use super::super::core::qobjectdefs::*; // 771
use super::qwidget::*; // 773
use super::super::core::qstring::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QShortcut_Class_Size() -> c_int;
  // proto:  void QShortcut::setKey(const QKeySequence & key);
  fn C_ZN9QShortcut6setKeyERK12QKeySequence(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QShortcut::metaObject();
  fn C_ZNK9QShortcut10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QWidget * QShortcut::parentWidget();
  fn C_ZNK9QShortcut12parentWidgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QShortcut::setAutoRepeat(bool on);
  fn C_ZN9QShortcut13setAutoRepeatEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QShortcut::isEnabled();
  fn C_ZNK9QShortcut9isEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QKeySequence QShortcut::key();
  fn C_ZNK9QShortcut3keyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QShortcut::~QShortcut();
  fn C_ZN9QShortcutD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QShortcut::setWhatsThis(const QString & text);
  fn C_ZN9QShortcut12setWhatsThisERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QShortcut::setEnabled(bool enable);
  fn C_ZN9QShortcut10setEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  int QShortcut::id();
  fn C_ZNK9QShortcut2idEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QString QShortcut::whatsThis();
  fn C_ZNK9QShortcut9whatsThisEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QShortcut::QShortcut(QWidget * parent);
  fn C_ZN9QShortcutC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  bool QShortcut::autoRepeat();
  fn C_ZNK9QShortcut10autoRepeatEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QShortcut_SlotProxy_connect__ZN9QShortcut9activatedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QShortcut_SlotProxy_connect__ZN9QShortcut20activatedAmbiguouslyEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QShortcut)=1
#[derive(Default)]
pub struct QShortcut {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _activated: QShortcut_activated_signal,
  pub _activatedAmbiguously: QShortcut_activatedAmbiguously_signal,
}

impl /*struct*/ QShortcut {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QShortcut {
    return QShortcut{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QShortcut {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QShortcut {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QShortcut::setKey(const QKeySequence & key);
impl /*struct*/ QShortcut {
  pub fn setKey<RetType, T: QShortcut_setKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setKey(self);
    // return 1;
  }
}

pub trait QShortcut_setKey<RetType> {
  fn setKey(self , rsthis: & QShortcut) -> RetType;
}

  // proto:  void QShortcut::setKey(const QKeySequence & key);
impl<'a> /*trait*/ QShortcut_setKey<()> for (&'a QKeySequence) {
  fn setKey(self , rsthis: & QShortcut) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut6setKeyERK12QKeySequence()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QShortcut6setKeyERK12QKeySequence(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QShortcut::metaObject();
impl /*struct*/ QShortcut {
  pub fn metaObject<RetType, T: QShortcut_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QShortcut_metaObject<RetType> {
  fn metaObject(self , rsthis: & QShortcut) -> RetType;
}

  // proto:  const QMetaObject * QShortcut::metaObject();
impl<'a> /*trait*/ QShortcut_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QShortcut) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut10metaObjectEv()};
    let mut ret = unsafe {C_ZNK9QShortcut10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QShortcut::parentWidget();
impl /*struct*/ QShortcut {
  pub fn parentWidget<RetType, T: QShortcut_parentWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parentWidget(self);
    // return 1;
  }
}

pub trait QShortcut_parentWidget<RetType> {
  fn parentWidget(self , rsthis: & QShortcut) -> RetType;
}

  // proto:  QWidget * QShortcut::parentWidget();
impl<'a> /*trait*/ QShortcut_parentWidget<QWidget> for () {
  fn parentWidget(self , rsthis: & QShortcut) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut12parentWidgetEv()};
    let mut ret = unsafe {C_ZNK9QShortcut12parentWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QShortcut::setAutoRepeat(bool on);
impl /*struct*/ QShortcut {
  pub fn setAutoRepeat<RetType, T: QShortcut_setAutoRepeat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoRepeat(self);
    // return 1;
  }
}

pub trait QShortcut_setAutoRepeat<RetType> {
  fn setAutoRepeat(self , rsthis: & QShortcut) -> RetType;
}

  // proto:  void QShortcut::setAutoRepeat(bool on);
impl<'a> /*trait*/ QShortcut_setAutoRepeat<()> for (i8) {
  fn setAutoRepeat(self , rsthis: & QShortcut) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut13setAutoRepeatEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN9QShortcut13setAutoRepeatEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QShortcut::isEnabled();
impl /*struct*/ QShortcut {
  pub fn isEnabled<RetType, T: QShortcut_isEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEnabled(self);
    // return 1;
  }
}

pub trait QShortcut_isEnabled<RetType> {
  fn isEnabled(self , rsthis: & QShortcut) -> RetType;
}

  // proto:  bool QShortcut::isEnabled();
impl<'a> /*trait*/ QShortcut_isEnabled<i8> for () {
  fn isEnabled(self , rsthis: & QShortcut) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut9isEnabledEv()};
    let mut ret = unsafe {C_ZNK9QShortcut9isEnabledEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QKeySequence QShortcut::key();
impl /*struct*/ QShortcut {
  pub fn key<RetType, T: QShortcut_key<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.key(self);
    // return 1;
  }
}

pub trait QShortcut_key<RetType> {
  fn key(self , rsthis: & QShortcut) -> RetType;
}

  // proto:  QKeySequence QShortcut::key();
impl<'a> /*trait*/ QShortcut_key<QKeySequence> for () {
  fn key(self , rsthis: & QShortcut) -> QKeySequence {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut3keyEv()};
    let mut ret = unsafe {C_ZNK9QShortcut3keyEv(rsthis.qclsinst)};
    let mut ret1 = QKeySequence::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QShortcut::~QShortcut();
impl /*struct*/ QShortcut {
  pub fn free<RetType, T: QShortcut_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QShortcut_free<RetType> {
  fn free(self , rsthis: & QShortcut) -> RetType;
}

  // proto:  void QShortcut::~QShortcut();
impl<'a> /*trait*/ QShortcut_free<()> for () {
  fn free(self , rsthis: & QShortcut) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcutD2Ev()};
     unsafe {C_ZN9QShortcutD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QShortcut::setWhatsThis(const QString & text);
impl /*struct*/ QShortcut {
  pub fn setWhatsThis<RetType, T: QShortcut_setWhatsThis<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWhatsThis(self);
    // return 1;
  }
}

pub trait QShortcut_setWhatsThis<RetType> {
  fn setWhatsThis(self , rsthis: & QShortcut) -> RetType;
}

  // proto:  void QShortcut::setWhatsThis(const QString & text);
impl<'a> /*trait*/ QShortcut_setWhatsThis<()> for (&'a QString) {
  fn setWhatsThis(self , rsthis: & QShortcut) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QShortcut12setWhatsThisERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QShortcut::setEnabled(bool enable);
impl /*struct*/ QShortcut {
  pub fn setEnabled<RetType, T: QShortcut_setEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEnabled(self);
    // return 1;
  }
}

pub trait QShortcut_setEnabled<RetType> {
  fn setEnabled(self , rsthis: & QShortcut) -> RetType;
}

  // proto:  void QShortcut::setEnabled(bool enable);
impl<'a> /*trait*/ QShortcut_setEnabled<()> for (i8) {
  fn setEnabled(self , rsthis: & QShortcut) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut10setEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN9QShortcut10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QShortcut::id();
impl /*struct*/ QShortcut {
  pub fn id<RetType, T: QShortcut_id<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.id(self);
    // return 1;
  }
}

pub trait QShortcut_id<RetType> {
  fn id(self , rsthis: & QShortcut) -> RetType;
}

  // proto:  int QShortcut::id();
impl<'a> /*trait*/ QShortcut_id<i32> for () {
  fn id(self , rsthis: & QShortcut) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut2idEv()};
    let mut ret = unsafe {C_ZNK9QShortcut2idEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QString QShortcut::whatsThis();
impl /*struct*/ QShortcut {
  pub fn whatsThis<RetType, T: QShortcut_whatsThis<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.whatsThis(self);
    // return 1;
  }
}

pub trait QShortcut_whatsThis<RetType> {
  fn whatsThis(self , rsthis: & QShortcut) -> RetType;
}

  // proto:  QString QShortcut::whatsThis();
impl<'a> /*trait*/ QShortcut_whatsThis<QString> for () {
  fn whatsThis(self , rsthis: & QShortcut) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut9whatsThisEv()};
    let mut ret = unsafe {C_ZNK9QShortcut9whatsThisEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QShortcut::QShortcut(QWidget * parent);
impl /*struct*/ QShortcut {
  pub fn new<T: QShortcut_new>(value: T) -> QShortcut {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QShortcut_new {
  fn new(self) -> QShortcut;
}

  // proto:  void QShortcut::QShortcut(QWidget * parent);
impl<'a> /*trait*/ QShortcut_new for (&'a QWidget) {
  fn new(self) -> QShortcut {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcutC2EP7QWidget()};
    let ctysz: c_int = unsafe{QShortcut_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QShortcutC2EP7QWidget(arg0)};
    let rsthis = QShortcut{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QShortcut::autoRepeat();
impl /*struct*/ QShortcut {
  pub fn autoRepeat<RetType, T: QShortcut_autoRepeat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoRepeat(self);
    // return 1;
  }
}

pub trait QShortcut_autoRepeat<RetType> {
  fn autoRepeat(self , rsthis: & QShortcut) -> RetType;
}

  // proto:  bool QShortcut::autoRepeat();
impl<'a> /*trait*/ QShortcut_autoRepeat<i8> for () {
  fn autoRepeat(self , rsthis: & QShortcut) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut10autoRepeatEv()};
    let mut ret = unsafe {C_ZNK9QShortcut10autoRepeatEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

#[derive(Default)] // for QShortcut_activated
pub struct QShortcut_activated_signal{poi:u64}
impl /* struct */ QShortcut {
  pub fn activated(&self) -> QShortcut_activated_signal {
     return QShortcut_activated_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QShortcut_activated_signal {
  pub fn connect<T: QShortcut_activated_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QShortcut_activated_signal_connect {
  fn connect(self, sigthis: QShortcut_activated_signal);
}

#[derive(Default)] // for QShortcut_activatedAmbiguously
pub struct QShortcut_activatedAmbiguously_signal{poi:u64}
impl /* struct */ QShortcut {
  pub fn activatedAmbiguously(&self) -> QShortcut_activatedAmbiguously_signal {
     return QShortcut_activatedAmbiguously_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QShortcut_activatedAmbiguously_signal {
  pub fn connect<T: QShortcut_activatedAmbiguously_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QShortcut_activatedAmbiguously_signal_connect {
  fn connect(self, sigthis: QShortcut_activatedAmbiguously_signal);
}

// activated()
extern fn QShortcut_activated_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QShortcut_activated_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QShortcut_activated_signal_connect for fn() {
  fn connect(self, sigthis: QShortcut_activated_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QShortcut_activated_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QShortcut_SlotProxy_connect__ZN9QShortcut9activatedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QShortcut_activated_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QShortcut_activated_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QShortcut_activated_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QShortcut_SlotProxy_connect__ZN9QShortcut9activatedEv(arg0, arg1, arg2)};
  }
}
// activatedAmbiguously()
extern fn QShortcut_activatedAmbiguously_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QShortcut_activatedAmbiguously_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QShortcut_activatedAmbiguously_signal_connect for fn() {
  fn connect(self, sigthis: QShortcut_activatedAmbiguously_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QShortcut_activatedAmbiguously_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QShortcut_SlotProxy_connect__ZN9QShortcut20activatedAmbiguouslyEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QShortcut_activatedAmbiguously_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QShortcut_activatedAmbiguously_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QShortcut_activatedAmbiguously_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QShortcut_SlotProxy_connect__ZN9QShortcut20activatedAmbiguouslyEv(arg0, arg1, arg2)};
  }
}
// <= body block end

