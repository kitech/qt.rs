// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QSpinBox::setMinimum(int min);
  fn _ZN8QSpinBox10setMinimumEi(arg0: c_int) -> i32;
  // proto: QString QSpinBox::cleanText();
  fn _ZNK8QSpinBox9cleanTextEv() -> i32;
  // proto: int QSpinBox::value();
  fn _ZNK8QSpinBox5valueEv() -> i32;
  // proto: void QSpinBox::FreeQSpinBox();
  fn _ZN8QSpinBoxD0Ev() -> i32;
  // proto: void QSpinBox::setMaximum(int max);
  fn _ZN8QSpinBox10setMaximumEi(arg0: c_int) -> i32;
  // proto: void QSpinBox::valueChanged(const QString & );
  fn _ZN8QSpinBox12valueChangedERK7QString(arg0: *const c_void) -> i32;
  // proto: void QSpinBox::setValue(int val);
  fn _ZN8QSpinBox8setValueEi(arg0: c_int) -> i32;
  // proto: void QSpinBox::setDisplayIntegerBase(int base);
  fn _ZN8QSpinBox21setDisplayIntegerBaseEi(arg0: c_int) -> i32;
  // proto: void QSpinBox::NewQSpinBox(QWidget * parent);
  fn _ZN8QSpinBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: int QSpinBox::singleStep();
  fn _ZNK8QSpinBox10singleStepEv() -> i32;
  // proto: void QSpinBox::NewQSpinBox(const QSpinBox & );
  fn _ZN8QSpinBoxC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: int QSpinBox::displayIntegerBase();
  fn _ZNK8QSpinBox18displayIntegerBaseEv() -> i32;
  // proto: void QSpinBox::setSuffix(const QString & suffix);
  fn _ZN8QSpinBox9setSuffixERK7QString(arg0: *const c_void) -> i32;
  // proto: int QSpinBox::maximum();
  fn _ZNK8QSpinBox7maximumEv() -> i32;
  // proto: void QSpinBox::setPrefix(const QString & prefix);
  fn _ZN8QSpinBox9setPrefixERK7QString(arg0: *const c_void) -> i32;
  // proto: void QSpinBox::valueChanged(int );
  fn _ZN8QSpinBox12valueChangedEi(arg0: c_int) -> i32;
  // proto: QString QSpinBox::prefix();
  fn _ZNK8QSpinBox6prefixEv() -> i32;
  // proto: const QMetaObject * QSpinBox::metaObject();
  fn _ZNK8QSpinBox10metaObjectEv() -> i32;
  // proto: QString QSpinBox::suffix();
  fn _ZNK8QSpinBox6suffixEv() -> i32;
  // proto: int QSpinBox::minimum();
  fn _ZNK8QSpinBox7minimumEv() -> i32;
  // proto: void QSpinBox::setSingleStep(int val);
  fn _ZN8QSpinBox13setSingleStepEi(arg0: c_int) -> i32;
  // proto: void QSpinBox::setRange(int min, int max);
  fn _ZN8QSpinBox8setRangeEii(arg0: c_int, arg1: c_int) -> i32;
}

// body block begin
// class sizeof(QSpinBox)=1
pub struct QSpinBox {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSpinBox {
  pub fn setMinimum<T: QSpinBox_setMinimum>(&mut self, value: T) -> i32 {
    value.setMinimum(self);
    return 1;
  }
}

pub trait QSpinBox_setMinimum {
  fn setMinimum(self, this: &mut QSpinBox) -> i32;
}

// proto: void QSpinBox::setMinimum(int min);
impl<'a> /*trait*/ QSpinBox_setMinimum for (i32) {
  fn setMinimum(self, this: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox10setMinimumEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QSpinBox10setMinimumEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn cleanText<T: QSpinBox_cleanText>(&mut self, value: T) -> i32 {
    value.cleanText(self);
    return 1;
  }
}

pub trait QSpinBox_cleanText {
  fn cleanText(self, this: &mut QSpinBox) -> i32;
}

// proto: QString QSpinBox::cleanText();
impl<'a> /*trait*/ QSpinBox_cleanText for () {
  fn cleanText(self, this: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox9cleanTextEv()};
    unsafe {_ZNK8QSpinBox9cleanTextEv()};
    return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn value<T: QSpinBox_value>(&mut self, value: T) -> i32 {
    value.value(self);
    return 1;
  }
}

pub trait QSpinBox_value {
  fn value(self, this: &mut QSpinBox) -> i32;
}

// proto: int QSpinBox::value();
impl<'a> /*trait*/ QSpinBox_value for () {
  fn value(self, this: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox5valueEv()};
    unsafe {_ZNK8QSpinBox5valueEv()};
    return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn FreeQSpinBox<T: QSpinBox_FreeQSpinBox>(&mut self, value: T) -> i32 {
    value.FreeQSpinBox(self);
    return 1;
  }
}

