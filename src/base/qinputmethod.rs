// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrectf::QRectF;
use super::qtransform::QTransform;
use super::qlocale::QLocale;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QRectF QInputMethod::inputItemRectangle();
  fn _ZNK12QInputMethod18inputItemRectangleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QInputMethod::metaObject();
  fn _ZNK12QInputMethod10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QTransform QInputMethod::inputItemTransform();
  fn _ZNK12QInputMethod18inputItemTransformEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputMethod::visibleChanged();
  fn _ZN12QInputMethod14visibleChangedEv(qthis: *mut c_void) ;
  // proto:  void QInputMethod::hide();
  fn _ZN12QInputMethod4hideEv(qthis: *mut c_void) ;
  // proto:  QRectF QInputMethod::keyboardRectangle();
  fn _ZNK12QInputMethod17keyboardRectangleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputMethod::keyboardRectangleChanged();
  fn _ZN12QInputMethod24keyboardRectangleChangedEv(qthis: *mut c_void) ;
  // proto:  void QInputMethod::show();
  fn _ZN12QInputMethod4showEv(qthis: *mut c_void) ;
  // proto:  void QInputMethod::NewQInputMethod();
  fn _ZN12QInputMethodC1Ev(qthis: *mut c_void) ;
  // proto:  bool QInputMethod::isAnimating();
  fn _ZNK12QInputMethod11isAnimatingEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QInputMethod::animatingChanged();
  fn _ZN12QInputMethod16animatingChangedEv(qthis: *mut c_void) ;
  // proto:  void QInputMethod::setVisible(bool visible);
  fn _ZN12QInputMethod10setVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QInputMethod::setInputItemRectangle(const QRectF & rect);
  fn _ZN12QInputMethod21setInputItemRectangleERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QInputMethod::localeChanged();
  fn _ZN12QInputMethod13localeChangedEv(qthis: *mut c_void) ;
  // proto:  void QInputMethod::commit();
  fn _ZN12QInputMethod6commitEv(qthis: *mut c_void) ;
  // proto:  void QInputMethod::setInputItemTransform(const QTransform & transform);
  fn _ZN12QInputMethod21setInputItemTransformERK10QTransform(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRectF QInputMethod::cursorRectangle();
  fn _ZNK12QInputMethod15cursorRectangleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QInputMethod::isVisible();
  fn _ZNK12QInputMethod9isVisibleEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QInputMethod::cursorRectangleChanged();
  fn _ZN12QInputMethod22cursorRectangleChangedEv(qthis: *mut c_void) ;
  // proto:  void QInputMethod::FreeQInputMethod();
  fn _ZN12QInputMethodD0Ev(qthis: *mut c_void) ;
  // proto:  QLocale QInputMethod::locale();
  fn _ZNK12QInputMethod6localeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputMethod::reset();
  fn _ZN12QInputMethod5resetEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QInputMethod)=1
pub struct QInputMethod {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QInputMethod {
  pub fn inputItemRectangle<T: QInputMethod_inputItemRectangle>(&mut self, value: T) -> QRectF {
    return value.inputItemRectangle(self);
    // return 1;
  }
}

pub trait QInputMethod_inputItemRectangle {
  fn inputItemRectangle(self, rsthis: &mut QInputMethod) -> QRectF;
}

// proto:  QRectF QInputMethod::inputItemRectangle();
impl<'a> /*trait*/ QInputMethod_inputItemRectangle for () {
  fn inputItemRectangle(self, rsthis: &mut QInputMethod) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod18inputItemRectangleEv()};
    let mut ret = unsafe {_ZNK12QInputMethod18inputItemRectangleEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn metaObject<T: QInputMethod_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QInputMethod_metaObject {
  fn metaObject(self, rsthis: &mut QInputMethod) ;
}

// proto:  const QMetaObject * QInputMethod::metaObject();
impl<'a> /*trait*/ QInputMethod_metaObject for () {
  fn metaObject(self, rsthis: &mut QInputMethod)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod10metaObjectEv()};
     unsafe {_ZNK12QInputMethod10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn inputItemTransform<T: QInputMethod_inputItemTransform>(&mut self, value: T) -> QTransform {
    return value.inputItemTransform(self);
    // return 1;
  }
}

