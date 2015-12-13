// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qstringlist::QStringList;
use super::qobject::QObject;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  double QInputDialog::doubleMaximum();
  fn _ZNK12QInputDialog13doubleMaximumEv(qthis: *mut c_void) -> c_double;
  // proto:  void QInputDialog::setIntMaximum(int max);
  fn _ZN12QInputDialog13setIntMaximumEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  const QMetaObject * QInputDialog::metaObject();
  fn _ZNK12QInputDialog10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QInputDialog::setIntStep(int step);
  fn _ZN12QInputDialog10setIntStepEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QInputDialog::intMaximum();
  fn _ZNK12QInputDialog10intMaximumEv(qthis: *mut c_void) -> c_int;
  // proto:  int QInputDialog::intStep();
  fn _ZNK12QInputDialog7intStepEv(qthis: *mut c_void) -> c_int;
  // proto:  int QInputDialog::doubleDecimals();
  fn _ZNK12QInputDialog14doubleDecimalsEv(qthis: *mut c_void) -> c_int;
  // proto:  void QInputDialog::setDoubleDecimals(int decimals);
  fn _ZN12QInputDialog17setDoubleDecimalsEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QInputDialog::intValueChanged(int value);
  fn _ZN12QInputDialog15intValueChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QInputDialog::setIntMinimum(int min);
  fn _ZN12QInputDialog13setIntMinimumEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QInputDialog::setTextValue(const QString & text);
  fn _ZN12QInputDialog12setTextValueERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QInputDialog::done(int result);
  fn _ZN12QInputDialog4doneEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QInputDialog::FreeQInputDialog();
  fn _ZN12QInputDialogD0Ev(qthis: *mut c_void) ;
  // proto:  void QInputDialog::NewQInputDialog(const QInputDialog & );
  fn _ZN12QInputDialogC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QInputDialog::textValueSelected(const QString & text);
  fn _ZN12QInputDialog17textValueSelectedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QInputDialog::setLabelText(const QString & text);
  fn _ZN12QInputDialog12setLabelTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QInputDialog::labelText();
  fn _ZNK12QInputDialog9labelTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputDialog::setOkButtonText(const QString & text);
  fn _ZN12QInputDialog15setOkButtonTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStringList QInputDialog::comboBoxItems();
  fn _ZNK12QInputDialog13comboBoxItemsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QInputDialog::intMinimum();
  fn _ZNK12QInputDialog10intMinimumEv(qthis: *mut c_void) -> c_int;
  // proto:  void QInputDialog::setComboBoxEditable(bool editable);
  fn _ZN12QInputDialog19setComboBoxEditableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QInputDialog::setVisible(bool visible);
  fn _ZN12QInputDialog10setVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QInputDialog::setDoubleMinimum(double min);
  fn _ZN12QInputDialog16setDoubleMinimumEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QInputDialog::doubleMinimum();
  fn _ZNK12QInputDialog13doubleMinimumEv(qthis: *mut c_void) -> c_double;
  // proto:  QString QInputDialog::cancelButtonText();
  fn _ZNK12QInputDialog16cancelButtonTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputDialog::setComboBoxItems(const QStringList & items);
  fn _ZN12QInputDialog16setComboBoxItemsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QInputDialog::isComboBoxEditable();
  fn _ZNK12QInputDialog18isComboBoxEditableEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QInputDialog::open(QObject * receiver, const char * member);
  fn _ZN12QInputDialog4openEP7QObjectPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char) ;
  // proto:  void QInputDialog::doubleValueChanged(double value);
  fn _ZN12QInputDialog18doubleValueChangedEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QString QInputDialog::okButtonText();
  fn _ZNK12QInputDialog12okButtonTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QInputDialog::sizeHint();
  fn _ZNK12QInputDialog8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputDialog::setIntRange(int min, int max);
  fn _ZN12QInputDialog11setIntRangeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  QSize QInputDialog::minimumSizeHint();
  fn _ZNK12QInputDialog15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputDialog::setDoubleValue(double value);
  fn _ZN12QInputDialog14setDoubleValueEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QInputDialog::setIntValue(int value);
  fn _ZN12QInputDialog11setIntValueEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  double QInputDialog::doubleValue();
  fn _ZNK12QInputDialog11doubleValueEv(qthis: *mut c_void) -> c_double;
  // proto:  void QInputDialog::setCancelButtonText(const QString & text);
  fn _ZN12QInputDialog19setCancelButtonTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QInputDialog::doubleValueSelected(double value);
  fn _ZN12QInputDialog19doubleValueSelectedEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QString QInputDialog::textValue();
  fn _ZNK12QInputDialog9textValueEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputDialog::textValueChanged(const QString & text);
  fn _ZN12QInputDialog16textValueChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QInputDialog::intValueSelected(int value);
  fn _ZN12QInputDialog16intValueSelectedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QInputDialog::setDoubleMaximum(double max);
  fn _ZN12QInputDialog16setDoubleMaximumEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  int QInputDialog::intValue();
  fn _ZNK12QInputDialog8intValueEv(qthis: *mut c_void) -> c_int;
  // proto:  void QInputDialog::setDoubleRange(double min, double max);
  fn _ZN12QInputDialog14setDoubleRangeEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
}

