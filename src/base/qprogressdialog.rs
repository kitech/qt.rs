// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qstring::QString;
use super::qprogressbar::QProgressBar;
use super::qsize::QSize;
use super::qlabel::QLabel;
use super::qpushbutton::QPushButton;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QProgressDialog::setAutoClose(bool close);
  fn _ZN15QProgressDialog12setAutoCloseEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QProgressDialog::open(QObject * receiver, const char * member);
  fn _ZN15QProgressDialog4openEP7QObjectPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char) ;
  // proto:  void QProgressDialog::setMaximum(int maximum);
  fn _ZN15QProgressDialog10setMaximumEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QProgressDialog::setMinimum(int minimum);
  fn _ZN15QProgressDialog10setMinimumEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QProgressDialog::setLabelText(const QString & text);
  fn _ZN15QProgressDialog12setLabelTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QProgressDialog::wasCanceled();
  fn _ZNK15QProgressDialog11wasCanceledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QProgressDialog::FreeQProgressDialog();
  fn _ZN15QProgressDialogD0Ev(qthis: *mut c_void) ;
  // proto:  int QProgressDialog::minimumDuration();
  fn _ZNK15QProgressDialog15minimumDurationEv(qthis: *mut c_void) -> c_int;
  // proto:  void QProgressDialog::setMinimumDuration(int ms);
  fn _ZN15QProgressDialog18setMinimumDurationEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QProgressDialog::maximum();
  fn _ZNK15QProgressDialog7maximumEv(qthis: *mut c_void) -> c_int;
  // proto:  void QProgressDialog::setBar(QProgressBar * bar);
  fn _ZN15QProgressDialog6setBarEP12QProgressBar(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QProgressDialog::cancel();
  fn _ZN15QProgressDialog6cancelEv(qthis: *mut c_void) ;
  // proto:  bool QProgressDialog::autoClose();
  fn _ZNK15QProgressDialog9autoCloseEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QProgressDialog::minimum();
  fn _ZNK15QProgressDialog7minimumEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QProgressDialog::autoReset();
  fn _ZNK15QProgressDialog9autoResetEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QProgressDialog::reset();
  fn _ZN15QProgressDialog5resetEv(qthis: *mut c_void) ;
  // proto:  void QProgressDialog::NewQProgressDialog(const QProgressDialog & );
  fn _ZN15QProgressDialogC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QProgressDialog::setRange(int minimum, int maximum);
  fn _ZN15QProgressDialog8setRangeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QProgressDialog::canceled();
  fn _ZN15QProgressDialog8canceledEv(qthis: *mut c_void) ;
  // proto:  void QProgressDialog::setCancelButtonText(const QString & text);
  fn _ZN15QProgressDialog19setCancelButtonTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSize QProgressDialog::sizeHint();
  fn _ZNK15QProgressDialog8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QProgressDialog::labelText();
  fn _ZNK15QProgressDialog9labelTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QProgressDialog::setLabel(QLabel * label);
  fn _ZN15QProgressDialog8setLabelEP6QLabel(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QProgressDialog::metaObject();
  fn _ZNK15QProgressDialog10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QProgressDialog::setAutoReset(bool reset);
  fn _ZN15QProgressDialog12setAutoResetEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QProgressDialog::value();
  fn _ZNK15QProgressDialog5valueEv(qthis: *mut c_void) -> c_int;
  // proto:  void QProgressDialog::setCancelButton(QPushButton * button);
  fn _ZN15QProgressDialog15setCancelButtonEP11QPushButton(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QProgressDialog::setValue(int progress);
  fn _ZN15QProgressDialog8setValueEi(qthis: *mut c_void, arg0: c_int) ;
}

// body block begin
// class sizeof(QProgressDialog)=1
pub struct QProgressDialog {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QProgressDialog {
  pub fn setAutoClose<T: QProgressDialog_setAutoClose>(&mut self, value: T)  {
     value.setAutoClose(self);
    // return 1;
  }
}

pub trait QProgressDialog_setAutoClose {
  fn setAutoClose(self, rsthis: &mut QProgressDialog) ;
}

// proto:  void QProgressDialog::setAutoClose(bool close);
impl<'a> /*trait*/ QProgressDialog_setAutoClose for (i8) {
  fn setAutoClose(self, rsthis: &mut QProgressDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog12setAutoCloseEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QProgressDialog12setAutoCloseEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn open<T: QProgressDialog_open>(&mut self, value: T)  {
     value.open(self);
    // return 1;
  }
}

pub trait QProgressDialog_open {
  fn open(self, rsthis: &mut QProgressDialog) ;
}

// proto:  void QProgressDialog::open(QObject * receiver, const char * member);
impl<'a> /*trait*/ QProgressDialog_open for (&'a mut QObject, &'a  String) {
  fn open(self, rsthis: &mut QProgressDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog4openEP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
     unsafe {_ZN15QProgressDialog4openEP7QObjectPKc(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setMaximum<T: QProgressDialog_setMaximum>(&mut self, value: T)  {
     value.setMaximum(self);
    // return 1;
  }
}

pub trait QProgressDialog_setMaximum {
  fn setMaximum(self, rsthis: &mut QProgressDialog) ;
}

// proto:  void QProgressDialog::setMaximum(int maximum);
impl<'a> /*trait*/ QProgressDialog_setMaximum for (i32) {
  fn setMaximum(self, rsthis: &mut QProgressDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog10setMaximumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QProgressDialog10setMaximumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setMinimum<T: QProgressDialog_setMinimum>(&mut self, value: T)  {
     value.setMinimum(self);
    // return 1;
  }
}

pub trait QProgressDialog_setMinimum {
  fn setMinimum(self, rsthis: &mut QProgressDialog) ;
}

// proto:  void QProgressDialog::setMinimum(int minimum);
impl<'a> /*trait*/ QProgressDialog_setMinimum for (i32) {
  fn setMinimum(self, rsthis: &mut QProgressDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog10setMinimumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QProgressDialog10setMinimumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setLabelText<T: QProgressDialog_setLabelText>(&mut self, value: T)  {
     value.setLabelText(self);
    // return 1;
  }
}

pub trait QProgressDialog_setLabelText {
  fn setLabelText(self, rsthis: &mut QProgressDialog) ;
}

// proto:  void QProgressDialog::setLabelText(const QString & text);
impl<'a> /*trait*/ QProgressDialog_setLabelText for (&'a  QString) {
  fn setLabelText(self, rsthis: &mut QProgressDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog12setLabelTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QProgressDialog12setLabelTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn wasCanceled<T: QProgressDialog_wasCanceled>(&mut self, value: T) -> i8 {
    return value.wasCanceled(self);
    // return 1;
  }
}

pub trait QProgressDialog_wasCanceled {
  fn wasCanceled(self, rsthis: &mut QProgressDialog) -> i8;
}

// proto:  bool QProgressDialog::wasCanceled();
impl<'a> /*trait*/ QProgressDialog_wasCanceled for () {
  fn wasCanceled(self, rsthis: &mut QProgressDialog) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog11wasCanceledEv()};
    let mut ret = unsafe {_ZNK15QProgressDialog11wasCanceledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn FreeQProgressDialog<T: QProgressDialog_FreeQProgressDialog>(&mut self, value: T)  {
     value.FreeQProgressDialog(self);
    // return 1;
  }
}

pub trait QProgressDialog_FreeQProgressDialog {
  fn FreeQProgressDialog(self, rsthis: &mut QProgressDialog) ;
}

// proto:  void QProgressDialog::FreeQProgressDialog();
impl<'a> /*trait*/ QProgressDialog_FreeQProgressDialog for () {
  fn FreeQProgressDialog(self, rsthis: &mut QProgressDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialogD0Ev()};
     unsafe {_ZN15QProgressDialogD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn minimumDuration<T: QProgressDialog_minimumDuration>(&mut self, value: T) -> i32 {
    return value.minimumDuration(self);
    // return 1;
  }
}

pub trait QProgressDialog_minimumDuration {
  fn minimumDuration(self, rsthis: &mut QProgressDialog) -> i32;
}

// proto:  int QProgressDialog::minimumDuration();
impl<'a> /*trait*/ QProgressDialog_minimumDuration for () {
  fn minimumDuration(self, rsthis: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog15minimumDurationEv()};
    let mut ret = unsafe {_ZNK15QProgressDialog15minimumDurationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setMinimumDuration<T: QProgressDialog_setMinimumDuration>(&mut self, value: T)  {
     value.setMinimumDuration(self);
    // return 1;
  }
}

pub trait QProgressDialog_setMinimumDuration {
  fn setMinimumDuration(self, rsthis: &mut QProgressDialog) ;
}

// proto:  void QProgressDialog::setMinimumDuration(int ms);
impl<'a> /*trait*/ QProgressDialog_setMinimumDuration for (i32) {
  fn setMinimumDuration(self, rsthis: &mut QProgressDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog18setMinimumDurationEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QProgressDialog18setMinimumDurationEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn maximum<T: QProgressDialog_maximum>(&mut self, value: T) -> i32 {
    return value.maximum(self);
    // return 1;
  }
}

pub trait QProgressDialog_maximum {
  fn maximum(self, rsthis: &mut QProgressDialog) -> i32;
}

// proto:  int QProgressDialog::maximum();
impl<'a> /*trait*/ QProgressDialog_maximum for () {
  fn maximum(self, rsthis: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog7maximumEv()};
    let mut ret = unsafe {_ZNK15QProgressDialog7maximumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setBar<T: QProgressDialog_setBar>(&mut self, value: T)  {
     value.setBar(self);
    // return 1;
  }
}

pub trait QProgressDialog_setBar {
  fn setBar(self, rsthis: &mut QProgressDialog) ;
}

// proto:  void QProgressDialog::setBar(QProgressBar * bar);
impl<'a> /*trait*/ QProgressDialog_setBar for (&'a mut QProgressBar) {
  fn setBar(self, rsthis: &mut QProgressDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog6setBarEP12QProgressBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QProgressDialog6setBarEP12QProgressBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn cancel<T: QProgressDialog_cancel>(&mut self, value: T)  {
     value.cancel(self);
    // return 1;
  }
}

pub trait QProgressDialog_cancel {
  fn cancel(self, rsthis: &mut QProgressDialog) ;
}

// proto:  void QProgressDialog::cancel();
impl<'a> /*trait*/ QProgressDialog_cancel for () {
  fn cancel(self, rsthis: &mut QProgressDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog6cancelEv()};
     unsafe {_ZN15QProgressDialog6cancelEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn autoClose<T: QProgressDialog_autoClose>(&mut self, value: T) -> i8 {
    return value.autoClose(self);
    // return 1;
  }
}

pub trait QProgressDialog_autoClose {
  fn autoClose(self, rsthis: &mut QProgressDialog) -> i8;
}

// proto:  bool QProgressDialog::autoClose();
impl<'a> /*trait*/ QProgressDialog_autoClose for () {
  fn autoClose(self, rsthis: &mut QProgressDialog) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog9autoCloseEv()};
    let mut ret = unsafe {_ZNK15QProgressDialog9autoCloseEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn minimum<T: QProgressDialog_minimum>(&mut self, value: T) -> i32 {
    return value.minimum(self);
    // return 1;
  }
}

pub trait QProgressDialog_minimum {
  fn minimum(self, rsthis: &mut QProgressDialog) -> i32;
}

// proto:  int QProgressDialog::minimum();
impl<'a> /*trait*/ QProgressDialog_minimum for () {
  fn minimum(self, rsthis: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog7minimumEv()};
    let mut ret = unsafe {_ZNK15QProgressDialog7minimumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn autoReset<T: QProgressDialog_autoReset>(&mut self, value: T) -> i8 {
    return value.autoReset(self);
    // return 1;
  }
}

pub trait QProgressDialog_autoReset {
  fn autoReset(self, rsthis: &mut QProgressDialog) -> i8;
}

// proto:  bool QProgressDialog::autoReset();
impl<'a> /*trait*/ QProgressDialog_autoReset for () {
  fn autoReset(self, rsthis: &mut QProgressDialog) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog9autoResetEv()};
    let mut ret = unsafe {_ZNK15QProgressDialog9autoResetEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn reset<T: QProgressDialog_reset>(&mut self, value: T)  {
     value.reset(self);
    // return 1;
  }
}

pub trait QProgressDialog_reset {
  fn reset(self, rsthis: &mut QProgressDialog) ;
}

// proto:  void QProgressDialog::reset();
impl<'a> /*trait*/ QProgressDialog_reset for () {
  fn reset(self, rsthis: &mut QProgressDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog5resetEv()};
     unsafe {_ZN15QProgressDialog5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn NewQProgressDialog<T: QProgressDialog_NewQProgressDialog>(value: T) -> QProgressDialog {
    let rsthis = value.NewQProgressDialog();
    return rsthis;
    // return 1;
  }
}

pub trait QProgressDialog_NewQProgressDialog {
  fn NewQProgressDialog(self) -> QProgressDialog;
}

// proto: void QProgressDialog::NewQProgressDialog(const QProgressDialog & );
impl<'a> /*trait*/ QProgressDialog_NewQProgressDialog for (&'a  QProgressDialog) {
  fn NewQProgressDialog(self) -> QProgressDialog {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialogC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QProgressDialogC1ERKS_(qthis, arg0)};
    let rsthis = QProgressDialog{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setRange<T: QProgressDialog_setRange>(&mut self, value: T)  {
     value.setRange(self);
    // return 1;
  }
}

pub trait QProgressDialog_setRange {
  fn setRange(self, rsthis: &mut QProgressDialog) ;
}

// proto:  void QProgressDialog::setRange(int minimum, int maximum);
impl<'a> /*trait*/ QProgressDialog_setRange for (i32, i32) {
  fn setRange(self, rsthis: &mut QProgressDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog8setRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN15QProgressDialog8setRangeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn canceled<T: QProgressDialog_canceled>(&mut self, value: T)  {
     value.canceled(self);
    // return 1;
  }
}

pub trait QProgressDialog_canceled {
  fn canceled(self, rsthis: &mut QProgressDialog) ;
}

// proto:  void QProgressDialog::canceled();
impl<'a> /*trait*/ QProgressDialog_canceled for () {
  fn canceled(self, rsthis: &mut QProgressDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog8canceledEv()};
     unsafe {_ZN15QProgressDialog8canceledEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setCancelButtonText<T: QProgressDialog_setCancelButtonText>(&mut self, value: T)  {
     value.setCancelButtonText(self);
    // return 1;
  }
}

pub trait QProgressDialog_setCancelButtonText {
  fn setCancelButtonText(self, rsthis: &mut QProgressDialog) ;
}

// proto:  void QProgressDialog::setCancelButtonText(const QString & text);
impl<'a> /*trait*/ QProgressDialog_setCancelButtonText for (&'a  QString) {
  fn setCancelButtonText(self, rsthis: &mut QProgressDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog19setCancelButtonTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QProgressDialog19setCancelButtonTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn sizeHint<T: QProgressDialog_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QProgressDialog_sizeHint {
  fn sizeHint(self, rsthis: &mut QProgressDialog) -> QSize;
}

// proto:  QSize QProgressDialog::sizeHint();
impl<'a> /*trait*/ QProgressDialog_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QProgressDialog) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog8sizeHintEv()};
    let mut ret = unsafe {_ZNK15QProgressDialog8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn labelText<T: QProgressDialog_labelText>(&mut self, value: T) -> QString {
    return value.labelText(self);
    // return 1;
  }
}

pub trait QProgressDialog_labelText {
  fn labelText(self, rsthis: &mut QProgressDialog) -> QString;
}

// proto:  QString QProgressDialog::labelText();
impl<'a> /*trait*/ QProgressDialog_labelText for () {
  fn labelText(self, rsthis: &mut QProgressDialog) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog9labelTextEv()};
    let mut ret = unsafe {_ZNK15QProgressDialog9labelTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setLabel<T: QProgressDialog_setLabel>(&mut self, value: T)  {
     value.setLabel(self);
    // return 1;
  }
}

pub trait QProgressDialog_setLabel {
  fn setLabel(self, rsthis: &mut QProgressDialog) ;
}

// proto:  void QProgressDialog::setLabel(QLabel * label);
impl<'a> /*trait*/ QProgressDialog_setLabel for (&'a mut QLabel) {
  fn setLabel(self, rsthis: &mut QProgressDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog8setLabelEP6QLabel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QProgressDialog8setLabelEP6QLabel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn metaObject<T: QProgressDialog_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QProgressDialog_metaObject {
  fn metaObject(self, rsthis: &mut QProgressDialog) ;
}

// proto:  const QMetaObject * QProgressDialog::metaObject();
impl<'a> /*trait*/ QProgressDialog_metaObject for () {
  fn metaObject(self, rsthis: &mut QProgressDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog10metaObjectEv()};
     unsafe {_ZNK15QProgressDialog10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setAutoReset<T: QProgressDialog_setAutoReset>(&mut self, value: T)  {
     value.setAutoReset(self);
    // return 1;
  }
}

pub trait QProgressDialog_setAutoReset {
  fn setAutoReset(self, rsthis: &mut QProgressDialog) ;
}

// proto:  void QProgressDialog::setAutoReset(bool reset);
impl<'a> /*trait*/ QProgressDialog_setAutoReset for (i8) {
  fn setAutoReset(self, rsthis: &mut QProgressDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog12setAutoResetEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QProgressDialog12setAutoResetEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn value<T: QProgressDialog_value>(&mut self, value: T) -> i32 {
    return value.value(self);
    // return 1;
  }
}

pub trait QProgressDialog_value {
  fn value(self, rsthis: &mut QProgressDialog) -> i32;
}

// proto:  int QProgressDialog::value();
impl<'a> /*trait*/ QProgressDialog_value for () {
  fn value(self, rsthis: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog5valueEv()};
    let mut ret = unsafe {_ZNK15QProgressDialog5valueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setCancelButton<T: QProgressDialog_setCancelButton>(&mut self, value: T)  {
     value.setCancelButton(self);
    // return 1;
  }
}

pub trait QProgressDialog_setCancelButton {
  fn setCancelButton(self, rsthis: &mut QProgressDialog) ;
}

// proto:  void QProgressDialog::setCancelButton(QPushButton * button);
impl<'a> /*trait*/ QProgressDialog_setCancelButton for (&'a mut QPushButton) {
  fn setCancelButton(self, rsthis: &mut QProgressDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog15setCancelButtonEP11QPushButton()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QProgressDialog15setCancelButtonEP11QPushButton(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setValue<T: QProgressDialog_setValue>(&mut self, value: T)  {
     value.setValue(self);
    // return 1;
  }
}

pub trait QProgressDialog_setValue {
  fn setValue(self, rsthis: &mut QProgressDialog) ;
}

// proto:  void QProgressDialog::setValue(int progress);
impl<'a> /*trait*/ QProgressDialog_setValue for (i32) {
  fn setValue(self, rsthis: &mut QProgressDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog8setValueEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QProgressDialog8setValueEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

