// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
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
use super::qdialog::QDialog; // 773
use std::ops::Deref;
use super::qwidget::QWidget; // 773
use super::super::core::qstring::QString; // 771
use super::super::core::qstringlist::QStringList; // 771
use super::super::core::qobject::QObject; // 771
use super::super::core::qsize::QSize; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  double QInputDialog::doubleMaximum();
  fn _ZNK12QInputDialog13doubleMaximumEv(qthis: *mut c_void) -> c_double;
  // proto:  void QInputDialog::setIntMaximum(int max);
  fn _ZN12QInputDialog13setIntMaximumEi(qthis: *mut c_void, arg0: c_int);
  // proto:  const QMetaObject * QInputDialog::metaObject();
  fn _ZNK12QInputDialog10metaObjectEv(qthis: *mut c_void);
  // proto:  void QInputDialog::setIntStep(int step);
  fn _ZN12QInputDialog10setIntStepEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QInputDialog::intMaximum();
  fn _ZNK12QInputDialog10intMaximumEv(qthis: *mut c_void) -> c_int;
  // proto:  int QInputDialog::intStep();
  fn _ZNK12QInputDialog7intStepEv(qthis: *mut c_void) -> c_int;
  // proto:  int QInputDialog::doubleDecimals();
  fn _ZNK12QInputDialog14doubleDecimalsEv(qthis: *mut c_void) -> c_int;
  // proto:  void QInputDialog::setDoubleDecimals(int decimals);
  fn _ZN12QInputDialog17setDoubleDecimalsEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QInputDialog::intValueChanged(int value);
  fn _ZN12QInputDialog15intValueChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QInputDialog::setIntMinimum(int min);
  fn _ZN12QInputDialog13setIntMinimumEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QInputDialog::setTextValue(const QString & text);
  fn _ZN12QInputDialog12setTextValueERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QInputDialog::done(int result);
  fn _ZN12QInputDialog4doneEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QInputDialog::~QInputDialog();
  fn _ZN12QInputDialogD0Ev(qthis: *mut c_void);
  // proto:  void QInputDialog::QInputDialog(const QInputDialog & );
  fn _ZN12QInputDialogC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QInputDialog::textValueSelected(const QString & text);
  fn _ZN12QInputDialog17textValueSelectedERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QInputDialog::setLabelText(const QString & text);
  fn _ZN12QInputDialog12setLabelTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QInputDialog::labelText();
  fn _ZNK12QInputDialog9labelTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputDialog::setOkButtonText(const QString & text);
  fn _ZN12QInputDialog15setOkButtonTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringList QInputDialog::comboBoxItems();
  fn _ZNK12QInputDialog13comboBoxItemsEv(qthis: *mut c_void);
  // proto:  int QInputDialog::intMinimum();
  fn _ZNK12QInputDialog10intMinimumEv(qthis: *mut c_void) -> c_int;
  // proto:  void QInputDialog::setComboBoxEditable(bool editable);
  fn _ZN12QInputDialog19setComboBoxEditableEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QInputDialog::setVisible(bool visible);
  fn _ZN12QInputDialog10setVisibleEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QInputDialog::setDoubleMinimum(double min);
  fn _ZN12QInputDialog16setDoubleMinimumEd(qthis: *mut c_void, arg0: c_double);
  // proto:  double QInputDialog::doubleMinimum();
  fn _ZNK12QInputDialog13doubleMinimumEv(qthis: *mut c_void) -> c_double;
  // proto:  QString QInputDialog::cancelButtonText();
  fn _ZNK12QInputDialog16cancelButtonTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputDialog::setComboBoxItems(const QStringList & items);
  fn _ZN12QInputDialog16setComboBoxItemsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QInputDialog::isComboBoxEditable();
  fn _ZNK12QInputDialog18isComboBoxEditableEv(qthis: *mut c_void) -> c_char;
  // proto:  void QInputDialog::open(QObject * receiver, const char * member);
  fn _ZN12QInputDialog4openEP7QObjectPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char);
  // proto:  void QInputDialog::doubleValueChanged(double value);
  fn _ZN12QInputDialog18doubleValueChangedEd(qthis: *mut c_void, arg0: c_double);
  // proto:  QString QInputDialog::okButtonText();
  fn _ZNK12QInputDialog12okButtonTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QInputDialog::sizeHint();
  fn _ZNK12QInputDialog8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputDialog::setIntRange(int min, int max);
  fn _ZN12QInputDialog11setIntRangeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  QSize QInputDialog::minimumSizeHint();
  fn _ZNK12QInputDialog15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputDialog::setDoubleValue(double value);
  fn _ZN12QInputDialog14setDoubleValueEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QInputDialog::setIntValue(int value);
  fn _ZN12QInputDialog11setIntValueEi(qthis: *mut c_void, arg0: c_int);
  // proto:  double QInputDialog::doubleValue();
  fn _ZNK12QInputDialog11doubleValueEv(qthis: *mut c_void) -> c_double;
  // proto:  void QInputDialog::setCancelButtonText(const QString & text);
  fn _ZN12QInputDialog19setCancelButtonTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QInputDialog::doubleValueSelected(double value);
  fn _ZN12QInputDialog19doubleValueSelectedEd(qthis: *mut c_void, arg0: c_double);
  // proto:  QString QInputDialog::textValue();
  fn _ZNK12QInputDialog9textValueEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputDialog::textValueChanged(const QString & text);
  fn _ZN12QInputDialog16textValueChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QInputDialog::intValueSelected(int value);
  fn _ZN12QInputDialog16intValueSelectedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QInputDialog::setDoubleMaximum(double max);
  fn _ZN12QInputDialog16setDoubleMaximumEd(qthis: *mut c_void, arg0: c_double);
  // proto:  int QInputDialog::intValue();
  fn _ZNK12QInputDialog8intValueEv(qthis: *mut c_void) -> c_int;
  // proto:  void QInputDialog::setDoubleRange(double min, double max);
  fn _ZN12QInputDialog14setDoubleRangeEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
} // <= ext block end

