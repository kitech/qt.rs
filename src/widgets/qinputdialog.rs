// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qinputdialog.h
// dst-file: /src/widgets/qinputdialog.rs
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
use super::qdialog::*; // 773
use std::ops::Deref;
use super::super::core::qobjectdefs::*; // 771
use super::qwidget::*; // 773
use super::super::core::qstring::*; // 771
use super::super::core::qstringlist::*; // 771
use super::super::core::qobject::*; // 771
use super::super::core::qsize::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QInputDialog_Class_Size() -> c_int;
  // proto:  double QInputDialog::doubleMaximum();
  fn C_ZNK12QInputDialog13doubleMaximumEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QInputDialog::setIntMaximum(int max);
  fn C_ZN12QInputDialog13setIntMaximumEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  const QMetaObject * QInputDialog::metaObject();
  fn C_ZNK12QInputDialog10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QInputDialog::setIntStep(int step);
  fn C_ZN12QInputDialog10setIntStepEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QInputDialog::intMaximum();
  fn C_ZNK12QInputDialog10intMaximumEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QInputDialog::intStep();
  fn C_ZNK12QInputDialog7intStepEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QInputDialog::doubleDecimals();
  fn C_ZNK12QInputDialog14doubleDecimalsEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QInputDialog::setDoubleDecimals(int decimals);
  fn C_ZN12QInputDialog17setDoubleDecimalsEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QInputDialog::setIntMinimum(int min);
  fn C_ZN12QInputDialog13setIntMinimumEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QInputDialog::setTextValue(const QString & text);
  fn C_ZN12QInputDialog12setTextValueERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QInputDialog::done(int result);
  fn C_ZN12QInputDialog4doneEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QInputDialog::~QInputDialog();
  fn C_ZN12QInputDialogD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QInputDialog::setLabelText(const QString & text);
  fn C_ZN12QInputDialog12setLabelTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QInputDialog::labelText();
  fn C_ZNK12QInputDialog9labelTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QInputDialog::setOkButtonText(const QString & text);
  fn C_ZN12QInputDialog15setOkButtonTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QStringList QInputDialog::comboBoxItems();
  fn C_ZNK12QInputDialog13comboBoxItemsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QInputDialog::intMinimum();
  fn C_ZNK12QInputDialog10intMinimumEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QInputDialog::setComboBoxEditable(bool editable);
  fn C_ZN12QInputDialog19setComboBoxEditableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QInputDialog::setVisible(bool visible);
  fn C_ZN12QInputDialog10setVisibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QInputDialog::setDoubleMinimum(double min);
  fn C_ZN12QInputDialog16setDoubleMinimumEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  double QInputDialog::doubleMinimum();
  fn C_ZNK12QInputDialog13doubleMinimumEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QString QInputDialog::cancelButtonText();
  fn C_ZNK12QInputDialog16cancelButtonTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QInputDialog::setComboBoxItems(const QStringList & items);
  fn C_ZN12QInputDialog16setComboBoxItemsERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QInputDialog::isComboBoxEditable();
  fn C_ZNK12QInputDialog18isComboBoxEditableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QInputDialog::open(QObject * receiver, const char * member);
  fn C_ZN12QInputDialog4openEP7QObjectPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char);
  // proto:  QString QInputDialog::okButtonText();
  fn C_ZNK12QInputDialog12okButtonTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QInputDialog::sizeHint();
  fn C_ZNK12QInputDialog8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QInputDialog::setIntRange(int min, int max);
  fn C_ZN12QInputDialog11setIntRangeEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  QSize QInputDialog::minimumSizeHint();
  fn C_ZNK12QInputDialog15minimumSizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QInputDialog::setDoubleValue(double value);
  fn C_ZN12QInputDialog14setDoubleValueEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QInputDialog::setIntValue(int value);
  fn C_ZN12QInputDialog11setIntValueEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  double QInputDialog::doubleValue();
  fn C_ZNK12QInputDialog11doubleValueEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QInputDialog::setCancelButtonText(const QString & text);
  fn C_ZN12QInputDialog19setCancelButtonTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QInputDialog::textValue();
  fn C_ZNK12QInputDialog9textValueEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QInputDialog::setDoubleMaximum(double max);
  fn C_ZN12QInputDialog16setDoubleMaximumEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  int QInputDialog::intValue();
  fn C_ZNK12QInputDialog8intValueEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QInputDialog::setDoubleRange(double min, double max);
  fn C_ZN12QInputDialog14setDoubleRangeEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  fn QInputDialog_SlotProxy_connect__ZN12QInputDialog15intValueChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QInputDialog_SlotProxy_connect__ZN12QInputDialog17textValueSelectedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QInputDialog_SlotProxy_connect__ZN12QInputDialog16intValueSelectedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QInputDialog_SlotProxy_connect__ZN12QInputDialog19doubleValueSelectedEd(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QInputDialog_SlotProxy_connect__ZN12QInputDialog18doubleValueChangedEd(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QInputDialog_SlotProxy_connect__ZN12QInputDialog16textValueChangedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QInputDialog)=1
#[derive(Default)]
pub struct QInputDialog {
  qbase: QDialog,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _doubleValueChanged: QInputDialog_doubleValueChanged_signal,
  pub _intValueChanged: QInputDialog_intValueChanged_signal,
  pub _textValueChanged: QInputDialog_textValueChanged_signal,
  pub _intValueSelected: QInputDialog_intValueSelected_signal,
  pub _textValueSelected: QInputDialog_textValueSelected_signal,
  pub _doubleValueSelected: QInputDialog_doubleValueSelected_signal,
}

impl /*struct*/ QInputDialog {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QInputDialog {
    return QInputDialog{qbase: QDialog::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QInputDialog {
  type Target = QDialog;

  fn deref(&self) -> &QDialog {
    return & self.qbase;
  }
}
impl AsRef<QDialog> for QInputDialog {
  fn as_ref(& self) -> & QDialog {
    return & self.qbase;
  }
}
  // proto:  double QInputDialog::doubleMaximum();
impl /*struct*/ QInputDialog {
  pub fn doubleMaximum<RetType, T: QInputDialog_doubleMaximum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.doubleMaximum(self);
    // return 1;
  }
}

pub trait QInputDialog_doubleMaximum<RetType> {
  fn doubleMaximum(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  double QInputDialog::doubleMaximum();
impl<'a> /*trait*/ QInputDialog_doubleMaximum<f64> for () {
  fn doubleMaximum(self , rsthis: & QInputDialog) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog13doubleMaximumEv()};
    let mut ret = unsafe {C_ZNK12QInputDialog13doubleMaximumEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QInputDialog::setIntMaximum(int max);
impl /*struct*/ QInputDialog {
  pub fn setIntMaximum<RetType, T: QInputDialog_setIntMaximum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIntMaximum(self);
    // return 1;
  }
}

pub trait QInputDialog_setIntMaximum<RetType> {
  fn setIntMaximum(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setIntMaximum(int max);
impl<'a> /*trait*/ QInputDialog_setIntMaximum<()> for (i32) {
  fn setIntMaximum(self , rsthis: & QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog13setIntMaximumEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QInputDialog13setIntMaximumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QInputDialog::metaObject();
impl /*struct*/ QInputDialog {
  pub fn metaObject<RetType, T: QInputDialog_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QInputDialog_metaObject<RetType> {
  fn metaObject(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  const QMetaObject * QInputDialog::metaObject();
impl<'a> /*trait*/ QInputDialog_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QInputDialog) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog10metaObjectEv()};
    let mut ret = unsafe {C_ZNK12QInputDialog10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QInputDialog::setIntStep(int step);
impl /*struct*/ QInputDialog {
  pub fn setIntStep<RetType, T: QInputDialog_setIntStep<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIntStep(self);
    // return 1;
  }
}

pub trait QInputDialog_setIntStep<RetType> {
  fn setIntStep(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setIntStep(int step);
impl<'a> /*trait*/ QInputDialog_setIntStep<()> for (i32) {
  fn setIntStep(self , rsthis: & QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog10setIntStepEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QInputDialog10setIntStepEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QInputDialog::intMaximum();
impl /*struct*/ QInputDialog {
  pub fn intMaximum<RetType, T: QInputDialog_intMaximum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.intMaximum(self);
    // return 1;
  }
}

pub trait QInputDialog_intMaximum<RetType> {
  fn intMaximum(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  int QInputDialog::intMaximum();
impl<'a> /*trait*/ QInputDialog_intMaximum<i32> for () {
  fn intMaximum(self , rsthis: & QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog10intMaximumEv()};
    let mut ret = unsafe {C_ZNK12QInputDialog10intMaximumEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QInputDialog::intStep();
impl /*struct*/ QInputDialog {
  pub fn intStep<RetType, T: QInputDialog_intStep<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.intStep(self);
    // return 1;
  }
}

pub trait QInputDialog_intStep<RetType> {
  fn intStep(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  int QInputDialog::intStep();
impl<'a> /*trait*/ QInputDialog_intStep<i32> for () {
  fn intStep(self , rsthis: & QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog7intStepEv()};
    let mut ret = unsafe {C_ZNK12QInputDialog7intStepEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QInputDialog::doubleDecimals();
impl /*struct*/ QInputDialog {
  pub fn doubleDecimals<RetType, T: QInputDialog_doubleDecimals<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.doubleDecimals(self);
    // return 1;
  }
}

pub trait QInputDialog_doubleDecimals<RetType> {
  fn doubleDecimals(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  int QInputDialog::doubleDecimals();
impl<'a> /*trait*/ QInputDialog_doubleDecimals<i32> for () {
  fn doubleDecimals(self , rsthis: & QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog14doubleDecimalsEv()};
    let mut ret = unsafe {C_ZNK12QInputDialog14doubleDecimalsEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QInputDialog::setDoubleDecimals(int decimals);
impl /*struct*/ QInputDialog {
  pub fn setDoubleDecimals<RetType, T: QInputDialog_setDoubleDecimals<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDoubleDecimals(self);
    // return 1;
  }
}

pub trait QInputDialog_setDoubleDecimals<RetType> {
  fn setDoubleDecimals(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setDoubleDecimals(int decimals);
impl<'a> /*trait*/ QInputDialog_setDoubleDecimals<()> for (i32) {
  fn setDoubleDecimals(self , rsthis: & QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog17setDoubleDecimalsEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QInputDialog17setDoubleDecimalsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputDialog::setIntMinimum(int min);
impl /*struct*/ QInputDialog {
  pub fn setIntMinimum<RetType, T: QInputDialog_setIntMinimum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIntMinimum(self);
    // return 1;
  }
}

pub trait QInputDialog_setIntMinimum<RetType> {
  fn setIntMinimum(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setIntMinimum(int min);
impl<'a> /*trait*/ QInputDialog_setIntMinimum<()> for (i32) {
  fn setIntMinimum(self , rsthis: & QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog13setIntMinimumEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QInputDialog13setIntMinimumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputDialog::setTextValue(const QString & text);
impl /*struct*/ QInputDialog {
  pub fn setTextValue<RetType, T: QInputDialog_setTextValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextValue(self);
    // return 1;
  }
}

pub trait QInputDialog_setTextValue<RetType> {
  fn setTextValue(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setTextValue(const QString & text);
impl<'a> /*trait*/ QInputDialog_setTextValue<()> for (&'a QString) {
  fn setTextValue(self , rsthis: & QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog12setTextValueERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QInputDialog12setTextValueERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputDialog::done(int result);
impl /*struct*/ QInputDialog {
  pub fn done<RetType, T: QInputDialog_done<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.done(self);
    // return 1;
  }
}

pub trait QInputDialog_done<RetType> {
  fn done(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::done(int result);
impl<'a> /*trait*/ QInputDialog_done<()> for (i32) {
  fn done(self , rsthis: & QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog4doneEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QInputDialog4doneEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputDialog::~QInputDialog();
impl /*struct*/ QInputDialog {
  pub fn free<RetType, T: QInputDialog_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QInputDialog_free<RetType> {
  fn free(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::~QInputDialog();
impl<'a> /*trait*/ QInputDialog_free<()> for () {
  fn free(self , rsthis: & QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialogD2Ev()};
     unsafe {C_ZN12QInputDialogD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QInputDialog::setLabelText(const QString & text);
impl /*struct*/ QInputDialog {
  pub fn setLabelText<RetType, T: QInputDialog_setLabelText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLabelText(self);
    // return 1;
  }
}

pub trait QInputDialog_setLabelText<RetType> {
  fn setLabelText(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setLabelText(const QString & text);
impl<'a> /*trait*/ QInputDialog_setLabelText<()> for (&'a QString) {
  fn setLabelText(self , rsthis: & QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog12setLabelTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QInputDialog12setLabelTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QInputDialog::labelText();
impl /*struct*/ QInputDialog {
  pub fn labelText<RetType, T: QInputDialog_labelText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.labelText(self);
    // return 1;
  }
}

pub trait QInputDialog_labelText<RetType> {
  fn labelText(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  QString QInputDialog::labelText();
impl<'a> /*trait*/ QInputDialog_labelText<QString> for () {
  fn labelText(self , rsthis: & QInputDialog) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog9labelTextEv()};
    let mut ret = unsafe {C_ZNK12QInputDialog9labelTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QInputDialog::setOkButtonText(const QString & text);
impl /*struct*/ QInputDialog {
  pub fn setOkButtonText<RetType, T: QInputDialog_setOkButtonText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOkButtonText(self);
    // return 1;
  }
}

pub trait QInputDialog_setOkButtonText<RetType> {
  fn setOkButtonText(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setOkButtonText(const QString & text);
impl<'a> /*trait*/ QInputDialog_setOkButtonText<()> for (&'a QString) {
  fn setOkButtonText(self , rsthis: & QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog15setOkButtonTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QInputDialog15setOkButtonTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringList QInputDialog::comboBoxItems();
impl /*struct*/ QInputDialog {
  pub fn comboBoxItems<RetType, T: QInputDialog_comboBoxItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.comboBoxItems(self);
    // return 1;
  }
}

pub trait QInputDialog_comboBoxItems<RetType> {
  fn comboBoxItems(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  QStringList QInputDialog::comboBoxItems();
impl<'a> /*trait*/ QInputDialog_comboBoxItems<QStringList> for () {
  fn comboBoxItems(self , rsthis: & QInputDialog) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog13comboBoxItemsEv()};
    let mut ret = unsafe {C_ZNK12QInputDialog13comboBoxItemsEv(rsthis.qclsinst)};
    let mut ret1 = QStringList::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QInputDialog::intMinimum();
impl /*struct*/ QInputDialog {
  pub fn intMinimum<RetType, T: QInputDialog_intMinimum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.intMinimum(self);
    // return 1;
  }
}

pub trait QInputDialog_intMinimum<RetType> {
  fn intMinimum(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  int QInputDialog::intMinimum();
impl<'a> /*trait*/ QInputDialog_intMinimum<i32> for () {
  fn intMinimum(self , rsthis: & QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog10intMinimumEv()};
    let mut ret = unsafe {C_ZNK12QInputDialog10intMinimumEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QInputDialog::setComboBoxEditable(bool editable);
impl /*struct*/ QInputDialog {
  pub fn setComboBoxEditable<RetType, T: QInputDialog_setComboBoxEditable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setComboBoxEditable(self);
    // return 1;
  }
}

pub trait QInputDialog_setComboBoxEditable<RetType> {
  fn setComboBoxEditable(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setComboBoxEditable(bool editable);
impl<'a> /*trait*/ QInputDialog_setComboBoxEditable<()> for (i8) {
  fn setComboBoxEditable(self , rsthis: & QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog19setComboBoxEditableEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN12QInputDialog19setComboBoxEditableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputDialog::setVisible(bool visible);
impl /*struct*/ QInputDialog {
  pub fn setVisible<RetType, T: QInputDialog_setVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QInputDialog_setVisible<RetType> {
  fn setVisible(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setVisible(bool visible);
impl<'a> /*trait*/ QInputDialog_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: & QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN12QInputDialog10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputDialog::setDoubleMinimum(double min);
impl /*struct*/ QInputDialog {
  pub fn setDoubleMinimum<RetType, T: QInputDialog_setDoubleMinimum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDoubleMinimum(self);
    // return 1;
  }
}

pub trait QInputDialog_setDoubleMinimum<RetType> {
  fn setDoubleMinimum(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setDoubleMinimum(double min);
impl<'a> /*trait*/ QInputDialog_setDoubleMinimum<()> for (f64) {
  fn setDoubleMinimum(self , rsthis: & QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog16setDoubleMinimumEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN12QInputDialog16setDoubleMinimumEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  double QInputDialog::doubleMinimum();
impl /*struct*/ QInputDialog {
  pub fn doubleMinimum<RetType, T: QInputDialog_doubleMinimum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.doubleMinimum(self);
    // return 1;
  }
}

pub trait QInputDialog_doubleMinimum<RetType> {
  fn doubleMinimum(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  double QInputDialog::doubleMinimum();
impl<'a> /*trait*/ QInputDialog_doubleMinimum<f64> for () {
  fn doubleMinimum(self , rsthis: & QInputDialog) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog13doubleMinimumEv()};
    let mut ret = unsafe {C_ZNK12QInputDialog13doubleMinimumEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  QString QInputDialog::cancelButtonText();
impl /*struct*/ QInputDialog {
  pub fn cancelButtonText<RetType, T: QInputDialog_cancelButtonText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cancelButtonText(self);
    // return 1;
  }
}

pub trait QInputDialog_cancelButtonText<RetType> {
  fn cancelButtonText(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  QString QInputDialog::cancelButtonText();
impl<'a> /*trait*/ QInputDialog_cancelButtonText<QString> for () {
  fn cancelButtonText(self , rsthis: & QInputDialog) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog16cancelButtonTextEv()};
    let mut ret = unsafe {C_ZNK12QInputDialog16cancelButtonTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QInputDialog::setComboBoxItems(const QStringList & items);
impl /*struct*/ QInputDialog {
  pub fn setComboBoxItems<RetType, T: QInputDialog_setComboBoxItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setComboBoxItems(self);
    // return 1;
  }
}

pub trait QInputDialog_setComboBoxItems<RetType> {
  fn setComboBoxItems(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setComboBoxItems(const QStringList & items);
impl<'a> /*trait*/ QInputDialog_setComboBoxItems<()> for (&'a QStringList) {
  fn setComboBoxItems(self , rsthis: & QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog16setComboBoxItemsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QInputDialog16setComboBoxItemsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QInputDialog::isComboBoxEditable();
impl /*struct*/ QInputDialog {
  pub fn isComboBoxEditable<RetType, T: QInputDialog_isComboBoxEditable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isComboBoxEditable(self);
    // return 1;
  }
}

pub trait QInputDialog_isComboBoxEditable<RetType> {
  fn isComboBoxEditable(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  bool QInputDialog::isComboBoxEditable();
impl<'a> /*trait*/ QInputDialog_isComboBoxEditable<i8> for () {
  fn isComboBoxEditable(self , rsthis: & QInputDialog) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog18isComboBoxEditableEv()};
    let mut ret = unsafe {C_ZNK12QInputDialog18isComboBoxEditableEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QInputDialog::open(QObject * receiver, const char * member);
impl /*struct*/ QInputDialog {
  pub fn open<RetType, T: QInputDialog_open<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.open(self);
    // return 1;
  }
}

pub trait QInputDialog_open<RetType> {
  fn open(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::open(QObject * receiver, const char * member);
impl<'a> /*trait*/ QInputDialog_open<()> for (&'a QObject, &'a  String) {
  fn open(self , rsthis: & QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog4openEP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {C_ZN12QInputDialog4openEP7QObjectPKc(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QString QInputDialog::okButtonText();
impl /*struct*/ QInputDialog {
  pub fn okButtonText<RetType, T: QInputDialog_okButtonText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.okButtonText(self);
    // return 1;
  }
}

pub trait QInputDialog_okButtonText<RetType> {
  fn okButtonText(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  QString QInputDialog::okButtonText();
impl<'a> /*trait*/ QInputDialog_okButtonText<QString> for () {
  fn okButtonText(self , rsthis: & QInputDialog) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog12okButtonTextEv()};
    let mut ret = unsafe {C_ZNK12QInputDialog12okButtonTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QInputDialog::sizeHint();
impl /*struct*/ QInputDialog {
  pub fn sizeHint<RetType, T: QInputDialog_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QInputDialog_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  QSize QInputDialog::sizeHint();
impl<'a> /*trait*/ QInputDialog_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QInputDialog) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog8sizeHintEv()};
    let mut ret = unsafe {C_ZNK12QInputDialog8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QInputDialog::setIntRange(int min, int max);
impl /*struct*/ QInputDialog {
  pub fn setIntRange<RetType, T: QInputDialog_setIntRange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIntRange(self);
    // return 1;
  }
}

pub trait QInputDialog_setIntRange<RetType> {
  fn setIntRange(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setIntRange(int min, int max);
impl<'a> /*trait*/ QInputDialog_setIntRange<()> for (i32, i32) {
  fn setIntRange(self , rsthis: & QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog11setIntRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN12QInputDialog11setIntRangeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QSize QInputDialog::minimumSizeHint();
impl /*struct*/ QInputDialog {
  pub fn minimumSizeHint<RetType, T: QInputDialog_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QInputDialog_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  QSize QInputDialog::minimumSizeHint();
impl<'a> /*trait*/ QInputDialog_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QInputDialog) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog15minimumSizeHintEv()};
    let mut ret = unsafe {C_ZNK12QInputDialog15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QInputDialog::setDoubleValue(double value);
impl /*struct*/ QInputDialog {
  pub fn setDoubleValue<RetType, T: QInputDialog_setDoubleValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDoubleValue(self);
    // return 1;
  }
}

pub trait QInputDialog_setDoubleValue<RetType> {
  fn setDoubleValue(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setDoubleValue(double value);
impl<'a> /*trait*/ QInputDialog_setDoubleValue<()> for (f64) {
  fn setDoubleValue(self , rsthis: & QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog14setDoubleValueEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN12QInputDialog14setDoubleValueEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputDialog::setIntValue(int value);
impl /*struct*/ QInputDialog {
  pub fn setIntValue<RetType, T: QInputDialog_setIntValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIntValue(self);
    // return 1;
  }
}

pub trait QInputDialog_setIntValue<RetType> {
  fn setIntValue(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setIntValue(int value);
impl<'a> /*trait*/ QInputDialog_setIntValue<()> for (i32) {
  fn setIntValue(self , rsthis: & QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog11setIntValueEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QInputDialog11setIntValueEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  double QInputDialog::doubleValue();
impl /*struct*/ QInputDialog {
  pub fn doubleValue<RetType, T: QInputDialog_doubleValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.doubleValue(self);
    // return 1;
  }
}

pub trait QInputDialog_doubleValue<RetType> {
  fn doubleValue(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  double QInputDialog::doubleValue();
impl<'a> /*trait*/ QInputDialog_doubleValue<f64> for () {
  fn doubleValue(self , rsthis: & QInputDialog) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog11doubleValueEv()};
    let mut ret = unsafe {C_ZNK12QInputDialog11doubleValueEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QInputDialog::setCancelButtonText(const QString & text);
impl /*struct*/ QInputDialog {
  pub fn setCancelButtonText<RetType, T: QInputDialog_setCancelButtonText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCancelButtonText(self);
    // return 1;
  }
}

pub trait QInputDialog_setCancelButtonText<RetType> {
  fn setCancelButtonText(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setCancelButtonText(const QString & text);
impl<'a> /*trait*/ QInputDialog_setCancelButtonText<()> for (&'a QString) {
  fn setCancelButtonText(self , rsthis: & QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog19setCancelButtonTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QInputDialog19setCancelButtonTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QInputDialog::textValue();
impl /*struct*/ QInputDialog {
  pub fn textValue<RetType, T: QInputDialog_textValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textValue(self);
    // return 1;
  }
}

pub trait QInputDialog_textValue<RetType> {
  fn textValue(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  QString QInputDialog::textValue();
impl<'a> /*trait*/ QInputDialog_textValue<QString> for () {
  fn textValue(self , rsthis: & QInputDialog) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog9textValueEv()};
    let mut ret = unsafe {C_ZNK12QInputDialog9textValueEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QInputDialog::setDoubleMaximum(double max);
impl /*struct*/ QInputDialog {
  pub fn setDoubleMaximum<RetType, T: QInputDialog_setDoubleMaximum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDoubleMaximum(self);
    // return 1;
  }
}

pub trait QInputDialog_setDoubleMaximum<RetType> {
  fn setDoubleMaximum(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setDoubleMaximum(double max);
impl<'a> /*trait*/ QInputDialog_setDoubleMaximum<()> for (f64) {
  fn setDoubleMaximum(self , rsthis: & QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog16setDoubleMaximumEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN12QInputDialog16setDoubleMaximumEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QInputDialog::intValue();
impl /*struct*/ QInputDialog {
  pub fn intValue<RetType, T: QInputDialog_intValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.intValue(self);
    // return 1;
  }
}

pub trait QInputDialog_intValue<RetType> {
  fn intValue(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  int QInputDialog::intValue();
impl<'a> /*trait*/ QInputDialog_intValue<i32> for () {
  fn intValue(self , rsthis: & QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog8intValueEv()};
    let mut ret = unsafe {C_ZNK12QInputDialog8intValueEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QInputDialog::setDoubleRange(double min, double max);
impl /*struct*/ QInputDialog {
  pub fn setDoubleRange<RetType, T: QInputDialog_setDoubleRange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDoubleRange(self);
    // return 1;
  }
}

pub trait QInputDialog_setDoubleRange<RetType> {
  fn setDoubleRange(self , rsthis: & QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setDoubleRange(double min, double max);
impl<'a> /*trait*/ QInputDialog_setDoubleRange<()> for (f64, f64) {
  fn setDoubleRange(self , rsthis: & QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog14setDoubleRangeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {C_ZN12QInputDialog14setDoubleRangeEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

#[derive(Default)] // for QInputDialog_doubleValueChanged
pub struct QInputDialog_doubleValueChanged_signal{poi:u64}
impl /* struct */ QInputDialog {
  pub fn doubleValueChanged(&self) -> QInputDialog_doubleValueChanged_signal {
     return QInputDialog_doubleValueChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QInputDialog_doubleValueChanged_signal {
  pub fn connect<T: QInputDialog_doubleValueChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QInputDialog_doubleValueChanged_signal_connect {
  fn connect(self, sigthis: QInputDialog_doubleValueChanged_signal);
}

#[derive(Default)] // for QInputDialog_intValueChanged
pub struct QInputDialog_intValueChanged_signal{poi:u64}
impl /* struct */ QInputDialog {
  pub fn intValueChanged(&self) -> QInputDialog_intValueChanged_signal {
     return QInputDialog_intValueChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QInputDialog_intValueChanged_signal {
  pub fn connect<T: QInputDialog_intValueChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QInputDialog_intValueChanged_signal_connect {
  fn connect(self, sigthis: QInputDialog_intValueChanged_signal);
}

#[derive(Default)] // for QInputDialog_textValueChanged
pub struct QInputDialog_textValueChanged_signal{poi:u64}
impl /* struct */ QInputDialog {
  pub fn textValueChanged(&self) -> QInputDialog_textValueChanged_signal {
     return QInputDialog_textValueChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QInputDialog_textValueChanged_signal {
  pub fn connect<T: QInputDialog_textValueChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QInputDialog_textValueChanged_signal_connect {
  fn connect(self, sigthis: QInputDialog_textValueChanged_signal);
}

#[derive(Default)] // for QInputDialog_intValueSelected
pub struct QInputDialog_intValueSelected_signal{poi:u64}
impl /* struct */ QInputDialog {
  pub fn intValueSelected(&self) -> QInputDialog_intValueSelected_signal {
     return QInputDialog_intValueSelected_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QInputDialog_intValueSelected_signal {
  pub fn connect<T: QInputDialog_intValueSelected_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QInputDialog_intValueSelected_signal_connect {
  fn connect(self, sigthis: QInputDialog_intValueSelected_signal);
}

#[derive(Default)] // for QInputDialog_textValueSelected
pub struct QInputDialog_textValueSelected_signal{poi:u64}
impl /* struct */ QInputDialog {
  pub fn textValueSelected(&self) -> QInputDialog_textValueSelected_signal {
     return QInputDialog_textValueSelected_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QInputDialog_textValueSelected_signal {
  pub fn connect<T: QInputDialog_textValueSelected_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QInputDialog_textValueSelected_signal_connect {
  fn connect(self, sigthis: QInputDialog_textValueSelected_signal);
}

#[derive(Default)] // for QInputDialog_doubleValueSelected
pub struct QInputDialog_doubleValueSelected_signal{poi:u64}
impl /* struct */ QInputDialog {
  pub fn doubleValueSelected(&self) -> QInputDialog_doubleValueSelected_signal {
     return QInputDialog_doubleValueSelected_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QInputDialog_doubleValueSelected_signal {
  pub fn connect<T: QInputDialog_doubleValueSelected_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QInputDialog_doubleValueSelected_signal_connect {
  fn connect(self, sigthis: QInputDialog_doubleValueSelected_signal);
}

// intValueChanged(int)
extern fn QInputDialog_intValueChanged_signal_connect_cb_0(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QInputDialog_intValueChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QInputDialog_intValueChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QInputDialog_intValueChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputDialog_intValueChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QInputDialog_SlotProxy_connect__ZN12QInputDialog15intValueChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QInputDialog_intValueChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QInputDialog_intValueChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputDialog_intValueChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QInputDialog_SlotProxy_connect__ZN12QInputDialog15intValueChangedEi(arg0, arg1, arg2)};
  }
}
// textValueSelected(const class QString &)
extern fn QInputDialog_textValueSelected_signal_connect_cb_1(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QInputDialog_textValueSelected_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(QString)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QInputDialog_textValueSelected_signal_connect for fn(QString) {
  fn connect(self, sigthis: QInputDialog_textValueSelected_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputDialog_textValueSelected_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QInputDialog_SlotProxy_connect__ZN12QInputDialog17textValueSelectedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QInputDialog_textValueSelected_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QInputDialog_textValueSelected_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputDialog_textValueSelected_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QInputDialog_SlotProxy_connect__ZN12QInputDialog17textValueSelectedERK7QString(arg0, arg1, arg2)};
  }
}
// intValueSelected(int)
extern fn QInputDialog_intValueSelected_signal_connect_cb_2(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QInputDialog_intValueSelected_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QInputDialog_intValueSelected_signal_connect for fn(i32) {
  fn connect(self, sigthis: QInputDialog_intValueSelected_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputDialog_intValueSelected_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QInputDialog_SlotProxy_connect__ZN12QInputDialog16intValueSelectedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QInputDialog_intValueSelected_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QInputDialog_intValueSelected_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputDialog_intValueSelected_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QInputDialog_SlotProxy_connect__ZN12QInputDialog16intValueSelectedEi(arg0, arg1, arg2)};
  }
}
// doubleValueSelected(double)
extern fn QInputDialog_doubleValueSelected_signal_connect_cb_3(rsfptr:fn(f64), arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as f64;
  rsfptr(rsarg0);
}
extern fn QInputDialog_doubleValueSelected_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(f64)>, arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as f64;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QInputDialog_doubleValueSelected_signal_connect for fn(f64) {
  fn connect(self, sigthis: QInputDialog_doubleValueSelected_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputDialog_doubleValueSelected_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QInputDialog_SlotProxy_connect__ZN12QInputDialog19doubleValueSelectedEd(arg0, arg1, arg2)};
  }
}
impl /* trait */ QInputDialog_doubleValueSelected_signal_connect for Box<Fn(f64)> {
  fn connect(self, sigthis: QInputDialog_doubleValueSelected_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputDialog_doubleValueSelected_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QInputDialog_SlotProxy_connect__ZN12QInputDialog19doubleValueSelectedEd(arg0, arg1, arg2)};
  }
}
// doubleValueChanged(double)
extern fn QInputDialog_doubleValueChanged_signal_connect_cb_4(rsfptr:fn(f64), arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as f64;
  rsfptr(rsarg0);
}
extern fn QInputDialog_doubleValueChanged_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn(f64)>, arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as f64;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QInputDialog_doubleValueChanged_signal_connect for fn(f64) {
  fn connect(self, sigthis: QInputDialog_doubleValueChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputDialog_doubleValueChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QInputDialog_SlotProxy_connect__ZN12QInputDialog18doubleValueChangedEd(arg0, arg1, arg2)};
  }
}
impl /* trait */ QInputDialog_doubleValueChanged_signal_connect for Box<Fn(f64)> {
  fn connect(self, sigthis: QInputDialog_doubleValueChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputDialog_doubleValueChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QInputDialog_SlotProxy_connect__ZN12QInputDialog18doubleValueChangedEd(arg0, arg1, arg2)};
  }
}
// textValueChanged(const class QString &)
extern fn QInputDialog_textValueChanged_signal_connect_cb_5(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QInputDialog_textValueChanged_signal_connect_cb_box_5(rsfptr_raw:*mut Box<Fn(QString)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QInputDialog_textValueChanged_signal_connect for fn(QString) {
  fn connect(self, sigthis: QInputDialog_textValueChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputDialog_textValueChanged_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QInputDialog_SlotProxy_connect__ZN12QInputDialog16textValueChangedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QInputDialog_textValueChanged_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QInputDialog_textValueChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QInputDialog_textValueChanged_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QInputDialog_SlotProxy_connect__ZN12QInputDialog16textValueChangedERK7QString(arg0, arg1, arg2)};
  }
}
// <= body block end

