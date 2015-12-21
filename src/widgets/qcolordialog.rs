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
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QColorDialog::QColorDialog(const QColorDialog & );
  fn _ZN12QColorDialogC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QColorDialog::currentColorChanged(const QColor & color);
  fn _ZN12QColorDialog19currentColorChangedERK6QColor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QColor QColorDialog::currentColor();
  fn _ZNK12QColorDialog12currentColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QColor QColorDialog::customColor(int index);
  fn _ZN12QColorDialog11customColorEi(arg0: c_int) -> *mut c_void;
  // proto:  const QMetaObject * QColorDialog::metaObject();
  fn _ZNK12QColorDialog10metaObjectEv(qthis: *mut c_void);
  // proto:  void QColorDialog::QColorDialog(const QColor & initial, QWidget * parent);
  fn _ZN12QColorDialogC1ERK6QColorP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto: static void QColorDialog::setStandardColor(int index, QColor color);
  fn _ZN12QColorDialog16setStandardColorEi6QColor(arg0: c_int, arg1: *mut c_void);
  // proto:  void QColorDialog::open(QObject * receiver, const char * member);
  fn _ZN12QColorDialog4openEP7QObjectPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char);
  // proto:  QColor QColorDialog::selectedColor();
  fn _ZNK12QColorDialog13selectedColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QColorDialog::~QColorDialog();
  fn _ZN12QColorDialogD0Ev(qthis: *mut c_void);
  // proto:  void QColorDialog::setVisible(bool visible);
  fn _ZN12QColorDialog10setVisibleEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QColorDialog::colorSelected(const QColor & color);
  fn _ZN12QColorDialog13colorSelectedERK6QColor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QColorDialog::setCurrentColor(const QColor & color);
  fn _ZN12QColorDialog15setCurrentColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QColor QColorDialog::standardColor(int index);
  fn _ZN12QColorDialog13standardColorEi(arg0: c_int) -> *mut c_void;
  // proto: static QRgb QColorDialog::getRgba(QRgb rgba, bool * ok, QWidget * parent);
  fn _ZN12QColorDialog7getRgbaEjPbP7QWidget(arg0: c_uint, arg1: *mut c_char, arg2: *mut c_void) -> c_uint;
  // proto: static void QColorDialog::setCustomColor(int index, QColor color);
  fn _ZN12QColorDialog14setCustomColorEi6QColor(arg0: c_int, arg1: *mut c_void);
  // proto:  void QColorDialog::QColorDialog(QWidget * parent);
  fn _ZN12QColorDialogC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static int QColorDialog::customCount();
  fn _ZN12QColorDialog11customCountEv() -> c_int;
}

// body block begin
// class sizeof(QColorDialog)=1
pub struct QColorDialog {
  pub qclsinst: *mut c_void,
}

  // proto:  void QColorDialog::QColorDialog(const QColorDialog & );
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

  // proto:  void QColorDialog::QColorDialog(const QColorDialog & );
impl<'a> /*trait*/ QColorDialog_NewQColorDialog for (QColorDialog) {
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

  // proto:  void QColorDialog::currentColorChanged(const QColor & color);
impl /*struct*/ QColorDialog {
  pub fn currentColorChanged<RetType, T: QColorDialog_currentColorChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentColorChanged(self);
    // return 1;
  }
}

pub trait QColorDialog_currentColorChanged<RetType> {
  fn currentColorChanged(self , rsthis: &mut QColorDialog) -> RetType;
}

  // proto:  void QColorDialog::currentColorChanged(const QColor & color);
impl<'a> /*trait*/ QColorDialog_currentColorChanged<()> for (QColor) {
  fn currentColorChanged(self , rsthis: &mut QColorDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog19currentColorChangedERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QColorDialog19currentColorChangedERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QColor QColorDialog::currentColor();
impl /*struct*/ QColorDialog {
  pub fn currentColor<RetType, T: QColorDialog_currentColor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentColor(self);
    // return 1;
  }
}

pub trait QColorDialog_currentColor<RetType> {
  fn currentColor(self , rsthis: &mut QColorDialog) -> RetType;
}

  // proto:  QColor QColorDialog::currentColor();