// body block begin =>
// class sizeof(QInputDialog)=1
pub struct QInputDialog {
  qbase: QDialog,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QInputDialog {
  pub fn inheritFrom(qthis: *mut c_void) -> QInputDialog {
    return QInputDialog{qbase: QDialog::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QInputDialog {
  type Target = QDialog;

  fn deref(&self) -> &QDialog {
    return &self.qbase;
  }
}
impl AsRef<QDialog> for QInputDialog {
  fn as_ref(&self) -> &QDialog {
    return &self.qbase;
  }
}
  // proto:  double QInputDialog::doubleMaximum();
impl /*struct*/ QInputDialog {
  pub fn doubleMaximum<RetType, T: QInputDialog_doubleMaximum<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.doubleMaximum(self);
    // return 1;
  }
}

pub trait QInputDialog_doubleMaximum<RetType> {
  fn doubleMaximum(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  double QInputDialog::doubleMaximum();
impl<'a> /*trait*/ QInputDialog_doubleMaximum<f64> for () {
  fn doubleMaximum(self , rsthis: &mut QInputDialog) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog13doubleMaximumEv()};
    let mut ret = unsafe {_ZNK12QInputDialog13doubleMaximumEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QInputDialog::setIntMaximum(int max);
impl /*struct*/ QInputDialog {
  pub fn setIntMaximum<RetType, T: QInputDialog_setIntMaximum<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setIntMaximum(self);
    // return 1;
  }
}

pub trait QInputDialog_setIntMaximum<RetType> {
  fn setIntMaximum(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setIntMaximum(int max);
impl<'a> /*trait*/ QInputDialog_setIntMaximum<()> for (i32) {
  fn setIntMaximum(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog13setIntMaximumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QInputDialog13setIntMaximumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QInputDialog::metaObject();
impl /*struct*/ QInputDialog {
  pub fn metaObject<RetType, T: QInputDialog_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QInputDialog_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  const QMetaObject * QInputDialog::metaObject();
impl<'a> /*trait*/ QInputDialog_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog10metaObjectEv()};
     unsafe {_ZNK12QInputDialog10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QInputDialog::setIntStep(int step);
impl /*struct*/ QInputDialog {
  pub fn setIntStep<RetType, T: QInputDialog_setIntStep<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setIntStep(self);
    // return 1;
  }
}

pub trait QInputDialog_setIntStep<RetType> {
  fn setIntStep(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setIntStep(int step);
impl<'a> /*trait*/ QInputDialog_setIntStep<()> for (i32) {
  fn setIntStep(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog10setIntStepEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QInputDialog10setIntStepEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QInputDialog::intMaximum();
impl /*struct*/ QInputDialog {
  pub fn intMaximum<RetType, T: QInputDialog_intMaximum<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intMaximum(self);
    // return 1;
  }
}

pub trait QInputDialog_intMaximum<RetType> {
  fn intMaximum(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  int QInputDialog::intMaximum();
impl<'a> /*trait*/ QInputDialog_intMaximum<i32> for () {
  fn intMaximum(self , rsthis: &mut QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog10intMaximumEv()};
    let mut ret = unsafe {_ZNK12QInputDialog10intMaximumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QInputDialog::intStep();
impl /*struct*/ QInputDialog {
  pub fn intStep<RetType, T: QInputDialog_intStep<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intStep(self);
    // return 1;
  }
}

pub trait QInputDialog_intStep<RetType> {
  fn intStep(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  int QInputDialog::intStep();
impl<'a> /*trait*/ QInputDialog_intStep<i32> for () {
  fn intStep(self , rsthis: &mut QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog7intStepEv()};
    let mut ret = unsafe {_ZNK12QInputDialog7intStepEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QInputDialog::doubleDecimals();
impl /*struct*/ QInputDialog {
  pub fn doubleDecimals<RetType, T: QInputDialog_doubleDecimals<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.doubleDecimals(self);
    // return 1;
  }
}

pub trait QInputDialog_doubleDecimals<RetType> {
  fn doubleDecimals(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  int QInputDialog::doubleDecimals();
impl<'a> /*trait*/ QInputDialog_doubleDecimals<i32> for () {
  fn doubleDecimals(self , rsthis: &mut QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog14doubleDecimalsEv()};
    let mut ret = unsafe {_ZNK12QInputDialog14doubleDecimalsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QInputDialog::setDoubleDecimals(int decimals);
impl /*struct*/ QInputDialog {
  pub fn setDoubleDecimals<RetType, T: QInputDialog_setDoubleDecimals<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDoubleDecimals(self);
    // return 1;
  }
}

pub trait QInputDialog_setDoubleDecimals<RetType> {
  fn setDoubleDecimals(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setDoubleDecimals(int decimals);
impl<'a> /*trait*/ QInputDialog_setDoubleDecimals<()> for (i32) {
  fn setDoubleDecimals(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog17setDoubleDecimalsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QInputDialog17setDoubleDecimalsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputDialog::intValueChanged(int value);
impl /*struct*/ QInputDialog {
  pub fn intValueChanged<RetType, T: QInputDialog_intValueChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intValueChanged(self);
    // return 1;
  }
}

pub trait QInputDialog_intValueChanged<RetType> {
  fn intValueChanged(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::intValueChanged(int value);
impl<'a> /*trait*/ QInputDialog_intValueChanged<()> for (i32) {
  fn intValueChanged(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog15intValueChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QInputDialog15intValueChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputDialog::setIntMinimum(int min);
impl /*struct*/ QInputDialog {
  pub fn setIntMinimum<RetType, T: QInputDialog_setIntMinimum<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setIntMinimum(self);
    // return 1;
  }
}

pub trait QInputDialog_setIntMinimum<RetType> {
  fn setIntMinimum(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setIntMinimum(int min);
impl<'a> /*trait*/ QInputDialog_setIntMinimum<()> for (i32) {
  fn setIntMinimum(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog13setIntMinimumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QInputDialog13setIntMinimumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputDialog::setTextValue(const QString & text);
impl /*struct*/ QInputDialog {
  pub fn setTextValue<RetType, T: QInputDialog_setTextValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTextValue(self);
    // return 1;
  }
}

pub trait QInputDialog_setTextValue<RetType> {
  fn setTextValue(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setTextValue(const QString & text);
impl<'a> /*trait*/ QInputDialog_setTextValue<()> for (QString) {
  fn setTextValue(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog12setTextValueERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QInputDialog12setTextValueERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputDialog::done(int result);
impl /*struct*/ QInputDialog {
  pub fn done<RetType, T: QInputDialog_done<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.done(self);
    // return 1;
  }
}

pub trait QInputDialog_done<RetType> {
  fn done(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::done(int result);
impl<'a> /*trait*/ QInputDialog_done<()> for (i32) {
  fn done(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog4doneEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QInputDialog4doneEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputDialog::~QInputDialog();
impl /*struct*/ QInputDialog {
  pub fn FreeQInputDialog<RetType, T: QInputDialog_FreeQInputDialog<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQInputDialog(self);
    // return 1;
  }
}

pub trait QInputDialog_FreeQInputDialog<RetType> {
  fn FreeQInputDialog(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::~QInputDialog();
impl<'a> /*trait*/ QInputDialog_FreeQInputDialog<()> for () {
  fn FreeQInputDialog(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialogD0Ev()};
     unsafe {_ZN12QInputDialogD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QInputDialog::QInputDialog(const QInputDialog & );
impl /*struct*/ QInputDialog {
  pub fn NewQInputDialog<T: QInputDialog_NewQInputDialog>(value: T) -> QInputDialog {
    let rsthis = value.NewQInputDialog();
    return rsthis;
    // return 1;
  }
}

pub trait QInputDialog_NewQInputDialog {
  fn NewQInputDialog(self) -> QInputDialog;
}

  // proto:  void QInputDialog::QInputDialog(const QInputDialog & );
impl<'a> /*trait*/ QInputDialog_NewQInputDialog for (QInputDialog) {
  fn NewQInputDialog(self) -> QInputDialog {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialogC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QInputDialogC1ERKS_(qthis, arg0)};
    let rsthis = QInputDialog{/**/qbase: QDialog::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QInputDialog::textValueSelected(const QString & text);
impl /*struct*/ QInputDialog {
  pub fn textValueSelected<RetType, T: QInputDialog_textValueSelected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.textValueSelected(self);
    // return 1;
  }
}

pub trait QInputDialog_textValueSelected<RetType> {
  fn textValueSelected(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::textValueSelected(const QString & text);
impl<'a> /*trait*/ QInputDialog_textValueSelected<()> for (QString) {
  fn textValueSelected(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog17textValueSelectedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QInputDialog17textValueSelectedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputDialog::setLabelText(const QString & text);
impl /*struct*/ QInputDialog {
  pub fn setLabelText<RetType, T: QInputDialog_setLabelText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setLabelText(self);
    // return 1;
  }
}

pub trait QInputDialog_setLabelText<RetType> {
  fn setLabelText(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setLabelText(const QString & text);
impl<'a> /*trait*/ QInputDialog_setLabelText<()> for (QString) {
  fn setLabelText(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog12setLabelTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QInputDialog12setLabelTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QInputDialog::labelText();
impl /*struct*/ QInputDialog {
  pub fn labelText<RetType, T: QInputDialog_labelText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.labelText(self);
    // return 1;
  }
}

pub trait QInputDialog_labelText<RetType> {
  fn labelText(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  QString QInputDialog::labelText();
impl<'a> /*trait*/ QInputDialog_labelText<QString> for () {
  fn labelText(self , rsthis: &mut QInputDialog) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog9labelTextEv()};
    let mut ret = unsafe {_ZNK12QInputDialog9labelTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QInputDialog::setOkButtonText(const QString & text);
impl /*struct*/ QInputDialog {
  pub fn setOkButtonText<RetType, T: QInputDialog_setOkButtonText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setOkButtonText(self);
    // return 1;
  }
}

pub trait QInputDialog_setOkButtonText<RetType> {
  fn setOkButtonText(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setOkButtonText(const QString & text);
impl<'a> /*trait*/ QInputDialog_setOkButtonText<()> for (QString) {
  fn setOkButtonText(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog15setOkButtonTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QInputDialog15setOkButtonTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringList QInputDialog::comboBoxItems();
impl /*struct*/ QInputDialog {
  pub fn comboBoxItems<RetType, T: QInputDialog_comboBoxItems<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.comboBoxItems(self);
    // return 1;
  }
}

pub trait QInputDialog_comboBoxItems<RetType> {
  fn comboBoxItems(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  QStringList QInputDialog::comboBoxItems();
impl<'a> /*trait*/ QInputDialog_comboBoxItems<()> for () {
  fn comboBoxItems(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog13comboBoxItemsEv()};
     unsafe {_ZNK12QInputDialog13comboBoxItemsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QInputDialog::intMinimum();
impl /*struct*/ QInputDialog {
  pub fn intMinimum<RetType, T: QInputDialog_intMinimum<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intMinimum(self);
    // return 1;
  }
}

pub trait QInputDialog_intMinimum<RetType> {
  fn intMinimum(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  int QInputDialog::intMinimum();
impl<'a> /*trait*/ QInputDialog_intMinimum<i32> for () {
  fn intMinimum(self , rsthis: &mut QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog10intMinimumEv()};
    let mut ret = unsafe {_ZNK12QInputDialog10intMinimumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QInputDialog::setComboBoxEditable(bool editable);
impl /*struct*/ QInputDialog {
  pub fn setComboBoxEditable<RetType, T: QInputDialog_setComboBoxEditable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setComboBoxEditable(self);
    // return 1;
  }
}

pub trait QInputDialog_setComboBoxEditable<RetType> {
  fn setComboBoxEditable(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setComboBoxEditable(bool editable);
impl<'a> /*trait*/ QInputDialog_setComboBoxEditable<()> for (i8) {
  fn setComboBoxEditable(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog19setComboBoxEditableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QInputDialog19setComboBoxEditableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputDialog::setVisible(bool visible);
impl /*struct*/ QInputDialog {
  pub fn setVisible<RetType, T: QInputDialog_setVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QInputDialog_setVisible<RetType> {
  fn setVisible(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setVisible(bool visible);
impl<'a> /*trait*/ QInputDialog_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QInputDialog10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputDialog::setDoubleMinimum(double min);
impl /*struct*/ QInputDialog {
  pub fn setDoubleMinimum<RetType, T: QInputDialog_setDoubleMinimum<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDoubleMinimum(self);
    // return 1;
  }
}

pub trait QInputDialog_setDoubleMinimum<RetType> {
  fn setDoubleMinimum(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setDoubleMinimum(double min);
impl<'a> /*trait*/ QInputDialog_setDoubleMinimum<()> for (f64) {
  fn setDoubleMinimum(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog16setDoubleMinimumEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN12QInputDialog16setDoubleMinimumEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  double QInputDialog::doubleMinimum();
impl /*struct*/ QInputDialog {
  pub fn doubleMinimum<RetType, T: QInputDialog_doubleMinimum<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.doubleMinimum(self);
    // return 1;
  }
}

pub trait QInputDialog_doubleMinimum<RetType> {
  fn doubleMinimum(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  double QInputDialog::doubleMinimum();
impl<'a> /*trait*/ QInputDialog_doubleMinimum<f64> for () {
  fn doubleMinimum(self , rsthis: &mut QInputDialog) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog13doubleMinimumEv()};
    let mut ret = unsafe {_ZNK12QInputDialog13doubleMinimumEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QString QInputDialog::cancelButtonText();
impl /*struct*/ QInputDialog {
  pub fn cancelButtonText<RetType, T: QInputDialog_cancelButtonText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cancelButtonText(self);
    // return 1;
  }
}

pub trait QInputDialog_cancelButtonText<RetType> {
  fn cancelButtonText(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  QString QInputDialog::cancelButtonText();
impl<'a> /*trait*/ QInputDialog_cancelButtonText<QString> for () {
  fn cancelButtonText(self , rsthis: &mut QInputDialog) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog16cancelButtonTextEv()};
    let mut ret = unsafe {_ZNK12QInputDialog16cancelButtonTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QInputDialog::setComboBoxItems(const QStringList & items);
impl /*struct*/ QInputDialog {
  pub fn setComboBoxItems<RetType, T: QInputDialog_setComboBoxItems<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setComboBoxItems(self);
    // return 1;
  }
}

pub trait QInputDialog_setComboBoxItems<RetType> {
  fn setComboBoxItems(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setComboBoxItems(const QStringList & items);
impl<'a> /*trait*/ QInputDialog_setComboBoxItems<()> for (QStringList) {
  fn setComboBoxItems(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog16setComboBoxItemsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QInputDialog16setComboBoxItemsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QInputDialog::isComboBoxEditable();
impl /*struct*/ QInputDialog {
  pub fn isComboBoxEditable<RetType, T: QInputDialog_isComboBoxEditable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isComboBoxEditable(self);
    // return 1;
  }
}

pub trait QInputDialog_isComboBoxEditable<RetType> {
  fn isComboBoxEditable(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  bool QInputDialog::isComboBoxEditable();
impl<'a> /*trait*/ QInputDialog_isComboBoxEditable<i8> for () {
  fn isComboBoxEditable(self , rsthis: &mut QInputDialog) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog18isComboBoxEditableEv()};
    let mut ret = unsafe {_ZNK12QInputDialog18isComboBoxEditableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QInputDialog::open(QObject * receiver, const char * member);
impl /*struct*/ QInputDialog {
  pub fn open<RetType, T: QInputDialog_open<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.open(self);
    // return 1;
  }
}

pub trait QInputDialog_open<RetType> {
  fn open(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::open(QObject * receiver, const char * member);
impl<'a> /*trait*/ QInputDialog_open<()> for (QObject, &'a  String) {
  fn open(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog4openEP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {_ZN12QInputDialog4openEP7QObjectPKc(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QInputDialog::doubleValueChanged(double value);
impl /*struct*/ QInputDialog {
  pub fn doubleValueChanged<RetType, T: QInputDialog_doubleValueChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.doubleValueChanged(self);
    // return 1;
  }
}

pub trait QInputDialog_doubleValueChanged<RetType> {
  fn doubleValueChanged(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::doubleValueChanged(double value);
impl<'a> /*trait*/ QInputDialog_doubleValueChanged<()> for (f64) {
  fn doubleValueChanged(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog18doubleValueChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN12QInputDialog18doubleValueChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QInputDialog::okButtonText();
impl /*struct*/ QInputDialog {
  pub fn okButtonText<RetType, T: QInputDialog_okButtonText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.okButtonText(self);
    // return 1;
  }
}

pub trait QInputDialog_okButtonText<RetType> {
  fn okButtonText(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  QString QInputDialog::okButtonText();
impl<'a> /*trait*/ QInputDialog_okButtonText<QString> for () {
  fn okButtonText(self , rsthis: &mut QInputDialog) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog12okButtonTextEv()};
    let mut ret = unsafe {_ZNK12QInputDialog12okButtonTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QInputDialog::sizeHint();
impl /*struct*/ QInputDialog {
  pub fn sizeHint<RetType, T: QInputDialog_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QInputDialog_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  QSize QInputDialog::sizeHint();
impl<'a> /*trait*/ QInputDialog_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QInputDialog) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog8sizeHintEv()};
    let mut ret = unsafe {_ZNK12QInputDialog8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QInputDialog::setIntRange(int min, int max);
impl /*struct*/ QInputDialog {
  pub fn setIntRange<RetType, T: QInputDialog_setIntRange<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setIntRange(self);
    // return 1;
  }
}

pub trait QInputDialog_setIntRange<RetType> {
  fn setIntRange(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setIntRange(int min, int max);
impl<'a> /*trait*/ QInputDialog_setIntRange<()> for (i32, i32) {
  fn setIntRange(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog11setIntRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QInputDialog11setIntRangeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QSize QInputDialog::minimumSizeHint();
impl /*struct*/ QInputDialog {
  pub fn minimumSizeHint<RetType, T: QInputDialog_minimumSizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QInputDialog_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  QSize QInputDialog::minimumSizeHint();
impl<'a> /*trait*/ QInputDialog_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: &mut QInputDialog) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK12QInputDialog15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QInputDialog::setDoubleValue(double value);
impl /*struct*/ QInputDialog {
  pub fn setDoubleValue<RetType, T: QInputDialog_setDoubleValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDoubleValue(self);
    // return 1;
  }
}

pub trait QInputDialog_setDoubleValue<RetType> {
  fn setDoubleValue(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setDoubleValue(double value);
impl<'a> /*trait*/ QInputDialog_setDoubleValue<()> for (f64) {
  fn setDoubleValue(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog14setDoubleValueEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN12QInputDialog14setDoubleValueEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputDialog::setIntValue(int value);
impl /*struct*/ QInputDialog {
  pub fn setIntValue<RetType, T: QInputDialog_setIntValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setIntValue(self);
    // return 1;
  }
}

pub trait QInputDialog_setIntValue<RetType> {
  fn setIntValue(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setIntValue(int value);
impl<'a> /*trait*/ QInputDialog_setIntValue<()> for (i32) {
  fn setIntValue(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog11setIntValueEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QInputDialog11setIntValueEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  double QInputDialog::doubleValue();
impl /*struct*/ QInputDialog {
  pub fn doubleValue<RetType, T: QInputDialog_doubleValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.doubleValue(self);
    // return 1;
  }
}

pub trait QInputDialog_doubleValue<RetType> {
  fn doubleValue(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  double QInputDialog::doubleValue();
impl<'a> /*trait*/ QInputDialog_doubleValue<f64> for () {
  fn doubleValue(self , rsthis: &mut QInputDialog) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog11doubleValueEv()};
    let mut ret = unsafe {_ZNK12QInputDialog11doubleValueEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QInputDialog::setCancelButtonText(const QString & text);
impl /*struct*/ QInputDialog {
  pub fn setCancelButtonText<RetType, T: QInputDialog_setCancelButtonText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCancelButtonText(self);
    // return 1;
  }
}

pub trait QInputDialog_setCancelButtonText<RetType> {
  fn setCancelButtonText(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setCancelButtonText(const QString & text);
impl<'a> /*trait*/ QInputDialog_setCancelButtonText<()> for (QString) {
  fn setCancelButtonText(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog19setCancelButtonTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QInputDialog19setCancelButtonTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputDialog::doubleValueSelected(double value);
impl /*struct*/ QInputDialog {
  pub fn doubleValueSelected<RetType, T: QInputDialog_doubleValueSelected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.doubleValueSelected(self);
    // return 1;
  }
}

pub trait QInputDialog_doubleValueSelected<RetType> {
  fn doubleValueSelected(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::doubleValueSelected(double value);
impl<'a> /*trait*/ QInputDialog_doubleValueSelected<()> for (f64) {
  fn doubleValueSelected(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog19doubleValueSelectedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN12QInputDialog19doubleValueSelectedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QInputDialog::textValue();
impl /*struct*/ QInputDialog {
  pub fn textValue<RetType, T: QInputDialog_textValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.textValue(self);
    // return 1;
  }
}

pub trait QInputDialog_textValue<RetType> {
  fn textValue(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  QString QInputDialog::textValue();
impl<'a> /*trait*/ QInputDialog_textValue<QString> for () {
  fn textValue(self , rsthis: &mut QInputDialog) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog9textValueEv()};
    let mut ret = unsafe {_ZNK12QInputDialog9textValueEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QInputDialog::textValueChanged(const QString & text);
impl /*struct*/ QInputDialog {
  pub fn textValueChanged<RetType, T: QInputDialog_textValueChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.textValueChanged(self);
    // return 1;
  }
}

pub trait QInputDialog_textValueChanged<RetType> {
  fn textValueChanged(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::textValueChanged(const QString & text);
impl<'a> /*trait*/ QInputDialog_textValueChanged<()> for (QString) {
  fn textValueChanged(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog16textValueChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QInputDialog16textValueChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputDialog::intValueSelected(int value);
impl /*struct*/ QInputDialog {
  pub fn intValueSelected<RetType, T: QInputDialog_intValueSelected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intValueSelected(self);
    // return 1;
  }
}

pub trait QInputDialog_intValueSelected<RetType> {
  fn intValueSelected(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::intValueSelected(int value);
impl<'a> /*trait*/ QInputDialog_intValueSelected<()> for (i32) {
  fn intValueSelected(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog16intValueSelectedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QInputDialog16intValueSelectedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QInputDialog::setDoubleMaximum(double max);
impl /*struct*/ QInputDialog {
  pub fn setDoubleMaximum<RetType, T: QInputDialog_setDoubleMaximum<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDoubleMaximum(self);
    // return 1;
  }
}

pub trait QInputDialog_setDoubleMaximum<RetType> {
  fn setDoubleMaximum(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setDoubleMaximum(double max);
impl<'a> /*trait*/ QInputDialog_setDoubleMaximum<()> for (f64) {
  fn setDoubleMaximum(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog16setDoubleMaximumEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN12QInputDialog16setDoubleMaximumEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QInputDialog::intValue();
impl /*struct*/ QInputDialog {
  pub fn intValue<RetType, T: QInputDialog_intValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intValue(self);
    // return 1;
  }
}

pub trait QInputDialog_intValue<RetType> {
  fn intValue(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  int QInputDialog::intValue();
impl<'a> /*trait*/ QInputDialog_intValue<i32> for () {
  fn intValue(self , rsthis: &mut QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog8intValueEv()};
    let mut ret = unsafe {_ZNK12QInputDialog8intValueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QInputDialog::setDoubleRange(double min, double max);
impl /*struct*/ QInputDialog {
  pub fn setDoubleRange<RetType, T: QInputDialog_setDoubleRange<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDoubleRange(self);
    // return 1;
  }
}

pub trait QInputDialog_setDoubleRange<RetType> {
  fn setDoubleRange(self , rsthis: &mut QInputDialog) -> RetType;
}

  // proto:  void QInputDialog::setDoubleRange(double min, double max);
impl<'a> /*trait*/ QInputDialog_setDoubleRange<()> for (f64, f64) {
  fn setDoubleRange(self , rsthis: &mut QInputDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog14setDoubleRangeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN12QInputDialog14setDoubleRangeEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// <= body block end

