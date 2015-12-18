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
  pub fn currentColorChanged<RetType, T: QColorDialog_currentColorChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.currentColorChanged(self);
    // return 1;
  }
}

pub trait QColorDialog_currentColorChanged<RetType> {
  fn currentColorChanged(self, rsthis: &mut QColorDialog) -> RetType;
}

// proto:  void QColorDialog::currentColorChanged(const QColor & color);
impl<'a> /*trait*/ QColorDialog_currentColorChanged<()> for (&'a  QColor) {
  fn currentColorChanged(self, rsthis: &mut QColorDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog19currentColorChangedERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QColorDialog19currentColorChangedERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn currentColor<RetType, T: QColorDialog_currentColor<RetType>>(&mut self, value: T) -> RetType {
    return value.currentColor(self);
    // return 1;
  }
}

pub trait QColorDialog_currentColor<RetType> {
  fn currentColor(self, rsthis: &mut QColorDialog) -> RetType;
}

// proto:  QColor QColorDialog::currentColor();
impl<'a> /*trait*/ QColorDialog_currentColor<QColor> for () {
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
  pub fn customColor<RetType, T: QColorDialog_customColor<RetType>>(&mut self, value: T) -> RetType {
    return value.customColor(self);
    // return 1;
  }
}

pub trait QColorDialog_customColor<RetType> {
  fn customColor(self, rsthis: &mut QColorDialog) -> RetType;
}

// proto: static QColor QColorDialog::customColor(int index);
impl<'a> /*trait*/ QColorDialog_customColor<QColor> for (i32) {
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
  pub fn metaObject<RetType, T: QColorDialog_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QColorDialog_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QColorDialog) -> RetType;
}

// proto:  const QMetaObject * QColorDialog::metaObject();
impl<'a> /*trait*/ QColorDialog_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QColorDialog) -> () {
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
  pub fn setStandardColor<RetType, T: QColorDialog_setStandardColor<RetType>>(&mut self, value: T) -> RetType {
    return value.setStandardColor(self);
    // return 1;
  }
}

pub trait QColorDialog_setStandardColor<RetType> {
  fn setStandardColor(self, rsthis: &mut QColorDialog) -> RetType;
}

