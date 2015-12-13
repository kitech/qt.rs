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
  pub fn currentFontChanged<T: QFontDialog_currentFontChanged>(&mut self, value: T)  {
     value.currentFontChanged(self);
    // return 1;
  }
}

pub trait QFontDialog_currentFontChanged {
  fn currentFontChanged(self, rsthis: &mut QFontDialog) ;
}

// proto:  void QFontDialog::currentFontChanged(const QFont & font);
impl<'a> /*trait*/ QFontDialog_currentFontChanged for (&'a  QFont) {
  fn currentFontChanged(self, rsthis: &mut QFontDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialog18currentFontChangedERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFontDialog18currentFontChangedERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFontDialog {
  pub fn open<T: QFontDialog_open>(&mut self, value: T)  {
     value.open(self);
    // return 1;
  }
}

pub trait QFontDialog_open {
  fn open(self, rsthis: &mut QFontDialog) ;
}

// proto:  void QFontDialog::open(QObject * receiver, const char * member);
impl<'a> /*trait*/ QFontDialog_open for (&'a mut QObject, &'a  String) {
  fn open(self, rsthis: &mut QFontDialog)  {
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
  pub fn currentFont<T: QFontDialog_currentFont>(&mut self, value: T) -> QFont {
    return value.currentFont(self);
    // return 1;
  }
}

pub trait QFontDialog_currentFont {
  fn currentFont(self, rsthis: &mut QFontDialog) -> QFont;
}

// proto:  QFont QFontDialog::currentFont();
impl<'a> /*trait*/ QFontDialog_currentFont for () {
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
  pub fn setVisible<T: QFontDialog_setVisible>(&mut self, value: T)  {
     value.setVisible(self);
    // return 1;
  }
}

pub trait QFontDialog_setVisible {
  fn setVisible(self, rsthis: &mut QFontDialog) ;
}

// proto:  void QFontDialog::setVisible(bool visible);
impl<'a> /*trait*/ QFontDialog_setVisible for (i8) {
  fn setVisible(self, rsthis: &mut QFontDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialog10setVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QFontDialog10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFontDialog {
  pub fn FreeQFontDialog<T: QFontDialog_FreeQFontDialog>(&mut self, value: T)  {
     value.FreeQFontDialog(self);
    // return 1;
  }
}

pub trait QFontDialog_FreeQFontDialog {
  fn FreeQFontDialog(self, rsthis: &mut QFontDialog) ;
}

// proto:  void QFontDialog::FreeQFontDialog();
impl<'a> /*trait*/ QFontDialog_FreeQFontDialog for () {
  fn FreeQFontDialog(self, rsthis: &mut QFontDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialogD0Ev()};
     unsafe {_ZN11QFontDialogD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFontDialog {
  pub fn fontSelected<T: QFontDialog_fontSelected>(&mut self, value: T)  {
     value.fontSelected(self);
    // return 1;
  }
}

pub trait QFontDialog_fontSelected {
  fn fontSelected(self, rsthis: &mut QFontDialog) ;
}

// proto:  void QFontDialog::fontSelected(const QFont & font);
impl<'a> /*trait*/ QFontDialog_fontSelected for (&'a  QFont) {
  fn fontSelected(self, rsthis: &mut QFontDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialog12fontSelectedERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFontDialog12fontSelectedERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFontDialog {
  pub fn getFont<T: QFontDialog_getFont>(&mut self, value: T) -> QFont {
    return value.getFont(self);
    // return 1;
  }
}

pub trait QFontDialog_getFont {
  fn getFont(self, rsthis: &mut QFontDialog) -> QFont;
}

// proto: static QFont QFontDialog::getFont(bool * ok, QWidget * parent);
impl<'a> /*trait*/ QFontDialog_getFont for (&'a mut i8, &'a mut QWidget) {
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
  pub fn metaObject<T: QFontDialog_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QFontDialog_metaObject {
  fn metaObject(self, rsthis: &mut QFontDialog) ;
}

// proto:  const QMetaObject * QFontDialog::metaObject();
impl<'a> /*trait*/ QFontDialog_metaObject for () {
  fn metaObject(self, rsthis: &mut QFontDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFontDialog10metaObjectEv()};
     unsafe {_ZNK11QFontDialog10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFontDialog {
  pub fn selectedFont<T: QFontDialog_selectedFont>(&mut self, value: T) -> QFont {
    return value.selectedFont(self);
    // return 1;
  }
}

pub trait QFontDialog_selectedFont {
  fn selectedFont(self, rsthis: &mut QFontDialog) -> QFont;
}

// proto:  QFont QFontDialog::selectedFont();
impl<'a> /*trait*/ QFontDialog_selectedFont for () {
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
  pub fn setCurrentFont<T: QFontDialog_setCurrentFont>(&mut self, value: T)  {
     value.setCurrentFont(self);
    // return 1;
  }
}

pub trait QFontDialog_setCurrentFont {
  fn setCurrentFont(self, rsthis: &mut QFontDialog) ;
}

// proto:  void QFontDialog::setCurrentFont(const QFont & font);
impl<'a> /*trait*/ QFontDialog_setCurrentFont for (&'a  QFont) {
  fn setCurrentFont(self, rsthis: &mut QFontDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialog14setCurrentFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFontDialog14setCurrentFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

