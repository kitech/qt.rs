// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtWidgets/qfontdialog.h
// dst-file: /src/widgets/qfontdialog.rs
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
use super::qwidget::QWidget; // 773
use super::super::core::qobject::QObject; // 771
use super::super::gui::qfont::QFont; // 771
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QFontDialog_Class_Size() -> c_int;
  // proto:  void QFontDialog::QFontDialog(QWidget * parent);
  fn _ZN11QFontDialogC2EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QFontDialog::QFontDialog(const QFontDialog & );
  fn _ZN11QFontDialogC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QFontDialog::open(QObject * receiver, const char * member);
  fn _ZN11QFontDialog4openEP7QObjectPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char);
  // proto:  void QFontDialog::QFontDialog(const QFont & initial, QWidget * parent);
  fn _ZN11QFontDialogC2ERK5QFontP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QFont QFontDialog::currentFont();
  fn _ZNK11QFontDialog11currentFontEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QFontDialog::setVisible(bool visible);
  fn _ZN11QFontDialog10setVisibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QFontDialog::~QFontDialog();
  fn _ZN11QFontDialogD2Ev(qthis: u64 /* *mut c_void*/);
  // proto: static QFont QFontDialog::getFont(bool * ok, QWidget * parent);
  fn _ZN11QFontDialog7getFontEPbP7QWidget(arg0: *mut c_char, arg1: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QFontDialog::metaObject();
  fn _ZNK11QFontDialog10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QFont QFontDialog::selectedFont();
  fn _ZNK11QFontDialog12selectedFontEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QFontDialog::setCurrentFont(const QFont & font);
  fn _ZN11QFontDialog14setCurrentFontERK5QFont(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QFontDialog_SlotProxy_connect__ZN11QFontDialog18currentFontChangedERK5QFont(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QFontDialog_SlotProxy_connect__ZN11QFontDialog12fontSelectedERK5QFont(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QFontDialog)=1
#[derive(Default)]
pub struct QFontDialog {
  qbase: QDialog,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _fontSelected: QFontDialog_fontSelected_signal,
  pub _currentFontChanged: QFontDialog_currentFontChanged_signal,
}

impl /*struct*/ QFontDialog {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFontDialog {
    return QFontDialog{qbase: QDialog::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QFontDialog {
  type Target = QDialog;

  fn deref(&self) -> &QDialog {
    return & self.qbase;
  }
}
impl AsRef<QDialog> for QFontDialog {
  fn as_ref(& self) -> & QDialog {
    return & self.qbase;
  }
}
  // proto:  void QFontDialog::QFontDialog(QWidget * parent);
impl /*struct*/ QFontDialog {
  pub fn new<T: QFontDialog_new>(value: T) -> QFontDialog {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QFontDialog_new {
  fn new(self) -> QFontDialog;
}

  // proto:  void QFontDialog::QFontDialog(QWidget * parent);
impl<'a> /*trait*/ QFontDialog_new for (&'a QWidget) {
  fn new(self) -> QFontDialog {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialogC2EP7QWidget()};
    let ctysz: c_int = unsafe{QFontDialog_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QFontDialogC2EP7QWidget(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QFontDialog{qbase: QDialog::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFontDialog::QFontDialog(const QFontDialog & );
impl<'a> /*trait*/ QFontDialog_new for (&'a QFontDialog) {
  fn new(self) -> QFontDialog {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialogC2ERKS_()};
    let ctysz: c_int = unsafe{QFontDialog_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QFontDialogC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QFontDialog{qbase: QDialog::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFontDialog::open(QObject * receiver, const char * member);
impl /*struct*/ QFontDialog {
  pub fn open<RetType, T: QFontDialog_open<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.open(self);
    // return 1;
  }
}

pub trait QFontDialog_open<RetType> {
  fn open(self , rsthis: & QFontDialog) -> RetType;
}

  // proto:  void QFontDialog::open(QObject * receiver, const char * member);
impl<'a> /*trait*/ QFontDialog_open<()> for (&'a QObject, &'a  String) {
  fn open(self , rsthis: & QFontDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialog4openEP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {_ZN11QFontDialog4openEP7QObjectPKc(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QFontDialog::QFontDialog(const QFont & initial, QWidget * parent);
impl<'a> /*trait*/ QFontDialog_new for (&'a QFont, &'a QWidget) {
  fn new(self) -> QFontDialog {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialogC2ERK5QFontP7QWidget()};
    let ctysz: c_int = unsafe{QFontDialog_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN11QFontDialogC2ERK5QFontP7QWidget(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QFontDialog{qbase: QDialog::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QFont QFontDialog::currentFont();
impl /*struct*/ QFontDialog {
  pub fn currentFont<RetType, T: QFontDialog_currentFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentFont(self);
    // return 1;
  }
}

pub trait QFontDialog_currentFont<RetType> {
  fn currentFont(self , rsthis: & QFontDialog) -> RetType;
}

  // proto:  QFont QFontDialog::currentFont();
impl<'a> /*trait*/ QFontDialog_currentFont<QFont> for () {
  fn currentFont(self , rsthis: & QFontDialog) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFontDialog11currentFontEv()};
    let mut ret = unsafe {_ZNK11QFontDialog11currentFontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFontDialog::setVisible(bool visible);
impl /*struct*/ QFontDialog {
  pub fn setVisible<RetType, T: QFontDialog_setVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QFontDialog_setVisible<RetType> {
  fn setVisible(self , rsthis: & QFontDialog) -> RetType;
}

  // proto:  void QFontDialog::setVisible(bool visible);
impl<'a> /*trait*/ QFontDialog_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: & QFontDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialog10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QFontDialog10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QFontDialog::~QFontDialog();
impl /*struct*/ QFontDialog {
  pub fn free<RetType, T: QFontDialog_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QFontDialog_free<RetType> {
  fn free(self , rsthis: & QFontDialog) -> RetType;
}

  // proto:  void QFontDialog::~QFontDialog();
impl<'a> /*trait*/ QFontDialog_free<()> for () {
  fn free(self , rsthis: & QFontDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialogD2Ev()};
     unsafe {_ZN11QFontDialogD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QFont QFontDialog::getFont(bool * ok, QWidget * parent);
impl /*struct*/ QFontDialog {
  pub fn getFont_s<RetType, T: QFontDialog_getFont_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.getFont_s();
    // return 1;
  }
}

pub trait QFontDialog_getFont_s<RetType> {
  fn getFont_s(self ) -> RetType;
}

  // proto: static QFont QFontDialog::getFont(bool * ok, QWidget * parent);
impl<'a> /*trait*/ QFontDialog_getFont_s<QFont> for (&'a mut Vec<i8>, &'a QWidget) {
  fn getFont_s(self ) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialog7getFontEPbP7QWidget()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QFontDialog7getFontEPbP7QWidget(arg0, arg1)};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QFontDialog::metaObject();
impl /*struct*/ QFontDialog {
  pub fn metaObject<RetType, T: QFontDialog_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QFontDialog_metaObject<RetType> {
  fn metaObject(self , rsthis: & QFontDialog) -> RetType;
}

  // proto:  const QMetaObject * QFontDialog::metaObject();
impl<'a> /*trait*/ QFontDialog_metaObject<()> for () {
  fn metaObject(self , rsthis: & QFontDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFontDialog10metaObjectEv()};
     unsafe {_ZNK11QFontDialog10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QFont QFontDialog::selectedFont();
impl /*struct*/ QFontDialog {
  pub fn selectedFont<RetType, T: QFontDialog_selectedFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectedFont(self);
    // return 1;
  }
}

pub trait QFontDialog_selectedFont<RetType> {
  fn selectedFont(self , rsthis: & QFontDialog) -> RetType;
}

  // proto:  QFont QFontDialog::selectedFont();
impl<'a> /*trait*/ QFontDialog_selectedFont<QFont> for () {
  fn selectedFont(self , rsthis: & QFontDialog) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFontDialog12selectedFontEv()};
    let mut ret = unsafe {_ZNK11QFontDialog12selectedFontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFontDialog::setCurrentFont(const QFont & font);
impl /*struct*/ QFontDialog {
  pub fn setCurrentFont<RetType, T: QFontDialog_setCurrentFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentFont(self);
    // return 1;
  }
}

pub trait QFontDialog_setCurrentFont<RetType> {
  fn setCurrentFont(self , rsthis: & QFontDialog) -> RetType;
}

  // proto:  void QFontDialog::setCurrentFont(const QFont & font);
impl<'a> /*trait*/ QFontDialog_setCurrentFont<()> for (&'a QFont) {
  fn setCurrentFont(self , rsthis: & QFontDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialog14setCurrentFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFontDialog14setCurrentFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QFontDialog_fontSelected
pub struct QFontDialog_fontSelected_signal{poi:u64}
impl /* struct */ QFontDialog {
  pub fn fontSelected(&self) -> QFontDialog_fontSelected_signal {
     return QFontDialog_fontSelected_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QFontDialog_fontSelected_signal {
  pub fn connect<T: QFontDialog_fontSelected_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QFontDialog_fontSelected_signal_connect {
  fn connect(self, sigthis: QFontDialog_fontSelected_signal);
}

#[derive(Default)] // for QFontDialog_currentFontChanged
pub struct QFontDialog_currentFontChanged_signal{poi:u64}
impl /* struct */ QFontDialog {
  pub fn currentFontChanged(&self) -> QFontDialog_currentFontChanged_signal {
     return QFontDialog_currentFontChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QFontDialog_currentFontChanged_signal {
  pub fn connect<T: QFontDialog_currentFontChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QFontDialog_currentFontChanged_signal_connect {
  fn connect(self, sigthis: QFontDialog_currentFontChanged_signal);
}

// currentFontChanged(const class QFont &)
extern fn QFontDialog_currentFontChanged_signal_connect_cb_0(rsfptr:fn(QFont), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QFont::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QFontDialog_currentFontChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QFont)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QFont::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QFontDialog_currentFontChanged_signal_connect for fn(QFont) {
  fn connect(self, sigthis: QFontDialog_currentFontChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFontDialog_currentFontChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QFontDialog_SlotProxy_connect__ZN11QFontDialog18currentFontChangedERK5QFont(arg0, arg1, arg2)};
  }
}
impl /* trait */ QFontDialog_currentFontChanged_signal_connect for Box<Fn(QFont)> {
  fn connect(self, sigthis: QFontDialog_currentFontChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFontDialog_currentFontChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QFontDialog_SlotProxy_connect__ZN11QFontDialog18currentFontChangedERK5QFont(arg0, arg1, arg2)};
  }
}
// fontSelected(const class QFont &)
extern fn QFontDialog_fontSelected_signal_connect_cb_1(rsfptr:fn(QFont), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QFont::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QFontDialog_fontSelected_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(QFont)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QFont::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QFontDialog_fontSelected_signal_connect for fn(QFont) {
  fn connect(self, sigthis: QFontDialog_fontSelected_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFontDialog_fontSelected_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QFontDialog_SlotProxy_connect__ZN11QFontDialog12fontSelectedERK5QFont(arg0, arg1, arg2)};
  }
}
impl /* trait */ QFontDialog_fontSelected_signal_connect for Box<Fn(QFont)> {
  fn connect(self, sigthis: QFontDialog_fontSelected_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFontDialog_fontSelected_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QFontDialog_SlotProxy_connect__ZN11QFontDialog12fontSelectedERK5QFont(arg0, arg1, arg2)};
  }
}
// <= body block end

