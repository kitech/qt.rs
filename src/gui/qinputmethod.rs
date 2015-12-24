// auto generated, do not modify.
// created: Thu Dec 24 23:00:39 2015
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
  // proto:  QRectF QInputMethod::inputItemRectangle();
  fn _ZNK12QInputMethod18inputItemRectangleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QInputMethod::metaObject();
  fn _ZNK12QInputMethod10metaObjectEv(qthis: *mut c_void);
  // proto:  QTransform QInputMethod::inputItemTransform();
  fn _ZNK12QInputMethod18inputItemTransformEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputMethod::visibleChanged();
  fn _ZN12QInputMethod14visibleChangedEv(qthis: *mut c_void);
  // proto:  void QInputMethod::hide();
  fn _ZN12QInputMethod4hideEv(qthis: *mut c_void);
  // proto:  QRectF QInputMethod::keyboardRectangle();
  fn _ZNK12QInputMethod17keyboardRectangleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputMethod::keyboardRectangleChanged();
  fn _ZN12QInputMethod24keyboardRectangleChangedEv(qthis: *mut c_void);
  // proto:  void QInputMethod::show();
  fn _ZN12QInputMethod4showEv(qthis: *mut c_void);
  // proto:  void QInputMethod::QInputMethod();
  fn _ZN12QInputMethodC1Ev(qthis: *mut c_void);
  // proto:  bool QInputMethod::isAnimating();
  fn _ZNK12QInputMethod11isAnimatingEv(qthis: *mut c_void) -> c_char;
  // proto:  void QInputMethod::animatingChanged();
  fn _ZN12QInputMethod16animatingChangedEv(qthis: *mut c_void);
  // proto:  void QInputMethod::setVisible(bool visible);
  fn _ZN12QInputMethod10setVisibleEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QInputMethod::setInputItemRectangle(const QRectF & rect);
  fn _ZN12QInputMethod21setInputItemRectangleERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QInputMethod::localeChanged();
  fn _ZN12QInputMethod13localeChangedEv(qthis: *mut c_void);
  // proto:  void QInputMethod::commit();
  fn _ZN12QInputMethod6commitEv(qthis: *mut c_void);
  // proto:  void QInputMethod::setInputItemTransform(const QTransform & transform);
  fn _ZN12QInputMethod21setInputItemTransformERK10QTransform(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRectF QInputMethod::cursorRectangle();
  fn _ZNK12QInputMethod15cursorRectangleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QInputMethod::isVisible();
  fn _ZNK12QInputMethod9isVisibleEv(qthis: *mut c_void) -> c_char;
  // proto:  void QInputMethod::cursorRectangleChanged();
  fn _ZN12QInputMethod22cursorRectangleChangedEv(qthis: *mut c_void);
  // proto:  void QInputMethod::~QInputMethod();
  fn _ZN12QInputMethodD0Ev(qthis: *mut c_void);
  // proto:  QLocale QInputMethod::locale();
  fn _ZNK12QInputMethod6localeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputMethod::reset();
  fn _ZN12QInputMethod5resetEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QInputMethod)=1
pub struct QInputMethod {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QInputMethod {
  pub fn inheritFrom(qthis: *mut c_void) -> QInputMethod {
    return QInputMethod{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
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
    let mut ret1 = QRectF::inheritFrom(ret);
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
    let mut ret1 = QTransform::inheritFrom(ret);
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
    let mut ret1 = QRectF::inheritFrom(ret);
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
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethodC1Ev()};
    unsafe {_ZN12QInputMethodC1Ev(qthis)};
    let rsthis = QInputMethod{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
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
    let mut ret1 = QRectF::inheritFrom(ret);
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
    let mut ret1 = QLocale::inheritFrom(ret);
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

// <= body block end

