// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qsize::QSize;
use super::qfont::QFont;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QFontComboBox::NewQFontComboBox(const QFontComboBox & );
  fn _ZN13QFontComboBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFontComboBox::FreeQFontComboBox();
  fn _ZN13QFontComboBoxD0Ev(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QFontComboBox::metaObject();
  fn _ZNK13QFontComboBox10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QFontComboBox::NewQFontComboBox(QWidget * parent);
  fn _ZN13QFontComboBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSize QFontComboBox::sizeHint();
  fn _ZNK13QFontComboBox8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QFont QFontComboBox::currentFont();
  fn _ZNK13QFontComboBox11currentFontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFontComboBox::currentFontChanged(const QFont & f);
  fn _ZN13QFontComboBox18currentFontChangedERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFontComboBox::setCurrentFont(const QFont & f);
  fn _ZN13QFontComboBox14setCurrentFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QFontComboBoxC1ERKS_(qthis, arg0)};
    let rsthis = QFontComboBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFontComboBox {
  pub fn FreeQFontComboBox<T: QFontComboBox_FreeQFontComboBox>(&mut self, value: T)  {
     value.FreeQFontComboBox(self);
    // return 1;
  }
}

pub trait QFontComboBox_FreeQFontComboBox {
  fn FreeQFontComboBox(self, rsthis: &mut QFontComboBox) ;
}

// proto:  void QFontComboBox::FreeQFontComboBox();
impl<'a> /*trait*/ QFontComboBox_FreeQFontComboBox for () {
  fn FreeQFontComboBox(self, rsthis: &mut QFontComboBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontComboBoxD0Ev()};
     unsafe {_ZN13QFontComboBoxD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFontComboBox {
  pub fn metaObject<T: QFontComboBox_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QFontComboBox_metaObject {
  fn metaObject(self, rsthis: &mut QFontComboBox) ;
}

// proto:  const QMetaObject * QFontComboBox::metaObject();
impl<'a> /*trait*/ QFontComboBox_metaObject for () {
  fn metaObject(self, rsthis: &mut QFontComboBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontComboBox10metaObjectEv()};
     unsafe {_ZNK13QFontComboBox10metaObjectEv(rsthis.qclsinst)};
    // return 1;
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
  pub fn sizeHint<T: QFontComboBox_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QFontComboBox_sizeHint {
  fn sizeHint(self, rsthis: &mut QFontComboBox) -> QSize;
}

// proto:  QSize QFontComboBox::sizeHint();
impl<'a> /*trait*/ QFontComboBox_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QFontComboBox) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontComboBox8sizeHintEv()};
    let mut ret = unsafe {_ZNK13QFontComboBox8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFontComboBox {
  pub fn currentFont<T: QFontComboBox_currentFont>(&mut self, value: T) -> QFont {
    return value.currentFont(self);
    // return 1;
  }
}

pub trait QFontComboBox_currentFont {
  fn currentFont(self, rsthis: &mut QFontComboBox) -> QFont;
}

// proto:  QFont QFontComboBox::currentFont();
impl<'a> /*trait*/ QFontComboBox_currentFont for () {
  fn currentFont(self, rsthis: &mut QFontComboBox) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontComboBox11currentFontEv()};
    let mut ret = unsafe {_ZNK13QFontComboBox11currentFontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFontComboBox {
  pub fn currentFontChanged<T: QFontComboBox_currentFontChanged>(&mut self, value: T)  {
     value.currentFontChanged(self);
    // return 1;
  }
}

pub trait QFontComboBox_currentFontChanged {
  fn currentFontChanged(self, rsthis: &mut QFontComboBox) ;
}

// proto:  void QFontComboBox::currentFontChanged(const QFont & f);
impl<'a> /*trait*/ QFontComboBox_currentFontChanged for (&'a  QFont) {
  fn currentFontChanged(self, rsthis: &mut QFontComboBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontComboBox18currentFontChangedERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QFontComboBox18currentFontChangedERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFontComboBox {
  pub fn setCurrentFont<T: QFontComboBox_setCurrentFont>(&mut self, value: T)  {
     value.setCurrentFont(self);
    // return 1;
  }
}

pub trait QFontComboBox_setCurrentFont {
  fn setCurrentFont(self, rsthis: &mut QFontComboBox) ;
}

// proto:  void QFontComboBox::setCurrentFont(const QFont & f);
impl<'a> /*trait*/ QFontComboBox_setCurrentFont for (&'a  QFont) {
  fn setCurrentFont(self, rsthis: &mut QFontComboBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontComboBox14setCurrentFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QFontComboBox14setCurrentFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

