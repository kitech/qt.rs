// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QDialog::setExtension(QWidget * extension);
  fn _ZN7QDialog12setExtensionEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: int QDialog::result();
  fn _ZNK7QDialog6resultEv() -> i32;
  // proto: void QDialog::finished(int result);
  fn _ZN7QDialog8finishedEi(arg0: c_int) -> i32;
  // proto: void QDialog::done(int );
  fn _ZN7QDialog4doneEi(arg0: c_int) -> i32;
  // proto: void QDialog::open();
  fn _ZN7QDialog4openEv() -> i32;
  // proto: void QDialog::FreeQDialog();
  fn _ZN7QDialogD0Ev() -> i32;
  // proto: void QDialog::setResult(int r);
  fn _ZN7QDialog9setResultEi(arg0: c_int) -> i32;
  // proto: void QDialog::setSizeGripEnabled(bool );
  fn _ZN7QDialog18setSizeGripEnabledEb(arg0: int8_t) -> i32;
  // proto: void QDialog::showExtension(bool );
  fn _ZN7QDialog13showExtensionEb(arg0: int8_t) -> i32;
  // proto: const QMetaObject * QDialog::metaObject();
  fn _ZNK7QDialog10metaObjectEv() -> i32;
  // proto: QSize QDialog::minimumSizeHint();
  fn _ZNK7QDialog15minimumSizeHintEv() -> i32;
  // proto: QSize QDialog::sizeHint();
  fn _ZNK7QDialog8sizeHintEv() -> i32;
  // proto: void QDialog::accept();
  fn _ZN7QDialog6acceptEv() -> i32;
  // proto: void QDialog::setVisible(bool visible);
  fn _ZN7QDialog10setVisibleEb(arg0: int8_t) -> i32;
  // proto: QWidget * QDialog::extension();
  fn _ZNK7QDialog9extensionEv() -> i32;
  // proto: int QDialog::exec();
  fn _ZN7QDialog4execEv() -> i32;
  // proto: void QDialog::reject();
  fn _ZN7QDialog6rejectEv() -> i32;
  // proto: void QDialog::accepted();
  fn _ZN7QDialog8acceptedEv() -> i32;
  // proto: void QDialog::NewQDialog(const QDialog & );
  fn _ZN7QDialogC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QDialog::isSizeGripEnabled();
  fn _ZNK7QDialog17isSizeGripEnabledEv() -> i32;
  // proto: void QDialog::rejected();
  fn _ZN7QDialog8rejectedEv() -> i32;
  // proto: void QDialog::setModal(bool modal);
  fn _ZN7QDialog8setModalEb(arg0: int8_t) -> i32;
}

// body block begin
// class sizeof(QDialog)=1
pub struct QDialog {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDialog {
  pub fn setExtension<T: QDialog_setExtension>(&mut self, value: T) -> i32 {
    value.setExtension(self);
    return 1;
  }
}

pub trait QDialog_setExtension {
  fn setExtension(self, this: &mut QDialog) -> i32;
}

// proto: void QDialog::setExtension(QWidget * extension);
impl<'a> /*trait*/ QDialog_setExtension for (&'a mut QWidget) {
  fn setExtension(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog12setExtensionEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QDialog12setExtensionEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn result<T: QDialog_result>(&mut self, value: T) -> i32 {
    value.result(self);
    return 1;
  }
}

pub trait QDialog_result {
  fn result(self, this: &mut QDialog) -> i32;
}

// proto: int QDialog::result();
impl<'a> /*trait*/ QDialog_result for () {
  fn result(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog6resultEv()};
    unsafe {_ZNK7QDialog6resultEv()};
    return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn finished<T: QDialog_finished>(&mut self, value: T) -> i32 {
    value.finished(self);
    return 1;
  }
}

pub trait QDialog_finished {
  fn finished(self, this: &mut QDialog) -> i32;
}

// proto: void QDialog::finished(int result);
impl<'a> /*trait*/ QDialog_finished for (i32) {
  fn finished(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog8finishedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QDialog8finishedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn done<T: QDialog_done>(&mut self, value: T) -> i32 {
    value.done(self);
    return 1;
  }
}

pub trait QDialog_done {
  fn done(self, this: &mut QDialog) -> i32;
}

