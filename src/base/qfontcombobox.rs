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

// proto:  void QFontComboBox::FreeQFontComboBox();
impl /*struct*/ QFontComboBox {
  pub fn FreeQFontComboBox<RetType, T: QFontComboBox_FreeQFontComboBox<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQFontComboBox(self);
    // return 1;
  }
}

pub trait QFontComboBox_FreeQFontComboBox<RetType> {
  fn FreeQFontComboBox(self , rsthis: &mut QFontComboBox) -> RetType;
}

// proto:  void QFontComboBox::FreeQFontComboBox();
impl<'a> /*trait*/ QFontComboBox_FreeQFontComboBox<()> for () {
  fn FreeQFontComboBox(self , rsthis: &mut QFontComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontComboBoxD0Ev()};
     unsafe {_ZN13QFontComboBoxD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  const QMetaObject * QFontComboBox::metaObject();
impl /*struct*/ QFontComboBox {
  pub fn metaObject<RetType, T: QFontComboBox_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QFontComboBox_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QFontComboBox) -> RetType;
}

// proto:  const QMetaObject * QFontComboBox::metaObject();
impl<'a> /*trait*/ QFontComboBox_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QFontComboBox) -> () {
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

// proto:  QSize QFontComboBox::sizeHint();
impl /*struct*/ QFontComboBox {
  pub fn sizeHint<RetType, T: QFontComboBox_sizeHint<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QFontComboBox_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QFontComboBox) -> RetType;
}

// proto:  QSize QFontComboBox::sizeHint();
impl<'a> /*trait*/ QFontComboBox_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QFontComboBox) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontComboBox8sizeHintEv()};
    let mut ret = unsafe {_ZNK13QFontComboBox8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QFont QFontComboBox::currentFont();
impl /*struct*/ QFontComboBox {
  pub fn currentFont<RetType, T: QFontComboBox_currentFont<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.currentFont(self);
    // return 1;
  }
}

pub trait QFontComboBox_currentFont<RetType> {
  fn currentFont(self , rsthis: &mut QFontComboBox) -> RetType;
}

// proto:  QFont QFontComboBox::currentFont();
impl<'a> /*trait*/ QFontComboBox_currentFont<QFont> for () {
  fn currentFont(self , rsthis: &mut QFontComboBox) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontComboBox11currentFontEv()};
    let mut ret = unsafe {_ZNK13QFontComboBox11currentFontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QFontComboBox::currentFontChanged(const QFont & f);
impl /*struct*/ QFontComboBox {
  pub fn currentFontChanged<RetType, T: QFontComboBox_currentFontChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.currentFontChanged(self);
    // return 1;
  }
}

pub trait QFontComboBox_currentFontChanged<RetType> {
  fn currentFontChanged(self , rsthis: &mut QFontComboBox) -> RetType;
}

// proto:  void QFontComboBox::currentFontChanged(const QFont & f);
impl<'a> /*trait*/ QFontComboBox_currentFontChanged<()> for (&'a  QFont) {
  fn currentFontChanged(self , rsthis: &mut QFontComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontComboBox18currentFontChangedERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QFontComboBox18currentFontChangedERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QFontComboBox::setCurrentFont(const QFont & f);
impl /*struct*/ QFontComboBox {
  pub fn setCurrentFont<RetType, T: QFontComboBox_setCurrentFont<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setCurrentFont(self);
    // return 1;
  }
}

pub trait QFontComboBox_setCurrentFont<RetType> {
  fn setCurrentFont(self , rsthis: &mut QFontComboBox) -> RetType;
}

// proto:  void QFontComboBox::setCurrentFont(const QFont & f);
impl<'a> /*trait*/ QFontComboBox_setCurrentFont<()> for (&'a  QFont) {
  fn setCurrentFont(self , rsthis: &mut QFontComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontComboBox14setCurrentFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QFontComboBox14setCurrentFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