// proto: static void QColorDialog::setStandardColor(int index, QColor color);
impl<'a> /*trait*/ QColorDialog_setStandardColor<()> for (i32, QColor) {
  fn setStandardColor(self, rsthis: &mut QColorDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog16setStandardColorEi6QColor()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN12QColorDialog16setStandardColorEi6QColor(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn open<RetType, T: QColorDialog_open<RetType>>(&mut self, value: T) -> RetType {
    return value.open(self);
    // return 1;
  }
}

pub trait QColorDialog_open<RetType> {
  fn open(self, rsthis: &mut QColorDialog) -> RetType;
}

// proto:  void QColorDialog::open(QObject * receiver, const char * member);
impl<'a> /*trait*/ QColorDialog_open<()> for (&'a mut QObject, &'a  String) {
  fn open(self, rsthis: &mut QColorDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog4openEP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
     unsafe {_ZN12QColorDialog4openEP7QObjectPKc(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn selectedColor<RetType, T: QColorDialog_selectedColor<RetType>>(&mut self, value: T) -> RetType {
    return value.selectedColor(self);
    // return 1;
  }
}

pub trait QColorDialog_selectedColor<RetType> {
  fn selectedColor(self, rsthis: &mut QColorDialog) -> RetType;
}

// proto:  QColor QColorDialog::selectedColor();
impl<'a> /*trait*/ QColorDialog_selectedColor<QColor> for () {
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
  pub fn FreeQColorDialog<RetType, T: QColorDialog_FreeQColorDialog<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQColorDialog(self);
    // return 1;
  }
}

pub trait QColorDialog_FreeQColorDialog<RetType> {
  fn FreeQColorDialog(self, rsthis: &mut QColorDialog) -> RetType;
}

// proto:  void QColorDialog::FreeQColorDialog();
impl<'a> /*trait*/ QColorDialog_FreeQColorDialog<()> for () {
  fn FreeQColorDialog(self, rsthis: &mut QColorDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialogD0Ev()};
     unsafe {_ZN12QColorDialogD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn setVisible<RetType, T: QColorDialog_setVisible<RetType>>(&mut self, value: T) -> RetType {
    return value.setVisible(self);
    // return 1;
  }
}

pub trait QColorDialog_setVisible<RetType> {
  fn setVisible(self, rsthis: &mut QColorDialog) -> RetType;
}

// proto:  void QColorDialog::setVisible(bool visible);
impl<'a> /*trait*/ QColorDialog_setVisible<()> for (i8) {
  fn setVisible(self, rsthis: &mut QColorDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog10setVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QColorDialog10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn colorSelected<RetType, T: QColorDialog_colorSelected<RetType>>(&mut self, value: T) -> RetType {
    return value.colorSelected(self);
    // return 1;
  }
}

pub trait QColorDialog_colorSelected<RetType> {
  fn colorSelected(self, rsthis: &mut QColorDialog) -> RetType;
}

// proto:  void QColorDialog::colorSelected(const QColor & color);
impl<'a> /*trait*/ QColorDialog_colorSelected<()> for (&'a  QColor) {
  fn colorSelected(self, rsthis: &mut QColorDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog13colorSelectedERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QColorDialog13colorSelectedERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn setCurrentColor<RetType, T: QColorDialog_setCurrentColor<RetType>>(&mut self, value: T) -> RetType {
    return value.setCurrentColor(self);
    // return 1;
  }
}

pub trait QColorDialog_setCurrentColor<RetType> {
  fn setCurrentColor(self, rsthis: &mut QColorDialog) -> RetType;
}

// proto:  void QColorDialog::setCurrentColor(const QColor & color);
impl<'a> /*trait*/ QColorDialog_setCurrentColor<()> for (&'a  QColor) {
  fn setCurrentColor(self, rsthis: &mut QColorDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog15setCurrentColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QColorDialog15setCurrentColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColorDialog {
  pub fn standardColor<RetType, T: QColorDialog_standardColor<RetType>>(&mut self, value: T) -> RetType {
    return value.standardColor(self);
    // return 1;
  }
}

pub trait QColorDialog_standardColor<RetType> {
  fn standardColor(self, rsthis: &mut QColorDialog) -> RetType;
}

// proto: static QColor QColorDialog::standardColor(int index);
impl<'a> /*trait*/ QColorDialog_standardColor<QColor> for (i32) {
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
  pub fn getRgba<RetType, T: QColorDialog_getRgba<RetType>>(&mut self, value: T) -> RetType {
    return value.getRgba(self);
    // return 1;
  }
}

pub trait QColorDialog_getRgba<RetType> {
  fn getRgba(self, rsthis: &mut QColorDialog) -> RetType;
}

// proto: static unsigned int QColorDialog::getRgba(QRgb rgba, bool * ok, QWidget * parent);
impl<'a> /*trait*/ QColorDialog_getRgba<u32> for (u32, &'a mut i8, &'a mut QWidget) {
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
  pub fn setCustomColor<RetType, T: QColorDialog_setCustomColor<RetType>>(&mut self, value: T) -> RetType {
    return value.setCustomColor(self);
    // return 1;
  }
}

pub trait QColorDialog_setCustomColor<RetType> {
  fn setCustomColor(self, rsthis: &mut QColorDialog) -> RetType;
}

// proto: static void QColorDialog::setCustomColor(int index, QColor color);
impl<'a> /*trait*/ QColorDialog_setCustomColor<()> for (i32, QColor) {
  fn setCustomColor(self, rsthis: &mut QColorDialog) -> () {
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
  pub fn customCount<RetType, T: QColorDialog_customCount<RetType>>(&mut self, value: T) -> RetType {
    return value.customCount(self);
    // return 1;
  }
}

pub trait QColorDialog_customCount<RetType> {
  fn customCount(self, rsthis: &mut QColorDialog) -> RetType;
}

// proto: static int QColorDialog::customCount();
impl<'a> /*trait*/ QColorDialog_customCount<i32> for () {
  fn customCount(self, rsthis: &mut QColorDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog11customCountEv()};
    let mut ret = unsafe {_ZN12QColorDialog11customCountEv()};
    return ret as i32;
    // return 1;
  }
}

