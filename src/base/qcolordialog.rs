// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qcolor::QColor;
use super::qwidget::QWidget;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QColorDialog::NewQColorDialog(const QColorDialog & );
  fn _ZN12QColorDialogC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QColorDialog::currentColorChanged(const QColor & color);
  fn _ZN12QColorDialog19currentColorChangedERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QColor QColorDialog::currentColor();
  fn _ZNK12QColorDialog12currentColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QColor QColorDialog::customColor(int index);
  fn _ZN12QColorDialog11customColorEi(arg0: c_int) -> *mut c_void;
  // proto:  const QMetaObject * QColorDialog::metaObject();
  fn _ZNK12QColorDialog10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QColorDialog::NewQColorDialog(const QColor & initial, QWidget * parent);
  fn _ZN12QColorDialogC1ERK6QColorP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto: static void QColorDialog::setStandardColor(int index, QColor color);
  fn _ZN12QColorDialog16setStandardColorEi6QColor(arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QColorDialog::open(QObject * receiver, const char * member);
  fn _ZN12QColorDialog4openEP7QObjectPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char) ;
  // proto:  QColor QColorDialog::selectedColor();
  fn _ZNK12QColorDialog13selectedColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QColorDialog::FreeQColorDialog();
  fn _ZN12QColorDialogD0Ev(qthis: *mut c_void) ;
  // proto:  void QColorDialog::setVisible(bool visible);
  fn _ZN12QColorDialog10setVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QColorDialog::colorSelected(const QColor & color);
  fn _ZN12QColorDialog13colorSelectedERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QColorDialog::setCurrentColor(const QColor & color);
  fn _ZN12QColorDialog15setCurrentColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QColor QColorDialog::standardColor(int index);
  fn _ZN12QColorDialog13standardColorEi(arg0: c_int) -> *mut c_void;
  // proto: static unsigned int QColorDialog::getRgba(QRgb rgba, bool * ok, QWidget * parent);
  fn _ZN12QColorDialog7getRgbaEjPbP7QWidget(arg0: c_uint, arg1: *mut int8_t, arg2: *mut c_void) -> c_uint;
  // proto: static void QColorDialog::setCustomColor(int index, QColor color);
  fn _ZN12QColorDialog14setCustomColorEi6QColor(arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QColorDialog::NewQColorDialog(QWidget * parent);
  fn _ZN12QColorDialogC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static int QColorDialog::customCount();
  fn _ZN12QColorDialog11customCountEv() -> c_int;
}

// body block begin
// class sizeof(QColorDialog)=1
pub struct QColorDialog {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QColorDialog {
  pub fn NewQColorDialog<T: QColorDialog_NewQColorDialog>(value: T) -> QColorDialog {
    let rsthis = value.NewQColorDialog();
    return rsthis;
    // return 1;
  }
}

pub trait QColorDialog_NewQColorDialog {
  fn NewQColorDialog(self) -> QColorDialog;
}

// proto: void QColorDialog::NewQColorDialog(const QColorDialog & );
impl<'a> /*trait*/ QColorDialog_NewQColorDialog for (&'a  QColorDialog) {
  fn NewQColorDialog(self) -> QColorDialog {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialogC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QColorDialogC1ERKS_(qthis, arg0)};
    let rsthis = QColorDialog{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn currentColorChanged<T: QColorDialog_currentColorChanged>(&mut self, value: T)  {
     value.currentColorChanged(self);
    // return 1;
  }
}

pub trait QColorDialog_currentColorChanged {
  fn currentColorChanged(self, rsthis: &mut QColorDialog) ;
}

// proto:  void QColorDialog::currentColorChanged(const QColor & color);
impl<'a> /*trait*/ QColorDialog_currentColorChanged for (&'a  QColor) {
  fn currentColorChanged(self, rsthis: &mut QColorDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog19currentColorChangedERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QColorDialog19currentColorChangedERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn currentColor<T: QColorDialog_currentColor>(&mut self, value: T) -> QColor {
    return value.currentColor(self);
    // return 1;
  }
}

pub trait QColorDialog_currentColor {
  fn currentColor(self, rsthis: &mut QColorDialog) -> QColor;
}

// proto:  QColor QColorDialog::currentColor();
impl<'a> /*trait*/ QColorDialog_currentColor for () {
  fn currentColor(self, rsthis: &mut QColorDialog) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QColorDialog12currentColorEv()};
    let mut ret = unsafe {_ZNK12QColorDialog12currentColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn customColor<T: QColorDialog_customColor>(&mut self, value: T) -> QColor {
    return value.customColor(self);
    // return 1;
  }
}

