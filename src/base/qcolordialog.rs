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
  // proto: void QColorDialog::NewQColorDialog(const QColorDialog & );
  fn _ZN12QColorDialogC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QColorDialog::currentColorChanged(const QColor & color);
  fn _ZN12QColorDialog19currentColorChangedERK6QColor(arg0: *const c_void) -> i32;
  // proto: QColor QColorDialog::currentColor();
  fn _ZNK12QColorDialog12currentColorEv() -> i32;
  // proto: QColor QColorDialog::customColor(int index);
  fn _ZN12QColorDialog11customColorEi(arg0: c_int) -> i32;
  // proto: const QMetaObject * QColorDialog::metaObject();
  fn _ZNK12QColorDialog10metaObjectEv() -> i32;
  // proto: void QColorDialog::NewQColorDialog(const QColor & initial, QWidget * parent);
  fn _ZN12QColorDialogC1ERK6QColorP7QWidget(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: void QColorDialog::setStandardColor(int index, QColor color);
  fn _ZN12QColorDialog16setStandardColorEi6QColor(arg0: c_int, arg1: *mut c_void) -> i32;
  // proto: void QColorDialog::open(QObject * receiver, const char * member);
  fn _ZN12QColorDialog4openEP7QObjectPKc(arg0: *mut c_void, arg1: *const c_char) -> i32;
  // proto: QColor QColorDialog::selectedColor();
  fn _ZNK12QColorDialog13selectedColorEv() -> i32;
  // proto: void QColorDialog::FreeQColorDialog();
  fn _ZN12QColorDialogD0Ev() -> i32;
  // proto: void QColorDialog::setVisible(bool visible);
  fn _ZN12QColorDialog10setVisibleEb(arg0: int8_t) -> i32;
  // proto: void QColorDialog::colorSelected(const QColor & color);
  fn _ZN12QColorDialog13colorSelectedERK6QColor(arg0: *const c_void) -> i32;
  // proto: void QColorDialog::setCurrentColor(const QColor & color);
  fn _ZN12QColorDialog15setCurrentColorERK6QColor(arg0: *const c_void) -> i32;
  // proto: QColor QColorDialog::standardColor(int index);
  fn _ZN12QColorDialog13standardColorEi(arg0: c_int) -> i32;
  // proto: unsigned int QColorDialog::getRgba(QRgb rgba, bool * ok, QWidget * parent);
  fn _ZN12QColorDialog7getRgbaEjPbP7QWidget(arg0: c_uint, arg1: *mut int8_t, arg2: *mut c_void) -> i32;
  // proto: void QColorDialog::setCustomColor(int index, QColor color);
  fn _ZN12QColorDialog14setCustomColorEi6QColor(arg0: c_int, arg1: *mut c_void) -> i32;
  // proto: void QColorDialog::NewQColorDialog(QWidget * parent);
  fn _ZN12QColorDialogC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: int QColorDialog::customCount();
  fn _ZN12QColorDialog11customCountEv() -> i32;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QColorDialogC1ERKS_(qthis, arg0)};
    let rsthis = QColorDialog{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn currentColorChanged<T: QColorDialog_currentColorChanged>(&mut self, value: T) -> i32 {
    value.currentColorChanged(self);
    return 1;
  }
}

pub trait QColorDialog_currentColorChanged {
  fn currentColorChanged(self, this: &mut QColorDialog) -> i32;
}

// proto: void QColorDialog::currentColorChanged(const QColor & color);
impl<'a> /*trait*/ QColorDialog_currentColorChanged for (&'a  QColor) {
  fn currentColorChanged(self, this: &mut QColorDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog19currentColorChangedERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QColorDialog19currentColorChangedERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn currentColor<T: QColorDialog_currentColor>(&mut self, value: T) -> i32 {
    value.currentColor(self);
    return 1;
  }
}

pub trait QColorDialog_currentColor {
  fn currentColor(self, this: &mut QColorDialog) -> i32;
}

// proto: QColor QColorDialog::currentColor();
impl<'a> /*trait*/ QColorDialog_currentColor for () {
  fn currentColor(self, this: &mut QColorDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QColorDialog12currentColorEv()};
    unsafe {_ZNK12QColorDialog12currentColorEv()};
    return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn customColor<T: QColorDialog_customColor>(&mut self, value: T) -> i32 {
    value.customColor(self);
    return 1;
  }
}

