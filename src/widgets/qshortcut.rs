// auto generated, do not modify.
// created: Sat Dec 26 12:15:38 2015
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::gui::qkeysequence::QKeySequence; // 771
use super::qwidget::QWidget; // 773
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QShortcut_Class_Size() -> c_int;
  // proto:  void QShortcut::setKey(const QKeySequence & key);
  fn _ZN9QShortcut6setKeyERK12QKeySequence(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QShortcut::activated();
  fn _ZN9QShortcut9activatedEv(qthis: *mut c_void);
  // proto:  const QMetaObject * QShortcut::metaObject();
  fn _ZNK9QShortcut10metaObjectEv(qthis: *mut c_void);
  // proto:  QWidget * QShortcut::parentWidget();
  fn demth_ZNK9QShortcut12parentWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QShortcut::setAutoRepeat(bool on);
  fn _ZN9QShortcut13setAutoRepeatEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QShortcut::isEnabled();
  fn _ZNK9QShortcut9isEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  QKeySequence QShortcut::key();
  fn _ZNK9QShortcut3keyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QShortcut::~QShortcut();
  fn _ZN9QShortcutD0Ev(qthis: *mut c_void);
  // proto:  void QShortcut::setWhatsThis(const QString & text);
  fn _ZN9QShortcut12setWhatsThisERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QShortcut::setEnabled(bool enable);
  fn _ZN9QShortcut10setEnabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  int QShortcut::id();
  fn _ZNK9QShortcut2idEv(qthis: *mut c_void) -> c_int;
  // proto:  QString QShortcut::whatsThis();
  fn _ZNK9QShortcut9whatsThisEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QShortcut::QShortcut(QWidget * parent);
  fn dector_ZN9QShortcutC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QShortcutC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QShortcut::activatedAmbiguously();
  fn _ZN9QShortcut20activatedAmbiguouslyEv(qthis: *mut c_void);
  // proto:  bool QShortcut::autoRepeat();
  fn _ZNK9QShortcut10autoRepeatEv(qthis: *mut c_void) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QShortcut)=1
pub struct QShortcut {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QShortcut {
  pub fn inheritFrom(qthis: *mut c_void) -> QShortcut {
    return QShortcut{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
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
     unsafe {_ZN9QShortcut6setKeyERK12QKeySequence(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QShortcut::activated();
impl /*struct*/ QShortcut {
  pub fn activated<RetType, T: QShortcut_activated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activated(self);
    // return 1;
  }
}

pub trait QShortcut_activated<RetType> {
  fn activated(self , rsthis: & QShortcut) -> RetType;
}

  // proto:  void QShortcut::activated();
impl<'a> /*trait*/ QShortcut_activated<()> for () {
  fn activated(self , rsthis: & QShortcut) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut9activatedEv()};
     unsafe {_ZN9QShortcut9activatedEv(rsthis.qclsinst)};
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
impl<'a> /*trait*/ QShortcut_metaObject<()> for () {
  fn metaObject(self , rsthis: & QShortcut) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut10metaObjectEv()};
     unsafe {_ZNK9QShortcut10metaObjectEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {demth_ZNK9QShortcut12parentWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret);
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
     unsafe {_ZN9QShortcut13setAutoRepeatEb(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK9QShortcut9isEnabledEv(rsthis.qclsinst)};
    return ret as i8;
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
    let mut ret = unsafe {_ZNK9QShortcut3keyEv(rsthis.qclsinst)};
    let mut ret1 = QKeySequence::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QShortcut::~QShortcut();
impl /*struct*/ QShortcut {
  pub fn Free<RetType, T: QShortcut_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QShortcut_Free<RetType> {
  fn Free(self , rsthis: & QShortcut) -> RetType;
}

  // proto:  void QShortcut::~QShortcut();
impl<'a> /*trait*/ QShortcut_Free<()> for () {
  fn Free(self , rsthis: & QShortcut) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcutD0Ev()};
     unsafe {_ZN9QShortcutD0Ev(rsthis.qclsinst)};
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
     unsafe {_ZN9QShortcut12setWhatsThisERK7QString(rsthis.qclsinst, arg0)};
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
     unsafe {_ZN9QShortcut10setEnabledEb(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK9QShortcut2idEv(rsthis.qclsinst)};
    return ret as i32;
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
    let mut ret = unsafe {_ZNK9QShortcut9whatsThisEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QShortcut::QShortcut(QWidget * parent);
impl /*struct*/ QShortcut {
  pub fn New<T: QShortcut_New>(value: T) -> QShortcut {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QShortcut_New {
  fn New(self) -> QShortcut;
}

  // proto:  void QShortcut::QShortcut(QWidget * parent);
impl<'a> /*trait*/ QShortcut_New for (&'a QWidget) {
  fn New(self) -> QShortcut {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcutC1EP7QWidget()};
    let ctysz: c_int = unsafe{QShortcut_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QShortcutC1EP7QWidget(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN9QShortcutC1EP7QWidget(arg0)};
    let rsthis = QShortcut{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QShortcut::activatedAmbiguously();
impl /*struct*/ QShortcut {
  pub fn activatedAmbiguously<RetType, T: QShortcut_activatedAmbiguously<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activatedAmbiguously(self);
    // return 1;
  }
}

pub trait QShortcut_activatedAmbiguously<RetType> {
  fn activatedAmbiguously(self , rsthis: & QShortcut) -> RetType;
}

  // proto:  void QShortcut::activatedAmbiguously();
impl<'a> /*trait*/ QShortcut_activatedAmbiguously<()> for () {
  fn activatedAmbiguously(self , rsthis: & QShortcut) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut20activatedAmbiguouslyEv()};
     unsafe {_ZN9QShortcut20activatedAmbiguouslyEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {_ZNK9QShortcut10autoRepeatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

