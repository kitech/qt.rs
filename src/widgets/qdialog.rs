// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
// src-file: /QtWidgets/qdialog.h
// dst-file: /src/widgets/qdialog.rs
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
use super::qwidget::QWidget; // 773
use std::ops::Deref;
use super::super::core::qsize::QSize; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QDialog_Class_Size() -> c_int;
  // proto:  void QDialog::setExtension(QWidget * extension);
  fn _ZN7QDialog12setExtensionEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QDialog::result();
  fn _ZNK7QDialog6resultEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QDialog::finished(int result);
  fn _ZN7QDialog8finishedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QDialog::done(int );
  fn _ZN7QDialog4doneEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QDialog::open();
  fn _ZN7QDialog4openEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDialog::~QDialog();
  fn _ZN7QDialogD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QDialog::setResult(int r);
  fn _ZN7QDialog9setResultEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QDialog::setSizeGripEnabled(bool );
  fn _ZN7QDialog18setSizeGripEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QDialog::showExtension(bool );
  fn _ZN7QDialog13showExtensionEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  const QMetaObject * QDialog::metaObject();
  fn _ZNK7QDialog10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QDialog::minimumSizeHint();
  fn _ZNK7QDialog15minimumSizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QDialog::sizeHint();
  fn _ZNK7QDialog8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDialog::accept();
  fn _ZN7QDialog6acceptEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDialog::setVisible(bool visible);
  fn _ZN7QDialog10setVisibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QWidget * QDialog::extension();
  fn _ZNK7QDialog9extensionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QDialog::exec();
  fn _ZN7QDialog4execEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QDialog::reject();
  fn _ZN7QDialog6rejectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDialog::accepted();
  fn _ZN7QDialog8acceptedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDialog::QDialog(const QDialog & );
  fn dector_ZN7QDialogC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN7QDialogC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QDialog::isSizeGripEnabled();
  fn _ZNK7QDialog17isSizeGripEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QDialog::rejected();
  fn _ZN7QDialog8rejectedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDialog::setModal(bool modal);
  fn _ZN7QDialog8setModalEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  fn QDialog_SlotProxy_connect__ZN7QDialog8rejectedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDialog_SlotProxy_connect__ZN7QDialog8finishedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDialog_SlotProxy_connect__ZN7QDialog8acceptedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QDialog)=1
#[derive(Default)]
pub struct QDialog {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _finished_1: QDialog_finished_signal,
  pub _accepted_1: QDialog_accepted_signal,
  pub _rejected_1: QDialog_rejected_signal,
}