// proto: void QDialog::done(int );
impl<'a> /*trait*/ QDialog_done for (i32) {
  fn done(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog4doneEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QDialog4doneEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn open<T: QDialog_open>(&mut self, value: T) -> i32 {
    value.open(self);
    return 1;
  }
}

pub trait QDialog_open {
  fn open(self, this: &mut QDialog) -> i32;
}

// proto: void QDialog::open();
impl<'a> /*trait*/ QDialog_open for () {
  fn open(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog4openEv()};
    unsafe {_ZN7QDialog4openEv()};
    return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn FreeQDialog<T: QDialog_FreeQDialog>(&mut self, value: T) -> i32 {
    value.FreeQDialog(self);
    return 1;
  }
}

pub trait QDialog_FreeQDialog {
  fn FreeQDialog(self, this: &mut QDialog) -> i32;
}

// proto: void QDialog::FreeQDialog();
impl<'a> /*trait*/ QDialog_FreeQDialog for () {
  fn FreeQDialog(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialogD0Ev()};
    unsafe {_ZN7QDialogD0Ev()};
    return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn setResult<T: QDialog_setResult>(&mut self, value: T) -> i32 {
    value.setResult(self);
    return 1;
  }
}

pub trait QDialog_setResult {
  fn setResult(self, this: &mut QDialog) -> i32;
}

// proto: void QDialog::setResult(int r);
impl<'a> /*trait*/ QDialog_setResult for (i32) {
  fn setResult(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog9setResultEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QDialog9setResultEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn setSizeGripEnabled<T: QDialog_setSizeGripEnabled>(&mut self, value: T) -> i32 {
    value.setSizeGripEnabled(self);
    return 1;
  }
}

pub trait QDialog_setSizeGripEnabled {
  fn setSizeGripEnabled(self, this: &mut QDialog) -> i32;
}

// proto: void QDialog::setSizeGripEnabled(bool );
impl<'a> /*trait*/ QDialog_setSizeGripEnabled for (i8) {
  fn setSizeGripEnabled(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog18setSizeGripEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QDialog18setSizeGripEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn showExtension<T: QDialog_showExtension>(&mut self, value: T) -> i32 {
    value.showExtension(self);
    return 1;
  }
}

pub trait QDialog_showExtension {
  fn showExtension(self, this: &mut QDialog) -> i32;
}

// proto: void QDialog::showExtension(bool );
impl<'a> /*trait*/ QDialog_showExtension for (i8) {
  fn showExtension(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog13showExtensionEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QDialog13showExtensionEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn metaObject<T: QDialog_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QDialog_metaObject {
  fn metaObject(self, this: &mut QDialog) -> i32;
}

// proto: const QMetaObject * QDialog::metaObject();
impl<'a> /*trait*/ QDialog_metaObject for () {
  fn metaObject(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog10metaObjectEv()};
    unsafe {_ZNK7QDialog10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn minimumSizeHint<T: QDialog_minimumSizeHint>(&mut self, value: T) -> i32 {
    value.minimumSizeHint(self);
    return 1;
  }
}

pub trait QDialog_minimumSizeHint {
  fn minimumSizeHint(self, this: &mut QDialog) -> i32;
}

// proto: QSize QDialog::minimumSizeHint();
impl<'a> /*trait*/ QDialog_minimumSizeHint for () {
  fn minimumSizeHint(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog15minimumSizeHintEv()};
    unsafe {_ZNK7QDialog15minimumSizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn sizeHint<T: QDialog_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QDialog_sizeHint {
  fn sizeHint(self, this: &mut QDialog) -> i32;
}

// proto: QSize QDialog::sizeHint();
impl<'a> /*trait*/ QDialog_sizeHint for () {
  fn sizeHint(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog8sizeHintEv()};
    unsafe {_ZNK7QDialog8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn accept<T: QDialog_accept>(&mut self, value: T) -> i32 {
    value.accept(self);
    return 1;
  }
}

pub trait QDialog_accept {
  fn accept(self, this: &mut QDialog) -> i32;
}

// proto: void QDialog::accept();
impl<'a> /*trait*/ QDialog_accept for () {
  fn accept(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog6acceptEv()};
    unsafe {_ZN7QDialog6acceptEv()};
    return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn setVisible<T: QDialog_setVisible>(&mut self, value: T) -> i32 {
    value.setVisible(self);
    return 1;
  }
}

pub trait QDialog_setVisible {
  fn setVisible(self, this: &mut QDialog) -> i32;
}

// proto: void QDialog::setVisible(bool visible);
impl<'a> /*trait*/ QDialog_setVisible for (i8) {
  fn setVisible(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog10setVisibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QDialog10setVisibleEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn extension<T: QDialog_extension>(&mut self, value: T) -> i32 {
    value.extension(self);
    return 1;
  }
}

pub trait QDialog_extension {
  fn extension(self, this: &mut QDialog) -> i32;
}

// proto: QWidget * QDialog::extension();
impl<'a> /*trait*/ QDialog_extension for () {
  fn extension(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog9extensionEv()};
    unsafe {_ZNK7QDialog9extensionEv()};
    return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn exec<T: QDialog_exec>(&mut self, value: T) -> i32 {
    value.exec(self);
    return 1;
  }
}

