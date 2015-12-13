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
use super::qlabel::QLabel;
use super::qpushbutton::QPushButton;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QProgressDialog::setAutoClose(bool close);
  fn _ZN15QProgressDialog12setAutoCloseEb(arg0: int8_t) -> i32;
  // proto: void QProgressDialog::open(QObject * receiver, const char * member);
  fn _ZN15QProgressDialog4openEP7QObjectPKc(arg0: *mut c_void, arg1: *const c_char) -> i32;
  // proto: void QProgressDialog::setMaximum(int maximum);
  fn _ZN15QProgressDialog10setMaximumEi(arg0: c_int) -> i32;
  // proto: void QProgressDialog::setMinimum(int minimum);
  fn _ZN15QProgressDialog10setMinimumEi(arg0: c_int) -> i32;
  // proto: void QProgressDialog::setLabelText(const QString & text);
  fn _ZN15QProgressDialog12setLabelTextERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QProgressDialog::wasCanceled();
  fn _ZNK15QProgressDialog11wasCanceledEv() -> i32;
  // proto: void QProgressDialog::FreeQProgressDialog();
  fn _ZN15QProgressDialogD0Ev() -> i32;
  // proto: int QProgressDialog::minimumDuration();
  fn _ZNK15QProgressDialog15minimumDurationEv() -> i32;
  // proto: void QProgressDialog::setMinimumDuration(int ms);
  fn _ZN15QProgressDialog18setMinimumDurationEi(arg0: c_int) -> i32;
  // proto: int QProgressDialog::maximum();
  fn _ZNK15QProgressDialog7maximumEv() -> i32;
  // proto: void QProgressDialog::setBar(QProgressBar * bar);
  fn _ZN15QProgressDialog6setBarEP12QProgressBar(arg0: *mut c_void) -> i32;
  // proto: void QProgressDialog::cancel();
  fn _ZN15QProgressDialog6cancelEv() -> i32;
  // proto: bool QProgressDialog::autoClose();
  fn _ZNK15QProgressDialog9autoCloseEv() -> i32;
  // proto: int QProgressDialog::minimum();
  fn _ZNK15QProgressDialog7minimumEv() -> i32;
  // proto: bool QProgressDialog::autoReset();
  fn _ZNK15QProgressDialog9autoResetEv() -> i32;
  // proto: void QProgressDialog::reset();
  fn _ZN15QProgressDialog5resetEv() -> i32;
  // proto: void QProgressDialog::NewQProgressDialog(const QProgressDialog & );
  fn _ZN15QProgressDialogC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QProgressDialog::setRange(int minimum, int maximum);
  fn _ZN15QProgressDialog8setRangeEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QProgressDialog::canceled();
  fn _ZN15QProgressDialog8canceledEv() -> i32;
  // proto: void QProgressDialog::setCancelButtonText(const QString & text);
  fn _ZN15QProgressDialog19setCancelButtonTextERK7QString(arg0: *const c_void) -> i32;
  // proto: QSize QProgressDialog::sizeHint();
  fn _ZNK15QProgressDialog8sizeHintEv() -> i32;
  // proto: QString QProgressDialog::labelText();
  fn _ZNK15QProgressDialog9labelTextEv() -> i32;
  // proto: void QProgressDialog::setLabel(QLabel * label);
  fn _ZN15QProgressDialog8setLabelEP6QLabel(arg0: *mut c_void) -> i32;
  // proto: const QMetaObject * QProgressDialog::metaObject();
  fn _ZNK15QProgressDialog10metaObjectEv() -> i32;
  // proto: void QProgressDialog::setAutoReset(bool reset);
  fn _ZN15QProgressDialog12setAutoResetEb(arg0: int8_t) -> i32;
  // proto: int QProgressDialog::value();
  fn _ZNK15QProgressDialog5valueEv() -> i32;
  // proto: void QProgressDialog::setCancelButton(QPushButton * button);
  fn _ZN15QProgressDialog15setCancelButtonEP11QPushButton(arg0: *mut c_void) -> i32;
  // proto: void QProgressDialog::setValue(int progress);
  fn _ZN15QProgressDialog8setValueEi(arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QProgressDialog)=1
