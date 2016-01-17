// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtWidgets/qprogressdialog.h
// dst-file: /src/widgets/qprogressdialog.rs
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
use super::super::core::qobject::QObject; // 771
use super::super::core::qstring::QString; // 771
use super::qwidget::QWidget; // 773
use super::qprogressbar::QProgressBar; // 773
use super::super::core::qsize::QSize; // 771
use super::qlabel::QLabel; // 773
use super::qpushbutton::QPushButton; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QProgressDialog_Class_Size() -> c_int;
  // proto:  void QProgressDialog::setAutoClose(bool close);
  fn _ZN15QProgressDialog12setAutoCloseEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QProgressDialog::open(QObject * receiver, const char * member);
  fn _ZN15QProgressDialog4openEP7QObjectPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char);
  // proto:  void QProgressDialog::setMaximum(int maximum);
  fn _ZN15QProgressDialog10setMaximumEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QProgressDialog::setMinimum(int minimum);
  fn _ZN15QProgressDialog10setMinimumEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QProgressDialog::setLabelText(const QString & text);
  fn _ZN15QProgressDialog12setLabelTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QProgressDialog::wasCanceled();
  fn _ZNK15QProgressDialog11wasCanceledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QProgressDialog::~QProgressDialog();
  fn _ZN15QProgressDialogD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QProgressDialog::minimumDuration();
  fn _ZNK15QProgressDialog15minimumDurationEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QProgressDialog::setMinimumDuration(int ms);
  fn _ZN15QProgressDialog18setMinimumDurationEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QProgressDialog::maximum();
  fn _ZNK15QProgressDialog7maximumEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QProgressDialog::setBar(QProgressBar * bar);
  fn _ZN15QProgressDialog6setBarEP12QProgressBar(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QProgressDialog::cancel();
  fn _ZN15QProgressDialog6cancelEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QProgressDialog::autoClose();
  fn _ZNK15QProgressDialog9autoCloseEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QProgressDialog::minimum();
  fn _ZNK15QProgressDialog7minimumEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QProgressDialog::autoReset();
  fn _ZNK15QProgressDialog9autoResetEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QProgressDialog::reset();
  fn _ZN15QProgressDialog5resetEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QProgressDialog::QProgressDialog(const QProgressDialog & );
  fn _ZN15QProgressDialogC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QProgressDialog::setRange(int minimum, int maximum);
  fn _ZN15QProgressDialog8setRangeEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QProgressDialog::setCancelButtonText(const QString & text);
  fn _ZN15QProgressDialog19setCancelButtonTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSize QProgressDialog::sizeHint();
  fn _ZNK15QProgressDialog8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QProgressDialog::labelText();
  fn _ZNK15QProgressDialog9labelTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QProgressDialog::setLabel(QLabel * label);
  fn _ZN15QProgressDialog8setLabelEP6QLabel(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QProgressDialog::metaObject();
  fn _ZNK15QProgressDialog10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QProgressDialog::setAutoReset(bool reset);
  fn _ZN15QProgressDialog12setAutoResetEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  int QProgressDialog::value();
  fn _ZNK15QProgressDialog5valueEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QProgressDialog::setCancelButton(QPushButton * button);
  fn _ZN15QProgressDialog15setCancelButtonEP11QPushButton(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QProgressDialog::setValue(int progress);
  fn _ZN15QProgressDialog8setValueEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  fn QProgressDialog_SlotProxy_connect__ZN15QProgressDialog8canceledEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QProgressDialog)=1
#[derive(Default)]
pub struct QProgressDialog {
  qbase: QDialog,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _canceled: QProgressDialog_canceled_signal,
}

impl /*struct*/ QProgressDialog {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QProgressDialog {
    return QProgressDialog{qbase: QDialog::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QProgressDialog {
  type Target = QDialog;

  fn deref(&self) -> &QDialog {
    return & self.qbase;
  }
}
impl AsRef<QDialog> for QProgressDialog {
  fn as_ref(& self) -> & QDialog {
    return & self.qbase;
  }
}
  // proto:  void QProgressDialog::setAutoClose(bool close);
impl /*struct*/ QProgressDialog {
  pub fn setAutoClose<RetType, T: QProgressDialog_setAutoClose<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoClose(self);
    // return 1;
  }
}

pub trait QProgressDialog_setAutoClose<RetType> {
  fn setAutoClose(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  void QProgressDialog::setAutoClose(bool close);
impl<'a> /*trait*/ QProgressDialog_setAutoClose<()> for (i8) {
  fn setAutoClose(self , rsthis: & QProgressDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog12setAutoCloseEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QProgressDialog12setAutoCloseEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProgressDialog::open(QObject * receiver, const char * member);
impl /*struct*/ QProgressDialog {
  pub fn open<RetType, T: QProgressDialog_open<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.open(self);
    // return 1;
  }
}

pub trait QProgressDialog_open<RetType> {
  fn open(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  void QProgressDialog::open(QObject * receiver, const char * member);
impl<'a> /*trait*/ QProgressDialog_open<()> for (&'a QObject, &'a  String) {
  fn open(self , rsthis: & QProgressDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog4openEP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {_ZN15QProgressDialog4openEP7QObjectPKc(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QProgressDialog::setMaximum(int maximum);
impl /*struct*/ QProgressDialog {
  pub fn setMaximum<RetType, T: QProgressDialog_setMaximum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximum(self);
    // return 1;
  }
}

pub trait QProgressDialog_setMaximum<RetType> {
  fn setMaximum(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  void QProgressDialog::setMaximum(int maximum);
impl<'a> /*trait*/ QProgressDialog_setMaximum<()> for (i32) {
  fn setMaximum(self , rsthis: & QProgressDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog10setMaximumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QProgressDialog10setMaximumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProgressDialog::setMinimum(int minimum);
impl /*struct*/ QProgressDialog {
  pub fn setMinimum<RetType, T: QProgressDialog_setMinimum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimum(self);
    // return 1;
  }
}

pub trait QProgressDialog_setMinimum<RetType> {
  fn setMinimum(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  void QProgressDialog::setMinimum(int minimum);
impl<'a> /*trait*/ QProgressDialog_setMinimum<()> for (i32) {
  fn setMinimum(self , rsthis: & QProgressDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog10setMinimumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QProgressDialog10setMinimumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProgressDialog::setLabelText(const QString & text);
impl /*struct*/ QProgressDialog {
  pub fn setLabelText<RetType, T: QProgressDialog_setLabelText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLabelText(self);
    // return 1;
  }
}

pub trait QProgressDialog_setLabelText<RetType> {
  fn setLabelText(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  void QProgressDialog::setLabelText(const QString & text);
impl<'a> /*trait*/ QProgressDialog_setLabelText<()> for (&'a QString) {
  fn setLabelText(self , rsthis: & QProgressDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog12setLabelTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QProgressDialog12setLabelTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QProgressDialog::wasCanceled();
impl /*struct*/ QProgressDialog {
  pub fn wasCanceled<RetType, T: QProgressDialog_wasCanceled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.wasCanceled(self);
    // return 1;
  }
}

pub trait QProgressDialog_wasCanceled<RetType> {
  fn wasCanceled(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  bool QProgressDialog::wasCanceled();
impl<'a> /*trait*/ QProgressDialog_wasCanceled<i8> for () {
  fn wasCanceled(self , rsthis: & QProgressDialog) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog11wasCanceledEv()};
    let mut ret = unsafe {_ZNK15QProgressDialog11wasCanceledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QProgressDialog::~QProgressDialog();
impl /*struct*/ QProgressDialog {
  pub fn free<RetType, T: QProgressDialog_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QProgressDialog_free<RetType> {
  fn free(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  void QProgressDialog::~QProgressDialog();
impl<'a> /*trait*/ QProgressDialog_free<()> for () {
  fn free(self , rsthis: & QProgressDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialogD2Ev()};
     unsafe {_ZN15QProgressDialogD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QProgressDialog::minimumDuration();
impl /*struct*/ QProgressDialog {
  pub fn minimumDuration<RetType, T: QProgressDialog_minimumDuration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumDuration(self);
    // return 1;
  }
}

pub trait QProgressDialog_minimumDuration<RetType> {
  fn minimumDuration(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  int QProgressDialog::minimumDuration();
impl<'a> /*trait*/ QProgressDialog_minimumDuration<i32> for () {
  fn minimumDuration(self , rsthis: & QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog15minimumDurationEv()};
    let mut ret = unsafe {_ZNK15QProgressDialog15minimumDurationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QProgressDialog::setMinimumDuration(int ms);
impl /*struct*/ QProgressDialog {
  pub fn setMinimumDuration<RetType, T: QProgressDialog_setMinimumDuration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimumDuration(self);
    // return 1;
  }
}

pub trait QProgressDialog_setMinimumDuration<RetType> {
  fn setMinimumDuration(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  void QProgressDialog::setMinimumDuration(int ms);
impl<'a> /*trait*/ QProgressDialog_setMinimumDuration<()> for (i32) {
  fn setMinimumDuration(self , rsthis: & QProgressDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog18setMinimumDurationEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QProgressDialog18setMinimumDurationEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QProgressDialog::maximum();
impl /*struct*/ QProgressDialog {
  pub fn maximum<RetType, T: QProgressDialog_maximum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximum(self);
    // return 1;
  }
}

pub trait QProgressDialog_maximum<RetType> {
  fn maximum(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  int QProgressDialog::maximum();
impl<'a> /*trait*/ QProgressDialog_maximum<i32> for () {
  fn maximum(self , rsthis: & QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog7maximumEv()};
    let mut ret = unsafe {_ZNK15QProgressDialog7maximumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QProgressDialog::setBar(QProgressBar * bar);
impl /*struct*/ QProgressDialog {
  pub fn setBar<RetType, T: QProgressDialog_setBar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBar(self);
    // return 1;
  }
}

pub trait QProgressDialog_setBar<RetType> {
  fn setBar(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  void QProgressDialog::setBar(QProgressBar * bar);
impl<'a> /*trait*/ QProgressDialog_setBar<()> for (&'a QProgressBar) {
  fn setBar(self , rsthis: & QProgressDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog6setBarEP12QProgressBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QProgressDialog6setBarEP12QProgressBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProgressDialog::cancel();
impl /*struct*/ QProgressDialog {
  pub fn cancel<RetType, T: QProgressDialog_cancel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cancel(self);
    // return 1;
  }
}

pub trait QProgressDialog_cancel<RetType> {
  fn cancel(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  void QProgressDialog::cancel();
impl<'a> /*trait*/ QProgressDialog_cancel<()> for () {
  fn cancel(self , rsthis: & QProgressDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog6cancelEv()};
     unsafe {_ZN15QProgressDialog6cancelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QProgressDialog::autoClose();
impl /*struct*/ QProgressDialog {
  pub fn autoClose<RetType, T: QProgressDialog_autoClose<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoClose(self);
    // return 1;
  }
}

pub trait QProgressDialog_autoClose<RetType> {
  fn autoClose(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  bool QProgressDialog::autoClose();
impl<'a> /*trait*/ QProgressDialog_autoClose<i8> for () {
  fn autoClose(self , rsthis: & QProgressDialog) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog9autoCloseEv()};
    let mut ret = unsafe {_ZNK15QProgressDialog9autoCloseEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QProgressDialog::minimum();
impl /*struct*/ QProgressDialog {
  pub fn minimum<RetType, T: QProgressDialog_minimum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimum(self);
    // return 1;
  }
}

pub trait QProgressDialog_minimum<RetType> {
  fn minimum(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  int QProgressDialog::minimum();
impl<'a> /*trait*/ QProgressDialog_minimum<i32> for () {
  fn minimum(self , rsthis: & QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog7minimumEv()};
    let mut ret = unsafe {_ZNK15QProgressDialog7minimumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QProgressDialog::autoReset();
impl /*struct*/ QProgressDialog {
  pub fn autoReset<RetType, T: QProgressDialog_autoReset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoReset(self);
    // return 1;
  }
}

pub trait QProgressDialog_autoReset<RetType> {
  fn autoReset(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  bool QProgressDialog::autoReset();
impl<'a> /*trait*/ QProgressDialog_autoReset<i8> for () {
  fn autoReset(self , rsthis: & QProgressDialog) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog9autoResetEv()};
    let mut ret = unsafe {_ZNK15QProgressDialog9autoResetEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QProgressDialog::reset();
impl /*struct*/ QProgressDialog {
  pub fn reset<RetType, T: QProgressDialog_reset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QProgressDialog_reset<RetType> {
  fn reset(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  void QProgressDialog::reset();
impl<'a> /*trait*/ QProgressDialog_reset<()> for () {
  fn reset(self , rsthis: & QProgressDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog5resetEv()};
     unsafe {_ZN15QProgressDialog5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QProgressDialog::QProgressDialog(const QProgressDialog & );
impl /*struct*/ QProgressDialog {
  pub fn new<T: QProgressDialog_new>(value: T) -> QProgressDialog {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QProgressDialog_new {
  fn new(self) -> QProgressDialog;
}

  // proto:  void QProgressDialog::QProgressDialog(const QProgressDialog & );
impl<'a> /*trait*/ QProgressDialog_new for (&'a QProgressDialog) {
  fn new(self) -> QProgressDialog {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialogC2ERKS_()};
    let ctysz: c_int = unsafe{QProgressDialog_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QProgressDialogC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QProgressDialog{qbase: QDialog::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QProgressDialog::setRange(int minimum, int maximum);
impl /*struct*/ QProgressDialog {
  pub fn setRange<RetType, T: QProgressDialog_setRange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRange(self);
    // return 1;
  }
}

pub trait QProgressDialog_setRange<RetType> {
  fn setRange(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  void QProgressDialog::setRange(int minimum, int maximum);
impl<'a> /*trait*/ QProgressDialog_setRange<()> for (i32, i32) {
  fn setRange(self , rsthis: & QProgressDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog8setRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN15QProgressDialog8setRangeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QProgressDialog::setCancelButtonText(const QString & text);
impl /*struct*/ QProgressDialog {
  pub fn setCancelButtonText<RetType, T: QProgressDialog_setCancelButtonText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCancelButtonText(self);
    // return 1;
  }
}

pub trait QProgressDialog_setCancelButtonText<RetType> {
  fn setCancelButtonText(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  void QProgressDialog::setCancelButtonText(const QString & text);
impl<'a> /*trait*/ QProgressDialog_setCancelButtonText<()> for (&'a QString) {
  fn setCancelButtonText(self , rsthis: & QProgressDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog19setCancelButtonTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QProgressDialog19setCancelButtonTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QProgressDialog::sizeHint();
impl /*struct*/ QProgressDialog {
  pub fn sizeHint<RetType, T: QProgressDialog_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QProgressDialog_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  QSize QProgressDialog::sizeHint();
impl<'a> /*trait*/ QProgressDialog_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QProgressDialog) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog8sizeHintEv()};
    let mut ret = unsafe {_ZNK15QProgressDialog8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QProgressDialog::labelText();
impl /*struct*/ QProgressDialog {
  pub fn labelText<RetType, T: QProgressDialog_labelText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.labelText(self);
    // return 1;
  }
}

pub trait QProgressDialog_labelText<RetType> {
  fn labelText(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  QString QProgressDialog::labelText();
impl<'a> /*trait*/ QProgressDialog_labelText<QString> for () {
  fn labelText(self , rsthis: & QProgressDialog) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog9labelTextEv()};
    let mut ret = unsafe {_ZNK15QProgressDialog9labelTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QProgressDialog::setLabel(QLabel * label);
impl /*struct*/ QProgressDialog {
  pub fn setLabel<RetType, T: QProgressDialog_setLabel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLabel(self);
    // return 1;
  }
}

pub trait QProgressDialog_setLabel<RetType> {
  fn setLabel(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  void QProgressDialog::setLabel(QLabel * label);
impl<'a> /*trait*/ QProgressDialog_setLabel<()> for (&'a QLabel) {
  fn setLabel(self , rsthis: & QProgressDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog8setLabelEP6QLabel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QProgressDialog8setLabelEP6QLabel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QProgressDialog::metaObject();
impl /*struct*/ QProgressDialog {
  pub fn metaObject<RetType, T: QProgressDialog_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QProgressDialog_metaObject<RetType> {
  fn metaObject(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  const QMetaObject * QProgressDialog::metaObject();
impl<'a> /*trait*/ QProgressDialog_metaObject<()> for () {
  fn metaObject(self , rsthis: & QProgressDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog10metaObjectEv()};
     unsafe {_ZNK15QProgressDialog10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QProgressDialog::setAutoReset(bool reset);
impl /*struct*/ QProgressDialog {
  pub fn setAutoReset<RetType, T: QProgressDialog_setAutoReset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoReset(self);
    // return 1;
  }
}

pub trait QProgressDialog_setAutoReset<RetType> {
  fn setAutoReset(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  void QProgressDialog::setAutoReset(bool reset);
impl<'a> /*trait*/ QProgressDialog_setAutoReset<()> for (i8) {
  fn setAutoReset(self , rsthis: & QProgressDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog12setAutoResetEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QProgressDialog12setAutoResetEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QProgressDialog::value();
impl /*struct*/ QProgressDialog {
  pub fn value<RetType, T: QProgressDialog_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QProgressDialog_value<RetType> {
  fn value(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  int QProgressDialog::value();
impl<'a> /*trait*/ QProgressDialog_value<i32> for () {
  fn value(self , rsthis: & QProgressDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QProgressDialog5valueEv()};
    let mut ret = unsafe {_ZNK15QProgressDialog5valueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QProgressDialog::setCancelButton(QPushButton * button);
impl /*struct*/ QProgressDialog {
  pub fn setCancelButton<RetType, T: QProgressDialog_setCancelButton<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCancelButton(self);
    // return 1;
  }
}

pub trait QProgressDialog_setCancelButton<RetType> {
  fn setCancelButton(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  void QProgressDialog::setCancelButton(QPushButton * button);
impl<'a> /*trait*/ QProgressDialog_setCancelButton<()> for (&'a QPushButton) {
  fn setCancelButton(self , rsthis: & QProgressDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog15setCancelButtonEP11QPushButton()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QProgressDialog15setCancelButtonEP11QPushButton(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProgressDialog::setValue(int progress);
impl /*struct*/ QProgressDialog {
  pub fn setValue<RetType, T: QProgressDialog_setValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setValue(self);
    // return 1;
  }
}

pub trait QProgressDialog_setValue<RetType> {
  fn setValue(self , rsthis: & QProgressDialog) -> RetType;
}

  // proto:  void QProgressDialog::setValue(int progress);
impl<'a> /*trait*/ QProgressDialog_setValue<()> for (i32) {
  fn setValue(self , rsthis: & QProgressDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QProgressDialog8setValueEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QProgressDialog8setValueEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QProgressDialog_canceled
pub struct QProgressDialog_canceled_signal{poi:u64}
impl /* struct */ QProgressDialog {
  pub fn canceled(&self) -> QProgressDialog_canceled_signal {
     return QProgressDialog_canceled_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QProgressDialog_canceled_signal {
  pub fn connect<T: QProgressDialog_canceled_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QProgressDialog_canceled_signal_connect {
  fn connect(self, sigthis: QProgressDialog_canceled_signal);
}

// canceled()
extern fn QProgressDialog_canceled_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QProgressDialog_canceled_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QProgressDialog_canceled_signal_connect for fn() {
  fn connect(self, sigthis: QProgressDialog_canceled_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QProgressDialog_canceled_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QProgressDialog_SlotProxy_connect__ZN15QProgressDialog8canceledEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QProgressDialog_canceled_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QProgressDialog_canceled_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QProgressDialog_canceled_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QProgressDialog_SlotProxy_connect__ZN15QProgressDialog8canceledEv(arg0, arg1, arg2)};
  }
}
// <= body block end

