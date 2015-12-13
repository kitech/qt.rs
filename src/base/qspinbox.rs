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
  // proto:  void QSpinBox::setMinimum(int min);
  fn _ZN8QSpinBox10setMinimumEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QString QSpinBox::cleanText();
  fn _ZNK8QSpinBox9cleanTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QSpinBox::value();
  fn _ZNK8QSpinBox5valueEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSpinBox::FreeQSpinBox();
  fn _ZN8QSpinBoxD0Ev(qthis: *mut c_void) ;
  // proto:  void QSpinBox::setMaximum(int max);
  fn _ZN8QSpinBox10setMaximumEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QSpinBox::valueChanged(const QString & );
  fn _ZN8QSpinBox12valueChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSpinBox::setValue(int val);
  fn _ZN8QSpinBox8setValueEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QSpinBox::setDisplayIntegerBase(int base);
  fn _ZN8QSpinBox21setDisplayIntegerBaseEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QSpinBox::NewQSpinBox(QWidget * parent);
  fn _ZN8QSpinBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QSpinBox::singleStep();
  fn _ZNK8QSpinBox10singleStepEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSpinBox::NewQSpinBox(const QSpinBox & );
  fn _ZN8QSpinBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QSpinBox::displayIntegerBase();
  fn _ZNK8QSpinBox18displayIntegerBaseEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSpinBox::setSuffix(const QString & suffix);
  fn _ZN8QSpinBox9setSuffixERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QSpinBox::maximum();
  fn _ZNK8QSpinBox7maximumEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSpinBox::setPrefix(const QString & prefix);
  fn _ZN8QSpinBox9setPrefixERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSpinBox::valueChanged(int );
  fn _ZN8QSpinBox12valueChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QString QSpinBox::prefix();
  fn _ZNK8QSpinBox6prefixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QSpinBox::metaObject();
  fn _ZNK8QSpinBox10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QString QSpinBox::suffix();
  fn _ZNK8QSpinBox6suffixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QSpinBox::minimum();
  fn _ZNK8QSpinBox7minimumEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSpinBox::setSingleStep(int val);
  fn _ZN8QSpinBox13setSingleStepEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QSpinBox::setRange(int min, int max);
  fn _ZN8QSpinBox8setRangeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
}

