// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QDialog::setExtension(QWidget * extension);
  fn _ZN7QDialog12setExtensionEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QDialog::result();
  fn _ZNK7QDialog6resultEv(qthis: *mut c_void) -> c_int;
  // proto:  void QDialog::finished(int result);
  fn _ZN7QDialog8finishedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QDialog::done(int );
  fn _ZN7QDialog4doneEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QDialog::open();
  fn _ZN7QDialog4openEv(qthis: *mut c_void) ;
  // proto:  void QDialog::FreeQDialog();
  fn _ZN7QDialogD0Ev(qthis: *mut c_void) ;
  // proto:  void QDialog::setResult(int r);
  fn _ZN7QDialog9setResultEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QDialog::setSizeGripEnabled(bool );
  fn _ZN7QDialog18setSizeGripEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QDialog::showExtension(bool );
  fn _ZN7QDialog13showExtensionEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  const QMetaObject * QDialog::metaObject();
  fn _ZNK7QDialog10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QSize QDialog::minimumSizeHint();
  fn _ZNK7QDialog15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QDialog::sizeHint();
  fn _ZNK7QDialog8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDialog::accept();
  fn _ZN7QDialog6acceptEv(qthis: *mut c_void) ;
  // proto:  void QDialog::setVisible(bool visible);
  fn _ZN7QDialog10setVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QWidget * QDialog::extension();
  fn _ZNK7QDialog9extensionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QDialog::exec();
  fn _ZN7QDialog4execEv(qthis: *mut c_void) -> c_int;
  // proto:  void QDialog::reject();
  fn _ZN7QDialog6rejectEv(qthis: *mut c_void) ;
  // proto:  void QDialog::accepted();
  fn _ZN7QDialog8acceptedEv(qthis: *mut c_void) ;
  // proto:  void QDialog::NewQDialog(const QDialog & );
  fn _ZN7QDialogC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QDialog::isSizeGripEnabled();
  fn _ZNK7QDialog17isSizeGripEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QDialog::rejected();
  fn _ZN7QDialog8rejectedEv(qthis: *mut c_void) ;
  // proto:  void QDialog::setModal(bool modal);
  fn _ZN7QDialog8setModalEb(qthis: *mut c_void, arg0: int8_t) ;
}

