// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QGroupBox::isCheckable();
  fn _ZNK9QGroupBox11isCheckableEv() -> i32;
  // proto: void QGroupBox::setCheckable(bool checkable);
  fn _ZN9QGroupBox12setCheckableEb(arg0: int8_t) -> i32;
  // proto: const QMetaObject * QGroupBox::metaObject();
  fn _ZNK9QGroupBox10metaObjectEv() -> i32;
  // proto: bool QGroupBox::isFlat();
  fn _ZNK9QGroupBox6isFlatEv() -> i32;
  // proto: QSize QGroupBox::minimumSizeHint();
  fn _ZNK9QGroupBox15minimumSizeHintEv() -> i32;
  // proto: void QGroupBox::setFlat(bool flat);
  fn _ZN9QGroupBox7setFlatEb(arg0: int8_t) -> i32;
  // proto: void QGroupBox::FreeQGroupBox();
  fn _ZN9QGroupBoxD0Ev() -> i32;
  // proto: void QGroupBox::NewQGroupBox(QWidget * parent);
  fn _ZN9QGroupBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QGroupBox::toggled(bool );
  fn _ZN9QGroupBox7toggledEb(arg0: int8_t) -> i32;
  // proto: bool QGroupBox::isChecked();
  fn _ZNK9QGroupBox9isCheckedEv() -> i32;
  // proto: void QGroupBox::setChecked(bool checked);
  fn _ZN9QGroupBox10setCheckedEb(arg0: int8_t) -> i32;
  // proto: void QGroupBox::NewQGroupBox(const QGroupBox & );
  fn _ZN9QGroupBoxC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QString QGroupBox::title();
  fn _ZNK9QGroupBox5titleEv() -> i32;
  // proto: void QGroupBox::setAlignment(int alignment);
  fn _ZN9QGroupBox12setAlignmentEi(arg0: c_int) -> i32;
  // proto: void QGroupBox::setTitle(const QString & title);
  fn _ZN9QGroupBox8setTitleERK7QString(arg0: *const c_void) -> i32;
  // proto: void QGroupBox::NewQGroupBox(const QString & title, QWidget * parent);
  fn _ZN9QGroupBoxC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: void QGroupBox::clicked(bool checked);
  fn _ZN9QGroupBox7clickedEb(arg0: int8_t) -> i32;
}

// body block begin
// class sizeof(QGroupBox)=1
pub struct QGroupBox {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGroupBox {
  pub fn isCheckable<T: QGroupBox_isCheckable>(&mut self, value: T) -> i32 {
    value.isCheckable(self);
    return 1;
  }
}

pub trait QGroupBox_isCheckable {
  fn isCheckable(self, this: &mut QGroupBox) -> i32;
}

// proto: bool QGroupBox::isCheckable();
impl<'a> /*trait*/ QGroupBox_isCheckable for () {
  fn isCheckable(self, this: &mut QGroupBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox11isCheckableEv()};
    unsafe {_ZNK9QGroupBox11isCheckableEv()};
    return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn setCheckable<T: QGroupBox_setCheckable>(&mut self, value: T) -> i32 {
    value.setCheckable(self);
    return 1;
  }
}

pub trait QGroupBox_setCheckable {
  fn setCheckable(self, this: &mut QGroupBox) -> i32;
}

// proto: void QGroupBox::setCheckable(bool checkable);
impl<'a> /*trait*/ QGroupBox_setCheckable for (i8) {
  fn setCheckable(self, this: &mut QGroupBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox12setCheckableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QGroupBox12setCheckableEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn metaObject<T: QGroupBox_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QGroupBox_metaObject {
  fn metaObject(self, this: &mut QGroupBox) -> i32;
}

// proto: const QMetaObject * QGroupBox::metaObject();
impl<'a> /*trait*/ QGroupBox_metaObject for () {
  fn metaObject(self, this: &mut QGroupBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox10metaObjectEv()};
    unsafe {_ZNK9QGroupBox10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn isFlat<T: QGroupBox_isFlat>(&mut self, value: T) -> i32 {
    value.isFlat(self);
    return 1;
  }
}

pub trait QGroupBox_isFlat {
  fn isFlat(self, this: &mut QGroupBox) -> i32;
}

// proto: bool QGroupBox::isFlat();
impl<'a> /*trait*/ QGroupBox_isFlat for () {
  fn isFlat(self, this: &mut QGroupBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox6isFlatEv()};
    unsafe {_ZNK9QGroupBox6isFlatEv()};
    return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn minimumSizeHint<T: QGroupBox_minimumSizeHint>(&mut self, value: T) -> i32 {
    value.minimumSizeHint(self);
    return 1;
  }
}

pub trait QGroupBox_minimumSizeHint {
  fn minimumSizeHint(self, this: &mut QGroupBox) -> i32;
}

// proto: QSize QGroupBox::minimumSizeHint();
impl<'a> /*trait*/ QGroupBox_minimumSizeHint for () {
  fn minimumSizeHint(self, this: &mut QGroupBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox15minimumSizeHintEv()};
    unsafe {_ZNK9QGroupBox15minimumSizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn setFlat<T: QGroupBox_setFlat>(&mut self, value: T) -> i32 {
    value.setFlat(self);
    return 1;
  }
}

pub trait QGroupBox_setFlat {
  fn setFlat(self, this: &mut QGroupBox) -> i32;
}

// proto: void QGroupBox::setFlat(bool flat);
impl<'a> /*trait*/ QGroupBox_setFlat for (i8) {
  fn setFlat(self, this: &mut QGroupBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox7setFlatEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QGroupBox7setFlatEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn FreeQGroupBox<T: QGroupBox_FreeQGroupBox>(&mut self, value: T) -> i32 {
    value.FreeQGroupBox(self);
    return 1;
  }
}

