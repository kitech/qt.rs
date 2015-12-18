// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qfont::QFont;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QFontDialog::NewQFontDialog(QWidget * parent);
  fn _ZN11QFontDialogC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFontDialog::NewQFontDialog(const QFontDialog & );
  fn _ZN11QFontDialogC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFontDialog::currentFontChanged(const QFont & font);
  fn _ZN11QFontDialog18currentFontChangedERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFontDialog::open(QObject * receiver, const char * member);
  fn _ZN11QFontDialog4openEP7QObjectPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char) ;
  // proto:  void QFontDialog::NewQFontDialog(const QFont & initial, QWidget * parent);
  fn _ZN11QFontDialogC1ERK5QFontP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QFont QFontDialog::currentFont();
  fn _ZNK11QFontDialog11currentFontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFontDialog::setVisible(bool visible);
  fn _ZN11QFontDialog10setVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QFontDialog::FreeQFontDialog();
  fn _ZN11QFontDialogD0Ev(qthis: *mut c_void) ;
  // proto:  void QFontDialog::fontSelected(const QFont & font);
  fn _ZN11QFontDialog12fontSelectedERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QFont QFontDialog::getFont(bool * ok, QWidget * parent);
  fn _ZN11QFontDialog7getFontEPbP7QWidget(arg0: *mut int8_t, arg1: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QFontDialog::metaObject();
  fn _ZNK11QFontDialog10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QFont QFontDialog::selectedFont();
  fn _ZNK11QFontDialog12selectedFontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFontDialog::setCurrentFont(const QFont & font);
  fn _ZN11QFontDialog14setCurrentFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QFontDialog)=1
