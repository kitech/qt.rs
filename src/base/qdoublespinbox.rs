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
  // proto:  void QDoubleSpinBox::valueChanged(const QString & );
  fn _ZN14QDoubleSpinBox12valueChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QDoubleSpinBox::textFromValue(double val);
  fn _ZNK14QDoubleSpinBox13textFromValueEd(qthis: *mut c_void, arg0: c_double) -> *mut c_void;
  // proto:  void QDoubleSpinBox::setSingleStep(double val);
  fn _ZN14QDoubleSpinBox13setSingleStepEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QDoubleSpinBox::minimum();
  fn _ZNK14QDoubleSpinBox7minimumEv(qthis: *mut c_void) -> c_double;
  // proto:  double QDoubleSpinBox::valueFromText(const QString & text);
  fn _ZNK14QDoubleSpinBox13valueFromTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_double;
  // proto:  void QDoubleSpinBox::valueChanged(double );
  fn _ZN14QDoubleSpinBox12valueChangedEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  const QMetaObject * QDoubleSpinBox::metaObject();
  fn _ZNK14QDoubleSpinBox10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QDoubleSpinBox::setValue(double val);
  fn _ZN14QDoubleSpinBox8setValueEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QDoubleSpinBox::setSuffix(const QString & suffix);
  fn _ZN14QDoubleSpinBox9setSuffixERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QDoubleSpinBox::decimals();
  fn _ZNK14QDoubleSpinBox8decimalsEv(qthis: *mut c_void) -> c_int;
  // proto:  QString QDoubleSpinBox::prefix();
  fn _ZNK14QDoubleSpinBox6prefixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QDoubleSpinBox::singleStep();
  fn _ZNK14QDoubleSpinBox10singleStepEv(qthis: *mut c_void) -> c_double;
  // proto:  void QDoubleSpinBox::FreeQDoubleSpinBox();
  fn _ZN14QDoubleSpinBoxD0Ev(qthis: *mut c_void) ;
  // proto:  void QDoubleSpinBox::fixup(QString & str);
  fn _ZNK14QDoubleSpinBox5fixupER7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDoubleSpinBox::NewQDoubleSpinBox(const QDoubleSpinBox & );
  fn _ZN14QDoubleSpinBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDoubleSpinBox::setPrefix(const QString & prefix);
  fn _ZN14QDoubleSpinBox9setPrefixERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QDoubleSpinBox::cleanText();
  fn _ZNK14QDoubleSpinBox9cleanTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDoubleSpinBox::setMinimum(double min);
  fn _ZN14QDoubleSpinBox10setMinimumEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QDoubleSpinBox::setMaximum(double max);
  fn _ZN14QDoubleSpinBox10setMaximumEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QDoubleSpinBox::setDecimals(int prec);
  fn _ZN14QDoubleSpinBox11setDecimalsEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  double QDoubleSpinBox::value();
  fn _ZNK14QDoubleSpinBox5valueEv(qthis: *mut c_void) -> c_double;
  // proto:  void QDoubleSpinBox::setRange(double min, double max);
  fn _ZN14QDoubleSpinBox8setRangeEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  void QDoubleSpinBox::NewQDoubleSpinBox(QWidget * parent);
  fn _ZN14QDoubleSpinBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QDoubleSpinBox::maximum();
  fn _ZNK14QDoubleSpinBox7maximumEv(qthis: *mut c_void) -> c_double;
  // proto:  QString QDoubleSpinBox::suffix();
  fn _ZNK14QDoubleSpinBox6suffixEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QDoubleSpinBox)=1
pub struct QDoubleSpinBox {
  pub qclsinst: *mut c_void,
}