pub trait QColorDialog_customColor {
  fn customColor(self, rsthis: &mut QColorDialog) -> QColor;
}

// proto: static QColor QColorDialog::customColor(int index);
impl<'a> /*trait*/ QColorDialog_customColor for (i32) {
  fn customColor(self, rsthis: &mut QColorDialog) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog11customColorEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN12QColorDialog11customColorEi(arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn metaObject<T: QColorDialog_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QColorDialog_metaObject {
  fn metaObject(self, rsthis: &mut QColorDialog) ;
}

// proto:  const QMetaObject * QColorDialog::metaObject();
impl<'a> /*trait*/ QColorDialog_metaObject for () {
  fn metaObject(self, rsthis: &mut QColorDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QColorDialog10metaObjectEv()};
     unsafe {_ZNK12QColorDialog10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QColorDialog::NewQColorDialog(const QColor & initial, QWidget * parent);
impl<'a> /*trait*/ QColorDialog_NewQColorDialog for (&'a  QColor, &'a mut QWidget) {
  fn NewQColorDialog(self) -> QColorDialog {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialogC1ERK6QColorP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QColorDialogC1ERK6QColorP7QWidget(qthis, arg0, arg1)};
    let rsthis = QColorDialog{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn setStandardColor<T: QColorDialog_setStandardColor>(&mut self, value: T)  {
     value.setStandardColor(self);
    // return 1;
  }
}

pub trait QColorDialog_setStandardColor {
  fn setStandardColor(self, rsthis: &mut QColorDialog) ;
}

// proto: static void QColorDialog::setStandardColor(int index, QColor color);
impl<'a> /*trait*/ QColorDialog_setStandardColor for (i32, QColor) {
  fn setStandardColor(self, rsthis: &mut QColorDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog16setStandardColorEi6QColor()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN12QColorDialog16setStandardColorEi6QColor(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn open<T: QColorDialog_open>(&mut self, value: T)  {
     value.open(self);
    // return 1;
  }
}

pub trait QColorDialog_open {
  fn open(self, rsthis: &mut QColorDialog) ;
}

// proto:  void QColorDialog::open(QObject * receiver, const char * member);
impl<'a> /*trait*/ QColorDialog_open for (&'a mut QObject, &'a  String) {
  fn open(self, rsthis: &mut QColorDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog4openEP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
     unsafe {_ZN12QColorDialog4openEP7QObjectPKc(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn selectedColor<T: QColorDialog_selectedColor>(&mut self, value: T) -> QColor {
    return value.selectedColor(self);
    // return 1;
  }
}

pub trait QColorDialog_selectedColor {
  fn selectedColor(self, rsthis: &mut QColorDialog) -> QColor;
}

// proto:  QColor QColorDialog::selectedColor();
impl<'a> /*trait*/ QColorDialog_selectedColor for () {
  fn selectedColor(self, rsthis: &mut QColorDialog) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QColorDialog13selectedColorEv()};
    let mut ret = unsafe {_ZNK12QColorDialog13selectedColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn FreeQColorDialog<T: QColorDialog_FreeQColorDialog>(&mut self, value: T)  {
     value.FreeQColorDialog(self);
    // return 1;
  }
}

pub trait QColorDialog_FreeQColorDialog {
  fn FreeQColorDialog(self, rsthis: &mut QColorDialog) ;
}

// proto:  void QColorDialog::FreeQColorDialog();
impl<'a> /*trait*/ QColorDialog_FreeQColorDialog for () {
  fn FreeQColorDialog(self, rsthis: &mut QColorDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialogD0Ev()};
     unsafe {_ZN12QColorDialogD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn setVisible<T: QColorDialog_setVisible>(&mut self, value: T)  {
     value.setVisible(self);
    // return 1;
  }
}

pub trait QColorDialog_setVisible {
  fn setVisible(self, rsthis: &mut QColorDialog) ;
}

// proto:  void QColorDialog::setVisible(bool visible);
impl<'a> /*trait*/ QColorDialog_setVisible for (i8) {
  fn setVisible(self, rsthis: &mut QColorDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog10setVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QColorDialog10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn colorSelected<T: QColorDialog_colorSelected>(&mut self, value: T)  {
     value.colorSelected(self);
    // return 1;
  }
}

pub trait QColorDialog_colorSelected {
  fn colorSelected(self, rsthis: &mut QColorDialog) ;
}

// proto:  void QColorDialog::colorSelected(const QColor & color);
impl<'a> /*trait*/ QColorDialog_colorSelected for (&'a  QColor) {
  fn colorSelected(self, rsthis: &mut QColorDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog13colorSelectedERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QColorDialog13colorSelectedERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn setCurrentColor<T: QColorDialog_setCurrentColor>(&mut self, value: T)  {
     value.setCurrentColor(self);
    // return 1;
  }
}

pub trait QColorDialog_setCurrentColor {
  fn setCurrentColor(self, rsthis: &mut QColorDialog) ;
}

// proto:  void QColorDialog::setCurrentColor(const QColor & color);
impl<'a> /*trait*/ QColorDialog_setCurrentColor for (&'a  QColor) {
  fn setCurrentColor(self, rsthis: &mut QColorDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog15setCurrentColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QColorDialog15setCurrentColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn standardColor<T: QColorDialog_standardColor>(&mut self, value: T) -> QColor {
    return value.standardColor(self);
    // return 1;
  }
}

pub trait QColorDialog_standardColor {
  fn standardColor(self, rsthis: &mut QColorDialog) -> QColor;
}

// proto: static QColor QColorDialog::standardColor(int index);
impl<'a> /*trait*/ QColorDialog_standardColor for (i32) {
  fn standardColor(self, rsthis: &mut QColorDialog) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog13standardColorEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN12QColorDialog13standardColorEi(arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn getRgba<T: QColorDialog_getRgba>(&mut self, value: T) -> u32 {
    return value.getRgba(self);
    // return 1;
  }
}

pub trait QColorDialog_getRgba {
  fn getRgba(self, rsthis: &mut QColorDialog) -> u32;
}

// proto: static unsigned int QColorDialog::getRgba(QRgb rgba, bool * ok, QWidget * parent);
impl<'a> /*trait*/ QColorDialog_getRgba for (u32, &'a mut i8, &'a mut QWidget) {
  fn getRgba(self, rsthis: &mut QColorDialog) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog7getRgbaEjPbP7QWidget()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as *mut int8_t;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QColorDialog7getRgbaEjPbP7QWidget(arg0, arg1, arg2)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn setCustomColor<T: QColorDialog_setCustomColor>(&mut self, value: T)  {
     value.setCustomColor(self);
    // return 1;
  }
}

pub trait QColorDialog_setCustomColor {
  fn setCustomColor(self, rsthis: &mut QColorDialog) ;
}

// proto: static void QColorDialog::setCustomColor(int index, QColor color);
impl<'a> /*trait*/ QColorDialog_setCustomColor for (i32, QColor) {
  fn setCustomColor(self, rsthis: &mut QColorDialog)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog14setCustomColorEi6QColor()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN12QColorDialog14setCustomColorEi6QColor(arg0, arg1)};
    // return 1;
  }
}

// proto: void QColorDialog::NewQColorDialog(QWidget * parent);
impl<'a> /*trait*/ QColorDialog_NewQColorDialog for (&'a mut QWidget) {
  fn NewQColorDialog(self) -> QColorDialog {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialogC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QColorDialogC1EP7QWidget(qthis, arg0)};
    let rsthis = QColorDialog{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn customCount<T: QColorDialog_customCount>(&mut self, value: T) -> i32 {
    return value.customCount(self);
    // return 1;
  }
}

pub trait QColorDialog_customCount {
  fn customCount(self, rsthis: &mut QColorDialog) -> i32;
}

// proto: static int QColorDialog::customCount();
impl<'a> /*trait*/ QColorDialog_customCount for () {
  fn customCount(self, rsthis: &mut QColorDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog11customCountEv()};
    let mut ret = unsafe {_ZN12QColorDialog11customCountEv()};
    return ret as i32;
    // return 1;
  }
}