// body block begin
// class sizeof(QSpinBox)=1
pub struct QSpinBox {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSpinBox {
  pub fn setMinimum<T: QSpinBox_setMinimum>(&mut self, value: T)  {
     value.setMinimum(self);
    // return 1;
  }
}

pub trait QSpinBox_setMinimum {
  fn setMinimum(self, rsthis: &mut QSpinBox) ;
}

// proto:  void QSpinBox::setMinimum(int min);
impl<'a> /*trait*/ QSpinBox_setMinimum for (i32) {
  fn setMinimum(self, rsthis: &mut QSpinBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox10setMinimumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QSpinBox10setMinimumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn cleanText<T: QSpinBox_cleanText>(&mut self, value: T) -> QString {
    return value.cleanText(self);
    // return 1;
  }
}

pub trait QSpinBox_cleanText {
  fn cleanText(self, rsthis: &mut QSpinBox) -> QString;
}

// proto:  QString QSpinBox::cleanText();
impl<'a> /*trait*/ QSpinBox_cleanText for () {
  fn cleanText(self, rsthis: &mut QSpinBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox9cleanTextEv()};
    let mut ret = unsafe {_ZNK8QSpinBox9cleanTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn value<T: QSpinBox_value>(&mut self, value: T) -> i32 {
    return value.value(self);
    // return 1;
  }
}

pub trait QSpinBox_value {
  fn value(self, rsthis: &mut QSpinBox) -> i32;
}

// proto:  int QSpinBox::value();
impl<'a> /*trait*/ QSpinBox_value for () {
  fn value(self, rsthis: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox5valueEv()};
    let mut ret = unsafe {_ZNK8QSpinBox5valueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn FreeQSpinBox<T: QSpinBox_FreeQSpinBox>(&mut self, value: T)  {
     value.FreeQSpinBox(self);
    // return 1;
  }
}

pub trait QSpinBox_FreeQSpinBox {
  fn FreeQSpinBox(self, rsthis: &mut QSpinBox) ;
}

// proto:  void QSpinBox::FreeQSpinBox();
impl<'a> /*trait*/ QSpinBox_FreeQSpinBox for () {
  fn FreeQSpinBox(self, rsthis: &mut QSpinBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBoxD0Ev()};
     unsafe {_ZN8QSpinBoxD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn setMaximum<T: QSpinBox_setMaximum>(&mut self, value: T)  {
     value.setMaximum(self);
    // return 1;
  }
}

pub trait QSpinBox_setMaximum {
  fn setMaximum(self, rsthis: &mut QSpinBox) ;
}

// proto:  void QSpinBox::setMaximum(int max);
impl<'a> /*trait*/ QSpinBox_setMaximum for (i32) {
  fn setMaximum(self, rsthis: &mut QSpinBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox10setMaximumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QSpinBox10setMaximumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn valueChanged<T: QSpinBox_valueChanged>(&mut self, value: T)  {
     value.valueChanged(self);
    // return 1;
  }
}

pub trait QSpinBox_valueChanged {
  fn valueChanged(self, rsthis: &mut QSpinBox) ;
}

// proto:  void QSpinBox::valueChanged(const QString & );
impl<'a> /*trait*/ QSpinBox_valueChanged for (&'a  QString) {
  fn valueChanged(self, rsthis: &mut QSpinBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox12valueChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QSpinBox12valueChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn setValue<T: QSpinBox_setValue>(&mut self, value: T)  {
     value.setValue(self);
    // return 1;
  }
}

pub trait QSpinBox_setValue {
  fn setValue(self, rsthis: &mut QSpinBox) ;
}

// proto:  void QSpinBox::setValue(int val);
impl<'a> /*trait*/ QSpinBox_setValue for (i32) {
  fn setValue(self, rsthis: &mut QSpinBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox8setValueEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QSpinBox8setValueEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn setDisplayIntegerBase<T: QSpinBox_setDisplayIntegerBase>(&mut self, value: T)  {
     value.setDisplayIntegerBase(self);
    // return 1;
  }
}

pub trait QSpinBox_setDisplayIntegerBase {
  fn setDisplayIntegerBase(self, rsthis: &mut QSpinBox) ;
}

// proto:  void QSpinBox::setDisplayIntegerBase(int base);
impl<'a> /*trait*/ QSpinBox_setDisplayIntegerBase for (i32) {
  fn setDisplayIntegerBase(self, rsthis: &mut QSpinBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox21setDisplayIntegerBaseEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QSpinBox21setDisplayIntegerBaseEi(rsthis.qclsinst, arg0)};
    // return 1;
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
    return value.singleStep(self);
    // return 1;
  }
}

pub trait QSpinBox_singleStep {
  fn singleStep(self, rsthis: &mut QSpinBox) -> i32;
}

// proto:  int QSpinBox::singleStep();
impl<'a> /*trait*/ QSpinBox_singleStep for () {
  fn singleStep(self, rsthis: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox10singleStepEv()};
    let mut ret = unsafe {_ZNK8QSpinBox10singleStepEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QSpinBox::NewQSpinBox(const QSpinBox & );
impl<'a> /*trait*/ QSpinBox_NewQSpinBox for (&'a  QSpinBox) {
  fn NewQSpinBox(self) -> QSpinBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QSpinBoxC1ERKS_(qthis, arg0)};
    let rsthis = QSpinBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn displayIntegerBase<T: QSpinBox_displayIntegerBase>(&mut self, value: T) -> i32 {
    return value.displayIntegerBase(self);
    // return 1;
  }
}

pub trait QSpinBox_displayIntegerBase {
  fn displayIntegerBase(self, rsthis: &mut QSpinBox) -> i32;
}

// proto:  int QSpinBox::displayIntegerBase();
impl<'a> /*trait*/ QSpinBox_displayIntegerBase for () {
  fn displayIntegerBase(self, rsthis: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox18displayIntegerBaseEv()};
    let mut ret = unsafe {_ZNK8QSpinBox18displayIntegerBaseEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn setSuffix<T: QSpinBox_setSuffix>(&mut self, value: T)  {
     value.setSuffix(self);
    // return 1;
  }
}

pub trait QSpinBox_setSuffix {
  fn setSuffix(self, rsthis: &mut QSpinBox) ;
}