pub trait QGroupBox_FreeQGroupBox {
  fn FreeQGroupBox(self, this: &mut QGroupBox) -> i32;
}

// proto: void QGroupBox::FreeQGroupBox();
impl<'a> /*trait*/ QGroupBox_FreeQGroupBox for () {
  fn FreeQGroupBox(self, this: &mut QGroupBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBoxD0Ev()};
    unsafe {_ZN9QGroupBoxD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn NewQGroupBox<T: QGroupBox_NewQGroupBox>(value: T) -> QGroupBox {
    let rsthis = value.NewQGroupBox();
    return rsthis;
    // return 1;
  }
}

pub trait QGroupBox_NewQGroupBox {
  fn NewQGroupBox(self) -> QGroupBox;
}

// proto: void QGroupBox::NewQGroupBox(QWidget * parent);
impl<'a> /*trait*/ QGroupBox_NewQGroupBox for (&'a mut QWidget) {
  fn NewQGroupBox(self) -> QGroupBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBoxC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QGroupBoxC1EP7QWidget(qthis, arg0)};
    let rsthis = QGroupBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn toggled<T: QGroupBox_toggled>(&mut self, value: T) -> i32 {
    value.toggled(self);
    return 1;
  }
}

pub trait QGroupBox_toggled {
  fn toggled(self, this: &mut QGroupBox) -> i32;
}

// proto: void QGroupBox::toggled(bool );
impl<'a> /*trait*/ QGroupBox_toggled for (i8) {
  fn toggled(self, this: &mut QGroupBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox7toggledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QGroupBox7toggledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn isChecked<T: QGroupBox_isChecked>(&mut self, value: T) -> i32 {
    value.isChecked(self);
    return 1;
  }
}

pub trait QGroupBox_isChecked {
  fn isChecked(self, this: &mut QGroupBox) -> i32;
}

// proto: bool QGroupBox::isChecked();
impl<'a> /*trait*/ QGroupBox_isChecked for () {
  fn isChecked(self, this: &mut QGroupBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox9isCheckedEv()};
    unsafe {_ZNK9QGroupBox9isCheckedEv()};
    return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn setChecked<T: QGroupBox_setChecked>(&mut self, value: T) -> i32 {
    value.setChecked(self);
    return 1;
  }
}

pub trait QGroupBox_setChecked {
  fn setChecked(self, this: &mut QGroupBox) -> i32;
}

// proto: void QGroupBox::setChecked(bool checked);
impl<'a> /*trait*/ QGroupBox_setChecked for (i8) {
  fn setChecked(self, this: &mut QGroupBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox10setCheckedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QGroupBox10setCheckedEb(arg0)};
    return 1;
  }
}

// proto: void QGroupBox::NewQGroupBox(const QGroupBox & );
impl<'a> /*trait*/ QGroupBox_NewQGroupBox for (&'a  QGroupBox) {
  fn NewQGroupBox(self) -> QGroupBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QGroupBoxC1ERKS_(qthis, arg0)};
    let rsthis = QGroupBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn title<T: QGroupBox_title>(&mut self, value: T) -> i32 {
    value.title(self);
    return 1;
  }
}

pub trait QGroupBox_title {
  fn title(self, this: &mut QGroupBox) -> i32;
}

// proto: QString QGroupBox::title();
impl<'a> /*trait*/ QGroupBox_title for () {
  fn title(self, this: &mut QGroupBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox5titleEv()};
    unsafe {_ZNK9QGroupBox5titleEv()};
    return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn setAlignment<T: QGroupBox_setAlignment>(&mut self, value: T) -> i32 {
    value.setAlignment(self);
    return 1;
  }
}

pub trait QGroupBox_setAlignment {
  fn setAlignment(self, this: &mut QGroupBox) -> i32;
}

// proto: void QGroupBox::setAlignment(int alignment);
impl<'a> /*trait*/ QGroupBox_setAlignment for (i32) {
  fn setAlignment(self, this: &mut QGroupBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox12setAlignmentEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QGroupBox12setAlignmentEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn setTitle<T: QGroupBox_setTitle>(&mut self, value: T) -> i32 {
    value.setTitle(self);
    return 1;
  }
}

pub trait QGroupBox_setTitle {
  fn setTitle(self, this: &mut QGroupBox) -> i32;
}

// proto: void QGroupBox::setTitle(const QString & title);
impl<'a> /*trait*/ QGroupBox_setTitle for (&'a  QString) {
  fn setTitle(self, this: &mut QGroupBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox8setTitleERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QGroupBox8setTitleERK7QString(arg0)};
    return 1;
  }
}

// proto: void QGroupBox::NewQGroupBox(const QString & title, QWidget * parent);
impl<'a> /*trait*/ QGroupBox_NewQGroupBox for (&'a  QString, &'a mut QWidget) {
  fn NewQGroupBox(self) -> QGroupBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBoxC1ERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QGroupBoxC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let rsthis = QGroupBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn clicked<T: QGroupBox_clicked>(&mut self, value: T) -> i32 {
    value.clicked(self);
    return 1;
  }
}

pub trait QGroupBox_clicked {
  fn clicked(self, this: &mut QGroupBox) -> i32;
}

// proto: void QGroupBox::clicked(bool checked);
impl<'a> /*trait*/ QGroupBox_clicked for (i8) {
  fn clicked(self, this: &mut QGroupBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox7clickedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QGroupBox7clickedEb(arg0)};
    return 1;
  }
}