pub trait QInputMethod_inputItemTransform {
  fn inputItemTransform(self, rsthis: &mut QInputMethod) -> QTransform;
}

// proto:  QTransform QInputMethod::inputItemTransform();
impl<'a> /*trait*/ QInputMethod_inputItemTransform for () {
  fn inputItemTransform(self, rsthis: &mut QInputMethod) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod18inputItemTransformEv()};
    let mut ret = unsafe {_ZNK12QInputMethod18inputItemTransformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn visibleChanged<T: QInputMethod_visibleChanged>(&mut self, value: T)  {
     value.visibleChanged(self);
    // return 1;
  }
}

pub trait QInputMethod_visibleChanged {
  fn visibleChanged(self, rsthis: &mut QInputMethod) ;
}

// proto:  void QInputMethod::visibleChanged();
impl<'a> /*trait*/ QInputMethod_visibleChanged for () {
  fn visibleChanged(self, rsthis: &mut QInputMethod)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod14visibleChangedEv()};
     unsafe {_ZN12QInputMethod14visibleChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn hide<T: QInputMethod_hide>(&mut self, value: T)  {
     value.hide(self);
    // return 1;
  }
}

pub trait QInputMethod_hide {
  fn hide(self, rsthis: &mut QInputMethod) ;
}

// proto:  void QInputMethod::hide();
impl<'a> /*trait*/ QInputMethod_hide for () {
  fn hide(self, rsthis: &mut QInputMethod)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod4hideEv()};
     unsafe {_ZN12QInputMethod4hideEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn keyboardRectangle<T: QInputMethod_keyboardRectangle>(&mut self, value: T) -> QRectF {
    return value.keyboardRectangle(self);
    // return 1;
  }
}

pub trait QInputMethod_keyboardRectangle {
  fn keyboardRectangle(self, rsthis: &mut QInputMethod) -> QRectF;
}

// proto:  QRectF QInputMethod::keyboardRectangle();
impl<'a> /*trait*/ QInputMethod_keyboardRectangle for () {
  fn keyboardRectangle(self, rsthis: &mut QInputMethod) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod17keyboardRectangleEv()};
    let mut ret = unsafe {_ZNK12QInputMethod17keyboardRectangleEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn keyboardRectangleChanged<T: QInputMethod_keyboardRectangleChanged>(&mut self, value: T)  {
     value.keyboardRectangleChanged(self);
    // return 1;
  }
}

pub trait QInputMethod_keyboardRectangleChanged {
  fn keyboardRectangleChanged(self, rsthis: &mut QInputMethod) ;
}

// proto:  void QInputMethod::keyboardRectangleChanged();
impl<'a> /*trait*/ QInputMethod_keyboardRectangleChanged for () {
  fn keyboardRectangleChanged(self, rsthis: &mut QInputMethod)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod24keyboardRectangleChangedEv()};
     unsafe {_ZN12QInputMethod24keyboardRectangleChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn show<T: QInputMethod_show>(&mut self, value: T)  {
     value.show(self);
    // return 1;
  }
}

pub trait QInputMethod_show {
  fn show(self, rsthis: &mut QInputMethod) ;
}

// proto:  void QInputMethod::show();
impl<'a> /*trait*/ QInputMethod_show for () {
  fn show(self, rsthis: &mut QInputMethod)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod4showEv()};
     unsafe {_ZN12QInputMethod4showEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn NewQInputMethod<T: QInputMethod_NewQInputMethod>(value: T) -> QInputMethod {
    let rsthis = value.NewQInputMethod();
    return rsthis;
    // return 1;
  }
}

pub trait QInputMethod_NewQInputMethod {
  fn NewQInputMethod(self) -> QInputMethod;
}