pub trait QSpinBox_FreeQSpinBox {
  fn FreeQSpinBox(self, this: &mut QSpinBox) -> i32;
}

// proto: void QSpinBox::FreeQSpinBox();
impl<'a> /*trait*/ QSpinBox_FreeQSpinBox for () {
  fn FreeQSpinBox(self, this: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBoxD0Ev()};
    unsafe {_ZN8QSpinBoxD0Ev()};
    return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn setMaximum<T: QSpinBox_setMaximum>(&mut self, value: T) -> i32 {
    value.setMaximum(self);
    return 1;
  }
}

pub trait QSpinBox_setMaximum {
  fn setMaximum(self, this: &mut QSpinBox) -> i32;
}

// proto: void QSpinBox::setMaximum(int max);
impl<'a> /*trait*/ QSpinBox_setMaximum for (i32) {
  fn setMaximum(self, this: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox10setMaximumEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QSpinBox10setMaximumEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn valueChanged<T: QSpinBox_valueChanged>(&mut self, value: T) -> i32 {
    value.valueChanged(self);
    return 1;
  }
}

pub trait QSpinBox_valueChanged {
  fn valueChanged(self, this: &mut QSpinBox) -> i32;
}

// proto: void QSpinBox::valueChanged(const QString & );
impl<'a> /*trait*/ QSpinBox_valueChanged for (&'a  QString) {
  fn valueChanged(self, this: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox12valueChangedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QSpinBox12valueChangedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn setValue<T: QSpinBox_setValue>(&mut self, value: T) -> i32 {
    value.setValue(self);
    return 1;
  }
}

pub trait QSpinBox_setValue {
  fn setValue(self, this: &mut QSpinBox) -> i32;
}

// proto: void QSpinBox::setValue(int val);
impl<'a> /*trait*/ QSpinBox_setValue for (i32) {
  fn setValue(self, this: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox8setValueEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QSpinBox8setValueEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn setDisplayIntegerBase<T: QSpinBox_setDisplayIntegerBase>(&mut self, value: T) -> i32 {
    value.setDisplayIntegerBase(self);
    return 1;
  }
}

pub trait QSpinBox_setDisplayIntegerBase {
  fn setDisplayIntegerBase(self, this: &mut QSpinBox) -> i32;
}

// proto: void QSpinBox::setDisplayIntegerBase(int base);
impl<'a> /*trait*/ QSpinBox_setDisplayIntegerBase for (i32) {
  fn setDisplayIntegerBase(self, this: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox21setDisplayIntegerBaseEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QSpinBox21setDisplayIntegerBaseEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn NewQSpinBox<T: QSpinBox_NewQSpinBox>(value: T) -> QSpinBox {
    let rsthis = value.NewQSpinBox();
    return rsthis;
    // return 1;
  }
}

pub trait QSpinBox_NewQSpinBox {
  fn NewQSpinBox(self) -> QSpinBox;
}

// proto: void QSpinBox::NewQSpinBox(QWidget * parent);
impl<'a> /*trait*/ QSpinBox_NewQSpinBox for (&'a mut QWidget) {
  fn NewQSpinBox(self) -> QSpinBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBoxC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QSpinBoxC1EP7QWidget(qthis, arg0)};
    let rsthis = QSpinBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn singleStep<T: QSpinBox_singleStep>(&mut self, value: T) -> i32 {
    value.singleStep(self);
    return 1;
  }
}

pub trait QSpinBox_singleStep {
  fn singleStep(self, this: &mut QSpinBox) -> i32;
}

// proto: int QSpinBox::singleStep();
impl<'a> /*trait*/ QSpinBox_singleStep for () {
  fn singleStep(self, this: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox10singleStepEv()};
    unsafe {_ZNK8QSpinBox10singleStepEv()};
    return 1;
  }
}

// proto: void QSpinBox::NewQSpinBox(const QSpinBox & );
impl<'a> /*trait*/ QSpinBox_NewQSpinBox for (&'a  QSpinBox) {
  fn NewQSpinBox(self) -> QSpinBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QSpinBoxC1ERKS_(qthis, arg0)};
    let rsthis = QSpinBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn displayIntegerBase<T: QSpinBox_displayIntegerBase>(&mut self, value: T) -> i32 {
    value.displayIntegerBase(self);
    return 1;
  }
}

pub trait QSpinBox_displayIntegerBase {
  fn displayIntegerBase(self, this: &mut QSpinBox) -> i32;
}

// proto: int QSpinBox::displayIntegerBase();
impl<'a> /*trait*/ QSpinBox_displayIntegerBase for () {
  fn displayIntegerBase(self, this: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox18displayIntegerBaseEv()};
    unsafe {_ZNK8QSpinBox18displayIntegerBaseEv()};
    return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn setSuffix<T: QSpinBox_setSuffix>(&mut self, value: T) -> i32 {
    value.setSuffix(self);
    return 1;
  }
}

