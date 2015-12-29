// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
// src-file: /QtGui/qinputmethod.h
// dst-file: /src/gui/qinputmethod.rs
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
use super::super::core::qrect::QRectF; // 771
use super::qtransform::QTransform; // 773
use super::super::core::qvariant::QVariant; // 771
use super::super::core::qlocale::QLocale; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QInputMethod_Class_Size() -> c_int;
  // proto:  QRectF QInputMethod::inputItemRectangle();
  fn _ZNK12QInputMethod18inputItemRectangleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QInputMethod::metaObject();
  fn _ZNK12QInputMethod10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QTransform QInputMethod::inputItemTransform();
  fn _ZNK12QInputMethod18inputItemTransformEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QInputMethod::visibleChanged();
  fn _ZN12QInputMethod14visibleChangedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QInputMethod::hide();
  fn _ZN12QInputMethod4hideEv(qthis: u64 /* *mut c_void*/);
  // proto:  QRectF QInputMethod::keyboardRectangle();
  fn _ZNK12QInputMethod17keyboardRectangleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QInputMethod::keyboardRectangleChanged();
  fn _ZN12QInputMethod24keyboardRectangleChangedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QInputMethod::show();
  fn _ZN12QInputMethod4showEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QInputMethod::QInputMethod();
  fn dector_ZN12QInputMethodC1Ev() -> *mut c_void;
  fn _ZN12QInputMethodC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QInputMethod::isAnimating();
  fn _ZNK12QInputMethod11isAnimatingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QInputMethod::animatingChanged();
  fn _ZN12QInputMethod16animatingChangedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QInputMethod::setVisible(bool visible);
  fn _ZN12QInputMethod10setVisibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QInputMethod::setInputItemRectangle(const QRectF & rect);
  fn _ZN12QInputMethod21setInputItemRectangleERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QInputMethod::localeChanged();
  fn _ZN12QInputMethod13localeChangedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QInputMethod::commit();
  fn _ZN12QInputMethod6commitEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QInputMethod::setInputItemTransform(const QTransform & transform);
  fn _ZN12QInputMethod21setInputItemTransformERK10QTransform(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QRectF QInputMethod::cursorRectangle();
  fn _ZNK12QInputMethod15cursorRectangleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QInputMethod::isVisible();
  fn _ZNK12QInputMethod9isVisibleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QInputMethod::cursorRectangleChanged();
  fn _ZN12QInputMethod22cursorRectangleChangedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QInputMethod::~QInputMethod();
  fn _ZN12QInputMethodD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QLocale QInputMethod::locale();
  fn _ZNK12QInputMethod6localeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QInputMethod::reset();
  fn _ZN12QInputMethod5resetEv(qthis: u64 /* *mut c_void*/);
  fn QInputMethod_SlotProxy_connect__ZN12QInputMethod22cursorRectangleChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QInputMethod_SlotProxy_connect_box__ZN12QInputMethod22cursorRectangleChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QInputMethod_SlotProxy_connect__ZN12QInputMethod13localeChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QInputMethod_SlotProxy_connect_box__ZN12QInputMethod13localeChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QInputMethod_SlotProxy_connect__ZN12QInputMethod14visibleChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QInputMethod_SlotProxy_connect_box__ZN12QInputMethod14visibleChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QInputMethod_SlotProxy_connect__ZN12QInputMethod24keyboardRectangleChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QInputMethod_SlotProxy_connect_box__ZN12QInputMethod24keyboardRectangleChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QInputMethod_SlotProxy_connect__ZN12QInputMethod21inputDirectionChangedEN2Qt15LayoutDirectionE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QInputMethod_SlotProxy_connect_box__ZN12QInputMethod21inputDirectionChangedEN2Qt15LayoutDirectionE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QInputMethod_SlotProxy_connect__ZN12QInputMethod16animatingChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QInputMethod_SlotProxy_connect_box__ZN12QInputMethod16animatingChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QInputMethod)=1
#[derive(Default)]
pub struct QInputMethod {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _cursorRectangleChanged_1: QInputMethod_cursorRectangleChanged_signal,
  pub _localeChanged_1: QInputMethod_localeChanged_signal,
  pub _inputDirectionChanged_1: QInputMethod_inputDirectionChanged_signal,
  pub _animatingChanged_1: QInputMethod_animatingChanged_signal,
  pub _keyboardRectangleChanged_1: QInputMethod_keyboardRectangleChanged_signal,
  pub _visibleChanged_1: QInputMethod_visibleChanged_signal,
}