pub struct QFontDialog {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFontDialog {
  pub fn NewQFontDialog<T: QFontDialog_NewQFontDialog>(value: T) -> QFontDialog {
    let rsthis = value.NewQFontDialog();
    return rsthis;
    // return 1;
  }
}

pub trait QFontDialog_NewQFontDialog {
  fn NewQFontDialog(self) -> QFontDialog;
}

// proto: void QFontDialog::NewQFontDialog(QWidget * parent);
impl<'a> /*trait*/ QFontDialog_NewQFontDialog for (&'a mut QWidget) {
  fn NewQFontDialog(self) -> QFontDialog {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialogC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QFontDialogC1EP7QWidget(qthis, arg0)};
    let rsthis = QFontDialog{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QFontDialog::NewQFontDialog(const QFontDialog & );
impl<'a> /*trait*/ QFontDialog_NewQFontDialog for (&'a  QFontDialog) {
  fn NewQFontDialog(self) -> QFontDialog {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialogC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QFontDialogC1ERKS_(qthis, arg0)};
    let rsthis = QFontDialog{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFontDialog {
  pub fn currentFontChanged<RetType, T: QFontDialog_currentFontChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.currentFontChanged(self);
    // return 1;
  }
}

pub trait QFontDialog_currentFontChanged<RetType> {
  fn currentFontChanged(self, rsthis: &mut QFontDialog) -> RetType;
}

// proto:  void QFontDialog::currentFontChanged(const QFont & font);
impl<'a> /*trait*/ QFontDialog_currentFontChanged<()> for (&'a  QFont) {
  fn currentFontChanged(self, rsthis: &mut QFontDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialog18currentFontChangedERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFontDialog18currentFontChangedERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFontDialog {
  pub fn open<RetType, T: QFontDialog_open<RetType>>(&mut self, value: T) -> RetType {
    return value.open(self);
    // return 1;
  }
}

pub trait QFontDialog_open<RetType> {
  fn open(self, rsthis: &mut QFontDialog) -> RetType;
}

// proto:  void QFontDialog::open(QObject * receiver, const char * member);
impl<'a> /*trait*/ QFontDialog_open<()> for (&'a mut QObject, &'a  String) {
  fn open(self, rsthis: &mut QFontDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialog4openEP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
     unsafe {_ZN11QFontDialog4openEP7QObjectPKc(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto: void QFontDialog::NewQFontDialog(const QFont & initial, QWidget * parent);
impl<'a> /*trait*/ QFontDialog_NewQFontDialog for (&'a  QFont, &'a mut QWidget) {
  fn NewQFontDialog(self) -> QFontDialog {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialogC1ERK5QFontP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN11QFontDialogC1ERK5QFontP7QWidget(qthis, arg0, arg1)};
    let rsthis = QFontDialog{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFontDialog {
  pub fn currentFont<RetType, T: QFontDialog_currentFont<RetType>>(&mut self, value: T) -> RetType {
    return value.currentFont(self);
    // return 1;
  }
}

pub trait QFontDialog_currentFont<RetType> {
  fn currentFont(self, rsthis: &mut QFontDialog) -> RetType;
}

// proto:  QFont QFontDialog::currentFont();
impl<'a> /*trait*/ QFontDialog_currentFont<QFont> for () {
  fn currentFont(self, rsthis: &mut QFontDialog) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFontDialog11currentFontEv()};
    let mut ret = unsafe {_ZNK11QFontDialog11currentFontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFontDialog {
  pub fn setVisible<RetType, T: QFontDialog_setVisible<RetType>>(&mut self, value: T) -> RetType {
    return value.setVisible(self);
    // return 1;
  }
}

pub trait QFontDialog_setVisible<RetType> {
  fn setVisible(self, rsthis: &mut QFontDialog) -> RetType;
}

// proto:  void QFontDialog::setVisible(bool visible);
impl<'a> /*trait*/ QFontDialog_setVisible<()> for (i8) {
  fn setVisible(self, rsthis: &mut QFontDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialog10setVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QFontDialog10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFontDialog {
  pub fn FreeQFontDialog<RetType, T: QFontDialog_FreeQFontDialog<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQFontDialog(self);
    // return 1;
  }
}

pub trait QFontDialog_FreeQFontDialog<RetType> {
  fn FreeQFontDialog(self, rsthis: &mut QFontDialog) -> RetType;
}

// proto:  void QFontDialog::FreeQFontDialog();
impl<'a> /*trait*/ QFontDialog_FreeQFontDialog<()> for () {
  fn FreeQFontDialog(self, rsthis: &mut QFontDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialogD0Ev()};
     unsafe {_ZN11QFontDialogD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFontDialog {
  pub fn fontSelected<RetType, T: QFontDialog_fontSelected<RetType>>(&mut self, value: T) -> RetType {
    return value.fontSelected(self);
    // return 1;
  }
}

pub trait QFontDialog_fontSelected<RetType> {
  fn fontSelected(self, rsthis: &mut QFontDialog) -> RetType;
}

// proto:  void QFontDialog::fontSelected(const QFont & font);
impl<'a> /*trait*/ QFontDialog_fontSelected<()> for (&'a  QFont) {
  fn fontSelected(self, rsthis: &mut QFontDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialog12fontSelectedERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFontDialog12fontSelectedERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFontDialog {
  pub fn getFont<RetType, T: QFontDialog_getFont<RetType>>(&mut self, value: T) -> RetType {
    return value.getFont(self);
    // return 1;
  }
}

pub trait QFontDialog_getFont<RetType> {
  fn getFont(self, rsthis: &mut QFontDialog) -> RetType;
}

// proto: static QFont QFontDialog::getFont(bool * ok, QWidget * parent);
impl<'a> /*trait*/ QFontDialog_getFont<QFont> for (&'a mut i8, &'a mut QWidget) {
  fn getFont(self, rsthis: &mut QFontDialog) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialog7getFontEPbP7QWidget()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QFontDialog7getFontEPbP7QWidget(arg0, arg1)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFontDialog {
  pub fn metaObject<RetType, T: QFontDialog_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QFontDialog_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QFontDialog) -> RetType;
}

// proto:  const QMetaObject * QFontDialog::metaObject();
impl<'a> /*trait*/ QFontDialog_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QFontDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFontDialog10metaObjectEv()};
     unsafe {_ZNK11QFontDialog10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFontDialog {
  pub fn selectedFont<RetType, T: QFontDialog_selectedFont<RetType>>(&mut self, value: T) -> RetType {
    return value.selectedFont(self);
    // return 1;
  }
}

pub trait QFontDialog_selectedFont<RetType> {
  fn selectedFont(self, rsthis: &mut QFontDialog) -> RetType;
}

// proto:  QFont QFontDialog::selectedFont();
impl<'a> /*trait*/ QFontDialog_selectedFont<QFont> for () {
  fn selectedFont(self, rsthis: &mut QFontDialog) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFontDialog12selectedFontEv()};
    let mut ret = unsafe {_ZNK11QFontDialog12selectedFontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFontDialog {
  pub fn setCurrentFont<RetType, T: QFontDialog_setCurrentFont<RetType>>(&mut self, value: T) -> RetType {
    return value.setCurrentFont(self);
    // return 1;
  }
}

pub trait QFontDialog_setCurrentFont<RetType> {
  fn setCurrentFont(self, rsthis: &mut QFontDialog) -> RetType;
}

// proto:  void QFontDialog::setCurrentFont(const QFont & font);
impl<'a> /*trait*/ QFontDialog_setCurrentFont<()> for (&'a  QFont) {
  fn setCurrentFont(self, rsthis: &mut QFontDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialog14setCurrentFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFontDialog14setCurrentFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

