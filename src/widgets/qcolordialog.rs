// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
// src-file: /QtWidgets/qcolordialog.h
// dst-file: /src/widgets/qcolordialog.rs
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
use super::qdialog::QDialog; // 773
use std::ops::Deref;
use super::super::gui::qcolor::QColor; // 771
use super::qwidget::QWidget; // 773
use super::super::core::qobject::QObject; // 771
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QColorDialog_Class_Size() -> c_int;
  // proto:  void QColorDialog::QColorDialog(const QColorDialog & );
  fn dector_ZN12QColorDialogC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN12QColorDialogC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QColorDialog::currentColorChanged(const QColor & color);
  fn _ZN12QColorDialog19currentColorChangedERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QColor QColorDialog::currentColor();
  fn _ZNK12QColorDialog12currentColorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QColor QColorDialog::customColor(int index);
  fn _ZN12QColorDialog11customColorEi(arg0: c_int) -> *mut c_void;
  // proto:  const QMetaObject * QColorDialog::metaObject();
  fn _ZNK12QColorDialog10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QColorDialog::QColorDialog(const QColor & initial, QWidget * parent);
  fn dector_ZN12QColorDialogC1ERK6QColorP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN12QColorDialogC1ERK6QColorP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto: static void QColorDialog::setStandardColor(int index, QColor color);
  fn _ZN12QColorDialog16setStandardColorEi6QColor(arg0: c_int, arg1: *mut c_void);
  // proto:  void QColorDialog::open(QObject * receiver, const char * member);
  fn _ZN12QColorDialog4openEP7QObjectPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char);
  // proto:  QColor QColorDialog::selectedColor();
  fn _ZNK12QColorDialog13selectedColorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QColorDialog::~QColorDialog();
  fn _ZN12QColorDialogD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QColorDialog::setVisible(bool visible);
  fn _ZN12QColorDialog10setVisibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QColorDialog::colorSelected(const QColor & color);
  fn _ZN12QColorDialog13colorSelectedERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QColorDialog::setCurrentColor(const QColor & color);
  fn _ZN12QColorDialog15setCurrentColorERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QColor QColorDialog::standardColor(int index);
  fn _ZN12QColorDialog13standardColorEi(arg0: c_int) -> *mut c_void;
  // proto: static QRgb QColorDialog::getRgba(QRgb rgba, bool * ok, QWidget * parent);
  fn _ZN12QColorDialog7getRgbaEjPbP7QWidget(arg0: c_uint, arg1: *mut c_char, arg2: *mut c_void) -> c_uint;
  // proto: static void QColorDialog::setCustomColor(int index, QColor color);
  fn _ZN12QColorDialog14setCustomColorEi6QColor(arg0: c_int, arg1: *mut c_void);
  // proto:  void QColorDialog::QColorDialog(QWidget * parent);
  fn dector_ZN12QColorDialogC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN12QColorDialogC1EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static int QColorDialog::customCount();
  fn _ZN12QColorDialog11customCountEv() -> c_int;
  fn QColorDialog_SlotProxy_connect__ZN12QColorDialog19currentColorChangedERK6QColor(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QColorDialog_SlotProxy_connect_box__ZN12QColorDialog19currentColorChangedERK6QColor(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QColorDialog_SlotProxy_connect__ZN12QColorDialog13colorSelectedERK6QColor(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QColorDialog_SlotProxy_connect_box__ZN12QColorDialog13colorSelectedERK6QColor(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QColorDialog)=1
#[derive(Default)]
pub struct QColorDialog {
  qbase: QDialog,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _colorSelected_1: QColorDialog_colorSelected_signal,
  pub _currentColorChanged_1: QColorDialog_currentColorChanged_signal,
}

impl /*struct*/ QColorDialog {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QColorDialog {
    return QColorDialog{qbase: QDialog::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QColorDialog {
  type Target = QDialog;

  fn deref(&self) -> &QDialog {
    return & self.qbase;
  }
}
impl AsRef<QDialog> for QColorDialog {
  fn as_ref(& self) -> & QDialog {
    return & self.qbase;
  }
}
  // proto:  void QColorDialog::QColorDialog(const QColorDialog & );
impl /*struct*/ QColorDialog {
  pub fn New<T: QColorDialog_New>(value: T) -> QColorDialog {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QColorDialog_New {
  fn New(self) -> QColorDialog;
}

  // proto:  void QColorDialog::QColorDialog(const QColorDialog & );
impl<'a> /*trait*/ QColorDialog_New for (&'a QColorDialog) {
  fn New(self) -> QColorDialog {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialogC1ERKS_()};
    let ctysz: c_int = unsafe{QColorDialog_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN12QColorDialogC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN12QColorDialogC1ERKS_(arg0)} as u64;
    let rsthis = QColorDialog{qbase: QDialog::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QColorDialog::currentColorChanged(const QColor & color);
impl /*struct*/ QColorDialog {
  pub fn currentColorChanged<RetType, T: QColorDialog_currentColorChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentColorChanged(self);
    // return 1;
  }
}

pub trait QColorDialog_currentColorChanged<RetType> {
  fn currentColorChanged(self , rsthis: & QColorDialog) -> RetType;
}

  // proto:  void QColorDialog::currentColorChanged(const QColor & color);
impl<'a> /*trait*/ QColorDialog_currentColorChanged<()> for (&'a QColor) {
  fn currentColorChanged(self , rsthis: & QColorDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog19currentColorChangedERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QColorDialog19currentColorChangedERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QColor QColorDialog::currentColor();
impl /*struct*/ QColorDialog {
  pub fn currentColor<RetType, T: QColorDialog_currentColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentColor(self);
    // return 1;
  }
}

pub trait QColorDialog_currentColor<RetType> {
  fn currentColor(self , rsthis: & QColorDialog) -> RetType;
}

  // proto:  QColor QColorDialog::currentColor();
impl<'a> /*trait*/ QColorDialog_currentColor<QColor> for () {
  fn currentColor(self , rsthis: & QColorDialog) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QColorDialog12currentColorEv()};
    let mut ret = unsafe {_ZNK12QColorDialog12currentColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret as u64);
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
    let mut ret1 = QColor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QColorDialog::metaObject();
impl /*struct*/ QColorDialog {
  pub fn metaObject<RetType, T: QColorDialog_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QColorDialog_metaObject<RetType> {
  fn metaObject(self , rsthis: & QColorDialog) -> RetType;
}

  // proto:  const QMetaObject * QColorDialog::metaObject();
impl<'a> /*trait*/ QColorDialog_metaObject<()> for () {
  fn metaObject(self , rsthis: & QColorDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QColorDialog10metaObjectEv()};
     unsafe {_ZNK12QColorDialog10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QColorDialog::QColorDialog(const QColor & initial, QWidget * parent);
impl<'a> /*trait*/ QColorDialog_New for (&'a QColor, &'a QWidget) {
  fn New(self) -> QColorDialog {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialogC1ERK6QColorP7QWidget()};
    let ctysz: c_int = unsafe{QColorDialog_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN12QColorDialogC1ERK6QColorP7QWidget(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN12QColorDialogC1ERK6QColorP7QWidget(arg0, arg1)} as u64;
    let rsthis = QColorDialog{qbase: QDialog::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn open<RetType, T: QColorDialog_open<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.open(self);
    // return 1;
  }
}

pub trait QColorDialog_open<RetType> {
  fn open(self , rsthis: & QColorDialog) -> RetType;
}

  // proto:  void QColorDialog::open(QObject * receiver, const char * member);
impl<'a> /*trait*/ QColorDialog_open<()> for (&'a QObject, &'a  String) {
  fn open(self , rsthis: & QColorDialog) -> () {
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
  pub fn selectedColor<RetType, T: QColorDialog_selectedColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectedColor(self);
    // return 1;
  }
}

pub trait QColorDialog_selectedColor<RetType> {
  fn selectedColor(self , rsthis: & QColorDialog) -> RetType;
}

  // proto:  QColor QColorDialog::selectedColor();
impl<'a> /*trait*/ QColorDialog_selectedColor<QColor> for () {
  fn selectedColor(self , rsthis: & QColorDialog) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QColorDialog13selectedColorEv()};
    let mut ret = unsafe {_ZNK12QColorDialog13selectedColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QColorDialog::~QColorDialog();
impl /*struct*/ QColorDialog {
  pub fn Free<RetType, T: QColorDialog_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QColorDialog_Free<RetType> {
  fn Free(self , rsthis: & QColorDialog) -> RetType;
}

  // proto:  void QColorDialog::~QColorDialog();
impl<'a> /*trait*/ QColorDialog_Free<()> for () {
  fn Free(self , rsthis: & QColorDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialogD0Ev()};
     unsafe {_ZN12QColorDialogD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QColorDialog::setVisible(bool visible);
impl /*struct*/ QColorDialog {
  pub fn setVisible<RetType, T: QColorDialog_setVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QColorDialog_setVisible<RetType> {
  fn setVisible(self , rsthis: & QColorDialog) -> RetType;
}

  // proto:  void QColorDialog::setVisible(bool visible);
impl<'a> /*trait*/ QColorDialog_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: & QColorDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QColorDialog10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QColorDialog::colorSelected(const QColor & color);
impl /*struct*/ QColorDialog {
  pub fn colorSelected<RetType, T: QColorDialog_colorSelected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.colorSelected(self);
    // return 1;
  }
}

pub trait QColorDialog_colorSelected<RetType> {
  fn colorSelected(self , rsthis: & QColorDialog) -> RetType;
}

  // proto:  void QColorDialog::colorSelected(const QColor & color);
impl<'a> /*trait*/ QColorDialog_colorSelected<()> for (&'a QColor) {
  fn colorSelected(self , rsthis: & QColorDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialog13colorSelectedERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QColorDialog13colorSelectedERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QColorDialog::setCurrentColor(const QColor & color);
impl /*struct*/ QColorDialog {
  pub fn setCurrentColor<RetType, T: QColorDialog_setCurrentColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentColor(self);
    // return 1;
  }
}

pub trait QColorDialog_setCurrentColor<RetType> {
  fn setCurrentColor(self , rsthis: & QColorDialog) -> RetType;
}

  // proto:  void QColorDialog::setCurrentColor(const QColor & color);
impl<'a> /*trait*/ QColorDialog_setCurrentColor<()> for (&'a QColor) {
  fn setCurrentColor(self , rsthis: & QColorDialog) -> () {
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
    let mut ret1 = QColor::inheritFrom(ret as u64);
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
impl<'a> /*trait*/ QColorDialog_getRgba_s<u32> for (u32, &'a mut Vec<i8>, &'a QWidget) {
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
impl<'a> /*trait*/ QColorDialog_New for (&'a QWidget) {
  fn New(self) -> QColorDialog {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QColorDialogC1EP7QWidget()};
    let ctysz: c_int = unsafe{QColorDialog_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN12QColorDialogC1EP7QWidget(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN12QColorDialogC1EP7QWidget(arg0)} as u64;
    let rsthis = QColorDialog{qbase: QDialog::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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

#[derive(Default)] // for QColorDialog_colorSelected
pub struct QColorDialog_colorSelected_signal{poi:u64}
impl /* struct */ QColorDialog {
  pub fn colorSelected_1(&self) -> QColorDialog_colorSelected_signal {
     return QColorDialog_colorSelected_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QColorDialog_colorSelected_signal {
  pub fn connect<T: QColorDialog_colorSelected_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QColorDialog_colorSelected_signal_connect {
  fn connect(self, sigthis: QColorDialog_colorSelected_signal);
}

#[derive(Default)] // for QColorDialog_currentColorChanged
pub struct QColorDialog_currentColorChanged_signal{poi:u64}
impl /* struct */ QColorDialog {
  pub fn currentColorChanged_1(&self) -> QColorDialog_currentColorChanged_signal {
     return QColorDialog_currentColorChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QColorDialog_currentColorChanged_signal {
  pub fn connect<T: QColorDialog_currentColorChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QColorDialog_currentColorChanged_signal_connect {
  fn connect(self, sigthis: QColorDialog_currentColorChanged_signal);
}

// currentColorChanged(const class QColor &)
extern fn QColorDialog_currentColorChanged_signal_connect_cb_0(rsfptr:fn(QColor), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
extern fn QColorDialog_currentColorChanged_signal_connect_cb_box_0(rsfptr_raw:*mut c_void, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QColorDialog_currentColorChanged_signal_connect for fn(QColor) {
  fn connect(self, sigthis: QColorDialog_currentColorChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QColorDialog_currentColorChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QColorDialog_SlotProxy_connect__ZN12QColorDialog19currentColorChangedERK6QColor(arg0, arg1, arg2)};
  }
}
impl /* trait */ QColorDialog_currentColorChanged_signal_connect for Box<fn(QColor)> {
  fn connect(self, sigthis: QColorDialog_currentColorChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QColorDialog_currentColorChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QColorDialog_SlotProxy_connect__ZN12QColorDialog19currentColorChangedERK6QColor(arg0, arg1, arg2)};
  }
}
// colorSelected(const class QColor &)
extern fn QColorDialog_colorSelected_signal_connect_cb_1(rsfptr:fn(QColor), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
extern fn QColorDialog_colorSelected_signal_connect_cb_box_1(rsfptr_raw:*mut c_void, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QColorDialog_colorSelected_signal_connect for fn(QColor) {
  fn connect(self, sigthis: QColorDialog_colorSelected_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QColorDialog_colorSelected_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QColorDialog_SlotProxy_connect__ZN12QColorDialog13colorSelectedERK6QColor(arg0, arg1, arg2)};
  }
}
impl /* trait */ QColorDialog_colorSelected_signal_connect for Box<fn(QColor)> {
  fn connect(self, sigthis: QColorDialog_colorSelected_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QColorDialog_colorSelected_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QColorDialog_SlotProxy_connect__ZN12QColorDialog13colorSelectedERK6QColor(arg0, arg1, arg2)};
  }
}
// <= body block end

