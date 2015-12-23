// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
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
use super::qcombobox::QComboBox; // 773
use std::ops::Deref;
use super::qwidget::QWidget; // 773
use super::super::core::qsize::QSize; // 771
use super::super::gui::qfont::QFont; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QFontComboBox::QFontComboBox(const QFontComboBox & );
  fn _ZN13QFontComboBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QFontComboBox::~QFontComboBox();
  fn _ZN13QFontComboBoxD0Ev(qthis: *mut c_void);
  // proto:  const QMetaObject * QFontComboBox::metaObject();
  fn _ZNK13QFontComboBox10metaObjectEv(qthis: *mut c_void);
  // proto:  void QFontComboBox::QFontComboBox(QWidget * parent);
  fn _ZN13QFontComboBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSize QFontComboBox::sizeHint();
  fn _ZNK13QFontComboBox8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QFont QFontComboBox::currentFont();
  fn _ZNK13QFontComboBox11currentFontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFontComboBox::currentFontChanged(const QFont & f);
  fn _ZN13QFontComboBox18currentFontChangedERK5QFont(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QFontComboBox::setCurrentFont(const QFont & f);
  fn _ZN13QFontComboBox14setCurrentFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QFontComboBox)=1
pub struct QFontComboBox {
  qbase: QComboBox,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFontComboBox {
  pub fn inheritFrom(qthis: *mut c_void) -> QFontComboBox {
    return QFontComboBox{qbase: QComboBox::inheritFrom(qthis), qclsinst: qthis};
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
  // proto:  void QFontComboBox::QFontComboBox(const QFontComboBox & );
impl /*struct*/ QFontComboBox {
  pub fn New<T: QFontComboBox_New>(value: T) -> QFontComboBox {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QFontComboBox_New {
  fn New(self) -> QFontComboBox;
}

  // proto:  void QFontComboBox::QFontComboBox(const QFontComboBox & );
impl<'a> /*trait*/ QFontComboBox_New for (&'a QFontComboBox) {
  fn New(self) -> QFontComboBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontComboBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QFontComboBoxC1ERKS_(qthis, arg0)};
    let rsthis = QFontComboBox{/**/qbase: QComboBox::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFontComboBox::~QFontComboBox();
impl /*struct*/ QFontComboBox {
  pub fn Free<RetType, T: QFontComboBox_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QFontComboBox_Free<RetType> {
  fn Free(self , rsthis: & QFontComboBox) -> RetType;
}

  // proto:  void QFontComboBox::~QFontComboBox();
impl<'a> /*trait*/ QFontComboBox_Free<()> for () {
  fn Free(self , rsthis: & QFontComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontComboBoxD0Ev()};
     unsafe {_ZN13QFontComboBoxD0Ev(rsthis.qclsinst)};
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
impl<'a> /*trait*/ QFontComboBox_metaObject<()> for () {
  fn metaObject(self , rsthis: & QFontComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontComboBox10metaObjectEv()};
     unsafe {_ZNK13QFontComboBox10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFontComboBox::QFontComboBox(QWidget * parent);
impl<'a> /*trait*/ QFontComboBox_New for (&'a QWidget) {
  fn New(self) -> QFontComboBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontComboBoxC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QFontComboBoxC1EP7QWidget(qthis, arg0)};
    let rsthis = QFontComboBox{/**/qbase: QComboBox::inheritFrom(qthis), /**/qclsinst: qthis};
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
    let mut ret = unsafe {_ZNK13QFontComboBox8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
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
    let mut ret = unsafe {_ZNK13QFontComboBox11currentFontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFontComboBox::currentFontChanged(const QFont & f);
impl /*struct*/ QFontComboBox {
  pub fn currentFontChanged<RetType, T: QFontComboBox_currentFontChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentFontChanged(self);
    // return 1;
  }
}

pub trait QFontComboBox_currentFontChanged<RetType> {
  fn currentFontChanged(self , rsthis: & QFontComboBox) -> RetType;
}

  // proto:  void QFontComboBox::currentFontChanged(const QFont & f);
impl<'a> /*trait*/ QFontComboBox_currentFontChanged<()> for (&'a QFont) {
  fn currentFontChanged(self , rsthis: & QFontComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontComboBox18currentFontChangedERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QFontComboBox18currentFontChangedERK5QFont(rsthis.qclsinst, arg0)};
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
     unsafe {_ZN13QFontComboBox14setCurrentFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

