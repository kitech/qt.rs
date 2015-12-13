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

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QRectF QInputMethod::inputItemRectangle();
  fn _ZNK12QInputMethod18inputItemRectangleEv() -> i32;
  // proto: const QMetaObject * QInputMethod::metaObject();
  fn _ZNK12QInputMethod10metaObjectEv() -> i32;
  // proto: QTransform QInputMethod::inputItemTransform();
  fn _ZNK12QInputMethod18inputItemTransformEv() -> i32;
  // proto: void QInputMethod::visibleChanged();
  fn _ZN12QInputMethod14visibleChangedEv() -> i32;
  // proto: void QInputMethod::hide();
  fn _ZN12QInputMethod4hideEv() -> i32;
  // proto: QRectF QInputMethod::keyboardRectangle();
  fn _ZNK12QInputMethod17keyboardRectangleEv() -> i32;
  // proto: void QInputMethod::keyboardRectangleChanged();
  fn _ZN12QInputMethod24keyboardRectangleChangedEv() -> i32;
  // proto: void QInputMethod::show();
  fn _ZN12QInputMethod4showEv() -> i32;
  // proto: void QInputMethod::NewQInputMethod();
  fn _ZN12QInputMethodC1Ev(qthis: *mut c_void) -> i32;
  // proto: bool QInputMethod::isAnimating();
  fn _ZNK12QInputMethod11isAnimatingEv() -> i32;
  // proto: void QInputMethod::animatingChanged();
  fn _ZN12QInputMethod16animatingChangedEv() -> i32;
  // proto: void QInputMethod::setVisible(bool visible);
  fn _ZN12QInputMethod10setVisibleEb(arg0: int8_t) -> i32;
  // proto: void QInputMethod::setInputItemRectangle(const QRectF & rect);
  fn _ZN12QInputMethod21setInputItemRectangleERK6QRectF(arg0: *const c_void) -> i32;
  // proto: void QInputMethod::localeChanged();
  fn _ZN12QInputMethod13localeChangedEv() -> i32;
  // proto: void QInputMethod::commit();
  fn _ZN12QInputMethod6commitEv() -> i32;
  // proto: void QInputMethod::setInputItemTransform(const QTransform & transform);
  fn _ZN12QInputMethod21setInputItemTransformERK10QTransform(arg0: *const c_void) -> i32;
  // proto: QRectF QInputMethod::cursorRectangle();
  fn _ZNK12QInputMethod15cursorRectangleEv() -> i32;
  // proto: bool QInputMethod::isVisible();
  fn _ZNK12QInputMethod9isVisibleEv() -> i32;
  // proto: void QInputMethod::cursorRectangleChanged();
  fn _ZN12QInputMethod22cursorRectangleChangedEv() -> i32;
  // proto: void QInputMethod::FreeQInputMethod();
  fn _ZN12QInputMethodD0Ev() -> i32;
  // proto: QLocale QInputMethod::locale();
  fn _ZNK12QInputMethod6localeEv() -> i32;
  // proto: void QInputMethod::reset();
  fn _ZN12QInputMethod5resetEv() -> i32;
}

// body block begin
// class sizeof(QInputMethod)=1
pub struct QInputMethod {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QInputMethod {
  pub fn inputItemRectangle<T: QInputMethod_inputItemRectangle>(&mut self, value: T) -> i32 {
    value.inputItemRectangle(self);
    return 1;
  }
}

pub trait QInputMethod_inputItemRectangle {
  fn inputItemRectangle(self, this: &mut QInputMethod) -> i32;
}

// proto: QRectF QInputMethod::inputItemRectangle();
impl<'a> /*trait*/ QInputMethod_inputItemRectangle for () {
  fn inputItemRectangle(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod18inputItemRectangleEv()};
    unsafe {_ZNK12QInputMethod18inputItemRectangleEv()};
    return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn metaObject<T: QInputMethod_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QInputMethod_metaObject {
  fn metaObject(self, this: &mut QInputMethod) -> i32;
}

// proto: const QMetaObject * QInputMethod::metaObject();
impl<'a> /*trait*/ QInputMethod_metaObject for () {
  fn metaObject(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod10metaObjectEv()};
    unsafe {_ZNK12QInputMethod10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn inputItemTransform<T: QInputMethod_inputItemTransform>(&mut self, value: T) -> i32 {
    value.inputItemTransform(self);
    return 1;
  }
}

pub trait QInputMethod_inputItemTransform {
  fn inputItemTransform(self, this: &mut QInputMethod) -> i32;
}

// proto: QTransform QInputMethod::inputItemTransform();
impl<'a> /*trait*/ QInputMethod_inputItemTransform for () {
  fn inputItemTransform(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod18inputItemTransformEv()};
    unsafe {_ZNK12QInputMethod18inputItemTransformEv()};
    return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn visibleChanged<T: QInputMethod_visibleChanged>(&mut self, value: T) -> i32 {
    value.visibleChanged(self);
    return 1;
  }
}

