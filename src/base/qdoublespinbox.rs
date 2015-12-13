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
  // proto: void QDoubleSpinBox::valueChanged(const QString & );
  fn _ZN14QDoubleSpinBox12valueChangedERK7QString(arg0: *const c_void) -> i32;
  // proto: QString QDoubleSpinBox::textFromValue(double val);
  fn _ZNK14QDoubleSpinBox13textFromValueEd(arg0: c_double) -> i32;
  // proto: void QDoubleSpinBox::setSingleStep(double val);
  fn _ZN14QDoubleSpinBox13setSingleStepEd(arg0: c_double) -> i32;
  // proto: double QDoubleSpinBox::minimum();
  fn _ZNK14QDoubleSpinBox7minimumEv() -> i32;
  // proto: double QDoubleSpinBox::valueFromText(const QString & text);
  fn _ZNK14QDoubleSpinBox13valueFromTextERK7QString(arg0: *const c_void) -> i32;
  // proto: void QDoubleSpinBox::valueChanged(double );
  fn _ZN14QDoubleSpinBox12valueChangedEd(arg0: c_double) -> i32;
  // proto: const QMetaObject * QDoubleSpinBox::metaObject();
  fn _ZNK14QDoubleSpinBox10metaObjectEv() -> i32;
  // proto: void QDoubleSpinBox::setValue(double val);
  fn _ZN14QDoubleSpinBox8setValueEd(arg0: c_double) -> i32;
  // proto: void QDoubleSpinBox::setSuffix(const QString & suffix);
  fn _ZN14QDoubleSpinBox9setSuffixERK7QString(arg0: *const c_void) -> i32;
  // proto: int QDoubleSpinBox::decimals();
  fn _ZNK14QDoubleSpinBox8decimalsEv() -> i32;
  // proto: QString QDoubleSpinBox::prefix();
  fn _ZNK14QDoubleSpinBox6prefixEv() -> i32;
  // proto: double QDoubleSpinBox::singleStep();
  fn _ZNK14QDoubleSpinBox10singleStepEv() -> i32;
  // proto: void QDoubleSpinBox::FreeQDoubleSpinBox();
  fn _ZN14QDoubleSpinBoxD0Ev() -> i32;
  // proto: void QDoubleSpinBox::fixup(QString & str);
  fn _ZNK14QDoubleSpinBox5fixupER7QString(arg0: *mut c_void) -> i32;
  // proto: void QDoubleSpinBox::NewQDoubleSpinBox(const QDoubleSpinBox & );
  fn _ZN14QDoubleSpinBoxC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QDoubleSpinBox::setPrefix(const QString & prefix);
  fn _ZN14QDoubleSpinBox9setPrefixERK7QString(arg0: *const c_void) -> i32;
  // proto: QString QDoubleSpinBox::cleanText();
  fn _ZNK14QDoubleSpinBox9cleanTextEv() -> i32;
  // proto: void QDoubleSpinBox::setMinimum(double min);
  fn _ZN14QDoubleSpinBox10setMinimumEd(arg0: c_double) -> i32;
  // proto: void QDoubleSpinBox::setMaximum(double max);
  fn _ZN14QDoubleSpinBox10setMaximumEd(arg0: c_double) -> i32;
  // proto: void QDoubleSpinBox::setDecimals(int prec);
  fn _ZN14QDoubleSpinBox11setDecimalsEi(arg0: c_int) -> i32;
  // proto: double QDoubleSpinBox::value();
  fn _ZNK14QDoubleSpinBox5valueEv() -> i32;
  // proto: void QDoubleSpinBox::setRange(double min, double max);
  fn _ZN14QDoubleSpinBox8setRangeEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: void QDoubleSpinBox::NewQDoubleSpinBox(QWidget * parent);
  fn _ZN14QDoubleSpinBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: double QDoubleSpinBox::maximum();
  fn _ZNK14QDoubleSpinBox7maximumEv() -> i32;
  // proto: QString QDoubleSpinBox::suffix();
  fn _ZNK14QDoubleSpinBox6suffixEv() -> i32;
}