pub struct QProgressDialog {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QProgressDialog {
  pub fn setAutoClose<T: QProgressDialog_setAutoClose>(&mut self, value: T) -> i32 {
    value.setAutoClose(self);
    return 1;
  }
}

pub trait QProgressDialog_setAutoClose {
  fn setAutoClose(self, this: &mut QProgressDialog) -> i32;
}

// proto: void QProgressDialog::setAutoClose(bool close);
impl<'a> /*trait*/ QProgressDialog_setAutoClose for (i8) {
  fn setAutoClose(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog12setAutoCloseEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QProgressDialog12setAutoCloseEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn open<T: QProgressDialog_open>(&mut self, value: T) -> i32 {
    value.open(self);
    return 1;
  }
}

pub trait QProgressDialog_open {
  fn open(self, this: &mut QProgressDialog) -> i32;
}

// proto: void QProgressDialog::open(QObject * receiver, const char * member);
impl<'a> /*trait*/ QProgressDialog_open for (&'a mut QObject, &'a  String) {
  fn open(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog4openEP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN15QProgressDialog4openEP7QObjectPKc(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setMaximum<T: QProgressDialog_setMaximum>(&mut self, value: T) -> i32 {
    value.setMaximum(self);
    return 1;
  }
}

pub trait QProgressDialog_setMaximum {
  fn setMaximum(self, this: &mut QProgressDialog) -> i32;
}

// proto: void QProgressDialog::setMaximum(int maximum);
impl<'a> /*trait*/ QProgressDialog_setMaximum for (i32) {
  fn setMaximum(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog10setMaximumEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN15QProgressDialog10setMaximumEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setMinimum<T: QProgressDialog_setMinimum>(&mut self, value: T) -> i32 {
    value.setMinimum(self);
    return 1;
  }
}

pub trait QProgressDialog_setMinimum {
  fn setMinimum(self, this: &mut QProgressDialog) -> i32;
}

// proto: void QProgressDialog::setMinimum(int minimum);
impl<'a> /*trait*/ QProgressDialog_setMinimum for (i32) {
  fn setMinimum(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog10setMinimumEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN15QProgressDialog10setMinimumEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setLabelText<T: QProgressDialog_setLabelText>(&mut self, value: T) -> i32 {
    value.setLabelText(self);
    return 1;
  }
}

pub trait QProgressDialog_setLabelText {
  fn setLabelText(self, this: &mut QProgressDialog) -> i32;
}

// proto: void QProgressDialog::setLabelText(const QString & text);
impl<'a> /*trait*/ QProgressDialog_setLabelText for (&'a  QString) {
  fn setLabelText(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog12setLabelTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QProgressDialog12setLabelTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn wasCanceled<T: QProgressDialog_wasCanceled>(&mut self, value: T) -> i32 {
    value.wasCanceled(self);
    return 1;
  }
}

pub trait QProgressDialog_wasCanceled {
  fn wasCanceled(self, this: &mut QProgressDialog) -> i32;
}

// proto: bool QProgressDialog::wasCanceled();
impl<'a> /*trait*/ QProgressDialog_wasCanceled for () {
  fn wasCanceled(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog11wasCanceledEv()};
    unsafe {_ZNK15QProgressDialog11wasCanceledEv()};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn FreeQProgressDialog<T: QProgressDialog_FreeQProgressDialog>(&mut self, value: T) -> i32 {
    value.FreeQProgressDialog(self);
    return 1;
  }
}

pub trait QProgressDialog_FreeQProgressDialog {
  fn FreeQProgressDialog(self, this: &mut QProgressDialog) -> i32;
}

// proto: void QProgressDialog::FreeQProgressDialog();
impl<'a> /*trait*/ QProgressDialog_FreeQProgressDialog for () {
  fn FreeQProgressDialog(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialogD0Ev()};
    unsafe {_ZN15QProgressDialogD0Ev()};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn minimumDuration<T: QProgressDialog_minimumDuration>(&mut self, value: T) -> i32 {
    value.minimumDuration(self);
    return 1;
  }
}

pub trait QProgressDialog_minimumDuration {
  fn minimumDuration(self, this: &mut QProgressDialog) -> i32;
}

// proto: int QProgressDialog::minimumDuration();
impl<'a> /*trait*/ QProgressDialog_minimumDuration for () {
  fn minimumDuration(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog15minimumDurationEv()};
    unsafe {_ZNK15QProgressDialog15minimumDurationEv()};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setMinimumDuration<T: QProgressDialog_setMinimumDuration>(&mut self, value: T) -> i32 {
    value.setMinimumDuration(self);
    return 1;
  }
}

pub trait QProgressDialog_setMinimumDuration {
  fn setMinimumDuration(self, this: &mut QProgressDialog) -> i32;
}

// proto: void QProgressDialog::setMinimumDuration(int ms);
impl<'a> /*trait*/ QProgressDialog_setMinimumDuration for (i32) {
  fn setMinimumDuration(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog18setMinimumDurationEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN15QProgressDialog18setMinimumDurationEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn maximum<T: QProgressDialog_maximum>(&mut self, value: T) -> i32 {
    value.maximum(self);
    return 1;
  }
}

