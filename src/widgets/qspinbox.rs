// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtWidgets/qspinbox.h
// dst-file: /src/widgets/qspinbox.rs
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
use super::qabstractspinbox::QAbstractSpinBox; // 773
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::qwidget::QWidget; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSpinBox_Class_Size() -> c_int;
  // proto:  void QSpinBox::setMinimum(int min);
  fn _ZN8QSpinBox10setMinimumEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QString QSpinBox::cleanText();
  fn _ZNK8QSpinBox9cleanTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QSpinBox::value();
  fn _ZNK8QSpinBox5valueEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSpinBox::~QSpinBox();
  fn _ZN8QSpinBoxD0Ev(qthis: *mut c_void);
  // proto:  void QSpinBox::setMaximum(int max);
  fn _ZN8QSpinBox10setMaximumEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QSpinBox::valueChanged(const QString & );
  fn _ZN8QSpinBox12valueChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSpinBox::setValue(int val);
  fn _ZN8QSpinBox8setValueEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QSpinBox::setDisplayIntegerBase(int base);
  fn _ZN8QSpinBox21setDisplayIntegerBaseEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QSpinBox::QSpinBox(QWidget * parent);
  fn dector_ZN8QSpinBoxC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN8QSpinBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QSpinBox::singleStep();
  fn _ZNK8QSpinBox10singleStepEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSpinBox::QSpinBox(const QSpinBox & );
  fn dector_ZN8QSpinBoxC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN8QSpinBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QSpinBox::displayIntegerBase();
  fn _ZNK8QSpinBox18displayIntegerBaseEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSpinBox::setSuffix(const QString & suffix);
  fn _ZN8QSpinBox9setSuffixERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QSpinBox::maximum();
  fn _ZNK8QSpinBox7maximumEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSpinBox::setPrefix(const QString & prefix);
  fn _ZN8QSpinBox9setPrefixERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSpinBox::valueChanged(int );
  fn _ZN8QSpinBox12valueChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QString QSpinBox::prefix();
  fn _ZNK8QSpinBox6prefixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QSpinBox::metaObject();
  fn _ZNK8QSpinBox10metaObjectEv(qthis: *mut c_void);
  // proto:  QString QSpinBox::suffix();
  fn _ZNK8QSpinBox6suffixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QSpinBox::minimum();
  fn _ZNK8QSpinBox7minimumEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSpinBox::setSingleStep(int val);
  fn _ZN8QSpinBox13setSingleStepEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QSpinBox::setRange(int min, int max);
  fn _ZN8QSpinBox8setRangeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  fn QDoubleSpinBox_Class_Size() -> c_int;
  // proto:  void QDoubleSpinBox::valueChanged(const QString & );
  fn _ZN14QDoubleSpinBox12valueChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QDoubleSpinBox::textFromValue(double val);
  fn _ZNK14QDoubleSpinBox13textFromValueEd(qthis: *mut c_void, arg0: c_double) -> *mut c_void;
  // proto:  void QDoubleSpinBox::setSingleStep(double val);
  fn _ZN14QDoubleSpinBox13setSingleStepEd(qthis: *mut c_void, arg0: c_double);
  // proto:  double QDoubleSpinBox::minimum();
  fn _ZNK14QDoubleSpinBox7minimumEv(qthis: *mut c_void) -> c_double;
  // proto:  double QDoubleSpinBox::valueFromText(const QString & text);
  fn _ZNK14QDoubleSpinBox13valueFromTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_double;
  // proto:  void QDoubleSpinBox::valueChanged(double );
  fn _ZN14QDoubleSpinBox12valueChangedEd(qthis: *mut c_void, arg0: c_double);
  // proto:  const QMetaObject * QDoubleSpinBox::metaObject();
  fn _ZNK14QDoubleSpinBox10metaObjectEv(qthis: *mut c_void);
  // proto:  void QDoubleSpinBox::setValue(double val);
  fn _ZN14QDoubleSpinBox8setValueEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QDoubleSpinBox::setSuffix(const QString & suffix);
  fn _ZN14QDoubleSpinBox9setSuffixERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QDoubleSpinBox::decimals();
  fn _ZNK14QDoubleSpinBox8decimalsEv(qthis: *mut c_void) -> c_int;
  // proto:  QString QDoubleSpinBox::prefix();
  fn _ZNK14QDoubleSpinBox6prefixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QDoubleSpinBox::singleStep();
  fn _ZNK14QDoubleSpinBox10singleStepEv(qthis: *mut c_void) -> c_double;
  // proto:  void QDoubleSpinBox::~QDoubleSpinBox();
  fn _ZN14QDoubleSpinBoxD0Ev(qthis: *mut c_void);
  // proto:  void QDoubleSpinBox::fixup(QString & str);
  fn _ZNK14QDoubleSpinBox5fixupER7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDoubleSpinBox::QDoubleSpinBox(const QDoubleSpinBox & );
  fn dector_ZN14QDoubleSpinBoxC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN14QDoubleSpinBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDoubleSpinBox::setPrefix(const QString & prefix);
  fn _ZN14QDoubleSpinBox9setPrefixERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QDoubleSpinBox::cleanText();
  fn _ZNK14QDoubleSpinBox9cleanTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDoubleSpinBox::setMinimum(double min);
  fn _ZN14QDoubleSpinBox10setMinimumEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QDoubleSpinBox::setMaximum(double max);
  fn _ZN14QDoubleSpinBox10setMaximumEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QDoubleSpinBox::setDecimals(int prec);
  fn _ZN14QDoubleSpinBox11setDecimalsEi(qthis: *mut c_void, arg0: c_int);
  // proto:  double QDoubleSpinBox::value();
  fn _ZNK14QDoubleSpinBox5valueEv(qthis: *mut c_void) -> c_double;
  // proto:  void QDoubleSpinBox::setRange(double min, double max);
  fn _ZN14QDoubleSpinBox8setRangeEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QDoubleSpinBox::QDoubleSpinBox(QWidget * parent);
  fn dector_ZN14QDoubleSpinBoxC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN14QDoubleSpinBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  double QDoubleSpinBox::maximum();
  fn _ZNK14QDoubleSpinBox7maximumEv(qthis: *mut c_void) -> c_double;
  // proto:  QString QDoubleSpinBox::suffix();
  fn _ZNK14QDoubleSpinBox6suffixEv(qthis: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QSpinBox)=1