impl<'a> /*trait*/ QColorDialog_currentColor<QColor> for () {
  fn currentColor(self , rsthis: &mut QColorDialog) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QColorDialog12currentColorEv()};
    let mut ret = unsafe {_ZNK12QColorDialog12currentColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static QColor QColorDialog::customColor(int index);
impl /*struct*/ QColorDialog {
  pub fn customColor_s<RetType, T: QColorDialog_customColor_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.customColor_s();
    // return 1;
  }
}

pub trait QColorDialog_customColor_s<RetType> {
  fn customColor_s(self ) -> RetType;
}

  // proto: static QColor QColorDialog::customColor(int index);
impl<'a> /*trait*/ QColorDialog_customColor_s<QColor> for (i32) {
  fn customColor_s(self ) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog11customColorEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN12QColorDialog11customColorEi(arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QColorDialog::metaObject();
impl /*struct*/ QColorDialog {
  pub fn metaObject<RetType, T: QColorDialog_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QColorDialog_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QColorDialog) -> RetType;
}

  // proto:  const QMetaObject * QColorDialog::metaObject();
impl<'a> /*trait*/ QColorDialog_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QColorDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QColorDialog10metaObjectEv()};
     unsafe {_ZNK12QColorDialog10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QColorDialog::QColorDialog(const QColor & initial, QWidget * parent);
impl<'a> /*trait*/ QColorDialog_NewQColorDialog for (QColor, QWidget) {
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

  // proto: static void QColorDialog::setStandardColor(int index, QColor color);
impl /*struct*/ QColorDialog {
  pub fn setStandardColor_s<RetType, T: QColorDialog_setStandardColor_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setStandardColor_s();
    // return 1;
  }
}

pub trait QColorDialog_setStandardColor_s<RetType> {
  fn setStandardColor_s(self ) -> RetType;
}

  // proto: static void QColorDialog::setStandardColor(int index, QColor color);
impl<'a> /*trait*/ QColorDialog_setStandardColor_s<()> for (i32, QColor) {
  fn setStandardColor_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog16setStandardColorEi6QColor()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN12QColorDialog16setStandardColorEi6QColor(arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QColorDialog::open(QObject * receiver, const char * member);
impl /*struct*/ QColorDialog {
  pub fn open<RetType, T: QColorDialog_open<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.open(self);
    // return 1;
  }
}

pub trait QColorDialog_open<RetType> {
  fn open(self , rsthis: &mut QColorDialog) -> RetType;
}

  // proto:  void QColorDialog::open(QObject * receiver, const char * member);