// proto:  void QSpinBox::setSuffix(const QString & suffix);
impl<'a> /*trait*/ QSpinBox_setSuffix for (&'a  QString) {
  fn setSuffix(self, rsthis: &mut QSpinBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox9setSuffixERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QSpinBox9setSuffixERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn maximum<T: QSpinBox_maximum>(&mut self, value: T) -> i32 {
    return value.maximum(self);
    // return 1;
  }
}

pub trait QSpinBox_maximum {
  fn maximum(self, rsthis: &mut QSpinBox) -> i32;
}

// proto:  int QSpinBox::maximum();
impl<'a> /*trait*/ QSpinBox_maximum for () {
  fn maximum(self, rsthis: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox7maximumEv()};
    let mut ret = unsafe {_ZNK8QSpinBox7maximumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn setPrefix<T: QSpinBox_setPrefix>(&mut self, value: T)  {
     value.setPrefix(self);
    // return 1;
  }
}

pub trait QSpinBox_setPrefix {
  fn setPrefix(self, rsthis: &mut QSpinBox) ;
}

// proto:  void QSpinBox::setPrefix(const QString & prefix);
impl<'a> /*trait*/ QSpinBox_setPrefix for (&'a  QString) {
  fn setPrefix(self, rsthis: &mut QSpinBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox9setPrefixERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QSpinBox9setPrefixERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QSpinBox::valueChanged(int );
impl<'a> /*trait*/ QSpinBox_valueChanged for (i32) {
  fn valueChanged(self, rsthis: &mut QSpinBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox12valueChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QSpinBox12valueChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn prefix<T: QSpinBox_prefix>(&mut self, value: T) -> QString {
    return value.prefix(self);
    // return 1;
  }
}

pub trait QSpinBox_prefix {
  fn prefix(self, rsthis: &mut QSpinBox) -> QString;
}

// proto:  QString QSpinBox::prefix();
impl<'a> /*trait*/ QSpinBox_prefix for () {
  fn prefix(self, rsthis: &mut QSpinBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox6prefixEv()};
    let mut ret = unsafe {_ZNK8QSpinBox6prefixEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn metaObject<T: QSpinBox_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QSpinBox_metaObject {
  fn metaObject(self, rsthis: &mut QSpinBox) ;
}

// proto:  const QMetaObject * QSpinBox::metaObject();
impl<'a> /*trait*/ QSpinBox_metaObject for () {
  fn metaObject(self, rsthis: &mut QSpinBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox10metaObjectEv()};
     unsafe {_ZNK8QSpinBox10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn suffix<T: QSpinBox_suffix>(&mut self, value: T) -> QString {
    return value.suffix(self);
    // return 1;
  }
}

pub trait QSpinBox_suffix {
  fn suffix(self, rsthis: &mut QSpinBox) -> QString;
}

// proto:  QString QSpinBox::suffix();
impl<'a> /*trait*/ QSpinBox_suffix for () {
  fn suffix(self, rsthis: &mut QSpinBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox6suffixEv()};
    let mut ret = unsafe {_ZNK8QSpinBox6suffixEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn minimum<T: QSpinBox_minimum>(&mut self, value: T) -> i32 {
    return value.minimum(self);
    // return 1;
  }
}

pub trait QSpinBox_minimum {
  fn minimum(self, rsthis: &mut QSpinBox) -> i32;
}

// proto:  int QSpinBox::minimum();
impl<'a> /*trait*/ QSpinBox_minimum for () {
  fn minimum(self, rsthis: &mut QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox7minimumEv()};
    let mut ret = unsafe {_ZNK8QSpinBox7minimumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn setSingleStep<T: QSpinBox_setSingleStep>(&mut self, value: T)  {
     value.setSingleStep(self);
    // return 1;
  }
}

pub trait QSpinBox_setSingleStep {
  fn setSingleStep(self, rsthis: &mut QSpinBox) ;
}

// proto:  void QSpinBox::setSingleStep(int val);
impl<'a> /*trait*/ QSpinBox_setSingleStep for (i32) {
  fn setSingleStep(self, rsthis: &mut QSpinBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox13setSingleStepEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QSpinBox13setSingleStepEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSpinBox {
  pub fn setRange<T: QSpinBox_setRange>(&mut self, value: T)  {
     value.setRange(self);
    // return 1;
  }
}

pub trait QSpinBox_setRange {
  fn setRange(self, rsthis: &mut QSpinBox) ;
}

// proto:  void QSpinBox::setRange(int min, int max);
impl<'a> /*trait*/ QSpinBox_setRange for (i32, i32) {
  fn setRange(self, rsthis: &mut QSpinBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox8setRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QSpinBox8setRangeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

