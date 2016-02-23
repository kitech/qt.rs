// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qfontcombobox.h
// dst-file: /src/widgets/qfontcombobox.rs
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
use super::qcombobox::*; // 773
use std::ops::Deref;
use super::super::core::qobjectdefs::*; // 771
use super::qwidget::*; // 773
use super::super::core::qsize::*; // 771
use super::super::gui::qfont::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QFontComboBox_Class_Size() -> c_int;
  // proto:  void QFontComboBox::~QFontComboBox();
  fn C_ZN13QFontComboBoxD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QFontComboBox::metaObject();
  fn C_ZNK13QFontComboBox10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QFontComboBox::QFontComboBox(QWidget * parent);
  fn C_ZN13QFontComboBoxC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  QSize QFontComboBox::sizeHint();
  fn C_ZNK13QFontComboBox8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QFont QFontComboBox::currentFont();
  fn C_ZNK13QFontComboBox11currentFontEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QFontComboBox::setCurrentFont(const QFont & f);
  fn C_ZN13QFontComboBox14setCurrentFontERK5QFont(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QFontComboBox_SlotProxy_connect__ZN13QFontComboBox18currentFontChangedERK5QFont(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QFontComboBox)=1
#[derive(Default)]
pub struct QFontComboBox {
  qbase: QComboBox,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _currentFontChanged: QFontComboBox_currentFontChanged_signal,
}

impl /*struct*/ QFontComboBox {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFontComboBox {
    return QFontComboBox{qbase: QComboBox::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QFontComboBox {
  type Target = QComboBox;

  fn deref(&self) -> &QComboBox {
    return & self.qbase;
  }
}
impl AsRef<QComboBox> for QFontComboBox {
  fn as_ref(& self) -> & QComboBox {
    return & self.qbase;
  }
}
  // proto:  void QFontComboBox::~QFontComboBox();
impl /*struct*/ QFontComboBox {
  pub fn free<RetType, T: QFontComboBox_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QFontComboBox_free<RetType> {
  fn free(self , rsthis: & QFontComboBox) -> RetType;
}

  // proto:  void QFontComboBox::~QFontComboBox();
impl<'a> /*trait*/ QFontComboBox_free<()> for () {
  fn free(self , rsthis: & QFontComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontComboBoxD2Ev()};
     unsafe {C_ZN13QFontComboBoxD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QFontComboBox::metaObject();
impl /*struct*/ QFontComboBox {
  pub fn metaObject<RetType, T: QFontComboBox_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QFontComboBox_metaObject<RetType> {
  fn metaObject(self , rsthis: & QFontComboBox) -> RetType;
}

  // proto:  const QMetaObject * QFontComboBox::metaObject();
impl<'a> /*trait*/ QFontComboBox_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QFontComboBox) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontComboBox10metaObjectEv()};
    let mut ret = unsafe {C_ZNK13QFontComboBox10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFontComboBox::QFontComboBox(QWidget * parent);
impl /*struct*/ QFontComboBox {
  pub fn new<T: QFontComboBox_new>(value: T) -> QFontComboBox {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QFontComboBox_new {
  fn new(self) -> QFontComboBox;
}

  // proto:  void QFontComboBox::QFontComboBox(QWidget * parent);
impl<'a> /*trait*/ QFontComboBox_new for (Option<&'a QWidget>) {
  fn new(self) -> QFontComboBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontComboBoxC2EP7QWidget()};
    let ctysz: c_int = unsafe{QFontComboBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QFontComboBoxC2EP7QWidget(arg0)};
    let rsthis = QFontComboBox{qbase: QComboBox::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSize QFontComboBox::sizeHint();
impl /*struct*/ QFontComboBox {
  pub fn sizeHint<RetType, T: QFontComboBox_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QFontComboBox_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QFontComboBox) -> RetType;
}

  // proto:  QSize QFontComboBox::sizeHint();
impl<'a> /*trait*/ QFontComboBox_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QFontComboBox) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontComboBox8sizeHintEv()};
    let mut ret = unsafe {C_ZNK13QFontComboBox8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QFont QFontComboBox::currentFont();
impl /*struct*/ QFontComboBox {
  pub fn currentFont<RetType, T: QFontComboBox_currentFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentFont(self);
    // return 1;
  }
}

pub trait QFontComboBox_currentFont<RetType> {
  fn currentFont(self , rsthis: & QFontComboBox) -> RetType;
}

  // proto:  QFont QFontComboBox::currentFont();
impl<'a> /*trait*/ QFontComboBox_currentFont<QFont> for () {
  fn currentFont(self , rsthis: & QFontComboBox) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontComboBox11currentFontEv()};
    let mut ret = unsafe {C_ZNK13QFontComboBox11currentFontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFontComboBox::setCurrentFont(const QFont & f);
impl /*struct*/ QFontComboBox {
  pub fn setCurrentFont<RetType, T: QFontComboBox_setCurrentFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentFont(self);
    // return 1;
  }
}

pub trait QFontComboBox_setCurrentFont<RetType> {
  fn setCurrentFont(self , rsthis: & QFontComboBox) -> RetType;
}

  // proto:  void QFontComboBox::setCurrentFont(const QFont & f);
impl<'a> /*trait*/ QFontComboBox_setCurrentFont<()> for (&'a QFont) {
  fn setCurrentFont(self , rsthis: & QFontComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontComboBox14setCurrentFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QFontComboBox14setCurrentFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QFontComboBox_currentFontChanged
pub struct QFontComboBox_currentFontChanged_signal{poi:u64}
impl /* struct */ QFontComboBox {
  pub fn currentFontChanged(&self) -> QFontComboBox_currentFontChanged_signal {
     return QFontComboBox_currentFontChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QFontComboBox_currentFontChanged_signal {
  pub fn connect<T: QFontComboBox_currentFontChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QFontComboBox_currentFontChanged_signal_connect {
  fn connect(self, sigthis: QFontComboBox_currentFontChanged_signal);
}

// currentFontChanged(const class QFont &)
extern fn QFontComboBox_currentFontChanged_signal_connect_cb_0(rsfptr:fn(QFont), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QFont::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QFontComboBox_currentFontChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QFont)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QFont::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QFontComboBox_currentFontChanged_signal_connect for fn(QFont) {
  fn connect(self, sigthis: QFontComboBox_currentFontChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFontComboBox_currentFontChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QFontComboBox_SlotProxy_connect__ZN13QFontComboBox18currentFontChangedERK5QFont(arg0, arg1, arg2)};
  }
}
impl /* trait */ QFontComboBox_currentFontChanged_signal_connect for Box<Fn(QFont)> {
  fn connect(self, sigthis: QFontComboBox_currentFontChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFontComboBox_currentFontChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QFontComboBox_SlotProxy_connect__ZN13QFontComboBox18currentFontChangedERK5QFont(arg0, arg1, arg2)};
  }
}
// <= body block end