impl<'a> /*trait*/ QColorDialog_open<()> for (QObject, &'a  String) {
  fn open(self , rsthis: &mut QColorDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog4openEP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {_ZN12QColorDialog4openEP7QObjectPKc(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QColor QColorDialog::selectedColor();
impl /*struct*/ QColorDialog {
  pub fn selectedColor<RetType, T: QColorDialog_selectedColor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectedColor(self);
    // return 1;
  }
}

pub trait QColorDialog_selectedColor<RetType> {
  fn selectedColor(self , rsthis: &mut QColorDialog) -> RetType;
}

  // proto:  QColor QColorDialog::selectedColor();
impl<'a> /*trait*/ QColorDialog_selectedColor<QColor> for () {
  fn selectedColor(self , rsthis: &mut QColorDialog) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QColorDialog13selectedColorEv()};
    let mut ret = unsafe {_ZNK12QColorDialog13selectedColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QColorDialog::~QColorDialog();
impl /*struct*/ QColorDialog {
  pub fn FreeQColorDialog<RetType, T: QColorDialog_FreeQColorDialog<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQColorDialog(self);
    // return 1;
  }
}

pub trait QColorDialog_FreeQColorDialog<RetType> {
  fn FreeQColorDialog(self , rsthis: &mut QColorDialog) -> RetType;
}

  // proto:  void QColorDialog::~QColorDialog();
impl<'a> /*trait*/ QColorDialog_FreeQColorDialog<()> for () {
  fn FreeQColorDialog(self , rsthis: &mut QColorDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialogD0Ev()};
     unsafe {_ZN12QColorDialogD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QColorDialog::setVisible(bool visible);
impl /*struct*/ QColorDialog {
  pub fn setVisible<RetType, T: QColorDialog_setVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QColorDialog_setVisible<RetType> {
  fn setVisible(self , rsthis: &mut QColorDialog) -> RetType;
}

  // proto:  void QColorDialog::setVisible(bool visible);
impl<'a> /*trait*/ QColorDialog_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: &mut QColorDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QColorDialog10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QColorDialog::colorSelected(const QColor & color);
impl /*struct*/ QColorDialog {
  pub fn colorSelected<RetType, T: QColorDialog_colorSelected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.colorSelected(self);
    // return 1;
  }
}

pub trait QColorDialog_colorSelected<RetType> {
  fn colorSelected(self , rsthis: &mut QColorDialog) -> RetType;
}

  // proto:  void QColorDialog::colorSelected(const QColor & color);
impl<'a> /*trait*/ QColorDialog_colorSelected<()> for (QColor) {
  fn colorSelected(self , rsthis: &mut QColorDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog13colorSelectedERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QColorDialog13colorSelectedERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QColorDialog::setCurrentColor(const QColor & color);
impl /*struct*/ QColorDialog {
  pub fn setCurrentColor<RetType, T: QColorDialog_setCurrentColor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCurrentColor(self);
    // return 1;
  }
}

pub trait QColorDialog_setCurrentColor<RetType> {
  fn setCurrentColor(self , rsthis: &mut QColorDialog) -> RetType;
}

  // proto:  void QColorDialog::setCurrentColor(const QColor & color);
impl<'a> /*trait*/ QColorDialog_setCurrentColor<()> for (QColor) {
  fn setCurrentColor(self , rsthis: &mut QColorDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog15setCurrentColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QColorDialog15setCurrentColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QColor QColorDialog::standardColor(int index);
impl /*struct*/ QColorDialog {
  pub fn standardColor_s<RetType, T: QColorDialog_standardColor_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.standardColor_s();
    // return 1;
  }
}

pub trait QColorDialog_standardColor_s<RetType> {
  fn standardColor_s(self ) -> RetType;
}

  // proto: static QColor QColorDialog::standardColor(int index);
impl<'a> /*trait*/ QColorDialog_standardColor_s<QColor> for (i32) {
  fn standardColor_s(self ) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog13standardColorEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN12QColorDialog13standardColorEi(arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static QRgb QColorDialog::getRgba(QRgb rgba, bool * ok, QWidget * parent);
impl /*struct*/ QColorDialog {
  pub fn getRgba_s<RetType, T: QColorDialog_getRgba_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.getRgba_s();
    // return 1;
  }
}

pub trait QColorDialog_getRgba_s<RetType> {
  fn getRgba_s(self ) -> RetType;
}

  // proto: static QRgb QColorDialog::getRgba(QRgb rgba, bool * ok, QWidget * parent);
impl<'a> /*trait*/ QColorDialog_getRgba_s<u32> for (u32, &'a mut Vec<i8>, QWidget) {
  fn getRgba_s(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog7getRgbaEjPbP7QWidget()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QColorDialog7getRgbaEjPbP7QWidget(arg0, arg1, arg2)};
    return ret as u32;
    // return 1;
  }
}

  // proto: static void QColorDialog::setCustomColor(int index, QColor color);
impl /*struct*/ QColorDialog {
  pub fn setCustomColor_s<RetType, T: QColorDialog_setCustomColor_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setCustomColor_s();
    // return 1;
  }
}

pub trait QColorDialog_setCustomColor_s<RetType> {
  fn setCustomColor_s(self ) -> RetType;
}

  // proto: static void QColorDialog::setCustomColor(int index, QColor color);
impl<'a> /*trait*/ QColorDialog_setCustomColor_s<()> for (i32, QColor) {
  fn setCustomColor_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog14setCustomColorEi6QColor()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN12QColorDialog14setCustomColorEi6QColor(arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QColorDialog::QColorDialog(QWidget * parent);
impl<'a> /*trait*/ QColorDialog_NewQColorDialog for (QWidget) {
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

  // proto: static int QColorDialog::customCount();
impl /*struct*/ QColorDialog {
  pub fn customCount_s<RetType, T: QColorDialog_customCount_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.customCount_s();
    // return 1;
  }
}

pub trait QColorDialog_customCount_s<RetType> {
  fn customCount_s(self ) -> RetType;
}

  // proto: static int QColorDialog::customCount();
impl<'a> /*trait*/ QColorDialog_customCount_s<i32> for () {
  fn customCount_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog11customCountEv()};
    let mut ret = unsafe {_ZN12QColorDialog11customCountEv()};
    return ret as i32;
    // return 1;
  }
}