pub trait QProgressDialog_maximum {
  fn maximum(self, this: &mut QProgressDialog) -> i32;
}

// proto: int QProgressDialog::maximum();
impl<'a> /*trait*/ QProgressDialog_maximum for () {
  fn maximum(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog7maximumEv()};
    unsafe {_ZNK15QProgressDialog7maximumEv()};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setBar<T: QProgressDialog_setBar>(&mut self, value: T) -> i32 {
    value.setBar(self);
    return 1;
  }
}

pub trait QProgressDialog_setBar {
  fn setBar(self, this: &mut QProgressDialog) -> i32;
}

// proto: void QProgressDialog::setBar(QProgressBar * bar);
impl<'a> /*trait*/ QProgressDialog_setBar for (&'a mut QProgressBar) {
  fn setBar(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog6setBarEP12QProgressBar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QProgressDialog6setBarEP12QProgressBar(arg0)};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn cancel<T: QProgressDialog_cancel>(&mut self, value: T) -> i32 {
    value.cancel(self);
    return 1;
  }
}

pub trait QProgressDialog_cancel {
  fn cancel(self, this: &mut QProgressDialog) -> i32;
}

// proto: void QProgressDialog::cancel();
impl<'a> /*trait*/ QProgressDialog_cancel for () {
  fn cancel(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog6cancelEv()};
    unsafe {_ZN15QProgressDialog6cancelEv()};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn autoClose<T: QProgressDialog_autoClose>(&mut self, value: T) -> i32 {
    value.autoClose(self);
    return 1;
  }
}

pub trait QProgressDialog_autoClose {
  fn autoClose(self, this: &mut QProgressDialog) -> i32;
}

// proto: bool QProgressDialog::autoClose();
impl<'a> /*trait*/ QProgressDialog_autoClose for () {
  fn autoClose(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog9autoCloseEv()};
    unsafe {_ZNK15QProgressDialog9autoCloseEv()};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn minimum<T: QProgressDialog_minimum>(&mut self, value: T) -> i32 {
    value.minimum(self);
    return 1;
  }
}

pub trait QProgressDialog_minimum {
  fn minimum(self, this: &mut QProgressDialog) -> i32;
}

// proto: int QProgressDialog::minimum();
impl<'a> /*trait*/ QProgressDialog_minimum for () {
  fn minimum(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog7minimumEv()};
    unsafe {_ZNK15QProgressDialog7minimumEv()};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn autoReset<T: QProgressDialog_autoReset>(&mut self, value: T) -> i32 {
    value.autoReset(self);
    return 1;
  }
}

pub trait QProgressDialog_autoReset {
  fn autoReset(self, this: &mut QProgressDialog) -> i32;
}

// proto: bool QProgressDialog::autoReset();
impl<'a> /*trait*/ QProgressDialog_autoReset for () {
  fn autoReset(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog9autoResetEv()};
    unsafe {_ZNK15QProgressDialog9autoResetEv()};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn reset<T: QProgressDialog_reset>(&mut self, value: T) -> i32 {
    value.reset(self);
    return 1;
  }
}

pub trait QProgressDialog_reset {
  fn reset(self, this: &mut QProgressDialog) -> i32;
}

// proto: void QProgressDialog::reset();
impl<'a> /*trait*/ QProgressDialog_reset for () {
  fn reset(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog5resetEv()};
    unsafe {_ZN15QProgressDialog5resetEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QProgressDialogC1ERKS_(qthis, arg0)};
    let rsthis = QProgressDialog{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setRange<T: QProgressDialog_setRange>(&mut self, value: T) -> i32 {
    value.setRange(self);
    return 1;
  }
}

pub trait QProgressDialog_setRange {
  fn setRange(self, this: &mut QProgressDialog) -> i32;
}

// proto: void QProgressDialog::setRange(int minimum, int maximum);
impl<'a> /*trait*/ QProgressDialog_setRange for (i32, i32) {
  fn setRange(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog8setRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN15QProgressDialog8setRangeEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn canceled<T: QProgressDialog_canceled>(&mut self, value: T) -> i32 {
    value.canceled(self);
    return 1;
  }
}

pub trait QProgressDialog_canceled {
  fn canceled(self, this: &mut QProgressDialog) -> i32;
}

// proto: void QProgressDialog::canceled();
impl<'a> /*trait*/ QProgressDialog_canceled for () {
  fn canceled(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog8canceledEv()};
    unsafe {_ZN15QProgressDialog8canceledEv()};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setCancelButtonText<T: QProgressDialog_setCancelButtonText>(&mut self, value: T) -> i32 {
    value.setCancelButtonText(self);
    return 1;
  }
}