// proto: void QInputMethod::NewQInputMethod();
impl<'a> /*trait*/ QInputMethod_NewQInputMethod for () {
  fn NewQInputMethod(self) -> QInputMethod {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethodC1Ev()};
    unsafe {_ZN12QInputMethodC1Ev(qthis)};
    let rsthis = QInputMethod{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn isAnimating<T: QInputMethod_isAnimating>(&mut self, value: T) -> i8 {
    return value.isAnimating(self);
    // return 1;
  }
}

pub trait QInputMethod_isAnimating {
  fn isAnimating(self, rsthis: &mut QInputMethod) -> i8;
}

// proto:  bool QInputMethod::isAnimating();
impl<'a> /*trait*/ QInputMethod_isAnimating for () {
  fn isAnimating(self, rsthis: &mut QInputMethod) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod11isAnimatingEv()};
    let mut ret = unsafe {_ZNK12QInputMethod11isAnimatingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn animatingChanged<T: QInputMethod_animatingChanged>(&mut self, value: T)  {
     value.animatingChanged(self);
    // return 1;
  }
}

pub trait QInputMethod_animatingChanged {
  fn animatingChanged(self, rsthis: &mut QInputMethod) ;
}

// proto:  void QInputMethod::animatingChanged();
impl<'a> /*trait*/ QInputMethod_animatingChanged for () {
  fn animatingChanged(self, rsthis: &mut QInputMethod)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod16animatingChangedEv()};
     unsafe {_ZN12QInputMethod16animatingChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn setVisible<T: QInputMethod_setVisible>(&mut self, value: T)  {
     value.setVisible(self);
    // return 1;
  }
}

pub trait QInputMethod_setVisible {
  fn setVisible(self, rsthis: &mut QInputMethod) ;
}

// proto:  void QInputMethod::setVisible(bool visible);
impl<'a> /*trait*/ QInputMethod_setVisible for (i8) {
  fn setVisible(self, rsthis: &mut QInputMethod)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod10setVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QInputMethod10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn setInputItemRectangle<T: QInputMethod_setInputItemRectangle>(&mut self, value: T)  {
     value.setInputItemRectangle(self);
    // return 1;
  }
}

pub trait QInputMethod_setInputItemRectangle {
  fn setInputItemRectangle(self, rsthis: &mut QInputMethod) ;
}

// proto:  void QInputMethod::setInputItemRectangle(const QRectF & rect);
impl<'a> /*trait*/ QInputMethod_setInputItemRectangle for (&'a  QRectF) {
  fn setInputItemRectangle(self, rsthis: &mut QInputMethod)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod21setInputItemRectangleERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QInputMethod21setInputItemRectangleERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn localeChanged<T: QInputMethod_localeChanged>(&mut self, value: T)  {
     value.localeChanged(self);
    // return 1;
  }
}

pub trait QInputMethod_localeChanged {
  fn localeChanged(self, rsthis: &mut QInputMethod) ;
}

// proto:  void QInputMethod::localeChanged();
impl<'a> /*trait*/ QInputMethod_localeChanged for () {
  fn localeChanged(self, rsthis: &mut QInputMethod)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod13localeChangedEv()};
     unsafe {_ZN12QInputMethod13localeChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn commit<T: QInputMethod_commit>(&mut self, value: T)  {
     value.commit(self);
    // return 1;
  }
}

pub trait QInputMethod_commit {
  fn commit(self, rsthis: &mut QInputMethod) ;
}

// proto:  void QInputMethod::commit();
impl<'a> /*trait*/ QInputMethod_commit for () {
  fn commit(self, rsthis: &mut QInputMethod)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod6commitEv()};
     unsafe {_ZN12QInputMethod6commitEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn setInputItemTransform<T: QInputMethod_setInputItemTransform>(&mut self, value: T)  {
     value.setInputItemTransform(self);
    // return 1;
  }
}

pub trait QInputMethod_setInputItemTransform {
  fn setInputItemTransform(self, rsthis: &mut QInputMethod) ;
}

