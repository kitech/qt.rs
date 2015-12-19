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

// proto:  void QDialog::setExtension(QWidget * extension);
impl /*struct*/ QDialog {
  pub fn setExtension<RetType, T: QDialog_setExtension<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setExtension(self);
    // return 1;
  }
}

pub trait QDialog_setExtension<RetType> {
  fn setExtension(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  void QDialog::setExtension(QWidget * extension);
impl<'a> /*trait*/ QDialog_setExtension<()> for (&'a mut QWidget) {
  fn setExtension(self , rsthis: &mut QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog12setExtensionEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QDialog12setExtensionEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QDialog::result();
impl /*struct*/ QDialog {
  pub fn result<RetType, T: QDialog_result<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.result(self);
    // return 1;
  }
}

pub trait QDialog_result<RetType> {
  fn result(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  int QDialog::result();
impl<'a> /*trait*/ QDialog_result<i32> for () {
  fn result(self , rsthis: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog6resultEv()};
    let mut ret = unsafe {_ZNK7QDialog6resultEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QDialog::finished(int result);
impl /*struct*/ QDialog {
  pub fn finished<RetType, T: QDialog_finished<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.finished(self);
    // return 1;
  }
}

pub trait QDialog_finished<RetType> {
  fn finished(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  void QDialog::finished(int result);
impl<'a> /*trait*/ QDialog_finished<()> for (i32) {
  fn finished(self , rsthis: &mut QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog8finishedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QDialog8finishedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QDialog::done(int );
impl /*struct*/ QDialog {
  pub fn done<RetType, T: QDialog_done<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.done(self);
    // return 1;
  }
}

pub trait QDialog_done<RetType> {
  fn done(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  void QDialog::done(int );
impl<'a> /*trait*/ QDialog_done<()> for (i32) {
  fn done(self , rsthis: &mut QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog4doneEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QDialog4doneEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QDialog::open();
impl /*struct*/ QDialog {
  pub fn open<RetType, T: QDialog_open<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.open(self);
    // return 1;
  }
}

pub trait QDialog_open<RetType> {
  fn open(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  void QDialog::open();
impl<'a> /*trait*/ QDialog_open<()> for () {
  fn open(self , rsthis: &mut QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog4openEv()};
     unsafe {_ZN7QDialog4openEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QDialog::FreeQDialog();
impl /*struct*/ QDialog {
  pub fn FreeQDialog<RetType, T: QDialog_FreeQDialog<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQDialog(self);
    // return 1;
  }
}

pub trait QDialog_FreeQDialog<RetType> {
  fn FreeQDialog(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  void QDialog::FreeQDialog();
impl<'a> /*trait*/ QDialog_FreeQDialog<()> for () {
  fn FreeQDialog(self , rsthis: &mut QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialogD0Ev()};
     unsafe {_ZN7QDialogD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QDialog::setResult(int r);
impl /*struct*/ QDialog {
  pub fn setResult<RetType, T: QDialog_setResult<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setResult(self);
    // return 1;
  }
}

pub trait QDialog_setResult<RetType> {
  fn setResult(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  void QDialog::setResult(int r);
impl<'a> /*trait*/ QDialog_setResult<()> for (i32) {
  fn setResult(self , rsthis: &mut QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog9setResultEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QDialog9setResultEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QDialog::setSizeGripEnabled(bool );
impl /*struct*/ QDialog {
  pub fn setSizeGripEnabled<RetType, T: QDialog_setSizeGripEnabled<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setSizeGripEnabled(self);
    // return 1;
  }
}

pub trait QDialog_setSizeGripEnabled<RetType> {
  fn setSizeGripEnabled(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  void QDialog::setSizeGripEnabled(bool );
impl<'a> /*trait*/ QDialog_setSizeGripEnabled<()> for (i8) {
  fn setSizeGripEnabled(self , rsthis: &mut QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog18setSizeGripEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QDialog18setSizeGripEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QDialog::showExtension(bool );
impl /*struct*/ QDialog {
  pub fn showExtension<RetType, T: QDialog_showExtension<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.showExtension(self);
    // return 1;
  }
}

pub trait QDialog_showExtension<RetType> {
  fn showExtension(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  void QDialog::showExtension(bool );
impl<'a> /*trait*/ QDialog_showExtension<()> for (i8) {
  fn showExtension(self , rsthis: &mut QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog13showExtensionEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QDialog13showExtensionEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  const QMetaObject * QDialog::metaObject();
impl /*struct*/ QDialog {
  pub fn metaObject<RetType, T: QDialog_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDialog_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  const QMetaObject * QDialog::metaObject();
impl<'a> /*trait*/ QDialog_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog10metaObjectEv()};
     unsafe {_ZNK7QDialog10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QSize QDialog::minimumSizeHint();
impl /*struct*/ QDialog {
  pub fn minimumSizeHint<RetType, T: QDialog_minimumSizeHint<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QDialog_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  QSize QDialog::minimumSizeHint();
impl<'a> /*trait*/ QDialog_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: &mut QDialog) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK7QDialog15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QSize QDialog::sizeHint();
impl /*struct*/ QDialog {
  pub fn sizeHint<RetType, T: QDialog_sizeHint<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QDialog_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  QSize QDialog::sizeHint();
impl<'a> /*trait*/ QDialog_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QDialog) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog8sizeHintEv()};
    let mut ret = unsafe {_ZNK7QDialog8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QDialog::accept();
impl /*struct*/ QDialog {
  pub fn accept<RetType, T: QDialog_accept<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.accept(self);
    // return 1;
  }
}

pub trait QDialog_accept<RetType> {
  fn accept(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  void QDialog::accept();
impl<'a> /*trait*/ QDialog_accept<()> for () {
  fn accept(self , rsthis: &mut QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog6acceptEv()};
     unsafe {_ZN7QDialog6acceptEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QDialog::setVisible(bool visible);
impl /*struct*/ QDialog {
  pub fn setVisible<RetType, T: QDialog_setVisible<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QDialog_setVisible<RetType> {
  fn setVisible(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  void QDialog::setVisible(bool visible);
impl<'a> /*trait*/ QDialog_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: &mut QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog10setVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QDialog10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QWidget * QDialog::extension();
impl /*struct*/ QDialog {
  pub fn extension<RetType, T: QDialog_extension<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.extension(self);
    // return 1;
  }
}

pub trait QDialog_extension<RetType> {
  fn extension(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  QWidget * QDialog::extension();
impl<'a> /*trait*/ QDialog_extension<QWidget> for () {
  fn extension(self , rsthis: &mut QDialog) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog9extensionEv()};
    let mut ret = unsafe {_ZNK7QDialog9extensionEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QDialog::exec();
impl /*struct*/ QDialog {
  pub fn exec<RetType, T: QDialog_exec<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.exec(self);
    // return 1;
  }
}

pub trait QDialog_exec<RetType> {
  fn exec(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  int QDialog::exec();
impl<'a> /*trait*/ QDialog_exec<i32> for () {
  fn exec(self , rsthis: &mut QDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog4execEv()};
    let mut ret = unsafe {_ZN7QDialog4execEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QDialog::reject();
impl /*struct*/ QDialog {
  pub fn reject<RetType, T: QDialog_reject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.reject(self);
    // return 1;
  }
}

pub trait QDialog_reject<RetType> {
  fn reject(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  void QDialog::reject();
impl<'a> /*trait*/ QDialog_reject<()> for () {
  fn reject(self , rsthis: &mut QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog6rejectEv()};
     unsafe {_ZN7QDialog6rejectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QDialog::accepted();
impl /*struct*/ QDialog {
  pub fn accepted<RetType, T: QDialog_accepted<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.accepted(self);
    // return 1;
  }
}

pub trait QDialog_accepted<RetType> {
  fn accepted(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  void QDialog::accepted();
impl<'a> /*trait*/ QDialog_accepted<()> for () {
  fn accepted(self , rsthis: &mut QDialog) -> () {
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

// proto:  bool QDialog::isSizeGripEnabled();
impl /*struct*/ QDialog {
  pub fn isSizeGripEnabled<RetType, T: QDialog_isSizeGripEnabled<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isSizeGripEnabled(self);
    // return 1;
  }
}

pub trait QDialog_isSizeGripEnabled<RetType> {
  fn isSizeGripEnabled(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  bool QDialog::isSizeGripEnabled();
impl<'a> /*trait*/ QDialog_isSizeGripEnabled<i8> for () {
  fn isSizeGripEnabled(self , rsthis: &mut QDialog) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QDialog17isSizeGripEnabledEv()};
    let mut ret = unsafe {_ZNK7QDialog17isSizeGripEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QDialog::rejected();
impl /*struct*/ QDialog {
  pub fn rejected<RetType, T: QDialog_rejected<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.rejected(self);
    // return 1;
  }
}

pub trait QDialog_rejected<RetType> {
  fn rejected(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  void QDialog::rejected();
impl<'a> /*trait*/ QDialog_rejected<()> for () {
  fn rejected(self , rsthis: &mut QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog8rejectedEv()};
     unsafe {_ZN7QDialog8rejectedEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QDialog::setModal(bool modal);
impl /*struct*/ QDialog {
  pub fn setModal<RetType, T: QDialog_setModal<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setModal(self);
    // return 1;
  }
}

pub trait QDialog_setModal<RetType> {
  fn setModal(self , rsthis: &mut QDialog) -> RetType;
}

// proto:  void QDialog::setModal(bool modal);
impl<'a> /*trait*/ QDialog_setModal<()> for (i8) {
  fn setModal(self , rsthis: &mut QDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QDialog8setModalEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QDialog8setModalEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