impl /*struct*/ QDialog {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDialog {
    return QDialog{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QDialog {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QDialog {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  void QDialog::setExtension(QWidget * extension);
impl /*struct*/ QDialog {
  pub fn setExtension<RetType, T: QDialog_setExtension<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setExtension(self);
    // return 1;
  }
}

pub trait QDialog_setExtension<RetType> {
  fn setExtension(self , rsthis: & QDialog) -> RetType;
}

  // proto:  void QDialog::setExtension(QWidget * extension);
impl<'a> /*trait*/ QDialog_setExtension<()> for (&'a QWidget) {
  fn setExtension(self , rsthis: & QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog12setExtensionEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QDialog12setExtensionEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QDialog::result();
impl /*struct*/ QDialog {
  pub fn result<RetType, T: QDialog_result<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.result(self);
    // return 1;
  }
}

pub trait QDialog_result<RetType> {
  fn result(self , rsthis: & QDialog) -> RetType;
}

  // proto:  int QDialog::result();
impl<'a> /*trait*/ QDialog_result<i32> for () {
  fn result(self , rsthis: & QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog6resultEv()};
    let mut ret = unsafe {_ZNK7QDialog6resultEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QDialog::finished(int result);
impl /*struct*/ QDialog {
  pub fn finished<RetType, T: QDialog_finished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.finished(self);
    // return 1;
  }
}

pub trait QDialog_finished<RetType> {
  fn finished(self , rsthis: & QDialog) -> RetType;
}

  // proto:  void QDialog::finished(int result);
impl<'a> /*trait*/ QDialog_finished<()> for (i32) {
  fn finished(self , rsthis: & QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog8finishedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QDialog8finishedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDialog::done(int );
impl /*struct*/ QDialog {
  pub fn done<RetType, T: QDialog_done<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.done(self);
    // return 1;
  }
}

pub trait QDialog_done<RetType> {
  fn done(self , rsthis: & QDialog) -> RetType;
}

  // proto:  void QDialog::done(int );
impl<'a> /*trait*/ QDialog_done<()> for (i32) {
  fn done(self , rsthis: & QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog4doneEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QDialog4doneEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDialog::open();
impl /*struct*/ QDialog {
  pub fn open<RetType, T: QDialog_open<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.open(self);
    // return 1;
  }
}

pub trait QDialog_open<RetType> {
  fn open(self , rsthis: & QDialog) -> RetType;
}

  // proto:  void QDialog::open();
impl<'a> /*trait*/ QDialog_open<()> for () {
  fn open(self , rsthis: & QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog4openEv()};
     unsafe {_ZN7QDialog4openEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDialog::~QDialog();
impl /*struct*/ QDialog {
  pub fn free<RetType, T: QDialog_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QDialog_free<RetType> {
  fn free(self , rsthis: & QDialog) -> RetType;
}

  // proto:  void QDialog::~QDialog();
impl<'a> /*trait*/ QDialog_free<()> for () {
  fn free(self , rsthis: & QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialogD0Ev()};
     unsafe {_ZN7QDialogD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDialog::setResult(int r);
impl /*struct*/ QDialog {
  pub fn setResult<RetType, T: QDialog_setResult<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setResult(self);
    // return 1;
  }
}

pub trait QDialog_setResult<RetType> {
  fn setResult(self , rsthis: & QDialog) -> RetType;
}

  // proto:  void QDialog::setResult(int r);
impl<'a> /*trait*/ QDialog_setResult<()> for (i32) {
  fn setResult(self , rsthis: & QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog9setResultEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QDialog9setResultEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDialog::setSizeGripEnabled(bool );
impl /*struct*/ QDialog {
  pub fn setSizeGripEnabled<RetType, T: QDialog_setSizeGripEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSizeGripEnabled(self);
    // return 1;
  }
}

pub trait QDialog_setSizeGripEnabled<RetType> {
  fn setSizeGripEnabled(self , rsthis: & QDialog) -> RetType;
}

  // proto:  void QDialog::setSizeGripEnabled(bool );
impl<'a> /*trait*/ QDialog_setSizeGripEnabled<()> for (i8) {
  fn setSizeGripEnabled(self , rsthis: & QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog18setSizeGripEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN7QDialog18setSizeGripEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDialog::showExtension(bool );
impl /*struct*/ QDialog {
  pub fn showExtension<RetType, T: QDialog_showExtension<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showExtension(self);
    // return 1;
  }
}

pub trait QDialog_showExtension<RetType> {
  fn showExtension(self , rsthis: & QDialog) -> RetType;
}

  // proto:  void QDialog::showExtension(bool );
impl<'a> /*trait*/ QDialog_showExtension<()> for (i8) {
  fn showExtension(self , rsthis: & QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog13showExtensionEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN7QDialog13showExtensionEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QDialog::metaObject();
impl /*struct*/ QDialog {
  pub fn metaObject<RetType, T: QDialog_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDialog_metaObject<RetType> {
  fn metaObject(self , rsthis: & QDialog) -> RetType;
}

  // proto:  const QMetaObject * QDialog::metaObject();
impl<'a> /*trait*/ QDialog_metaObject<()> for () {
  fn metaObject(self , rsthis: & QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog10metaObjectEv()};
     unsafe {_ZNK7QDialog10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QDialog::minimumSizeHint();
impl /*struct*/ QDialog {
  pub fn minimumSizeHint<RetType, T: QDialog_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QDialog_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QDialog) -> RetType;
}

  // proto:  QSize QDialog::minimumSizeHint();
impl<'a> /*trait*/ QDialog_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QDialog) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK7QDialog15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QDialog::sizeHint();
impl /*struct*/ QDialog {
  pub fn sizeHint<RetType, T: QDialog_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QDialog_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QDialog) -> RetType;
}

  // proto:  QSize QDialog::sizeHint();
impl<'a> /*trait*/ QDialog_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QDialog) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog8sizeHintEv()};
    let mut ret = unsafe {_ZNK7QDialog8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDialog::accept();
impl /*struct*/ QDialog {
  pub fn accept<RetType, T: QDialog_accept<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.accept(self);
    // return 1;
  }
}

pub trait QDialog_accept<RetType> {
  fn accept(self , rsthis: & QDialog) -> RetType;
}

  // proto:  void QDialog::accept();
impl<'a> /*trait*/ QDialog_accept<()> for () {
  fn accept(self , rsthis: & QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog6acceptEv()};
     unsafe {_ZN7QDialog6acceptEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDialog::setVisible(bool visible);
impl /*struct*/ QDialog {
  pub fn setVisible<RetType, T: QDialog_setVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QDialog_setVisible<RetType> {
  fn setVisible(self , rsthis: & QDialog) -> RetType;
}

  // proto:  void QDialog::setVisible(bool visible);
impl<'a> /*trait*/ QDialog_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: & QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN7QDialog10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QDialog::extension();
impl /*struct*/ QDialog {
  pub fn extension<RetType, T: QDialog_extension<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.extension(self);
    // return 1;
  }
}

pub trait QDialog_extension<RetType> {
  fn extension(self , rsthis: & QDialog) -> RetType;
}

  // proto:  QWidget * QDialog::extension();
impl<'a> /*trait*/ QDialog_extension<QWidget> for () {
  fn extension(self , rsthis: & QDialog) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog9extensionEv()};
    let mut ret = unsafe {_ZNK7QDialog9extensionEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QDialog::exec();
impl /*struct*/ QDialog {
  pub fn exec<RetType, T: QDialog_exec<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.exec(self);
    // return 1;
  }
}

pub trait QDialog_exec<RetType> {
  fn exec(self , rsthis: & QDialog) -> RetType;
}

  // proto:  int QDialog::exec();
impl<'a> /*trait*/ QDialog_exec<i32> for () {
  fn exec(self , rsthis: & QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog4execEv()};
    let mut ret = unsafe {_ZN7QDialog4execEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QDialog::reject();
impl /*struct*/ QDialog {
  pub fn reject<RetType, T: QDialog_reject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reject(self);
    // return 1;
  }
}

pub trait QDialog_reject<RetType> {
  fn reject(self , rsthis: & QDialog) -> RetType;
}

  // proto:  void QDialog::reject();
impl<'a> /*trait*/ QDialog_reject<()> for () {
  fn reject(self , rsthis: & QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog6rejectEv()};
     unsafe {_ZN7QDialog6rejectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDialog::accepted();
impl /*struct*/ QDialog {
  pub fn accepted<RetType, T: QDialog_accepted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.accepted(self);
    // return 1;
  }
}

pub trait QDialog_accepted<RetType> {
  fn accepted(self , rsthis: & QDialog) -> RetType;
}

  // proto:  void QDialog::accepted();
impl<'a> /*trait*/ QDialog_accepted<()> for () {
  fn accepted(self , rsthis: & QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog8acceptedEv()};
     unsafe {_ZN7QDialog8acceptedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDialog::QDialog(const QDialog & );
impl /*struct*/ QDialog {
  pub fn new<T: QDialog_new>(value: T) -> QDialog {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QDialog_new {
  fn new(self) -> QDialog;
}

  // proto:  void QDialog::QDialog(const QDialog & );
impl<'a> /*trait*/ QDialog_new for (&'a QDialog) {
  fn new(self) -> QDialog {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialogC1ERKS_()};
    let ctysz: c_int = unsafe{QDialog_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN7QDialogC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN7QDialogC1ERKS_(arg0)} as u64;
    let rsthis = QDialog{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QDialog::isSizeGripEnabled();
impl /*struct*/ QDialog {
  pub fn isSizeGripEnabled<RetType, T: QDialog_isSizeGripEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSizeGripEnabled(self);
    // return 1;
  }
}

pub trait QDialog_isSizeGripEnabled<RetType> {
  fn isSizeGripEnabled(self , rsthis: & QDialog) -> RetType;
}

  // proto:  bool QDialog::isSizeGripEnabled();
impl<'a> /*trait*/ QDialog_isSizeGripEnabled<i8> for () {
  fn isSizeGripEnabled(self , rsthis: & QDialog) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog17isSizeGripEnabledEv()};
    let mut ret = unsafe {_ZNK7QDialog17isSizeGripEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QDialog::rejected();
impl /*struct*/ QDialog {
  pub fn rejected<RetType, T: QDialog_rejected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rejected(self);
    // return 1;
  }
}

pub trait QDialog_rejected<RetType> {
  fn rejected(self , rsthis: & QDialog) -> RetType;
}

  // proto:  void QDialog::rejected();
impl<'a> /*trait*/ QDialog_rejected<()> for () {
  fn rejected(self , rsthis: & QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog8rejectedEv()};
     unsafe {_ZN7QDialog8rejectedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDialog::setModal(bool modal);
impl /*struct*/ QDialog {
  pub fn setModal<RetType, T: QDialog_setModal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setModal(self);
    // return 1;
  }
}

pub trait QDialog_setModal<RetType> {
  fn setModal(self , rsthis: & QDialog) -> RetType;
}

  // proto:  void QDialog::setModal(bool modal);
impl<'a> /*trait*/ QDialog_setModal<()> for (i8) {
  fn setModal(self , rsthis: & QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog8setModalEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN7QDialog8setModalEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QDialog_finished
pub struct QDialog_finished_signal{poi:u64}
impl /* struct */ QDialog {
  pub fn finished_1(&self) -> QDialog_finished_signal {
     return QDialog_finished_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDialog_finished_signal {
  pub fn connect<T: QDialog_finished_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDialog_finished_signal_connect {
  fn connect(self, sigthis: QDialog_finished_signal);
}

#[derive(Default)] // for QDialog_accepted
pub struct QDialog_accepted_signal{poi:u64}
impl /* struct */ QDialog {
  pub fn accepted_1(&self) -> QDialog_accepted_signal {
     return QDialog_accepted_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDialog_accepted_signal {
  pub fn connect<T: QDialog_accepted_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDialog_accepted_signal_connect {
  fn connect(self, sigthis: QDialog_accepted_signal);
}

#[derive(Default)] // for QDialog_rejected
pub struct QDialog_rejected_signal{poi:u64}
impl /* struct */ QDialog {
  pub fn rejected_1(&self) -> QDialog_rejected_signal {
     return QDialog_rejected_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDialog_rejected_signal {
  pub fn connect<T: QDialog_rejected_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDialog_rejected_signal_connect {
  fn connect(self, sigthis: QDialog_rejected_signal);
}

// rejected()
extern fn QDialog_rejected_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QDialog_rejected_signal_connect_cb_box_0(rsfptr_raw:*mut Fn(), ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  rsfptr();
}
impl /* trait */ QDialog_rejected_signal_connect for fn() {
  fn connect(self, sigthis: QDialog_rejected_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDialog_rejected_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDialog_SlotProxy_connect__ZN7QDialog8rejectedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDialog_rejected_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QDialog_rejected_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDialog_rejected_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QDialog_SlotProxy_connect__ZN7QDialog8rejectedEv(arg0, arg1, arg2)};
  }
}
// finished(int)
extern fn QDialog_finished_signal_connect_cb_1(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QDialog_finished_signal_connect_cb_box_1(rsfptr_raw:*mut Fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QDialog_finished_signal_connect for fn(i32) {
  fn connect(self, sigthis: QDialog_finished_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDialog_finished_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDialog_SlotProxy_connect__ZN7QDialog8finishedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDialog_finished_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QDialog_finished_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDialog_finished_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QDialog_SlotProxy_connect__ZN7QDialog8finishedEi(arg0, arg1, arg2)};
  }
}
// accepted()
extern fn QDialog_accepted_signal_connect_cb_2(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QDialog_accepted_signal_connect_cb_box_2(rsfptr_raw:*mut Fn(), ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  rsfptr();
}
impl /* trait */ QDialog_accepted_signal_connect for fn() {
  fn connect(self, sigthis: QDialog_accepted_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDialog_accepted_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDialog_SlotProxy_connect__ZN7QDialog8acceptedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDialog_accepted_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QDialog_accepted_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDialog_accepted_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QDialog_SlotProxy_connect__ZN7QDialog8acceptedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