// proto:  void QInputMethod::setInputItemTransform(const QTransform & transform);
impl<'a> /*trait*/ QInputMethod_setInputItemTransform for (&'a  QTransform) {
  fn setInputItemTransform(self, rsthis: &mut QInputMethod)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod21setInputItemTransformERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QInputMethod21setInputItemTransformERK10QTransform(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn cursorRectangle<T: QInputMethod_cursorRectangle>(&mut self, value: T) -> QRectF {
    return value.cursorRectangle(self);
    // return 1;
  }
}

pub trait QInputMethod_cursorRectangle {
  fn cursorRectangle(self, rsthis: &mut QInputMethod) -> QRectF;
}

// proto:  QRectF QInputMethod::cursorRectangle();
impl<'a> /*trait*/ QInputMethod_cursorRectangle for () {
  fn cursorRectangle(self, rsthis: &mut QInputMethod) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod15cursorRectangleEv()};
    let mut ret = unsafe {_ZNK12QInputMethod15cursorRectangleEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn isVisible<T: QInputMethod_isVisible>(&mut self, value: T) -> i8 {
    return value.isVisible(self);
    // return 1;
  }
}

pub trait QInputMethod_isVisible {
  fn isVisible(self, rsthis: &mut QInputMethod) -> i8;
}

// proto:  bool QInputMethod::isVisible();
impl<'a> /*trait*/ QInputMethod_isVisible for () {
  fn isVisible(self, rsthis: &mut QInputMethod) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod9isVisibleEv()};
    let mut ret = unsafe {_ZNK12QInputMethod9isVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn cursorRectangleChanged<T: QInputMethod_cursorRectangleChanged>(&mut self, value: T)  {
     value.cursorRectangleChanged(self);
    // return 1;
  }
}

pub trait QInputMethod_cursorRectangleChanged {
  fn cursorRectangleChanged(self, rsthis: &mut QInputMethod) ;
}

// proto:  void QInputMethod::cursorRectangleChanged();
impl<'a> /*trait*/ QInputMethod_cursorRectangleChanged for () {
  fn cursorRectangleChanged(self, rsthis: &mut QInputMethod)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod22cursorRectangleChangedEv()};
     unsafe {_ZN12QInputMethod22cursorRectangleChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn FreeQInputMethod<T: QInputMethod_FreeQInputMethod>(&mut self, value: T)  {
     value.FreeQInputMethod(self);
    // return 1;
  }
}

pub trait QInputMethod_FreeQInputMethod {
  fn FreeQInputMethod(self, rsthis: &mut QInputMethod) ;
}

// proto:  void QInputMethod::FreeQInputMethod();
impl<'a> /*trait*/ QInputMethod_FreeQInputMethod for () {
  fn FreeQInputMethod(self, rsthis: &mut QInputMethod)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethodD0Ev()};
     unsafe {_ZN12QInputMethodD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn locale<T: QInputMethod_locale>(&mut self, value: T) -> QLocale {
    return value.locale(self);
    // return 1;
  }
}

pub trait QInputMethod_locale {
  fn locale(self, rsthis: &mut QInputMethod) -> QLocale;
}

// proto:  QLocale QInputMethod::locale();
impl<'a> /*trait*/ QInputMethod_locale for () {
  fn locale(self, rsthis: &mut QInputMethod) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod6localeEv()};
    let mut ret = unsafe {_ZNK12QInputMethod6localeEv(rsthis.qclsinst)};
    let mut ret1 = QLocale{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn reset<T: QInputMethod_reset>(&mut self, value: T)  {
     value.reset(self);
    // return 1;
  }
}

pub trait QInputMethod_reset {
  fn reset(self, rsthis: &mut QInputMethod) ;
}

// proto:  void QInputMethod::reset();
impl<'a> /*trait*/ QInputMethod_reset for () {
  fn reset(self, rsthis: &mut QInputMethod)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod5resetEv()};
     unsafe {_ZN12QInputMethod5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

