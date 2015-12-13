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

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QFontComboBox::NewQFontComboBox(const QFontComboBox & );
  fn _ZN13QFontComboBoxC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QFontComboBox::FreeQFontComboBox();
  fn _ZN13QFontComboBoxD0Ev() -> i32;
  // proto: const QMetaObject * QFontComboBox::metaObject();
  fn _ZNK13QFontComboBox10metaObjectEv() -> i32;
  // proto: void QFontComboBox::NewQFontComboBox(QWidget * parent);
  fn _ZN13QFontComboBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QSize QFontComboBox::sizeHint();
  fn _ZNK13QFontComboBox8sizeHintEv() -> i32;
  // proto: QFont QFontComboBox::currentFont();
  fn _ZNK13QFontComboBox11currentFontEv() -> i32;
  // proto: void QFontComboBox::currentFontChanged(const QFont & f);
  fn _ZN13QFontComboBox18currentFontChangedERK5QFont(arg0: *const c_void) -> i32;
  // proto: void QFontComboBox::setCurrentFont(const QFont & f);
  fn _ZN13QFontComboBox14setCurrentFontERK5QFont(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QFontComboBox)=1
pub struct QFontComboBox {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFontComboBox {
  pub fn NewQFontComboBox<T: QFontComboBox_NewQFontComboBox>(value: T) -> QFontComboBox {
    let rsthis = value.NewQFontComboBox();
    return rsthis;
    // return 1;
  }
}

pub trait QFontComboBox_NewQFontComboBox {
  fn NewQFontComboBox(self) -> QFontComboBox;
}

// proto: void QFontComboBox::NewQFontComboBox(const QFontComboBox & );
impl<'a> /*trait*/ QFontComboBox_NewQFontComboBox for (&'a  QFontComboBox) {
  fn NewQFontComboBox(self) -> QFontComboBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontComboBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QFontComboBoxC1ERKS_(qthis, arg0)};
    let rsthis = QFontComboBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFontComboBox {
  pub fn FreeQFontComboBox<T: QFontComboBox_FreeQFontComboBox>(&mut self, value: T) -> i32 {
    value.FreeQFontComboBox(self);
    return 1;
  }
}

pub trait QFontComboBox_FreeQFontComboBox {
  fn FreeQFontComboBox(self, this: &mut QFontComboBox) -> i32;
}

// proto: void QFontComboBox::FreeQFontComboBox();
impl<'a> /*trait*/ QFontComboBox_FreeQFontComboBox for () {
  fn FreeQFontComboBox(self, this: &mut QFontComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontComboBoxD0Ev()};
    unsafe {_ZN13QFontComboBoxD0Ev()};
    return 1;
  }
}

impl /*struct*/ QFontComboBox {
  pub fn metaObject<T: QFontComboBox_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QFontComboBox_metaObject {
  fn metaObject(self, this: &mut QFontComboBox) -> i32;
}

// proto: const QMetaObject * QFontComboBox::metaObject();
impl<'a> /*trait*/ QFontComboBox_metaObject for () {
  fn metaObject(self, this: &mut QFontComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontComboBox10metaObjectEv()};
    unsafe {_ZNK13QFontComboBox10metaObjectEv()};
    return 1;
  }
}

// proto: void QFontComboBox::NewQFontComboBox(QWidget * parent);
impl<'a> /*trait*/ QFontComboBox_NewQFontComboBox for (&'a mut QWidget) {
  fn NewQFontComboBox(self) -> QFontComboBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontComboBoxC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QFontComboBoxC1EP7QWidget(qthis, arg0)};
    let rsthis = QFontComboBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFontComboBox {
  pub fn sizeHint<T: QFontComboBox_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QFontComboBox_sizeHint {
  fn sizeHint(self, this: &mut QFontComboBox) -> i32;
}

// proto: QSize QFontComboBox::sizeHint();
impl<'a> /*trait*/ QFontComboBox_sizeHint for () {
  fn sizeHint(self, this: &mut QFontComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontComboBox8sizeHintEv()};
    unsafe {_ZNK13QFontComboBox8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QFontComboBox {
  pub fn currentFont<T: QFontComboBox_currentFont>(&mut self, value: T) -> i32 {
    value.currentFont(self);
    return 1;
  }
}

pub trait QFontComboBox_currentFont {
  fn currentFont(self, this: &mut QFontComboBox) -> i32;
}

// proto: QFont QFontComboBox::currentFont();
impl<'a> /*trait*/ QFontComboBox_currentFont for () {
  fn currentFont(self, this: &mut QFontComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontComboBox11currentFontEv()};
    unsafe {_ZNK13QFontComboBox11currentFontEv()};
    return 1;
  }
}

impl /*struct*/ QFontComboBox {
  pub fn currentFontChanged<T: QFontComboBox_currentFontChanged>(&mut self, value: T) -> i32 {
    value.currentFontChanged(self);
    return 1;
  }
}

pub trait QFontComboBox_currentFontChanged {
  fn currentFontChanged(self, this: &mut QFontComboBox) -> i32;
}

// proto: void QFontComboBox::currentFontChanged(const QFont & f);
impl<'a> /*trait*/ QFontComboBox_currentFontChanged for (&'a  QFont) {
  fn currentFontChanged(self, this: &mut QFontComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontComboBox18currentFontChangedERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QFontComboBox18currentFontChangedERK5QFont(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontComboBox {
  pub fn setCurrentFont<T: QFontComboBox_setCurrentFont>(&mut self, value: T) -> i32 {
    value.setCurrentFont(self);
    return 1;
  }
}

pub trait QFontComboBox_setCurrentFont {
  fn setCurrentFont(self, this: &mut QFontComboBox) -> i32;
}

// proto: void QFontComboBox::setCurrentFont(const QFont & f);
impl<'a> /*trait*/ QFontComboBox_setCurrentFont for (&'a  QFont) {
  fn setCurrentFont(self, this: &mut QFontComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontComboBox14setCurrentFontERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QFontComboBox14setCurrentFontERK5QFont(arg0)};
    return 1;
  }
}