pub trait QColorDialog_customColor {
  fn customColor(self, this: &mut QColorDialog) -> i32;
}

// proto: QColor QColorDialog::customColor(int index);
impl<'a> /*trait*/ QColorDialog_customColor for (i32) {
  fn customColor(self, this: &mut QColorDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog11customColorEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QColorDialog11customColorEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn metaObject<T: QColorDialog_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QColorDialog_metaObject {
  fn metaObject(self, this: &mut QColorDialog) -> i32;
}

// proto: const QMetaObject * QColorDialog::metaObject();
impl<'a> /*trait*/ QColorDialog_metaObject for () {
  fn metaObject(self, this: &mut QColorDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QColorDialog10metaObjectEv()};
    unsafe {_ZNK12QColorDialog10metaObjectEv()};
    return 1;
  }
}

// proto: void QColorDialog::NewQColorDialog(const QColor & initial, QWidget * parent);
impl<'a> /*trait*/ QColorDialog_NewQColorDialog for (&'a  QColor, &'a mut QWidget) {
  fn NewQColorDialog(self) -> QColorDialog {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialogC1ERK6QColorP7QWidget()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QColorDialogC1ERK6QColorP7QWidget(qthis, arg0, arg1)};
    let rsthis = QColorDialog{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn setStandardColor<T: QColorDialog_setStandardColor>(&mut self, value: T) -> i32 {
    value.setStandardColor(self);
    return 1;
  }
}

pub trait QColorDialog_setStandardColor {
  fn setStandardColor(self, this: &mut QColorDialog) -> i32;
}

// proto: void QColorDialog::setStandardColor(int index, QColor color);
impl<'a> /*trait*/ QColorDialog_setStandardColor for (i32, QColor) {
  fn setStandardColor(self, this: &mut QColorDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog16setStandardColorEi6QColor()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QColorDialog16setStandardColorEi6QColor(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn open<T: QColorDialog_open>(&mut self, value: T) -> i32 {
    value.open(self);
    return 1;
  }
}

pub trait QColorDialog_open {
  fn open(self, this: &mut QColorDialog) -> i32;
}

// proto: void QColorDialog::open(QObject * receiver, const char * member);
impl<'a> /*trait*/ QColorDialog_open for (&'a mut QObject, &'a  String) {
  fn open(self, this: &mut QColorDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog4openEP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN12QColorDialog4openEP7QObjectPKc(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn selectedColor<T: QColorDialog_selectedColor>(&mut self, value: T) -> i32 {
    value.selectedColor(self);
    return 1;
  }
}

pub trait QColorDialog_selectedColor {
  fn selectedColor(self, this: &mut QColorDialog) -> i32;
}

// proto: QColor QColorDialog::selectedColor();
impl<'a> /*trait*/ QColorDialog_selectedColor for () {
  fn selectedColor(self, this: &mut QColorDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QColorDialog13selectedColorEv()};
    unsafe {_ZNK12QColorDialog13selectedColorEv()};
    return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn FreeQColorDialog<T: QColorDialog_FreeQColorDialog>(&mut self, value: T) -> i32 {
    value.FreeQColorDialog(self);
    return 1;
  }
}

pub trait QColorDialog_FreeQColorDialog {
  fn FreeQColorDialog(self, this: &mut QColorDialog) -> i32;
}

// proto: void QColorDialog::FreeQColorDialog();
impl<'a> /*trait*/ QColorDialog_FreeQColorDialog for () {
  fn FreeQColorDialog(self, this: &mut QColorDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialogD0Ev()};
    unsafe {_ZN12QColorDialogD0Ev()};
    return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn setVisible<T: QColorDialog_setVisible>(&mut self, value: T) -> i32 {
    value.setVisible(self);
    return 1;
  }
}