// proto:  void QDoubleSpinBox::valueChanged(const QString & );
impl /*struct*/ QDoubleSpinBox {
  pub fn valueChanged<RetType, T: QDoubleSpinBox_valueChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.valueChanged(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_valueChanged<RetType> {
  fn valueChanged(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  void QDoubleSpinBox::valueChanged(const QString & );
impl<'a> /*trait*/ QDoubleSpinBox_valueChanged<()> for (&'a  QString) {
  fn valueChanged(self , rsthis: &mut QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox12valueChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QDoubleSpinBox12valueChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QString QDoubleSpinBox::textFromValue(double val);
impl /*struct*/ QDoubleSpinBox {
  pub fn textFromValue<RetType, T: QDoubleSpinBox_textFromValue<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.textFromValue(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_textFromValue<RetType> {
  fn textFromValue(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  QString QDoubleSpinBox::textFromValue(double val);
impl<'a> /*trait*/ QDoubleSpinBox_textFromValue<QString> for (f64) {
  fn textFromValue(self , rsthis: &mut QDoubleSpinBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox13textFromValueEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK14QDoubleSpinBox13textFromValueEd(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QDoubleSpinBox::setSingleStep(double val);
impl /*struct*/ QDoubleSpinBox {
  pub fn setSingleStep<RetType, T: QDoubleSpinBox_setSingleStep<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setSingleStep(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_setSingleStep<RetType> {
  fn setSingleStep(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  void QDoubleSpinBox::setSingleStep(double val);
impl<'a> /*trait*/ QDoubleSpinBox_setSingleStep<()> for (f64) {
  fn setSingleStep(self , rsthis: &mut QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox13setSingleStepEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN14QDoubleSpinBox13setSingleStepEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  double QDoubleSpinBox::minimum();
impl /*struct*/ QDoubleSpinBox {
  pub fn minimum<RetType, T: QDoubleSpinBox_minimum<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.minimum(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_minimum<RetType> {
  fn minimum(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  double QDoubleSpinBox::minimum();
impl<'a> /*trait*/ QDoubleSpinBox_minimum<f64> for () {
  fn minimum(self , rsthis: &mut QDoubleSpinBox) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox7minimumEv()};
    let mut ret = unsafe {_ZNK14QDoubleSpinBox7minimumEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  double QDoubleSpinBox::valueFromText(const QString & text);
impl /*struct*/ QDoubleSpinBox {
  pub fn valueFromText<RetType, T: QDoubleSpinBox_valueFromText<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.valueFromText(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_valueFromText<RetType> {
  fn valueFromText(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  double QDoubleSpinBox::valueFromText(const QString & text);
impl<'a> /*trait*/ QDoubleSpinBox_valueFromText<f64> for (&'a  QString) {
  fn valueFromText(self , rsthis: &mut QDoubleSpinBox) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox13valueFromTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QDoubleSpinBox13valueFromTextERK7QString(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

// proto:  void QDoubleSpinBox::valueChanged(double );
impl<'a> /*trait*/ QDoubleSpinBox_valueChanged<()> for (f64) {
  fn valueChanged(self , rsthis: &mut QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox12valueChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN14QDoubleSpinBox12valueChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  const QMetaObject * QDoubleSpinBox::metaObject();
impl /*struct*/ QDoubleSpinBox {
  pub fn metaObject<RetType, T: QDoubleSpinBox_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  const QMetaObject * QDoubleSpinBox::metaObject();
impl<'a> /*trait*/ QDoubleSpinBox_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox10metaObjectEv()};
     unsafe {_ZNK14QDoubleSpinBox10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QDoubleSpinBox::setValue(double val);
impl /*struct*/ QDoubleSpinBox {
  pub fn setValue<RetType, T: QDoubleSpinBox_setValue<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setValue(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_setValue<RetType> {
  fn setValue(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  void QDoubleSpinBox::setValue(double val);
impl<'a> /*trait*/ QDoubleSpinBox_setValue<()> for (f64) {
  fn setValue(self , rsthis: &mut QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox8setValueEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN14QDoubleSpinBox8setValueEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QDoubleSpinBox::setSuffix(const QString & suffix);
impl /*struct*/ QDoubleSpinBox {
  pub fn setSuffix<RetType, T: QDoubleSpinBox_setSuffix<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setSuffix(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_setSuffix<RetType> {
  fn setSuffix(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  void QDoubleSpinBox::setSuffix(const QString & suffix);
impl<'a> /*trait*/ QDoubleSpinBox_setSuffix<()> for (&'a  QString) {
  fn setSuffix(self , rsthis: &mut QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox9setSuffixERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QDoubleSpinBox9setSuffixERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QDoubleSpinBox::decimals();
impl /*struct*/ QDoubleSpinBox {
  pub fn decimals<RetType, T: QDoubleSpinBox_decimals<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.decimals(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_decimals<RetType> {
  fn decimals(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  int QDoubleSpinBox::decimals();
impl<'a> /*trait*/ QDoubleSpinBox_decimals<i32> for () {
  fn decimals(self , rsthis: &mut QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox8decimalsEv()};
    let mut ret = unsafe {_ZNK14QDoubleSpinBox8decimalsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QString QDoubleSpinBox::prefix();
impl /*struct*/ QDoubleSpinBox {
  pub fn prefix<RetType, T: QDoubleSpinBox_prefix<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.prefix(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_prefix<RetType> {
  fn prefix(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  QString QDoubleSpinBox::prefix();
impl<'a> /*trait*/ QDoubleSpinBox_prefix<QString> for () {
  fn prefix(self , rsthis: &mut QDoubleSpinBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox6prefixEv()};
    let mut ret = unsafe {_ZNK14QDoubleSpinBox6prefixEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  double QDoubleSpinBox::singleStep();
impl /*struct*/ QDoubleSpinBox {
  pub fn singleStep<RetType, T: QDoubleSpinBox_singleStep<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.singleStep(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_singleStep<RetType> {
  fn singleStep(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  double QDoubleSpinBox::singleStep();
impl<'a> /*trait*/ QDoubleSpinBox_singleStep<f64> for () {
  fn singleStep(self , rsthis: &mut QDoubleSpinBox) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox10singleStepEv()};
    let mut ret = unsafe {_ZNK14QDoubleSpinBox10singleStepEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  void QDoubleSpinBox::FreeQDoubleSpinBox();
impl /*struct*/ QDoubleSpinBox {
  pub fn FreeQDoubleSpinBox<RetType, T: QDoubleSpinBox_FreeQDoubleSpinBox<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQDoubleSpinBox(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_FreeQDoubleSpinBox<RetType> {
  fn FreeQDoubleSpinBox(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  void QDoubleSpinBox::FreeQDoubleSpinBox();
impl<'a> /*trait*/ QDoubleSpinBox_FreeQDoubleSpinBox<()> for () {
  fn FreeQDoubleSpinBox(self , rsthis: &mut QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBoxD0Ev()};
     unsafe {_ZN14QDoubleSpinBoxD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QDoubleSpinBox::fixup(QString & str);
impl /*struct*/ QDoubleSpinBox {
  pub fn fixup<RetType, T: QDoubleSpinBox_fixup<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.fixup(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_fixup<RetType> {
  fn fixup(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  void QDoubleSpinBox::fixup(QString & str);
impl<'a> /*trait*/ QDoubleSpinBox_fixup<()> for (&'a mut QString) {
  fn fixup(self , rsthis: &mut QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox5fixupER7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK14QDoubleSpinBox5fixupER7QString(rsthis.qclsinst, arg0)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QDoubleSpinBoxC1ERKS_(qthis, arg0)};
    let rsthis = QDoubleSpinBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QDoubleSpinBox::setPrefix(const QString & prefix);
impl /*struct*/ QDoubleSpinBox {
  pub fn setPrefix<RetType, T: QDoubleSpinBox_setPrefix<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setPrefix(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_setPrefix<RetType> {
  fn setPrefix(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  void QDoubleSpinBox::setPrefix(const QString & prefix);
impl<'a> /*trait*/ QDoubleSpinBox_setPrefix<()> for (&'a  QString) {
  fn setPrefix(self , rsthis: &mut QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox9setPrefixERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QDoubleSpinBox9setPrefixERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QString QDoubleSpinBox::cleanText();
impl /*struct*/ QDoubleSpinBox {
  pub fn cleanText<RetType, T: QDoubleSpinBox_cleanText<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.cleanText(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_cleanText<RetType> {
  fn cleanText(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  QString QDoubleSpinBox::cleanText();
impl<'a> /*trait*/ QDoubleSpinBox_cleanText<QString> for () {
  fn cleanText(self , rsthis: &mut QDoubleSpinBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox9cleanTextEv()};
    let mut ret = unsafe {_ZNK14QDoubleSpinBox9cleanTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QDoubleSpinBox::setMinimum(double min);
impl /*struct*/ QDoubleSpinBox {
  pub fn setMinimum<RetType, T: QDoubleSpinBox_setMinimum<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setMinimum(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_setMinimum<RetType> {
  fn setMinimum(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  void QDoubleSpinBox::setMinimum(double min);
impl<'a> /*trait*/ QDoubleSpinBox_setMinimum<()> for (f64) {
  fn setMinimum(self , rsthis: &mut QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox10setMinimumEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN14QDoubleSpinBox10setMinimumEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QDoubleSpinBox::setMaximum(double max);
impl /*struct*/ QDoubleSpinBox {
  pub fn setMaximum<RetType, T: QDoubleSpinBox_setMaximum<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setMaximum(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_setMaximum<RetType> {
  fn setMaximum(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  void QDoubleSpinBox::setMaximum(double max);
impl<'a> /*trait*/ QDoubleSpinBox_setMaximum<()> for (f64) {
  fn setMaximum(self , rsthis: &mut QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox10setMaximumEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN14QDoubleSpinBox10setMaximumEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QDoubleSpinBox::setDecimals(int prec);
impl /*struct*/ QDoubleSpinBox {
  pub fn setDecimals<RetType, T: QDoubleSpinBox_setDecimals<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setDecimals(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_setDecimals<RetType> {
  fn setDecimals(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  void QDoubleSpinBox::setDecimals(int prec);
impl<'a> /*trait*/ QDoubleSpinBox_setDecimals<()> for (i32) {
  fn setDecimals(self , rsthis: &mut QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox11setDecimalsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QDoubleSpinBox11setDecimalsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  double QDoubleSpinBox::value();
impl /*struct*/ QDoubleSpinBox {
  pub fn value<RetType, T: QDoubleSpinBox_value<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_value<RetType> {
  fn value(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  double QDoubleSpinBox::value();
impl<'a> /*trait*/ QDoubleSpinBox_value<f64> for () {
  fn value(self , rsthis: &mut QDoubleSpinBox) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox5valueEv()};
    let mut ret = unsafe {_ZNK14QDoubleSpinBox5valueEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  void QDoubleSpinBox::setRange(double min, double max);
impl /*struct*/ QDoubleSpinBox {
  pub fn setRange<RetType, T: QDoubleSpinBox_setRange<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setRange(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_setRange<RetType> {
  fn setRange(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  void QDoubleSpinBox::setRange(double min, double max);
impl<'a> /*trait*/ QDoubleSpinBox_setRange<()> for (f64, f64) {
  fn setRange(self , rsthis: &mut QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox8setRangeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN14QDoubleSpinBox8setRangeEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
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

// proto:  double QDoubleSpinBox::maximum();
impl /*struct*/ QDoubleSpinBox {
  pub fn maximum<RetType, T: QDoubleSpinBox_maximum<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.maximum(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_maximum<RetType> {
  fn maximum(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  double QDoubleSpinBox::maximum();
impl<'a> /*trait*/ QDoubleSpinBox_maximum<f64> for () {
  fn maximum(self , rsthis: &mut QDoubleSpinBox) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox7maximumEv()};
    let mut ret = unsafe {_ZNK14QDoubleSpinBox7maximumEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  QString QDoubleSpinBox::suffix();
impl /*struct*/ QDoubleSpinBox {
  pub fn suffix<RetType, T: QDoubleSpinBox_suffix<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.suffix(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_suffix<RetType> {
  fn suffix(self , rsthis: &mut QDoubleSpinBox) -> RetType;
}

// proto:  QString QDoubleSpinBox::suffix();
impl<'a> /*trait*/ QDoubleSpinBox_suffix<QString> for () {
  fn suffix(self , rsthis: &mut QDoubleSpinBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox6suffixEv()};
    let mut ret = unsafe {_ZNK14QDoubleSpinBox6suffixEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