pub trait QDialog_exec {
  fn exec(self, this: &mut QDialog) -> i32;
}

// proto: int QDialog::exec();
impl<'a> /*trait*/ QDialog_exec for () {
  fn exec(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog4execEv()};
    unsafe {_ZN7QDialog4execEv()};
    return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn reject<T: QDialog_reject>(&mut self, value: T) -> i32 {
    value.reject(self);
    return 1;
  }
}

pub trait QDialog_reject {
  fn reject(self, this: &mut QDialog) -> i32;
}

// proto: void QDialog::reject();
impl<'a> /*trait*/ QDialog_reject for () {
  fn reject(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog6rejectEv()};
    unsafe {_ZN7QDialog6rejectEv()};
    return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn accepted<T: QDialog_accepted>(&mut self, value: T) -> i32 {
    value.accepted(self);
    return 1;
  }
}

pub trait QDialog_accepted {
  fn accepted(self, this: &mut QDialog) -> i32;
}

// proto: void QDialog::accepted();
impl<'a> /*trait*/ QDialog_accepted for () {
  fn accepted(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog8acceptedEv()};
    unsafe {_ZN7QDialog8acceptedEv()};
    return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn NewQDialog<T: QDialog_NewQDialog>(value: T) -> QDialog {
    let rsthis = value.NewQDialog();
    return rsthis;
    // return 1;
  }
}

pub trait QDialog_NewQDialog {
  fn NewQDialog(self) -> QDialog;
}

// proto: void QDialog::NewQDialog(const QDialog & );
impl<'a> /*trait*/ QDialog_NewQDialog for (&'a  QDialog) {
  fn NewQDialog(self) -> QDialog {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialogC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QDialogC1ERKS_(qthis, arg0)};
    let rsthis = QDialog{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn isSizeGripEnabled<T: QDialog_isSizeGripEnabled>(&mut self, value: T) -> i32 {
    value.isSizeGripEnabled(self);
    return 1;
  }
}

pub trait QDialog_isSizeGripEnabled {
  fn isSizeGripEnabled(self, this: &mut QDialog) -> i32;
}

// proto: bool QDialog::isSizeGripEnabled();
impl<'a> /*trait*/ QDialog_isSizeGripEnabled for () {
  fn isSizeGripEnabled(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog17isSizeGripEnabledEv()};
    unsafe {_ZNK7QDialog17isSizeGripEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn rejected<T: QDialog_rejected>(&mut self, value: T) -> i32 {
    value.rejected(self);
    return 1;
  }
}

pub trait QDialog_rejected {
  fn rejected(self, this: &mut QDialog) -> i32;
}

// proto: void QDialog::rejected();
impl<'a> /*trait*/ QDialog_rejected for () {
  fn rejected(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog8rejectedEv()};
    unsafe {_ZN7QDialog8rejectedEv()};
    return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn setModal<T: QDialog_setModal>(&mut self, value: T) -> i32 {
    value.setModal(self);
    return 1;
  }
}

pub trait QDialog_setModal {
  fn setModal(self, this: &mut QDialog) -> i32;
}

// proto: void QDialog::setModal(bool modal);
impl<'a> /*trait*/ QDialog_setModal for (i8) {
  fn setModal(self, this: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog8setModalEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QDialog8setModalEb(arg0)};
    return 1;
  }
}