// body block begin
// class sizeof(QDoubleSpinBox)=1
pub struct QDoubleSpinBox {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDoubleSpinBox {
  pub fn valueChanged<T: QDoubleSpinBox_valueChanged>(&mut self, value: T) -> i32 {
    value.valueChanged(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_valueChanged {
  fn valueChanged(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: void QDoubleSpinBox::valueChanged(const QString & );
impl<'a> /*trait*/ QDoubleSpinBox_valueChanged for (&'a  QString) {
  fn valueChanged(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox12valueChangedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QDoubleSpinBox12valueChangedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn textFromValue<T: QDoubleSpinBox_textFromValue>(&mut self, value: T) -> i32 {
    value.textFromValue(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_textFromValue {
  fn textFromValue(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: QString QDoubleSpinBox::textFromValue(double val);
impl<'a> /*trait*/ QDoubleSpinBox_textFromValue for (f64) {
  fn textFromValue(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox13textFromValueEd()};
    let arg0 = self  as c_double;
    unsafe {_ZNK14QDoubleSpinBox13textFromValueEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn setSingleStep<T: QDoubleSpinBox_setSingleStep>(&mut self, value: T) -> i32 {
    value.setSingleStep(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_setSingleStep {
  fn setSingleStep(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: void QDoubleSpinBox::setSingleStep(double val);
impl<'a> /*trait*/ QDoubleSpinBox_setSingleStep for (f64) {
  fn setSingleStep(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox13setSingleStepEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN14QDoubleSpinBox13setSingleStepEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn minimum<T: QDoubleSpinBox_minimum>(&mut self, value: T) -> i32 {
    value.minimum(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_minimum {
  fn minimum(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: double QDoubleSpinBox::minimum();
impl<'a> /*trait*/ QDoubleSpinBox_minimum for () {
  fn minimum(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox7minimumEv()};
    unsafe {_ZNK14QDoubleSpinBox7minimumEv()};
    return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn valueFromText<T: QDoubleSpinBox_valueFromText>(&mut self, value: T) -> i32 {
    value.valueFromText(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_valueFromText {
  fn valueFromText(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: double QDoubleSpinBox::valueFromText(const QString & text);
impl<'a> /*trait*/ QDoubleSpinBox_valueFromText for (&'a  QString) {
  fn valueFromText(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox13valueFromTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK14QDoubleSpinBox13valueFromTextERK7QString(arg0)};
    return 1;
  }
}

// proto: void QDoubleSpinBox::valueChanged(double );
impl<'a> /*trait*/ QDoubleSpinBox_valueChanged for (f64) {
  fn valueChanged(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox12valueChangedEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN14QDoubleSpinBox12valueChangedEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn metaObject<T: QDoubleSpinBox_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_metaObject {
  fn metaObject(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: const QMetaObject * QDoubleSpinBox::metaObject();
impl<'a> /*trait*/ QDoubleSpinBox_metaObject for () {
  fn metaObject(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox10metaObjectEv()};
    unsafe {_ZNK14QDoubleSpinBox10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn setValue<T: QDoubleSpinBox_setValue>(&mut self, value: T) -> i32 {
    value.setValue(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_setValue {
  fn setValue(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: void QDoubleSpinBox::setValue(double val);
impl<'a> /*trait*/ QDoubleSpinBox_setValue for (f64) {
  fn setValue(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox8setValueEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN14QDoubleSpinBox8setValueEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn setSuffix<T: QDoubleSpinBox_setSuffix>(&mut self, value: T) -> i32 {
    value.setSuffix(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_setSuffix {
  fn setSuffix(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: void QDoubleSpinBox::setSuffix(const QString & suffix);
impl<'a> /*trait*/ QDoubleSpinBox_setSuffix for (&'a  QString) {
  fn setSuffix(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox9setSuffixERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QDoubleSpinBox9setSuffixERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn decimals<T: QDoubleSpinBox_decimals>(&mut self, value: T) -> i32 {
    value.decimals(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_decimals {
  fn decimals(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: int QDoubleSpinBox::decimals();
impl<'a> /*trait*/ QDoubleSpinBox_decimals for () {
  fn decimals(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox8decimalsEv()};
    unsafe {_ZNK14QDoubleSpinBox8decimalsEv()};
    return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn prefix<T: QDoubleSpinBox_prefix>(&mut self, value: T) -> i32 {
    value.prefix(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_prefix {
  fn prefix(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: QString QDoubleSpinBox::prefix();
impl<'a> /*trait*/ QDoubleSpinBox_prefix for () {
  fn prefix(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox6prefixEv()};
    unsafe {_ZNK14QDoubleSpinBox6prefixEv()};
    return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn singleStep<T: QDoubleSpinBox_singleStep>(&mut self, value: T) -> i32 {
    value.singleStep(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_singleStep {
  fn singleStep(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: double QDoubleSpinBox::singleStep();
impl<'a> /*trait*/ QDoubleSpinBox_singleStep for () {
  fn singleStep(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox10singleStepEv()};
    unsafe {_ZNK14QDoubleSpinBox10singleStepEv()};
    return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn FreeQDoubleSpinBox<T: QDoubleSpinBox_FreeQDoubleSpinBox>(&mut self, value: T) -> i32 {
    value.FreeQDoubleSpinBox(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_FreeQDoubleSpinBox {
  fn FreeQDoubleSpinBox(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: void QDoubleSpinBox::FreeQDoubleSpinBox();
impl<'a> /*trait*/ QDoubleSpinBox_FreeQDoubleSpinBox for () {
  fn FreeQDoubleSpinBox(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBoxD0Ev()};
    unsafe {_ZN14QDoubleSpinBoxD0Ev()};
    return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn fixup<T: QDoubleSpinBox_fixup>(&mut self, value: T) -> i32 {
    value.fixup(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_fixup {
  fn fixup(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: void QDoubleSpinBox::fixup(QString & str);
impl<'a> /*trait*/ QDoubleSpinBox_fixup for (&'a mut QString) {
  fn fixup(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox5fixupER7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK14QDoubleSpinBox5fixupER7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn NewQDoubleSpinBox<T: QDoubleSpinBox_NewQDoubleSpinBox>(value: T) -> QDoubleSpinBox {
    let rsthis = value.NewQDoubleSpinBox();
    return rsthis;
    // return 1;
  }
}

pub trait QDoubleSpinBox_NewQDoubleSpinBox {
  fn NewQDoubleSpinBox(self) -> QDoubleSpinBox;
}

// proto: void QDoubleSpinBox::NewQDoubleSpinBox(const QDoubleSpinBox & );
impl<'a> /*trait*/ QDoubleSpinBox_NewQDoubleSpinBox for (&'a  QDoubleSpinBox) {
  fn NewQDoubleSpinBox(self) -> QDoubleSpinBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QDoubleSpinBoxC1ERKS_(qthis, arg0)};
    let rsthis = QDoubleSpinBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn setPrefix<T: QDoubleSpinBox_setPrefix>(&mut self, value: T) -> i32 {
    value.setPrefix(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_setPrefix {
  fn setPrefix(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: void QDoubleSpinBox::setPrefix(const QString & prefix);
impl<'a> /*trait*/ QDoubleSpinBox_setPrefix for (&'a  QString) {
  fn setPrefix(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox9setPrefixERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QDoubleSpinBox9setPrefixERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn cleanText<T: QDoubleSpinBox_cleanText>(&mut self, value: T) -> i32 {
    value.cleanText(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_cleanText {
  fn cleanText(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: QString QDoubleSpinBox::cleanText();
impl<'a> /*trait*/ QDoubleSpinBox_cleanText for () {
  fn cleanText(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox9cleanTextEv()};
    unsafe {_ZNK14QDoubleSpinBox9cleanTextEv()};
    return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn setMinimum<T: QDoubleSpinBox_setMinimum>(&mut self, value: T) -> i32 {
    value.setMinimum(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_setMinimum {
  fn setMinimum(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: void QDoubleSpinBox::setMinimum(double min);
impl<'a> /*trait*/ QDoubleSpinBox_setMinimum for (f64) {
  fn setMinimum(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox10setMinimumEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN14QDoubleSpinBox10setMinimumEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn setMaximum<T: QDoubleSpinBox_setMaximum>(&mut self, value: T) -> i32 {
    value.setMaximum(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_setMaximum {
  fn setMaximum(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: void QDoubleSpinBox::setMaximum(double max);
impl<'a> /*trait*/ QDoubleSpinBox_setMaximum for (f64) {
  fn setMaximum(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox10setMaximumEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN14QDoubleSpinBox10setMaximumEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn setDecimals<T: QDoubleSpinBox_setDecimals>(&mut self, value: T) -> i32 {
    value.setDecimals(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_setDecimals {
  fn setDecimals(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: void QDoubleSpinBox::setDecimals(int prec);
impl<'a> /*trait*/ QDoubleSpinBox_setDecimals for (i32) {
  fn setDecimals(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox11setDecimalsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QDoubleSpinBox11setDecimalsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn value<T: QDoubleSpinBox_value>(&mut self, value: T) -> i32 {
    value.value(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_value {
  fn value(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: double QDoubleSpinBox::value();
impl<'a> /*trait*/ QDoubleSpinBox_value for () {
  fn value(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox5valueEv()};
    unsafe {_ZNK14QDoubleSpinBox5valueEv()};
    return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn setRange<T: QDoubleSpinBox_setRange>(&mut self, value: T) -> i32 {
    value.setRange(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_setRange {
  fn setRange(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: void QDoubleSpinBox::setRange(double min, double max);
impl<'a> /*trait*/ QDoubleSpinBox_setRange for (f64, f64) {
  fn setRange(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox8setRangeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN14QDoubleSpinBox8setRangeEdd(arg0, arg1)};
    return 1;
  }
}

// proto: void QDoubleSpinBox::NewQDoubleSpinBox(QWidget * parent);
impl<'a> /*trait*/ QDoubleSpinBox_NewQDoubleSpinBox for (&'a mut QWidget) {
  fn NewQDoubleSpinBox(self) -> QDoubleSpinBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBoxC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QDoubleSpinBoxC1EP7QWidget(qthis, arg0)};
    let rsthis = QDoubleSpinBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn maximum<T: QDoubleSpinBox_maximum>(&mut self, value: T) -> i32 {
    value.maximum(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_maximum {
  fn maximum(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: double QDoubleSpinBox::maximum();
impl<'a> /*trait*/ QDoubleSpinBox_maximum for () {
  fn maximum(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox7maximumEv()};
    unsafe {_ZNK14QDoubleSpinBox7maximumEv()};
    return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn suffix<T: QDoubleSpinBox_suffix>(&mut self, value: T) -> i32 {
    value.suffix(self);
    return 1;
  }
}

pub trait QDoubleSpinBox_suffix {
  fn suffix(self, this: &mut QDoubleSpinBox) -> i32;
}

// proto: QString QDoubleSpinBox::suffix();
impl<'a> /*trait*/ QDoubleSpinBox_suffix for () {
  fn suffix(self, this: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox6suffixEv()};
    unsafe {_ZNK14QDoubleSpinBox6suffixEv()};
    return 1;
  }
}