pub trait QSpinBox_setSuffix {
  fn setSuffix(self, this: &mut QSpinBox) -> i32;
}

// proto: void QSpinBox::setSuffix(const QString & suffix);
impl<'a> /*trait*/ QSpinBox_setSuffix for (&'a  QString) {
  fn setSuffix(self, this: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox9setSuffixERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QSpinBox9setSuffixERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn maximum<T: QSpinBox_maximum>(&mut self, value: T) -> i32 {
    value.maximum(self);
    return 1;
  }
}

pub trait QSpinBox_maximum {
  fn maximum(self, this: &mut QSpinBox) -> i32;
}

// proto: int QSpinBox::maximum();
impl<'a> /*trait*/ QSpinBox_maximum for () {
  fn maximum(self, this: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox7maximumEv()};
    unsafe {_ZNK8QSpinBox7maximumEv()};
    return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn setPrefix<T: QSpinBox_setPrefix>(&mut self, value: T) -> i32 {
    value.setPrefix(self);
    return 1;
  }
}

pub trait QSpinBox_setPrefix {
  fn setPrefix(self, this: &mut QSpinBox) -> i32;
}

// proto: void QSpinBox::setPrefix(const QString & prefix);
impl<'a> /*trait*/ QSpinBox_setPrefix for (&'a  QString) {
  fn setPrefix(self, this: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox9setPrefixERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QSpinBox9setPrefixERK7QString(arg0)};
    return 1;
  }
}

// proto: void QSpinBox::valueChanged(int );
impl<'a> /*trait*/ QSpinBox_valueChanged for (i32) {
  fn valueChanged(self, this: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox12valueChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QSpinBox12valueChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn prefix<T: QSpinBox_prefix>(&mut self, value: T) -> i32 {
    value.prefix(self);
    return 1;
  }
}

pub trait QSpinBox_prefix {
  fn prefix(self, this: &mut QSpinBox) -> i32;
}

// proto: QString QSpinBox::prefix();
impl<'a> /*trait*/ QSpinBox_prefix for () {
  fn prefix(self, this: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox6prefixEv()};
    unsafe {_ZNK8QSpinBox6prefixEv()};
    return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn metaObject<T: QSpinBox_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QSpinBox_metaObject {
  fn metaObject(self, this: &mut QSpinBox) -> i32;
}

// proto: const QMetaObject * QSpinBox::metaObject();
impl<'a> /*trait*/ QSpinBox_metaObject for () {
  fn metaObject(self, this: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox10metaObjectEv()};
    unsafe {_ZNK8QSpinBox10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn suffix<T: QSpinBox_suffix>(&mut self, value: T) -> i32 {
    value.suffix(self);
    return 1;
  }
}

pub trait QSpinBox_suffix {
  fn suffix(self, this: &mut QSpinBox) -> i32;
}

// proto: QString QSpinBox::suffix();
impl<'a> /*trait*/ QSpinBox_suffix for () {
  fn suffix(self, this: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox6suffixEv()};
    unsafe {_ZNK8QSpinBox6suffixEv()};
    return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn minimum<T: QSpinBox_minimum>(&mut self, value: T) -> i32 {
    value.minimum(self);
    return 1;
  }
}

pub trait QSpinBox_minimum {
  fn minimum(self, this: &mut QSpinBox) -> i32;
}

// proto: int QSpinBox::minimum();
impl<'a> /*trait*/ QSpinBox_minimum for () {
  fn minimum(self, this: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox7minimumEv()};
    unsafe {_ZNK8QSpinBox7minimumEv()};
    return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn setSingleStep<T: QSpinBox_setSingleStep>(&mut self, value: T) -> i32 {
    value.setSingleStep(self);
    return 1;
  }
}

pub trait QSpinBox_setSingleStep {
  fn setSingleStep(self, this: &mut QSpinBox) -> i32;
}

// proto: void QSpinBox::setSingleStep(int val);
impl<'a> /*trait*/ QSpinBox_setSingleStep for (i32) {
  fn setSingleStep(self, this: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox13setSingleStepEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QSpinBox13setSingleStepEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn setRange<T: QSpinBox_setRange>(&mut self, value: T) -> i32 {
    value.setRange(self);
    return 1;
  }
}

pub trait QSpinBox_setRange {
  fn setRange(self, this: &mut QSpinBox) -> i32;
}

// proto: void QSpinBox::setRange(int min, int max);
impl<'a> /*trait*/ QSpinBox_setRange for (i32, i32) {
  fn setRange(self, this: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox8setRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN8QSpinBox8setRangeEii(arg0, arg1)};
    return 1;
  }
}