pub trait QProgressDialog_setCancelButtonText {
  fn setCancelButtonText(self, this: &mut QProgressDialog) -> i32;
}

// proto: void QProgressDialog::setCancelButtonText(const QString & text);
impl<'a> /*trait*/ QProgressDialog_setCancelButtonText for (&'a  QString) {
  fn setCancelButtonText(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog19setCancelButtonTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QProgressDialog19setCancelButtonTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn sizeHint<T: QProgressDialog_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QProgressDialog_sizeHint {
  fn sizeHint(self, this: &mut QProgressDialog) -> i32;
}

// proto: QSize QProgressDialog::sizeHint();
impl<'a> /*trait*/ QProgressDialog_sizeHint for () {
  fn sizeHint(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog8sizeHintEv()};
    unsafe {_ZNK15QProgressDialog8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn labelText<T: QProgressDialog_labelText>(&mut self, value: T) -> i32 {
    value.labelText(self);
    return 1;
  }
}

pub trait QProgressDialog_labelText {
  fn labelText(self, this: &mut QProgressDialog) -> i32;
}

// proto: QString QProgressDialog::labelText();
impl<'a> /*trait*/ QProgressDialog_labelText for () {
  fn labelText(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog9labelTextEv()};
    unsafe {_ZNK15QProgressDialog9labelTextEv()};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setLabel<T: QProgressDialog_setLabel>(&mut self, value: T) -> i32 {
    value.setLabel(self);
    return 1;
  }
}

pub trait QProgressDialog_setLabel {
  fn setLabel(self, this: &mut QProgressDialog) -> i32;
}

// proto: void QProgressDialog::setLabel(QLabel * label);
impl<'a> /*trait*/ QProgressDialog_setLabel for (&'a mut QLabel) {
  fn setLabel(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog8setLabelEP6QLabel()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QProgressDialog8setLabelEP6QLabel(arg0)};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn metaObject<T: QProgressDialog_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QProgressDialog_metaObject {
  fn metaObject(self, this: &mut QProgressDialog) -> i32;
}

// proto: const QMetaObject * QProgressDialog::metaObject();
impl<'a> /*trait*/ QProgressDialog_metaObject for () {
  fn metaObject(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog10metaObjectEv()};
    unsafe {_ZNK15QProgressDialog10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setAutoReset<T: QProgressDialog_setAutoReset>(&mut self, value: T) -> i32 {
    value.setAutoReset(self);
    return 1;
  }
}

pub trait QProgressDialog_setAutoReset {
  fn setAutoReset(self, this: &mut QProgressDialog) -> i32;
}

// proto: void QProgressDialog::setAutoReset(bool reset);
impl<'a> /*trait*/ QProgressDialog_setAutoReset for (i8) {
  fn setAutoReset(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog12setAutoResetEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QProgressDialog12setAutoResetEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn value<T: QProgressDialog_value>(&mut self, value: T) -> i32 {
    value.value(self);
    return 1;
  }
}

pub trait QProgressDialog_value {
  fn value(self, this: &mut QProgressDialog) -> i32;
}

// proto: int QProgressDialog::value();
impl<'a> /*trait*/ QProgressDialog_value for () {
  fn value(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog5valueEv()};
    unsafe {_ZNK15QProgressDialog5valueEv()};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setCancelButton<T: QProgressDialog_setCancelButton>(&mut self, value: T) -> i32 {
    value.setCancelButton(self);
    return 1;
  }
}

pub trait QProgressDialog_setCancelButton {
  fn setCancelButton(self, this: &mut QProgressDialog) -> i32;
}

// proto: void QProgressDialog::setCancelButton(QPushButton * button);
impl<'a> /*trait*/ QProgressDialog_setCancelButton for (&'a mut QPushButton) {
  fn setCancelButton(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog15setCancelButtonEP11QPushButton()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QProgressDialog15setCancelButtonEP11QPushButton(arg0)};
    return 1;
  }
}

impl /*struct*/ QProgressDialog {
  pub fn setValue<T: QProgressDialog_setValue>(&mut self, value: T) -> i32 {
    value.setValue(self);
    return 1;
  }
}

pub trait QProgressDialog_setValue {
  fn setValue(self, this: &mut QProgressDialog) -> i32;
}

// proto: void QProgressDialog::setValue(int progress);
impl<'a> /*trait*/ QProgressDialog_setValue for (i32) {
  fn setValue(self, this: &mut QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog8setValueEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN15QProgressDialog8setValueEi(arg0)};
    return 1;
  }
}