pub trait QInputMethod_visibleChanged {
  fn visibleChanged(self, this: &mut QInputMethod) -> i32;
}

// proto: void QInputMethod::visibleChanged();
impl<'a> /*trait*/ QInputMethod_visibleChanged for () {
  fn visibleChanged(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod14visibleChangedEv()};
    unsafe {_ZN12QInputMethod14visibleChangedEv()};
    return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn hide<T: QInputMethod_hide>(&mut self, value: T) -> i32 {
    value.hide(self);
    return 1;
  }
}

pub trait QInputMethod_hide {
  fn hide(self, this: &mut QInputMethod) -> i32;
}

// proto: void QInputMethod::hide();
impl<'a> /*trait*/ QInputMethod_hide for () {
  fn hide(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod4hideEv()};
    unsafe {_ZN12QInputMethod4hideEv()};
    return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn keyboardRectangle<T: QInputMethod_keyboardRectangle>(&mut self, value: T) -> i32 {
    value.keyboardRectangle(self);
    return 1;
  }
}

pub trait QInputMethod_keyboardRectangle {
  fn keyboardRectangle(self, this: &mut QInputMethod) -> i32;
}

// proto: QRectF QInputMethod::keyboardRectangle();
impl<'a> /*trait*/ QInputMethod_keyboardRectangle for () {
  fn keyboardRectangle(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod17keyboardRectangleEv()};
    unsafe {_ZNK12QInputMethod17keyboardRectangleEv()};
    return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn keyboardRectangleChanged<T: QInputMethod_keyboardRectangleChanged>(&mut self, value: T) -> i32 {
    value.keyboardRectangleChanged(self);
    return 1;
  }
}

pub trait QInputMethod_keyboardRectangleChanged {
  fn keyboardRectangleChanged(self, this: &mut QInputMethod) -> i32;
}

// proto: void QInputMethod::keyboardRectangleChanged();
impl<'a> /*trait*/ QInputMethod_keyboardRectangleChanged for () {
  fn keyboardRectangleChanged(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod24keyboardRectangleChangedEv()};
    unsafe {_ZN12QInputMethod24keyboardRectangleChangedEv()};
    return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn show<T: QInputMethod_show>(&mut self, value: T) -> i32 {
    value.show(self);
    return 1;
  }
}

pub trait QInputMethod_show {
  fn show(self, this: &mut QInputMethod) -> i32;
}

// proto: void QInputMethod::show();
impl<'a> /*trait*/ QInputMethod_show for () {
  fn show(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod4showEv()};
    unsafe {_ZN12QInputMethod4showEv()};
    return 1;
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
  pub fn isAnimating<T: QInputMethod_isAnimating>(&mut self, value: T) -> i32 {
    value.isAnimating(self);
    return 1;
  }
}

pub trait QInputMethod_isAnimating {
  fn isAnimating(self, this: &mut QInputMethod) -> i32;
}

// proto: bool QInputMethod::isAnimating();
impl<'a> /*trait*/ QInputMethod_isAnimating for () {
  fn isAnimating(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod11isAnimatingEv()};
    unsafe {_ZNK12QInputMethod11isAnimatingEv()};
    return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn animatingChanged<T: QInputMethod_animatingChanged>(&mut self, value: T) -> i32 {
    value.animatingChanged(self);
    return 1;
  }
}

pub trait QInputMethod_animatingChanged {
  fn animatingChanged(self, this: &mut QInputMethod) -> i32;
}

// proto: void QInputMethod::animatingChanged();
impl<'a> /*trait*/ QInputMethod_animatingChanged for () {
  fn animatingChanged(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod16animatingChangedEv()};
    unsafe {_ZN12QInputMethod16animatingChangedEv()};
    return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn setVisible<T: QInputMethod_setVisible>(&mut self, value: T) -> i32 {
    value.setVisible(self);
    return 1;
  }
}

pub trait QInputMethod_setVisible {
  fn setVisible(self, this: &mut QInputMethod) -> i32;
}

// proto: void QInputMethod::setVisible(bool visible);
impl<'a> /*trait*/ QInputMethod_setVisible for (i8) {
  fn setVisible(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod10setVisibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN12QInputMethod10setVisibleEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn setInputItemRectangle<T: QInputMethod_setInputItemRectangle>(&mut self, value: T) -> i32 {
    value.setInputItemRectangle(self);
    return 1;
  }
}

pub trait QInputMethod_setInputItemRectangle {
  fn setInputItemRectangle(self, this: &mut QInputMethod) -> i32;
}