// body block begin
// class sizeof(QInputDialog)=1
pub struct QInputDialog {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QInputDialog {
  pub fn doubleMaximum<T: QInputDialog_doubleMaximum>(&mut self, value: T) -> f64 {
    return value.doubleMaximum(self);
    // return 1;
  }
}

pub trait QInputDialog_doubleMaximum {
  fn doubleMaximum(self, rsthis: &mut QInputDialog) -> f64;
}

// proto:  double QInputDialog::doubleMaximum();
impl<'a> /*trait*/ QInputDialog_doubleMaximum for () {
  fn doubleMaximum(self, rsthis: &mut QInputDialog) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog13doubleMaximumEv()};
    let mut ret = unsafe {_ZNK12QInputDialog13doubleMaximumEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn setIntMaximum<T: QInputDialog_setIntMaximum>(&mut self, value: T)  {
     value.setIntMaximum(self);
    // return 1;
  }
}

pub trait QInputDialog_setIntMaximum {
  fn setIntMaximum(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::setIntMaximum(int max);
impl<'a> /*trait*/ QInputDialog_setIntMaximum for (i32) {
  fn setIntMaximum(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog13setIntMaximumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QInputDialog13setIntMaximumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn metaObject<T: QInputDialog_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QInputDialog_metaObject {
  fn metaObject(self, rsthis: &mut QInputDialog) ;
}

// proto:  const QMetaObject * QInputDialog::metaObject();
impl<'a> /*trait*/ QInputDialog_metaObject for () {
  fn metaObject(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog10metaObjectEv()};
     unsafe {_ZNK12QInputDialog10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn setIntStep<T: QInputDialog_setIntStep>(&mut self, value: T)  {
     value.setIntStep(self);
    // return 1;
  }
}

pub trait QInputDialog_setIntStep {
  fn setIntStep(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::setIntStep(int step);
impl<'a> /*trait*/ QInputDialog_setIntStep for (i32) {
  fn setIntStep(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog10setIntStepEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QInputDialog10setIntStepEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn intMaximum<T: QInputDialog_intMaximum>(&mut self, value: T) -> i32 {
    return value.intMaximum(self);
    // return 1;
  }
}

pub trait QInputDialog_intMaximum {
  fn intMaximum(self, rsthis: &mut QInputDialog) -> i32;
}

// proto:  int QInputDialog::intMaximum();
impl<'a> /*trait*/ QInputDialog_intMaximum for () {
  fn intMaximum(self, rsthis: &mut QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog10intMaximumEv()};
    let mut ret = unsafe {_ZNK12QInputDialog10intMaximumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn intStep<T: QInputDialog_intStep>(&mut self, value: T) -> i32 {
    return value.intStep(self);
    // return 1;
  }
}

pub trait QInputDialog_intStep {
  fn intStep(self, rsthis: &mut QInputDialog) -> i32;
}

// proto:  int QInputDialog::intStep();
impl<'a> /*trait*/ QInputDialog_intStep for () {
  fn intStep(self, rsthis: &mut QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog7intStepEv()};
    let mut ret = unsafe {_ZNK12QInputDialog7intStepEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn doubleDecimals<T: QInputDialog_doubleDecimals>(&mut self, value: T) -> i32 {
    return value.doubleDecimals(self);
    // return 1;
  }
}

pub trait QInputDialog_doubleDecimals {
  fn doubleDecimals(self, rsthis: &mut QInputDialog) -> i32;
}

// proto:  int QInputDialog::doubleDecimals();
impl<'a> /*trait*/ QInputDialog_doubleDecimals for () {
  fn doubleDecimals(self, rsthis: &mut QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog14doubleDecimalsEv()};
    let mut ret = unsafe {_ZNK12QInputDialog14doubleDecimalsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn setDoubleDecimals<T: QInputDialog_setDoubleDecimals>(&mut self, value: T)  {
     value.setDoubleDecimals(self);
    // return 1;
  }
}

pub trait QInputDialog_setDoubleDecimals {
  fn setDoubleDecimals(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::setDoubleDecimals(int decimals);
impl<'a> /*trait*/ QInputDialog_setDoubleDecimals for (i32) {
  fn setDoubleDecimals(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog17setDoubleDecimalsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QInputDialog17setDoubleDecimalsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn intValueChanged<T: QInputDialog_intValueChanged>(&mut self, value: T)  {
     value.intValueChanged(self);
    // return 1;
  }
}

pub trait QInputDialog_intValueChanged {
  fn intValueChanged(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::intValueChanged(int value);
impl<'a> /*trait*/ QInputDialog_intValueChanged for (i32) {
  fn intValueChanged(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog15intValueChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QInputDialog15intValueChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn setIntMinimum<T: QInputDialog_setIntMinimum>(&mut self, value: T)  {
     value.setIntMinimum(self);
    // return 1;
  }
}

pub trait QInputDialog_setIntMinimum {
  fn setIntMinimum(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::setIntMinimum(int min);
impl<'a> /*trait*/ QInputDialog_setIntMinimum for (i32) {
  fn setIntMinimum(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog13setIntMinimumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QInputDialog13setIntMinimumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn setTextValue<T: QInputDialog_setTextValue>(&mut self, value: T)  {
     value.setTextValue(self);
    // return 1;
  }
}

pub trait QInputDialog_setTextValue {
  fn setTextValue(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::setTextValue(const QString & text);
impl<'a> /*trait*/ QInputDialog_setTextValue for (&'a  QString) {
  fn setTextValue(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog12setTextValueERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QInputDialog12setTextValueERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn done<T: QInputDialog_done>(&mut self, value: T)  {
     value.done(self);
    // return 1;
  }
}

pub trait QInputDialog_done {
  fn done(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::done(int result);
impl<'a> /*trait*/ QInputDialog_done for (i32) {
  fn done(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog4doneEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QInputDialog4doneEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn FreeQInputDialog<T: QInputDialog_FreeQInputDialog>(&mut self, value: T)  {
     value.FreeQInputDialog(self);
    // return 1;
  }
}

pub trait QInputDialog_FreeQInputDialog {
  fn FreeQInputDialog(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::FreeQInputDialog();
impl<'a> /*trait*/ QInputDialog_FreeQInputDialog for () {
  fn FreeQInputDialog(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialogD0Ev()};
     unsafe {_ZN12QInputDialogD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

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

// proto: void QInputDialog::NewQInputDialog(const QInputDialog & );
impl<'a> /*trait*/ QInputDialog_NewQInputDialog for (&'a  QInputDialog) {
  fn NewQInputDialog(self) -> QInputDialog {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialogC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QInputDialogC1ERKS_(qthis, arg0)};
    let rsthis = QInputDialog{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn textValueSelected<T: QInputDialog_textValueSelected>(&mut self, value: T)  {
     value.textValueSelected(self);
    // return 1;
  }
}

pub trait QInputDialog_textValueSelected {
  fn textValueSelected(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::textValueSelected(const QString & text);
impl<'a> /*trait*/ QInputDialog_textValueSelected for (&'a  QString) {
  fn textValueSelected(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog17textValueSelectedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QInputDialog17textValueSelectedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn setLabelText<T: QInputDialog_setLabelText>(&mut self, value: T)  {
     value.setLabelText(self);
    // return 1;
  }
}

pub trait QInputDialog_setLabelText {
  fn setLabelText(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::setLabelText(const QString & text);
impl<'a> /*trait*/ QInputDialog_setLabelText for (&'a  QString) {
  fn setLabelText(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog12setLabelTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QInputDialog12setLabelTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn labelText<T: QInputDialog_labelText>(&mut self, value: T) -> QString {
    return value.labelText(self);
    // return 1;
  }
}

pub trait QInputDialog_labelText {
  fn labelText(self, rsthis: &mut QInputDialog) -> QString;
}

// proto:  QString QInputDialog::labelText();
impl<'a> /*trait*/ QInputDialog_labelText for () {
  fn labelText(self, rsthis: &mut QInputDialog) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog9labelTextEv()};
    let mut ret = unsafe {_ZNK12QInputDialog9labelTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn setOkButtonText<T: QInputDialog_setOkButtonText>(&mut self, value: T)  {
     value.setOkButtonText(self);
    // return 1;
  }
}

pub trait QInputDialog_setOkButtonText {
  fn setOkButtonText(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::setOkButtonText(const QString & text);
impl<'a> /*trait*/ QInputDialog_setOkButtonText for (&'a  QString) {
  fn setOkButtonText(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog15setOkButtonTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QInputDialog15setOkButtonTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn comboBoxItems<T: QInputDialog_comboBoxItems>(&mut self, value: T) -> QStringList {
    return value.comboBoxItems(self);
    // return 1;
  }
}

pub trait QInputDialog_comboBoxItems {
  fn comboBoxItems(self, rsthis: &mut QInputDialog) -> QStringList;
}

// proto:  QStringList QInputDialog::comboBoxItems();
impl<'a> /*trait*/ QInputDialog_comboBoxItems for () {
  fn comboBoxItems(self, rsthis: &mut QInputDialog) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog13comboBoxItemsEv()};
    let mut ret = unsafe {_ZNK12QInputDialog13comboBoxItemsEv(rsthis.qclsinst)};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn intMinimum<T: QInputDialog_intMinimum>(&mut self, value: T) -> i32 {
    return value.intMinimum(self);
    // return 1;
  }
}

pub trait QInputDialog_intMinimum {
  fn intMinimum(self, rsthis: &mut QInputDialog) -> i32;
}

// proto:  int QInputDialog::intMinimum();
impl<'a> /*trait*/ QInputDialog_intMinimum for () {
  fn intMinimum(self, rsthis: &mut QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog10intMinimumEv()};
    let mut ret = unsafe {_ZNK12QInputDialog10intMinimumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn setComboBoxEditable<T: QInputDialog_setComboBoxEditable>(&mut self, value: T)  {
     value.setComboBoxEditable(self);
    // return 1;
  }
}

pub trait QInputDialog_setComboBoxEditable {
  fn setComboBoxEditable(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::setComboBoxEditable(bool editable);
impl<'a> /*trait*/ QInputDialog_setComboBoxEditable for (i8) {
  fn setComboBoxEditable(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog19setComboBoxEditableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QInputDialog19setComboBoxEditableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn setVisible<T: QInputDialog_setVisible>(&mut self, value: T)  {
     value.setVisible(self);
    // return 1;
  }
}

pub trait QInputDialog_setVisible {
  fn setVisible(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::setVisible(bool visible);
impl<'a> /*trait*/ QInputDialog_setVisible for (i8) {
  fn setVisible(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog10setVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QInputDialog10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn setDoubleMinimum<T: QInputDialog_setDoubleMinimum>(&mut self, value: T)  {
     value.setDoubleMinimum(self);
    // return 1;
  }
}

pub trait QInputDialog_setDoubleMinimum {
  fn setDoubleMinimum(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::setDoubleMinimum(double min);
impl<'a> /*trait*/ QInputDialog_setDoubleMinimum for (f64) {
  fn setDoubleMinimum(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog16setDoubleMinimumEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN12QInputDialog16setDoubleMinimumEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn doubleMinimum<T: QInputDialog_doubleMinimum>(&mut self, value: T) -> f64 {
    return value.doubleMinimum(self);
    // return 1;
  }
}

pub trait QInputDialog_doubleMinimum {
  fn doubleMinimum(self, rsthis: &mut QInputDialog) -> f64;
}

// proto:  double QInputDialog::doubleMinimum();
impl<'a> /*trait*/ QInputDialog_doubleMinimum for () {
  fn doubleMinimum(self, rsthis: &mut QInputDialog) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog13doubleMinimumEv()};
    let mut ret = unsafe {_ZNK12QInputDialog13doubleMinimumEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn cancelButtonText<T: QInputDialog_cancelButtonText>(&mut self, value: T) -> QString {
    return value.cancelButtonText(self);
    // return 1;
  }
}

pub trait QInputDialog_cancelButtonText {
  fn cancelButtonText(self, rsthis: &mut QInputDialog) -> QString;
}

// proto:  QString QInputDialog::cancelButtonText();
impl<'a> /*trait*/ QInputDialog_cancelButtonText for () {
  fn cancelButtonText(self, rsthis: &mut QInputDialog) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog16cancelButtonTextEv()};
    let mut ret = unsafe {_ZNK12QInputDialog16cancelButtonTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn setComboBoxItems<T: QInputDialog_setComboBoxItems>(&mut self, value: T)  {
     value.setComboBoxItems(self);
    // return 1;
  }
}

pub trait QInputDialog_setComboBoxItems {
  fn setComboBoxItems(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::setComboBoxItems(const QStringList & items);
impl<'a> /*trait*/ QInputDialog_setComboBoxItems for (&'a  QStringList) {
  fn setComboBoxItems(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog16setComboBoxItemsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QInputDialog16setComboBoxItemsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn isComboBoxEditable<T: QInputDialog_isComboBoxEditable>(&mut self, value: T) -> i8 {
    return value.isComboBoxEditable(self);
    // return 1;
  }
}

pub trait QInputDialog_isComboBoxEditable {
  fn isComboBoxEditable(self, rsthis: &mut QInputDialog) -> i8;
}

// proto:  bool QInputDialog::isComboBoxEditable();
impl<'a> /*trait*/ QInputDialog_isComboBoxEditable for () {
  fn isComboBoxEditable(self, rsthis: &mut QInputDialog) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog18isComboBoxEditableEv()};
    let mut ret = unsafe {_ZNK12QInputDialog18isComboBoxEditableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn open<T: QInputDialog_open>(&mut self, value: T)  {
     value.open(self);
    // return 1;
  }
}

pub trait QInputDialog_open {
  fn open(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::open(QObject * receiver, const char * member);
impl<'a> /*trait*/ QInputDialog_open for (&'a mut QObject, &'a  String) {
  fn open(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog4openEP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
     unsafe {_ZN12QInputDialog4openEP7QObjectPKc(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn doubleValueChanged<T: QInputDialog_doubleValueChanged>(&mut self, value: T)  {
     value.doubleValueChanged(self);
    // return 1;
  }
}

pub trait QInputDialog_doubleValueChanged {
  fn doubleValueChanged(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::doubleValueChanged(double value);
impl<'a> /*trait*/ QInputDialog_doubleValueChanged for (f64) {
  fn doubleValueChanged(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog18doubleValueChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN12QInputDialog18doubleValueChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn okButtonText<T: QInputDialog_okButtonText>(&mut self, value: T) -> QString {
    return value.okButtonText(self);
    // return 1;
  }
}

pub trait QInputDialog_okButtonText {
  fn okButtonText(self, rsthis: &mut QInputDialog) -> QString;
}

// proto:  QString QInputDialog::okButtonText();
impl<'a> /*trait*/ QInputDialog_okButtonText for () {
  fn okButtonText(self, rsthis: &mut QInputDialog) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog12okButtonTextEv()};
    let mut ret = unsafe {_ZNK12QInputDialog12okButtonTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn sizeHint<T: QInputDialog_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QInputDialog_sizeHint {
  fn sizeHint(self, rsthis: &mut QInputDialog) -> QSize;
}

// proto:  QSize QInputDialog::sizeHint();
impl<'a> /*trait*/ QInputDialog_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QInputDialog) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog8sizeHintEv()};
    let mut ret = unsafe {_ZNK12QInputDialog8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn setIntRange<T: QInputDialog_setIntRange>(&mut self, value: T)  {
     value.setIntRange(self);
    // return 1;
  }
}

pub trait QInputDialog_setIntRange {
  fn setIntRange(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::setIntRange(int min, int max);
impl<'a> /*trait*/ QInputDialog_setIntRange for (i32, i32) {
  fn setIntRange(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog11setIntRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QInputDialog11setIntRangeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn minimumSizeHint<T: QInputDialog_minimumSizeHint>(&mut self, value: T) -> QSize {
    return value.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QInputDialog_minimumSizeHint {
  fn minimumSizeHint(self, rsthis: &mut QInputDialog) -> QSize;
}

// proto:  QSize QInputDialog::minimumSizeHint();
impl<'a> /*trait*/ QInputDialog_minimumSizeHint for () {
  fn minimumSizeHint(self, rsthis: &mut QInputDialog) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK12QInputDialog15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn setDoubleValue<T: QInputDialog_setDoubleValue>(&mut self, value: T)  {
     value.setDoubleValue(self);
    // return 1;
  }
}

pub trait QInputDialog_setDoubleValue {
  fn setDoubleValue(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::setDoubleValue(double value);
impl<'a> /*trait*/ QInputDialog_setDoubleValue for (f64) {
  fn setDoubleValue(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog14setDoubleValueEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN12QInputDialog14setDoubleValueEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn setIntValue<T: QInputDialog_setIntValue>(&mut self, value: T)  {
     value.setIntValue(self);
    // return 1;
  }
}

pub trait QInputDialog_setIntValue {
  fn setIntValue(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::setIntValue(int value);
impl<'a> /*trait*/ QInputDialog_setIntValue for (i32) {
  fn setIntValue(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog11setIntValueEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QInputDialog11setIntValueEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn doubleValue<T: QInputDialog_doubleValue>(&mut self, value: T) -> f64 {
    return value.doubleValue(self);
    // return 1;
  }
}

pub trait QInputDialog_doubleValue {
  fn doubleValue(self, rsthis: &mut QInputDialog) -> f64;
}

// proto:  double QInputDialog::doubleValue();
impl<'a> /*trait*/ QInputDialog_doubleValue for () {
  fn doubleValue(self, rsthis: &mut QInputDialog) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog11doubleValueEv()};
    let mut ret = unsafe {_ZNK12QInputDialog11doubleValueEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn setCancelButtonText<T: QInputDialog_setCancelButtonText>(&mut self, value: T)  {
     value.setCancelButtonText(self);
    // return 1;
  }
}

pub trait QInputDialog_setCancelButtonText {
  fn setCancelButtonText(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::setCancelButtonText(const QString & text);
impl<'a> /*trait*/ QInputDialog_setCancelButtonText for (&'a  QString) {
  fn setCancelButtonText(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog19setCancelButtonTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QInputDialog19setCancelButtonTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn doubleValueSelected<T: QInputDialog_doubleValueSelected>(&mut self, value: T)  {
     value.doubleValueSelected(self);
    // return 1;
  }
}

pub trait QInputDialog_doubleValueSelected {
  fn doubleValueSelected(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::doubleValueSelected(double value);
impl<'a> /*trait*/ QInputDialog_doubleValueSelected for (f64) {
  fn doubleValueSelected(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog19doubleValueSelectedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN12QInputDialog19doubleValueSelectedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn textValue<T: QInputDialog_textValue>(&mut self, value: T) -> QString {
    return value.textValue(self);
    // return 1;
  }
}

pub trait QInputDialog_textValue {
  fn textValue(self, rsthis: &mut QInputDialog) -> QString;
}

// proto:  QString QInputDialog::textValue();
impl<'a> /*trait*/ QInputDialog_textValue for () {
  fn textValue(self, rsthis: &mut QInputDialog) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog9textValueEv()};
    let mut ret = unsafe {_ZNK12QInputDialog9textValueEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn textValueChanged<T: QInputDialog_textValueChanged>(&mut self, value: T)  {
     value.textValueChanged(self);
    // return 1;
  }
}

pub trait QInputDialog_textValueChanged {
  fn textValueChanged(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::textValueChanged(const QString & text);
impl<'a> /*trait*/ QInputDialog_textValueChanged for (&'a  QString) {
  fn textValueChanged(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog16textValueChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QInputDialog16textValueChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn intValueSelected<T: QInputDialog_intValueSelected>(&mut self, value: T)  {
     value.intValueSelected(self);
    // return 1;
  }
}

pub trait QInputDialog_intValueSelected {
  fn intValueSelected(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::intValueSelected(int value);
impl<'a> /*trait*/ QInputDialog_intValueSelected for (i32) {
  fn intValueSelected(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog16intValueSelectedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QInputDialog16intValueSelectedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn setDoubleMaximum<T: QInputDialog_setDoubleMaximum>(&mut self, value: T)  {
     value.setDoubleMaximum(self);
    // return 1;
  }
}

pub trait QInputDialog_setDoubleMaximum {
  fn setDoubleMaximum(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::setDoubleMaximum(double max);
impl<'a> /*trait*/ QInputDialog_setDoubleMaximum for (f64) {
  fn setDoubleMaximum(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog16setDoubleMaximumEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN12QInputDialog16setDoubleMaximumEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn intValue<T: QInputDialog_intValue>(&mut self, value: T) -> i32 {
    return value.intValue(self);
    // return 1;
  }
}

pub trait QInputDialog_intValue {
  fn intValue(self, rsthis: &mut QInputDialog) -> i32;
}

// proto:  int QInputDialog::intValue();
impl<'a> /*trait*/ QInputDialog_intValue for () {
  fn intValue(self, rsthis: &mut QInputDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QInputDialog8intValueEv()};
    let mut ret = unsafe {_ZNK12QInputDialog8intValueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QInputDialog {
  pub fn setDoubleRange<T: QInputDialog_setDoubleRange>(&mut self, value: T)  {
     value.setDoubleRange(self);
    // return 1;
  }
}

pub trait QInputDialog_setDoubleRange {
  fn setDoubleRange(self, rsthis: &mut QInputDialog) ;
}

// proto:  void QInputDialog::setDoubleRange(double min, double max);
impl<'a> /*trait*/ QInputDialog_setDoubleRange for (f64, f64) {
  fn setDoubleRange(self, rsthis: &mut QInputDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QInputDialog14setDoubleRangeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN12QInputDialog14setDoubleRangeEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