pub trait QColorDialog_setVisible {
  fn setVisible(self, this: &mut QColorDialog) -> i32;
}

// proto: void QColorDialog::setVisible(bool visible);
impl<'a> /*trait*/ QColorDialog_setVisible for (i8) {
  fn setVisible(self, this: &mut QColorDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog10setVisibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN12QColorDialog10setVisibleEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn colorSelected<T: QColorDialog_colorSelected>(&mut self, value: T) -> i32 {
    value.colorSelected(self);
    return 1;
  }
}

pub trait QColorDialog_colorSelected {
  fn colorSelected(self, this: &mut QColorDialog) -> i32;
}

// proto: void QColorDialog::colorSelected(const QColor & color);
impl<'a> /*trait*/ QColorDialog_colorSelected for (&'a  QColor) {
  fn colorSelected(self, this: &mut QColorDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog13colorSelectedERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QColorDialog13colorSelectedERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn setCurrentColor<T: QColorDialog_setCurrentColor>(&mut self, value: T) -> i32 {
    value.setCurrentColor(self);
    return 1;
  }
}

pub trait QColorDialog_setCurrentColor {
  fn setCurrentColor(self, this: &mut QColorDialog) -> i32;
}

// proto: void QColorDialog::setCurrentColor(const QColor & color);
impl<'a> /*trait*/ QColorDialog_setCurrentColor for (&'a  QColor) {
  fn setCurrentColor(self, this: &mut QColorDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog15setCurrentColorERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QColorDialog15setCurrentColorERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn standardColor<T: QColorDialog_standardColor>(&mut self, value: T) -> i32 {
    value.standardColor(self);
    return 1;
  }
}

pub trait QColorDialog_standardColor {
  fn standardColor(self, this: &mut QColorDialog) -> i32;
}

// proto: QColor QColorDialog::standardColor(int index);
impl<'a> /*trait*/ QColorDialog_standardColor for (i32) {
  fn standardColor(self, this: &mut QColorDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog13standardColorEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QColorDialog13standardColorEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn getRgba<T: QColorDialog_getRgba>(&mut self, value: T) -> i32 {
    value.getRgba(self);
    return 1;
  }
}

pub trait QColorDialog_getRgba {
  fn getRgba(self, this: &mut QColorDialog) -> i32;
}

// proto: unsigned int QColorDialog::getRgba(QRgb rgba, bool * ok, QWidget * parent);
impl<'a> /*trait*/ QColorDialog_getRgba for (u32, &'a mut i8, &'a mut QWidget) {
  fn getRgba(self, this: &mut QColorDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog7getRgbaEjPbP7QWidget()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as *mut int8_t;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN12QColorDialog7getRgbaEjPbP7QWidget(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn setCustomColor<T: QColorDialog_setCustomColor>(&mut self, value: T) -> i32 {
    value.setCustomColor(self);
    return 1;
  }
}

pub trait QColorDialog_setCustomColor {
  fn setCustomColor(self, this: &mut QColorDialog) -> i32;
}

// proto: void QColorDialog::setCustomColor(int index, QColor color);
impl<'a> /*trait*/ QColorDialog_setCustomColor for (i32, QColor) {
  fn setCustomColor(self, this: &mut QColorDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog14setCustomColorEi6QColor()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QColorDialog14setCustomColorEi6QColor(arg0, arg1)};
    return 1;
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
    value.customCount(self);
    return 1;
  }
}

pub trait QColorDialog_customCount {
  fn customCount(self, this: &mut QColorDialog) -> i32;
}

// proto: int QColorDialog::customCount();
impl<'a> /*trait*/ QColorDialog_customCount for () {
  fn customCount(self, this: &mut QColorDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog11customCountEv()};
    unsafe {_ZN12QColorDialog11customCountEv()};
    return 1;
  }
}