// proto: void QInputMethod::setInputItemRectangle(const QRectF & rect);
impl<'a> /*trait*/ QInputMethod_setInputItemRectangle for (&'a  QRectF) {
  fn setInputItemRectangle(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod21setInputItemRectangleERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QInputMethod21setInputItemRectangleERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn localeChanged<T: QInputMethod_localeChanged>(&mut self, value: T) -> i32 {
    value.localeChanged(self);
    return 1;
  }
}

pub trait QInputMethod_localeChanged {
  fn localeChanged(self, this: &mut QInputMethod) -> i32;
}

// proto: void QInputMethod::localeChanged();
impl<'a> /*trait*/ QInputMethod_localeChanged for () {
  fn localeChanged(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod13localeChangedEv()};
    unsafe {_ZN12QInputMethod13localeChangedEv()};
    return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn commit<T: QInputMethod_commit>(&mut self, value: T) -> i32 {
    value.commit(self);
    return 1;
  }
}

pub trait QInputMethod_commit {
  fn commit(self, this: &mut QInputMethod) -> i32;
}

// proto: void QInputMethod::commit();
impl<'a> /*trait*/ QInputMethod_commit for () {
  fn commit(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod6commitEv()};
    unsafe {_ZN12QInputMethod6commitEv()};
    return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn setInputItemTransform<T: QInputMethod_setInputItemTransform>(&mut self, value: T) -> i32 {
    value.setInputItemTransform(self);
    return 1;
  }
}

pub trait QInputMethod_setInputItemTransform {
  fn setInputItemTransform(self, this: &mut QInputMethod) -> i32;
}

// proto: void QInputMethod::setInputItemTransform(const QTransform & transform);
impl<'a> /*trait*/ QInputMethod_setInputItemTransform for (&'a  QTransform) {
  fn setInputItemTransform(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod21setInputItemTransformERK10QTransform()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QInputMethod21setInputItemTransformERK10QTransform(arg0)};
    return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn cursorRectangle<T: QInputMethod_cursorRectangle>(&mut self, value: T) -> i32 {
    value.cursorRectangle(self);
    return 1;
  }
}

pub trait QInputMethod_cursorRectangle {
  fn cursorRectangle(self, this: &mut QInputMethod) -> i32;
}

// proto: QRectF QInputMethod::cursorRectangle();
impl<'a> /*trait*/ QInputMethod_cursorRectangle for () {
  fn cursorRectangle(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod15cursorRectangleEv()};
    unsafe {_ZNK12QInputMethod15cursorRectangleEv()};
    return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn isVisible<T: QInputMethod_isVisible>(&mut self, value: T) -> i32 {
    value.isVisible(self);
    return 1;
  }
}

pub trait QInputMethod_isVisible {
  fn isVisible(self, this: &mut QInputMethod) -> i32;
}

// proto: bool QInputMethod::isVisible();
impl<'a> /*trait*/ QInputMethod_isVisible for () {
  fn isVisible(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod9isVisibleEv()};
    unsafe {_ZNK12QInputMethod9isVisibleEv()};
    return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn cursorRectangleChanged<T: QInputMethod_cursorRectangleChanged>(&mut self, value: T) -> i32 {
    value.cursorRectangleChanged(self);
    return 1;
  }
}

pub trait QInputMethod_cursorRectangleChanged {
  fn cursorRectangleChanged(self, this: &mut QInputMethod) -> i32;
}

// proto: void QInputMethod::cursorRectangleChanged();
impl<'a> /*trait*/ QInputMethod_cursorRectangleChanged for () {
  fn cursorRectangleChanged(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod22cursorRectangleChangedEv()};
    unsafe {_ZN12QInputMethod22cursorRectangleChangedEv()};
    return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn FreeQInputMethod<T: QInputMethod_FreeQInputMethod>(&mut self, value: T) -> i32 {
    value.FreeQInputMethod(self);
    return 1;
  }
}

pub trait QInputMethod_FreeQInputMethod {
  fn FreeQInputMethod(self, this: &mut QInputMethod) -> i32;
}

// proto: void QInputMethod::FreeQInputMethod();
impl<'a> /*trait*/ QInputMethod_FreeQInputMethod for () {
  fn FreeQInputMethod(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethodD0Ev()};
    unsafe {_ZN12QInputMethodD0Ev()};
    return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn locale<T: QInputMethod_locale>(&mut self, value: T) -> i32 {
    value.locale(self);
    return 1;
  }
}

pub trait QInputMethod_locale {
  fn locale(self, this: &mut QInputMethod) -> i32;
}

// proto: QLocale QInputMethod::locale();
impl<'a> /*trait*/ QInputMethod_locale for () {
  fn locale(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputMethod6localeEv()};
    unsafe {_ZNK12QInputMethod6localeEv()};
    return 1;
  }
}

impl /*struct*/ QInputMethod {
  pub fn reset<T: QInputMethod_reset>(&mut self, value: T) -> i32 {
    value.reset(self);
    return 1;
  }
}

pub trait QInputMethod_reset {
  fn reset(self, this: &mut QInputMethod) -> i32;
}

// proto: void QInputMethod::reset();
impl<'a> /*trait*/ QInputMethod_reset for () {
  fn reset(self, this: &mut QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputMethod5resetEv()};
    unsafe {_ZN12QInputMethod5resetEv()};
    return 1;
  }
}