impl /*struct*/ QInputMethod {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QInputMethod {
    return QInputMethod{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QInputMethod {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QInputMethod {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  QRectF QInputMethod::inputItemRectangle();
impl /*struct*/ QInputMethod {
  pub fn inputItemRectangle<RetType, T: QInputMethod_inputItemRectangle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.inputItemRectangle(self);
    // return 1;
  }
}

pub trait QInputMethod_inputItemRectangle<RetType> {
  fn inputItemRectangle(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  QRectF QInputMethod::inputItemRectangle();
impl<'a> /*trait*/ QInputMethod_inputItemRectangle<QRectF> for () {
  fn inputItemRectangle(self , rsthis: & QInputMethod) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod18inputItemRectangleEv()};
    let mut ret = unsafe {_ZNK12QInputMethod18inputItemRectangleEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QInputMethod::metaObject();
impl /*struct*/ QInputMethod {
  pub fn metaObject<RetType, T: QInputMethod_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QInputMethod_metaObject<RetType> {
  fn metaObject(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  const QMetaObject * QInputMethod::metaObject();
impl<'a> /*trait*/ QInputMethod_metaObject<()> for () {
  fn metaObject(self , rsthis: & QInputMethod) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod10metaObjectEv()};
     unsafe {_ZNK12QInputMethod10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QTransform QInputMethod::inputItemTransform();
impl /*struct*/ QInputMethod {
  pub fn inputItemTransform<RetType, T: QInputMethod_inputItemTransform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.inputItemTransform(self);
    // return 1;
  }
}

pub trait QInputMethod_inputItemTransform<RetType> {
  fn inputItemTransform(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  QTransform QInputMethod::inputItemTransform();
impl<'a> /*trait*/ QInputMethod_inputItemTransform<QTransform> for () {
  fn inputItemTransform(self , rsthis: & QInputMethod) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod18inputItemTransformEv()};
    let mut ret = unsafe {_ZNK12QInputMethod18inputItemTransformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QInputMethod::visibleChanged();
impl /*struct*/ QInputMethod {
  pub fn visibleChanged<RetType, T: QInputMethod_visibleChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.visibleChanged(self);
    // return 1;
  }
}

pub trait QInputMethod_visibleChanged<RetType> {
  fn visibleChanged(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  void QInputMethod::visibleChanged();
impl<'a> /*trait*/ QInputMethod_visibleChanged<()> for () {
  fn visibleChanged(self , rsthis: & QInputMethod) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod14visibleChangedEv()};
     unsafe {_ZN12QInputMethod14visibleChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QInputMethod::hide();
impl /*struct*/ QInputMethod {
  pub fn hide<RetType, T: QInputMethod_hide<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hide(self);
    // return 1;
  }
}

pub trait QInputMethod_hide<RetType> {
  fn hide(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  void QInputMethod::hide();
impl<'a> /*trait*/ QInputMethod_hide<()> for () {
  fn hide(self , rsthis: & QInputMethod) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod4hideEv()};
     unsafe {_ZN12QInputMethod4hideEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRectF QInputMethod::keyboardRectangle();
impl /*struct*/ QInputMethod {
  pub fn keyboardRectangle<RetType, T: QInputMethod_keyboardRectangle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keyboardRectangle(self);
    // return 1;
  }
}

pub trait QInputMethod_keyboardRectangle<RetType> {
  fn keyboardRectangle(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  QRectF QInputMethod::keyboardRectangle();
impl<'a> /*trait*/ QInputMethod_keyboardRectangle<QRectF> for () {
  fn keyboardRectangle(self , rsthis: & QInputMethod) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod17keyboardRectangleEv()};
    let mut ret = unsafe {_ZNK12QInputMethod17keyboardRectangleEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QInputMethod::keyboardRectangleChanged();
impl /*struct*/ QInputMethod {
  pub fn keyboardRectangleChanged<RetType, T: QInputMethod_keyboardRectangleChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keyboardRectangleChanged(self);
    // return 1;
  }
}

pub trait QInputMethod_keyboardRectangleChanged<RetType> {
  fn keyboardRectangleChanged(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  void QInputMethod::keyboardRectangleChanged();
impl<'a> /*trait*/ QInputMethod_keyboardRectangleChanged<()> for () {
  fn keyboardRectangleChanged(self , rsthis: & QInputMethod) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod24keyboardRectangleChangedEv()};
     unsafe {_ZN12QInputMethod24keyboardRectangleChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QInputMethod::show();
impl /*struct*/ QInputMethod {
  pub fn show<RetType, T: QInputMethod_show<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.show(self);
    // return 1;
  }
}

pub trait QInputMethod_show<RetType> {
  fn show(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  void QInputMethod::show();
impl<'a> /*trait*/ QInputMethod_show<()> for () {
  fn show(self , rsthis: & QInputMethod) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod4showEv()};
     unsafe {_ZN12QInputMethod4showEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QInputMethod::QInputMethod();
impl /*struct*/ QInputMethod {
  pub fn New<T: QInputMethod_New>(value: T) -> QInputMethod {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QInputMethod_New {
  fn New(self) -> QInputMethod;
}

  // proto:  void QInputMethod::QInputMethod();
impl<'a> /*trait*/ QInputMethod_New for () {
  fn New(self) -> QInputMethod {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethodC1Ev()};
    let ctysz: c_int = unsafe{QInputMethod_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN12QInputMethodC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN12QInputMethodC1Ev()} as u64;
    let rsthis = QInputMethod{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QInputMethod::isAnimating();
impl /*struct*/ QInputMethod {
  pub fn isAnimating<RetType, T: QInputMethod_isAnimating<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isAnimating(self);
    // return 1;
  }
}

pub trait QInputMethod_isAnimating<RetType> {
  fn isAnimating(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  bool QInputMethod::isAnimating();
impl<'a> /*trait*/ QInputMethod_isAnimating<i8> for () {
  fn isAnimating(self , rsthis: & QInputMethod) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod11isAnimatingEv()};
    let mut ret = unsafe {_ZNK12QInputMethod11isAnimatingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QInputMethod::animatingChanged();
impl /*struct*/ QInputMethod {
  pub fn animatingChanged<RetType, T: QInputMethod_animatingChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.animatingChanged(self);
    // return 1;
  }
}

pub trait QInputMethod_animatingChanged<RetType> {
  fn animatingChanged(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  void QInputMethod::animatingChanged();
impl<'a> /*trait*/ QInputMethod_animatingChanged<()> for () {
  fn animatingChanged(self , rsthis: & QInputMethod) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod16animatingChangedEv()};
     unsafe {_ZN12QInputMethod16animatingChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QInputMethod::setVisible(bool visible);
impl /*struct*/ QInputMethod {
  pub fn setVisible<RetType, T: QInputMethod_setVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QInputMethod_setVisible<RetType> {
  fn setVisible(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  void QInputMethod::setVisible(bool visible);
impl<'a> /*trait*/ QInputMethod_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: & QInputMethod) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QInputMethod10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputMethod::setInputItemRectangle(const QRectF & rect);
impl /*struct*/ QInputMethod {
  pub fn setInputItemRectangle<RetType, T: QInputMethod_setInputItemRectangle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setInputItemRectangle(self);
    // return 1;
  }
}

pub trait QInputMethod_setInputItemRectangle<RetType> {
  fn setInputItemRectangle(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  void QInputMethod::setInputItemRectangle(const QRectF & rect);
impl<'a> /*trait*/ QInputMethod_setInputItemRectangle<()> for (&'a QRectF) {
  fn setInputItemRectangle(self , rsthis: & QInputMethod) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod21setInputItemRectangleERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QInputMethod21setInputItemRectangleERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputMethod::localeChanged();
impl /*struct*/ QInputMethod {
  pub fn localeChanged<RetType, T: QInputMethod_localeChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.localeChanged(self);
    // return 1;
  }
}

pub trait QInputMethod_localeChanged<RetType> {
  fn localeChanged(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  void QInputMethod::localeChanged();
impl<'a> /*trait*/ QInputMethod_localeChanged<()> for () {
  fn localeChanged(self , rsthis: & QInputMethod) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod13localeChangedEv()};
     unsafe {_ZN12QInputMethod13localeChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QInputMethod::commit();
impl /*struct*/ QInputMethod {
  pub fn commit<RetType, T: QInputMethod_commit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.commit(self);
    // return 1;
  }
}

pub trait QInputMethod_commit<RetType> {
  fn commit(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  void QInputMethod::commit();
impl<'a> /*trait*/ QInputMethod_commit<()> for () {
  fn commit(self , rsthis: & QInputMethod) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod6commitEv()};
     unsafe {_ZN12QInputMethod6commitEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QInputMethod::setInputItemTransform(const QTransform & transform);
impl /*struct*/ QInputMethod {
  pub fn setInputItemTransform<RetType, T: QInputMethod_setInputItemTransform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setInputItemTransform(self);
    // return 1;
  }
}

pub trait QInputMethod_setInputItemTransform<RetType> {
  fn setInputItemTransform(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  void QInputMethod::setInputItemTransform(const QTransform & transform);
impl<'a> /*trait*/ QInputMethod_setInputItemTransform<()> for (&'a QTransform) {
  fn setInputItemTransform(self , rsthis: & QInputMethod) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod21setInputItemTransformERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QInputMethod21setInputItemTransformERK10QTransform(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QInputMethod::cursorRectangle();
impl /*struct*/ QInputMethod {
  pub fn cursorRectangle<RetType, T: QInputMethod_cursorRectangle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursorRectangle(self);
    // return 1;
  }
}

pub trait QInputMethod_cursorRectangle<RetType> {
  fn cursorRectangle(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  QRectF QInputMethod::cursorRectangle();
impl<'a> /*trait*/ QInputMethod_cursorRectangle<QRectF> for () {
  fn cursorRectangle(self , rsthis: & QInputMethod) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod15cursorRectangleEv()};
    let mut ret = unsafe {_ZNK12QInputMethod15cursorRectangleEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QInputMethod::isVisible();
impl /*struct*/ QInputMethod {
  pub fn isVisible<RetType, T: QInputMethod_isVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isVisible(self);
    // return 1;
  }
}

pub trait QInputMethod_isVisible<RetType> {
  fn isVisible(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  bool QInputMethod::isVisible();
impl<'a> /*trait*/ QInputMethod_isVisible<i8> for () {
  fn isVisible(self , rsthis: & QInputMethod) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod9isVisibleEv()};
    let mut ret = unsafe {_ZNK12QInputMethod9isVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QInputMethod::cursorRectangleChanged();
impl /*struct*/ QInputMethod {
  pub fn cursorRectangleChanged<RetType, T: QInputMethod_cursorRectangleChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursorRectangleChanged(self);
    // return 1;
  }
}

pub trait QInputMethod_cursorRectangleChanged<RetType> {
  fn cursorRectangleChanged(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  void QInputMethod::cursorRectangleChanged();
impl<'a> /*trait*/ QInputMethod_cursorRectangleChanged<()> for () {
  fn cursorRectangleChanged(self , rsthis: & QInputMethod) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod22cursorRectangleChangedEv()};
     unsafe {_ZN12QInputMethod22cursorRectangleChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QInputMethod::~QInputMethod();
impl /*struct*/ QInputMethod {
  pub fn Free<RetType, T: QInputMethod_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QInputMethod_Free<RetType> {
  fn Free(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  void QInputMethod::~QInputMethod();
impl<'a> /*trait*/ QInputMethod_Free<()> for () {
  fn Free(self , rsthis: & QInputMethod) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethodD0Ev()};
     unsafe {_ZN12QInputMethodD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QLocale QInputMethod::locale();
impl /*struct*/ QInputMethod {
  pub fn locale<RetType, T: QInputMethod_locale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.locale(self);
    // return 1;
  }
}

pub trait QInputMethod_locale<RetType> {
  fn locale(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  QLocale QInputMethod::locale();
impl<'a> /*trait*/ QInputMethod_locale<QLocale> for () {
  fn locale(self , rsthis: & QInputMethod) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod6localeEv()};
    let mut ret = unsafe {_ZNK12QInputMethod6localeEv(rsthis.qclsinst)};
    let mut ret1 = QLocale::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QInputMethod::reset();
impl /*struct*/ QInputMethod {
  pub fn reset<RetType, T: QInputMethod_reset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QInputMethod_reset<RetType> {
  fn reset(self , rsthis: & QInputMethod) -> RetType;
}

  // proto:  void QInputMethod::reset();
impl<'a> /*trait*/ QInputMethod_reset<()> for () {
  fn reset(self , rsthis: & QInputMethod) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod5resetEv()};
     unsafe {_ZN12QInputMethod5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QInputMethod_cursorRectangleChanged
pub struct QInputMethod_cursorRectangleChanged_signal{poi:u64}
impl /* struct */ QInputMethod {
  pub fn cursorRectangleChanged_1(&self) -> QInputMethod_cursorRectangleChanged_signal {
     return QInputMethod_cursorRectangleChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QInputMethod_cursorRectangleChanged_signal {
  pub fn connect<T: QInputMethod_cursorRectangleChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QInputMethod_cursorRectangleChanged_signal_connect {
  fn connect(self, sigthis: QInputMethod_cursorRectangleChanged_signal);
}

#[derive(Default)] // for QInputMethod_localeChanged
pub struct QInputMethod_localeChanged_signal{poi:u64}
impl /* struct */ QInputMethod {
  pub fn localeChanged_1(&self) -> QInputMethod_localeChanged_signal {
     return QInputMethod_localeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QInputMethod_localeChanged_signal {
  pub fn connect<T: QInputMethod_localeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QInputMethod_localeChanged_signal_connect {
  fn connect(self, sigthis: QInputMethod_localeChanged_signal);
}

#[derive(Default)] // for QInputMethod_inputDirectionChanged
pub struct QInputMethod_inputDirectionChanged_signal{poi:u64}
impl /* struct */ QInputMethod {
  pub fn inputDirectionChanged_1(&self) -> QInputMethod_inputDirectionChanged_signal {
     return QInputMethod_inputDirectionChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QInputMethod_inputDirectionChanged_signal {
  pub fn connect<T: QInputMethod_inputDirectionChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QInputMethod_inputDirectionChanged_signal_connect {
  fn connect(self, sigthis: QInputMethod_inputDirectionChanged_signal);
}

#[derive(Default)] // for QInputMethod_animatingChanged
pub struct QInputMethod_animatingChanged_signal{poi:u64}
impl /* struct */ QInputMethod {
  pub fn animatingChanged_1(&self) -> QInputMethod_animatingChanged_signal {
     return QInputMethod_animatingChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QInputMethod_animatingChanged_signal {
  pub fn connect<T: QInputMethod_animatingChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QInputMethod_animatingChanged_signal_connect {
  fn connect(self, sigthis: QInputMethod_animatingChanged_signal);
}

#[derive(Default)] // for QInputMethod_keyboardRectangleChanged
pub struct QInputMethod_keyboardRectangleChanged_signal{poi:u64}
impl /* struct */ QInputMethod {
  pub fn keyboardRectangleChanged_1(&self) -> QInputMethod_keyboardRectangleChanged_signal {
     return QInputMethod_keyboardRectangleChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QInputMethod_keyboardRectangleChanged_signal {
  pub fn connect<T: QInputMethod_keyboardRectangleChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QInputMethod_keyboardRectangleChanged_signal_connect {
  fn connect(self, sigthis: QInputMethod_keyboardRectangleChanged_signal);
}

#[derive(Default)] // for QInputMethod_visibleChanged
pub struct QInputMethod_visibleChanged_signal{poi:u64}
impl /* struct */ QInputMethod {
  pub fn visibleChanged_1(&self) -> QInputMethod_visibleChanged_signal {
     return QInputMethod_visibleChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QInputMethod_visibleChanged_signal {
  pub fn connect<T: QInputMethod_visibleChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QInputMethod_visibleChanged_signal_connect {
  fn connect(self, sigthis: QInputMethod_visibleChanged_signal);
}

// cursorRectangleChanged()
extern fn QInputMethod_cursorRectangleChanged_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
}
extern fn QInputMethod_cursorRectangleChanged_signal_connect_cb_box_0(rsfptr_raw:*mut c_void, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QInputMethod_cursorRectangleChanged_signal_connect for fn() {
  fn connect(self, sigthis: QInputMethod_cursorRectangleChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputMethod_cursorRectangleChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QInputMethod_SlotProxy_connect__ZN12QInputMethod22cursorRectangleChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QInputMethod_cursorRectangleChanged_signal_connect for Box<fn()> {
  fn connect(self, sigthis: QInputMethod_cursorRectangleChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputMethod_cursorRectangleChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QInputMethod_SlotProxy_connect__ZN12QInputMethod22cursorRectangleChangedEv(arg0, arg1, arg2)};
  }
}
// localeChanged()
extern fn QInputMethod_localeChanged_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
}
extern fn QInputMethod_localeChanged_signal_connect_cb_box_1(rsfptr_raw:*mut c_void, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QInputMethod_localeChanged_signal_connect for fn() {
  fn connect(self, sigthis: QInputMethod_localeChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputMethod_localeChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QInputMethod_SlotProxy_connect__ZN12QInputMethod13localeChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QInputMethod_localeChanged_signal_connect for Box<fn()> {
  fn connect(self, sigthis: QInputMethod_localeChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputMethod_localeChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QInputMethod_SlotProxy_connect__ZN12QInputMethod13localeChangedEv(arg0, arg1, arg2)};
  }
}
// visibleChanged()
extern fn QInputMethod_visibleChanged_signal_connect_cb_2(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
}
extern fn QInputMethod_visibleChanged_signal_connect_cb_box_2(rsfptr_raw:*mut c_void, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QInputMethod_visibleChanged_signal_connect for fn() {
  fn connect(self, sigthis: QInputMethod_visibleChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputMethod_visibleChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QInputMethod_SlotProxy_connect__ZN12QInputMethod14visibleChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QInputMethod_visibleChanged_signal_connect for Box<fn()> {
  fn connect(self, sigthis: QInputMethod_visibleChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputMethod_visibleChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QInputMethod_SlotProxy_connect__ZN12QInputMethod14visibleChangedEv(arg0, arg1, arg2)};
  }
}
// keyboardRectangleChanged()
extern fn QInputMethod_keyboardRectangleChanged_signal_connect_cb_3(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
}
extern fn QInputMethod_keyboardRectangleChanged_signal_connect_cb_box_3(rsfptr_raw:*mut c_void, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QInputMethod_keyboardRectangleChanged_signal_connect for fn() {
  fn connect(self, sigthis: QInputMethod_keyboardRectangleChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputMethod_keyboardRectangleChanged_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QInputMethod_SlotProxy_connect__ZN12QInputMethod24keyboardRectangleChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QInputMethod_keyboardRectangleChanged_signal_connect for Box<fn()> {
  fn connect(self, sigthis: QInputMethod_keyboardRectangleChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputMethod_keyboardRectangleChanged_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QInputMethod_SlotProxy_connect__ZN12QInputMethod24keyboardRectangleChangedEv(arg0, arg1, arg2)};
  }
}
// inputDirectionChanged(Qt::LayoutDirection)
extern fn QInputMethod_inputDirectionChanged_signal_connect_cb_4(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
}
extern fn QInputMethod_inputDirectionChanged_signal_connect_cb_box_4(rsfptr_raw:*mut c_void, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QInputMethod_inputDirectionChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QInputMethod_inputDirectionChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputMethod_inputDirectionChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QInputMethod_SlotProxy_connect__ZN12QInputMethod21inputDirectionChangedEN2Qt15LayoutDirectionE(arg0, arg1, arg2)};
  }
}
impl /* trait */ QInputMethod_inputDirectionChanged_signal_connect for Box<fn(i32)> {
  fn connect(self, sigthis: QInputMethod_inputDirectionChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputMethod_inputDirectionChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QInputMethod_SlotProxy_connect__ZN12QInputMethod21inputDirectionChangedEN2Qt15LayoutDirectionE(arg0, arg1, arg2)};
  }
}
// animatingChanged()
extern fn QInputMethod_animatingChanged_signal_connect_cb_5(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
}
extern fn QInputMethod_animatingChanged_signal_connect_cb_box_5(rsfptr_raw:*mut c_void, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QInputMethod_animatingChanged_signal_connect for fn() {
  fn connect(self, sigthis: QInputMethod_animatingChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputMethod_animatingChanged_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QInputMethod_SlotProxy_connect__ZN12QInputMethod16animatingChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QInputMethod_animatingChanged_signal_connect for Box<fn()> {
  fn connect(self, sigthis: QInputMethod_animatingChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputMethod_animatingChanged_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QInputMethod_SlotProxy_connect__ZN12QInputMethod16animatingChangedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