// body block begin
// class sizeof(QDialog)=1
pub struct QDialog {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDialog {
  pub fn setExtension<T: QDialog_setExtension>(&mut self, value: T)  {
     value.setExtension(self);
    // return 1;
  }
}

pub trait QDialog_setExtension {
  fn setExtension(self, rsthis: &mut QDialog) ;
}

// proto:  void QDialog::setExtension(QWidget * extension);
impl<'a> /*trait*/ QDialog_setExtension for (&'a mut QWidget) {
  fn setExtension(self, rsthis: &mut QDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog12setExtensionEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QDialog12setExtensionEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn result<T: QDialog_result>(&mut self, value: T) -> i32 {
    return value.result(self);
    // return 1;
  }
}

pub trait QDialog_result {
  fn result(self, rsthis: &mut QDialog) -> i32;
}

// proto:  int QDialog::result();
impl<'a> /*trait*/ QDialog_result for () {
  fn result(self, rsthis: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog6resultEv()};
    let mut ret = unsafe {_ZNK7QDialog6resultEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn finished<T: QDialog_finished>(&mut self, value: T)  {
     value.finished(self);
    // return 1;
  }
}

pub trait QDialog_finished {
  fn finished(self, rsthis: &mut QDialog) ;
}

// proto:  void QDialog::finished(int result);
impl<'a> /*trait*/ QDialog_finished for (i32) {
  fn finished(self, rsthis: &mut QDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog8finishedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QDialog8finishedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn done<T: QDialog_done>(&mut self, value: T)  {
     value.done(self);
    // return 1;
  }
}

pub trait QDialog_done {
  fn done(self, rsthis: &mut QDialog) ;
}

// proto:  void QDialog::done(int );
impl<'a> /*trait*/ QDialog_done for (i32) {
  fn done(self, rsthis: &mut QDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog4doneEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QDialog4doneEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn open<T: QDialog_open>(&mut self, value: T)  {
     value.open(self);
    // return 1;
  }
}

pub trait QDialog_open {
  fn open(self, rsthis: &mut QDialog) ;
}

// proto:  void QDialog::open();
impl<'a> /*trait*/ QDialog_open for () {
  fn open(self, rsthis: &mut QDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog4openEv()};
     unsafe {_ZN7QDialog4openEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn FreeQDialog<T: QDialog_FreeQDialog>(&mut self, value: T)  {
     value.FreeQDialog(self);
    // return 1;
  }
}

pub trait QDialog_FreeQDialog {
  fn FreeQDialog(self, rsthis: &mut QDialog) ;
}

// proto:  void QDialog::FreeQDialog();
impl<'a> /*trait*/ QDialog_FreeQDialog for () {
  fn FreeQDialog(self, rsthis: &mut QDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialogD0Ev()};
     unsafe {_ZN7QDialogD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn setResult<T: QDialog_setResult>(&mut self, value: T)  {
     value.setResult(self);
    // return 1;
  }
}

pub trait QDialog_setResult {
  fn setResult(self, rsthis: &mut QDialog) ;
}

// proto:  void QDialog::setResult(int r);
impl<'a> /*trait*/ QDialog_setResult for (i32) {
  fn setResult(self, rsthis: &mut QDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog9setResultEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QDialog9setResultEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn setSizeGripEnabled<T: QDialog_setSizeGripEnabled>(&mut self, value: T)  {
     value.setSizeGripEnabled(self);
    // return 1;
  }
}

pub trait QDialog_setSizeGripEnabled {
  fn setSizeGripEnabled(self, rsthis: &mut QDialog) ;
}

// proto:  void QDialog::setSizeGripEnabled(bool );
impl<'a> /*trait*/ QDialog_setSizeGripEnabled for (i8) {
  fn setSizeGripEnabled(self, rsthis: &mut QDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog18setSizeGripEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QDialog18setSizeGripEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn showExtension<T: QDialog_showExtension>(&mut self, value: T)  {
     value.showExtension(self);
    // return 1;
  }
}

pub trait QDialog_showExtension {
  fn showExtension(self, rsthis: &mut QDialog) ;
}

// proto:  void QDialog::showExtension(bool );
impl<'a> /*trait*/ QDialog_showExtension for (i8) {
  fn showExtension(self, rsthis: &mut QDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog13showExtensionEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QDialog13showExtensionEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn metaObject<T: QDialog_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QDialog_metaObject {
  fn metaObject(self, rsthis: &mut QDialog) ;
}

// proto:  const QMetaObject * QDialog::metaObject();
impl<'a> /*trait*/ QDialog_metaObject for () {
  fn metaObject(self, rsthis: &mut QDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog10metaObjectEv()};
     unsafe {_ZNK7QDialog10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn minimumSizeHint<T: QDialog_minimumSizeHint>(&mut self, value: T) -> QSize {
    return value.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QDialog_minimumSizeHint {
  fn minimumSizeHint(self, rsthis: &mut QDialog) -> QSize;
}

// proto:  QSize QDialog::minimumSizeHint();
impl<'a> /*trait*/ QDialog_minimumSizeHint for () {
  fn minimumSizeHint(self, rsthis: &mut QDialog) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK7QDialog15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn sizeHint<T: QDialog_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QDialog_sizeHint {
  fn sizeHint(self, rsthis: &mut QDialog) -> QSize;
}

// proto:  QSize QDialog::sizeHint();
impl<'a> /*trait*/ QDialog_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QDialog) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog8sizeHintEv()};
    let mut ret = unsafe {_ZNK7QDialog8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn accept<T: QDialog_accept>(&mut self, value: T)  {
     value.accept(self);
    // return 1;
  }
}

pub trait QDialog_accept {
  fn accept(self, rsthis: &mut QDialog) ;
}

// proto:  void QDialog::accept();
impl<'a> /*trait*/ QDialog_accept for () {
  fn accept(self, rsthis: &mut QDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog6acceptEv()};
     unsafe {_ZN7QDialog6acceptEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn setVisible<T: QDialog_setVisible>(&mut self, value: T)  {
     value.setVisible(self);
    // return 1;
  }
}

pub trait QDialog_setVisible {
  fn setVisible(self, rsthis: &mut QDialog) ;
}

// proto:  void QDialog::setVisible(bool visible);
impl<'a> /*trait*/ QDialog_setVisible for (i8) {
  fn setVisible(self, rsthis: &mut QDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog10setVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QDialog10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn extension<T: QDialog_extension>(&mut self, value: T) -> QWidget {
    return value.extension(self);
    // return 1;
  }
}

pub trait QDialog_extension {
  fn extension(self, rsthis: &mut QDialog) -> QWidget;
}

// proto:  QWidget * QDialog::extension();
impl<'a> /*trait*/ QDialog_extension for () {
  fn extension(self, rsthis: &mut QDialog) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog9extensionEv()};
    let mut ret = unsafe {_ZNK7QDialog9extensionEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn exec<T: QDialog_exec>(&mut self, value: T) -> i32 {
    return value.exec(self);
    // return 1;
  }
}

pub trait QDialog_exec {
  fn exec(self, rsthis: &mut QDialog) -> i32;
}

// proto:  int QDialog::exec();
impl<'a> /*trait*/ QDialog_exec for () {
  fn exec(self, rsthis: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog4execEv()};
    let mut ret = unsafe {_ZN7QDialog4execEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn reject<T: QDialog_reject>(&mut self, value: T)  {
     value.reject(self);
    // return 1;
  }
}

pub trait QDialog_reject {
  fn reject(self, rsthis: &mut QDialog) ;
}

// proto:  void QDialog::reject();
impl<'a> /*trait*/ QDialog_reject for () {
  fn reject(self, rsthis: &mut QDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog6rejectEv()};
     unsafe {_ZN7QDialog6rejectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn accepted<T: QDialog_accepted>(&mut self, value: T)  {
     value.accepted(self);
    // return 1;
  }
}

pub trait QDialog_accepted {
  fn accepted(self, rsthis: &mut QDialog) ;
}

// proto:  void QDialog::accepted();
impl<'a> /*trait*/ QDialog_accepted for () {
  fn accepted(self, rsthis: &mut QDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog8acceptedEv()};
     unsafe {_ZN7QDialog8acceptedEv(rsthis.qclsinst)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QDialogC1ERKS_(qthis, arg0)};
    let rsthis = QDialog{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn isSizeGripEnabled<T: QDialog_isSizeGripEnabled>(&mut self, value: T) -> i8 {
    return value.isSizeGripEnabled(self);
    // return 1;
  }
}

pub trait QDialog_isSizeGripEnabled {
  fn isSizeGripEnabled(self, rsthis: &mut QDialog) -> i8;
}

// proto:  bool QDialog::isSizeGripEnabled();
impl<'a> /*trait*/ QDialog_isSizeGripEnabled for () {
  fn isSizeGripEnabled(self, rsthis: &mut QDialog) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog17isSizeGripEnabledEv()};
    let mut ret = unsafe {_ZNK7QDialog17isSizeGripEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn rejected<T: QDialog_rejected>(&mut self, value: T)  {
     value.rejected(self);
    // return 1;
  }
}

pub trait QDialog_rejected {
  fn rejected(self, rsthis: &mut QDialog) ;
}

// proto:  void QDialog::rejected();
impl<'a> /*trait*/ QDialog_rejected for () {
  fn rejected(self, rsthis: &mut QDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog8rejectedEv()};
     unsafe {_ZN7QDialog8rejectedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDialog {
  pub fn setModal<T: QDialog_setModal>(&mut self, value: T)  {
     value.setModal(self);
    // return 1;
  }
}

pub trait QDialog_setModal {
  fn setModal(self, rsthis: &mut QDialog) ;
}

// proto:  void QDialog::setModal(bool modal);
impl<'a> /*trait*/ QDialog_setModal for (i8) {
  fn setModal(self, rsthis: &mut QDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog8setModalEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QDialog8setModalEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