pub struct QSpinBox {
  qbase: QAbstractSpinBox,
  pub qclsinst: *mut c_void,
}

// class sizeof(QDoubleSpinBox)=1
pub struct QDoubleSpinBox {
  qbase: QAbstractSpinBox,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSpinBox {
  pub fn inheritFrom(qthis: *mut c_void) -> QSpinBox {
    return QSpinBox{qbase: QAbstractSpinBox::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QSpinBox {
  type Target = QAbstractSpinBox;

  fn deref(&self) -> &QAbstractSpinBox {
    return & self.qbase;
  }
}
impl AsRef<QAbstractSpinBox> for QSpinBox {
  fn as_ref(& self) -> & QAbstractSpinBox {
    return & self.qbase;
  }
}
  // proto:  void QSpinBox::setMinimum(int min);
impl /*struct*/ QSpinBox {
  pub fn setMinimum<RetType, T: QSpinBox_setMinimum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimum(self);
    // return 1;
  }
}

pub trait QSpinBox_setMinimum<RetType> {
  fn setMinimum(self , rsthis: & QSpinBox) -> RetType;
}

  // proto:  void QSpinBox::setMinimum(int min);
impl<'a> /*trait*/ QSpinBox_setMinimum<()> for (i32) {
  fn setMinimum(self , rsthis: & QSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox10setMinimumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QSpinBox10setMinimumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QSpinBox::cleanText();
impl /*struct*/ QSpinBox {
  pub fn cleanText<RetType, T: QSpinBox_cleanText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cleanText(self);
    // return 1;
  }
}

pub trait QSpinBox_cleanText<RetType> {
  fn cleanText(self , rsthis: & QSpinBox) -> RetType;
}

  // proto:  QString QSpinBox::cleanText();
impl<'a> /*trait*/ QSpinBox_cleanText<QString> for () {
  fn cleanText(self , rsthis: & QSpinBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox9cleanTextEv()};
    let mut ret = unsafe {_ZNK8QSpinBox9cleanTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QSpinBox::value();
impl /*struct*/ QSpinBox {
  pub fn value<RetType, T: QSpinBox_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QSpinBox_value<RetType> {
  fn value(self , rsthis: & QSpinBox) -> RetType;
}

  // proto:  int QSpinBox::value();
impl<'a> /*trait*/ QSpinBox_value<i32> for () {
  fn value(self , rsthis: & QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox5valueEv()};
    let mut ret = unsafe {_ZNK8QSpinBox5valueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSpinBox::~QSpinBox();
impl /*struct*/ QSpinBox {
  pub fn Free<RetType, T: QSpinBox_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QSpinBox_Free<RetType> {
  fn Free(self , rsthis: & QSpinBox) -> RetType;
}

  // proto:  void QSpinBox::~QSpinBox();
impl<'a> /*trait*/ QSpinBox_Free<()> for () {
  fn Free(self , rsthis: & QSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBoxD0Ev()};
     unsafe {_ZN8QSpinBoxD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSpinBox::setMaximum(int max);
impl /*struct*/ QSpinBox {
  pub fn setMaximum<RetType, T: QSpinBox_setMaximum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximum(self);
    // return 1;
  }
}

pub trait QSpinBox_setMaximum<RetType> {
  fn setMaximum(self , rsthis: & QSpinBox) -> RetType;
}

  // proto:  void QSpinBox::setMaximum(int max);
impl<'a> /*trait*/ QSpinBox_setMaximum<()> for (i32) {
  fn setMaximum(self , rsthis: & QSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox10setMaximumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QSpinBox10setMaximumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSpinBox::valueChanged(const QString & );
impl /*struct*/ QSpinBox {
  pub fn valueChanged<RetType, T: QSpinBox_valueChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.valueChanged(self);
    // return 1;
  }
}

pub trait QSpinBox_valueChanged<RetType> {
  fn valueChanged(self , rsthis: & QSpinBox) -> RetType;
}

  // proto:  void QSpinBox::valueChanged(const QString & );
impl<'a> /*trait*/ QSpinBox_valueChanged<()> for (&'a QString) {
  fn valueChanged(self , rsthis: & QSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox12valueChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QSpinBox12valueChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSpinBox::setValue(int val);
impl /*struct*/ QSpinBox {
  pub fn setValue<RetType, T: QSpinBox_setValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setValue(self);
    // return 1;
  }
}

pub trait QSpinBox_setValue<RetType> {
  fn setValue(self , rsthis: & QSpinBox) -> RetType;
}

  // proto:  void QSpinBox::setValue(int val);
impl<'a> /*trait*/ QSpinBox_setValue<()> for (i32) {
  fn setValue(self , rsthis: & QSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox8setValueEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QSpinBox8setValueEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSpinBox::setDisplayIntegerBase(int base);
impl /*struct*/ QSpinBox {
  pub fn setDisplayIntegerBase<RetType, T: QSpinBox_setDisplayIntegerBase<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDisplayIntegerBase(self);
    // return 1;
  }
}

pub trait QSpinBox_setDisplayIntegerBase<RetType> {
  fn setDisplayIntegerBase(self , rsthis: & QSpinBox) -> RetType;
}

  // proto:  void QSpinBox::setDisplayIntegerBase(int base);
impl<'a> /*trait*/ QSpinBox_setDisplayIntegerBase<()> for (i32) {
  fn setDisplayIntegerBase(self , rsthis: & QSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox21setDisplayIntegerBaseEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QSpinBox21setDisplayIntegerBaseEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSpinBox::QSpinBox(QWidget * parent);
impl /*struct*/ QSpinBox {
  pub fn New<T: QSpinBox_New>(value: T) -> QSpinBox {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QSpinBox_New {
  fn New(self) -> QSpinBox;
}

  // proto:  void QSpinBox::QSpinBox(QWidget * parent);
impl<'a> /*trait*/ QSpinBox_New for (&'a QWidget) {
  fn New(self) -> QSpinBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBoxC1EP7QWidget()};
    let ctysz: c_int = unsafe{QSpinBox_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN8QSpinBoxC1EP7QWidget(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN8QSpinBoxC1EP7QWidget(arg0)};
    let rsthis = QSpinBox{/**/qbase: QAbstractSpinBox::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QSpinBox::singleStep();
impl /*struct*/ QSpinBox {
  pub fn singleStep<RetType, T: QSpinBox_singleStep<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.singleStep(self);
    // return 1;
  }
}

pub trait QSpinBox_singleStep<RetType> {
  fn singleStep(self , rsthis: & QSpinBox) -> RetType;
}

  // proto:  int QSpinBox::singleStep();
impl<'a> /*trait*/ QSpinBox_singleStep<i32> for () {
  fn singleStep(self , rsthis: & QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox10singleStepEv()};
    let mut ret = unsafe {_ZNK8QSpinBox10singleStepEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSpinBox::QSpinBox(const QSpinBox & );
impl<'a> /*trait*/ QSpinBox_New for (&'a QSpinBox) {
  fn New(self) -> QSpinBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBoxC1ERKS_()};
    let ctysz: c_int = unsafe{QSpinBox_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN8QSpinBoxC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN8QSpinBoxC1ERKS_(arg0)};
    let rsthis = QSpinBox{/**/qbase: QAbstractSpinBox::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QSpinBox::displayIntegerBase();
impl /*struct*/ QSpinBox {
  pub fn displayIntegerBase<RetType, T: QSpinBox_displayIntegerBase<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.displayIntegerBase(self);
    // return 1;
  }
}

pub trait QSpinBox_displayIntegerBase<RetType> {
  fn displayIntegerBase(self , rsthis: & QSpinBox) -> RetType;
}

  // proto:  int QSpinBox::displayIntegerBase();
impl<'a> /*trait*/ QSpinBox_displayIntegerBase<i32> for () {
  fn displayIntegerBase(self , rsthis: & QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox18displayIntegerBaseEv()};
    let mut ret = unsafe {_ZNK8QSpinBox18displayIntegerBaseEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSpinBox::setSuffix(const QString & suffix);
impl /*struct*/ QSpinBox {
  pub fn setSuffix<RetType, T: QSpinBox_setSuffix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSuffix(self);
    // return 1;
  }
}

pub trait QSpinBox_setSuffix<RetType> {
  fn setSuffix(self , rsthis: & QSpinBox) -> RetType;
}

  // proto:  void QSpinBox::setSuffix(const QString & suffix);
impl<'a> /*trait*/ QSpinBox_setSuffix<()> for (&'a QString) {
  fn setSuffix(self , rsthis: & QSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox9setSuffixERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QSpinBox9setSuffixERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QSpinBox::maximum();
impl /*struct*/ QSpinBox {
  pub fn maximum<RetType, T: QSpinBox_maximum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximum(self);
    // return 1;
  }
}

pub trait QSpinBox_maximum<RetType> {
  fn maximum(self , rsthis: & QSpinBox) -> RetType;
}

  // proto:  int QSpinBox::maximum();
impl<'a> /*trait*/ QSpinBox_maximum<i32> for () {
  fn maximum(self , rsthis: & QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox7maximumEv()};
    let mut ret = unsafe {_ZNK8QSpinBox7maximumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSpinBox::setPrefix(const QString & prefix);
impl /*struct*/ QSpinBox {
  pub fn setPrefix<RetType, T: QSpinBox_setPrefix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPrefix(self);
    // return 1;
  }
}

pub trait QSpinBox_setPrefix<RetType> {
  fn setPrefix(self , rsthis: & QSpinBox) -> RetType;
}

  // proto:  void QSpinBox::setPrefix(const QString & prefix);
impl<'a> /*trait*/ QSpinBox_setPrefix<()> for (&'a QString) {
  fn setPrefix(self , rsthis: & QSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox9setPrefixERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QSpinBox9setPrefixERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSpinBox::valueChanged(int );
impl<'a> /*trait*/ QSpinBox_valueChanged<()> for (i32) {
  fn valueChanged(self , rsthis: & QSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox12valueChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QSpinBox12valueChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QSpinBox::prefix();
impl /*struct*/ QSpinBox {
  pub fn prefix<RetType, T: QSpinBox_prefix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.prefix(self);
    // return 1;
  }
}

pub trait QSpinBox_prefix<RetType> {
  fn prefix(self , rsthis: & QSpinBox) -> RetType;
}

  // proto:  QString QSpinBox::prefix();
impl<'a> /*trait*/ QSpinBox_prefix<QString> for () {
  fn prefix(self , rsthis: & QSpinBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox6prefixEv()};
    let mut ret = unsafe {_ZNK8QSpinBox6prefixEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QSpinBox::metaObject();
impl /*struct*/ QSpinBox {
  pub fn metaObject<RetType, T: QSpinBox_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSpinBox_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSpinBox) -> RetType;
}

  // proto:  const QMetaObject * QSpinBox::metaObject();
impl<'a> /*trait*/ QSpinBox_metaObject<()> for () {
  fn metaObject(self , rsthis: & QSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox10metaObjectEv()};
     unsafe {_ZNK8QSpinBox10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QSpinBox::suffix();
impl /*struct*/ QSpinBox {
  pub fn suffix<RetType, T: QSpinBox_suffix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.suffix(self);
    // return 1;
  }
}

pub trait QSpinBox_suffix<RetType> {
  fn suffix(self , rsthis: & QSpinBox) -> RetType;
}

  // proto:  QString QSpinBox::suffix();
impl<'a> /*trait*/ QSpinBox_suffix<QString> for () {
  fn suffix(self , rsthis: & QSpinBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox6suffixEv()};
    let mut ret = unsafe {_ZNK8QSpinBox6suffixEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QSpinBox::minimum();
impl /*struct*/ QSpinBox {
  pub fn minimum<RetType, T: QSpinBox_minimum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimum(self);
    // return 1;
  }
}

pub trait QSpinBox_minimum<RetType> {
  fn minimum(self , rsthis: & QSpinBox) -> RetType;
}

  // proto:  int QSpinBox::minimum();
impl<'a> /*trait*/ QSpinBox_minimum<i32> for () {
  fn minimum(self , rsthis: & QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSpinBox7minimumEv()};
    let mut ret = unsafe {_ZNK8QSpinBox7minimumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSpinBox::setSingleStep(int val);
impl /*struct*/ QSpinBox {
  pub fn setSingleStep<RetType, T: QSpinBox_setSingleStep<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSingleStep(self);
    // return 1;
  }
}

pub trait QSpinBox_setSingleStep<RetType> {
  fn setSingleStep(self , rsthis: & QSpinBox) -> RetType;
}

  // proto:  void QSpinBox::setSingleStep(int val);
impl<'a> /*trait*/ QSpinBox_setSingleStep<()> for (i32) {
  fn setSingleStep(self , rsthis: & QSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox13setSingleStepEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QSpinBox13setSingleStepEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSpinBox::setRange(int min, int max);
impl /*struct*/ QSpinBox {
  pub fn setRange<RetType, T: QSpinBox_setRange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRange(self);
    // return 1;
  }
}

pub trait QSpinBox_setRange<RetType> {
  fn setRange(self , rsthis: & QSpinBox) -> RetType;
}

  // proto:  void QSpinBox::setRange(int min, int max);
impl<'a> /*trait*/ QSpinBox_setRange<()> for (i32, i32) {
  fn setRange(self , rsthis: & QSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSpinBox8setRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QSpinBox8setRangeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QDoubleSpinBox {
  pub fn inheritFrom(qthis: *mut c_void) -> QDoubleSpinBox {
    return QDoubleSpinBox{qbase: QAbstractSpinBox::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QDoubleSpinBox {
  type Target = QAbstractSpinBox;

  fn deref(&self) -> &QAbstractSpinBox {
    return & self.qbase;
  }
}
impl AsRef<QAbstractSpinBox> for QDoubleSpinBox {
  fn as_ref(& self) -> & QAbstractSpinBox {
    return & self.qbase;
  }
}
  // proto:  void QDoubleSpinBox::valueChanged(const QString & );
impl /*struct*/ QDoubleSpinBox {
  pub fn valueChanged<RetType, T: QDoubleSpinBox_valueChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.valueChanged(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_valueChanged<RetType> {
  fn valueChanged(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  void QDoubleSpinBox::valueChanged(const QString & );
impl<'a> /*trait*/ QDoubleSpinBox_valueChanged<()> for (&'a QString) {
  fn valueChanged(self , rsthis: & QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox12valueChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QDoubleSpinBox12valueChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QDoubleSpinBox::textFromValue(double val);
impl /*struct*/ QDoubleSpinBox {
  pub fn textFromValue<RetType, T: QDoubleSpinBox_textFromValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textFromValue(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_textFromValue<RetType> {
  fn textFromValue(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  QString QDoubleSpinBox::textFromValue(double val);
impl<'a> /*trait*/ QDoubleSpinBox_textFromValue<QString> for (f64) {
  fn textFromValue(self , rsthis: & QDoubleSpinBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox13textFromValueEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK14QDoubleSpinBox13textFromValueEd(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDoubleSpinBox::setSingleStep(double val);
impl /*struct*/ QDoubleSpinBox {
  pub fn setSingleStep<RetType, T: QDoubleSpinBox_setSingleStep<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSingleStep(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_setSingleStep<RetType> {
  fn setSingleStep(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  void QDoubleSpinBox::setSingleStep(double val);
impl<'a> /*trait*/ QDoubleSpinBox_setSingleStep<()> for (f64) {
  fn setSingleStep(self , rsthis: & QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox13setSingleStepEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN14QDoubleSpinBox13setSingleStepEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  double QDoubleSpinBox::minimum();
impl /*struct*/ QDoubleSpinBox {
  pub fn minimum<RetType, T: QDoubleSpinBox_minimum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimum(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_minimum<RetType> {
  fn minimum(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  double QDoubleSpinBox::minimum();
impl<'a> /*trait*/ QDoubleSpinBox_minimum<f64> for () {
  fn minimum(self , rsthis: & QDoubleSpinBox) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox7minimumEv()};
    let mut ret = unsafe {_ZNK14QDoubleSpinBox7minimumEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  double QDoubleSpinBox::valueFromText(const QString & text);
impl /*struct*/ QDoubleSpinBox {
  pub fn valueFromText<RetType, T: QDoubleSpinBox_valueFromText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.valueFromText(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_valueFromText<RetType> {
  fn valueFromText(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  double QDoubleSpinBox::valueFromText(const QString & text);
impl<'a> /*trait*/ QDoubleSpinBox_valueFromText<f64> for (&'a QString) {
  fn valueFromText(self , rsthis: & QDoubleSpinBox) -> f64 {
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
  fn valueChanged(self , rsthis: & QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox12valueChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN14QDoubleSpinBox12valueChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QDoubleSpinBox::metaObject();
impl /*struct*/ QDoubleSpinBox {
  pub fn metaObject<RetType, T: QDoubleSpinBox_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_metaObject<RetType> {
  fn metaObject(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  const QMetaObject * QDoubleSpinBox::metaObject();
impl<'a> /*trait*/ QDoubleSpinBox_metaObject<()> for () {
  fn metaObject(self , rsthis: & QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox10metaObjectEv()};
     unsafe {_ZNK14QDoubleSpinBox10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDoubleSpinBox::setValue(double val);
impl /*struct*/ QDoubleSpinBox {
  pub fn setValue<RetType, T: QDoubleSpinBox_setValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setValue(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_setValue<RetType> {
  fn setValue(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  void QDoubleSpinBox::setValue(double val);
impl<'a> /*trait*/ QDoubleSpinBox_setValue<()> for (f64) {
  fn setValue(self , rsthis: & QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox8setValueEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN14QDoubleSpinBox8setValueEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDoubleSpinBox::setSuffix(const QString & suffix);
impl /*struct*/ QDoubleSpinBox {
  pub fn setSuffix<RetType, T: QDoubleSpinBox_setSuffix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSuffix(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_setSuffix<RetType> {
  fn setSuffix(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  void QDoubleSpinBox::setSuffix(const QString & suffix);
impl<'a> /*trait*/ QDoubleSpinBox_setSuffix<()> for (&'a QString) {
  fn setSuffix(self , rsthis: & QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox9setSuffixERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QDoubleSpinBox9setSuffixERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QDoubleSpinBox::decimals();
impl /*struct*/ QDoubleSpinBox {
  pub fn decimals<RetType, T: QDoubleSpinBox_decimals<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.decimals(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_decimals<RetType> {
  fn decimals(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  int QDoubleSpinBox::decimals();
impl<'a> /*trait*/ QDoubleSpinBox_decimals<i32> for () {
  fn decimals(self , rsthis: & QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox8decimalsEv()};
    let mut ret = unsafe {_ZNK14QDoubleSpinBox8decimalsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString QDoubleSpinBox::prefix();
impl /*struct*/ QDoubleSpinBox {
  pub fn prefix<RetType, T: QDoubleSpinBox_prefix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.prefix(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_prefix<RetType> {
  fn prefix(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  QString QDoubleSpinBox::prefix();
impl<'a> /*trait*/ QDoubleSpinBox_prefix<QString> for () {
  fn prefix(self , rsthis: & QDoubleSpinBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox6prefixEv()};
    let mut ret = unsafe {_ZNK14QDoubleSpinBox6prefixEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  double QDoubleSpinBox::singleStep();
impl /*struct*/ QDoubleSpinBox {
  pub fn singleStep<RetType, T: QDoubleSpinBox_singleStep<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.singleStep(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_singleStep<RetType> {
  fn singleStep(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  double QDoubleSpinBox::singleStep();
impl<'a> /*trait*/ QDoubleSpinBox_singleStep<f64> for () {
  fn singleStep(self , rsthis: & QDoubleSpinBox) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox10singleStepEv()};
    let mut ret = unsafe {_ZNK14QDoubleSpinBox10singleStepEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QDoubleSpinBox::~QDoubleSpinBox();
impl /*struct*/ QDoubleSpinBox {
  pub fn Free<RetType, T: QDoubleSpinBox_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_Free<RetType> {
  fn Free(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  void QDoubleSpinBox::~QDoubleSpinBox();
impl<'a> /*trait*/ QDoubleSpinBox_Free<()> for () {
  fn Free(self , rsthis: & QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBoxD0Ev()};
     unsafe {_ZN14QDoubleSpinBoxD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDoubleSpinBox::fixup(QString & str);
impl /*struct*/ QDoubleSpinBox {
  pub fn fixup<RetType, T: QDoubleSpinBox_fixup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fixup(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_fixup<RetType> {
  fn fixup(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  void QDoubleSpinBox::fixup(QString & str);
impl<'a> /*trait*/ QDoubleSpinBox_fixup<()> for (&'a QString) {
  fn fixup(self , rsthis: & QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox5fixupER7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK14QDoubleSpinBox5fixupER7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDoubleSpinBox::QDoubleSpinBox(const QDoubleSpinBox & );
impl /*struct*/ QDoubleSpinBox {
  pub fn New<T: QDoubleSpinBox_New>(value: T) -> QDoubleSpinBox {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QDoubleSpinBox_New {
  fn New(self) -> QDoubleSpinBox;
}

  // proto:  void QDoubleSpinBox::QDoubleSpinBox(const QDoubleSpinBox & );
impl<'a> /*trait*/ QDoubleSpinBox_New for (&'a QDoubleSpinBox) {
  fn New(self) -> QDoubleSpinBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBoxC1ERKS_()};
    let ctysz: c_int = unsafe{QDoubleSpinBox_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN14QDoubleSpinBoxC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN14QDoubleSpinBoxC1ERKS_(arg0)};
    let rsthis = QDoubleSpinBox{/**/qbase: QAbstractSpinBox::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDoubleSpinBox::setPrefix(const QString & prefix);
impl /*struct*/ QDoubleSpinBox {
  pub fn setPrefix<RetType, T: QDoubleSpinBox_setPrefix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPrefix(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_setPrefix<RetType> {
  fn setPrefix(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  void QDoubleSpinBox::setPrefix(const QString & prefix);
impl<'a> /*trait*/ QDoubleSpinBox_setPrefix<()> for (&'a QString) {
  fn setPrefix(self , rsthis: & QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox9setPrefixERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QDoubleSpinBox9setPrefixERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QDoubleSpinBox::cleanText();
impl /*struct*/ QDoubleSpinBox {
  pub fn cleanText<RetType, T: QDoubleSpinBox_cleanText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cleanText(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_cleanText<RetType> {
  fn cleanText(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  QString QDoubleSpinBox::cleanText();
impl<'a> /*trait*/ QDoubleSpinBox_cleanText<QString> for () {
  fn cleanText(self , rsthis: & QDoubleSpinBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox9cleanTextEv()};
    let mut ret = unsafe {_ZNK14QDoubleSpinBox9cleanTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDoubleSpinBox::setMinimum(double min);
impl /*struct*/ QDoubleSpinBox {
  pub fn setMinimum<RetType, T: QDoubleSpinBox_setMinimum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimum(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_setMinimum<RetType> {
  fn setMinimum(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  void QDoubleSpinBox::setMinimum(double min);
impl<'a> /*trait*/ QDoubleSpinBox_setMinimum<()> for (f64) {
  fn setMinimum(self , rsthis: & QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox10setMinimumEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN14QDoubleSpinBox10setMinimumEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDoubleSpinBox::setMaximum(double max);
impl /*struct*/ QDoubleSpinBox {
  pub fn setMaximum<RetType, T: QDoubleSpinBox_setMaximum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximum(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_setMaximum<RetType> {
  fn setMaximum(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  void QDoubleSpinBox::setMaximum(double max);
impl<'a> /*trait*/ QDoubleSpinBox_setMaximum<()> for (f64) {
  fn setMaximum(self , rsthis: & QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox10setMaximumEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN14QDoubleSpinBox10setMaximumEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDoubleSpinBox::setDecimals(int prec);
impl /*struct*/ QDoubleSpinBox {
  pub fn setDecimals<RetType, T: QDoubleSpinBox_setDecimals<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDecimals(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_setDecimals<RetType> {
  fn setDecimals(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  void QDoubleSpinBox::setDecimals(int prec);
impl<'a> /*trait*/ QDoubleSpinBox_setDecimals<()> for (i32) {
  fn setDecimals(self , rsthis: & QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox11setDecimalsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QDoubleSpinBox11setDecimalsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  double QDoubleSpinBox::value();
impl /*struct*/ QDoubleSpinBox {
  pub fn value<RetType, T: QDoubleSpinBox_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_value<RetType> {
  fn value(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  double QDoubleSpinBox::value();
impl<'a> /*trait*/ QDoubleSpinBox_value<f64> for () {
  fn value(self , rsthis: & QDoubleSpinBox) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox5valueEv()};
    let mut ret = unsafe {_ZNK14QDoubleSpinBox5valueEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QDoubleSpinBox::setRange(double min, double max);
impl /*struct*/ QDoubleSpinBox {
  pub fn setRange<RetType, T: QDoubleSpinBox_setRange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRange(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_setRange<RetType> {
  fn setRange(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  void QDoubleSpinBox::setRange(double min, double max);
impl<'a> /*trait*/ QDoubleSpinBox_setRange<()> for (f64, f64) {
  fn setRange(self , rsthis: & QDoubleSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBox8setRangeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN14QDoubleSpinBox8setRangeEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QDoubleSpinBox::QDoubleSpinBox(QWidget * parent);
impl<'a> /*trait*/ QDoubleSpinBox_New for (&'a QWidget) {
  fn New(self) -> QDoubleSpinBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDoubleSpinBoxC1EP7QWidget()};
    let ctysz: c_int = unsafe{QDoubleSpinBox_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN14QDoubleSpinBoxC1EP7QWidget(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN14QDoubleSpinBoxC1EP7QWidget(arg0)};
    let rsthis = QDoubleSpinBox{/**/qbase: QAbstractSpinBox::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  double QDoubleSpinBox::maximum();
impl /*struct*/ QDoubleSpinBox {
  pub fn maximum<RetType, T: QDoubleSpinBox_maximum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximum(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_maximum<RetType> {
  fn maximum(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  double QDoubleSpinBox::maximum();
impl<'a> /*trait*/ QDoubleSpinBox_maximum<f64> for () {
  fn maximum(self , rsthis: & QDoubleSpinBox) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox7maximumEv()};
    let mut ret = unsafe {_ZNK14QDoubleSpinBox7maximumEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QString QDoubleSpinBox::suffix();
impl /*struct*/ QDoubleSpinBox {
  pub fn suffix<RetType, T: QDoubleSpinBox_suffix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.suffix(self);
    // return 1;
  }
}

pub trait QDoubleSpinBox_suffix<RetType> {
  fn suffix(self , rsthis: & QDoubleSpinBox) -> RetType;
}

  // proto:  QString QDoubleSpinBox::suffix();
impl<'a> /*trait*/ QDoubleSpinBox_suffix<QString> for () {
  fn suffix(self , rsthis: & QDoubleSpinBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDoubleSpinBox6suffixEv()};
    let mut ret = unsafe {_ZNK14QDoubleSpinBox6suffixEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

