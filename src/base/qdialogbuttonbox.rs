// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QList<QAbstractButton *> QDialogButtonBox::buttons();
  fn _ZNK16QDialogButtonBox7buttonsEv() -> i32;
  // proto: void QDialogButtonBox::setCenterButtons(bool center);
  fn _ZN16QDialogButtonBox16setCenterButtonsEb(arg0: int8_t) -> i32;
  // proto: bool QDialogButtonBox::centerButtons();
  fn _ZNK16QDialogButtonBox13centerButtonsEv() -> i32;
  // proto: const QMetaObject * QDialogButtonBox::metaObject();
  fn _ZNK16QDialogButtonBox10metaObjectEv() -> i32;
  // proto: void QDialogButtonBox::FreeQDialogButtonBox();
  fn _ZN16QDialogButtonBoxD0Ev() -> i32;
  // proto: void QDialogButtonBox::NewQDialogButtonBox(QWidget * parent);
  fn _ZN16QDialogButtonBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QDialogButtonBox::accepted();
  fn _ZN16QDialogButtonBox8acceptedEv() -> i32;
  // proto: void QDialogButtonBox::NewQDialogButtonBox(const QDialogButtonBox & );
  fn _ZN16QDialogButtonBoxC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QDialogButtonBox::clear();
  fn _ZN16QDialogButtonBox5clearEv() -> i32;
  // proto: void QDialogButtonBox::rejected();
  fn _ZN16QDialogButtonBox8rejectedEv() -> i32;
  // proto: void QDialogButtonBox::helpRequested();
  fn _ZN16QDialogButtonBox13helpRequestedEv() -> i32;
}

// body block begin
// class sizeof(QDialogButtonBox)=1
pub struct QDialogButtonBox {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDialogButtonBox {
  pub fn buttons<T: QDialogButtonBox_buttons>(&mut self, value: T) -> i32 {
    value.buttons(self);
    return 1;
  }
}

pub trait QDialogButtonBox_buttons {
  fn buttons(self, this: &mut QDialogButtonBox) -> i32;
}

// proto: QList<QAbstractButton *> QDialogButtonBox::buttons();
impl<'a> /*trait*/ QDialogButtonBox_buttons for () {
  fn buttons(self, this: &mut QDialogButtonBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDialogButtonBox7buttonsEv()};
    unsafe {_ZNK16QDialogButtonBox7buttonsEv()};
    return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn setCenterButtons<T: QDialogButtonBox_setCenterButtons>(&mut self, value: T) -> i32 {
    value.setCenterButtons(self);
    return 1;
  }
}

pub trait QDialogButtonBox_setCenterButtons {
  fn setCenterButtons(self, this: &mut QDialogButtonBox) -> i32;
}

// proto: void QDialogButtonBox::setCenterButtons(bool center);
impl<'a> /*trait*/ QDialogButtonBox_setCenterButtons for (i8) {
  fn setCenterButtons(self, this: &mut QDialogButtonBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox16setCenterButtonsEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN16QDialogButtonBox16setCenterButtonsEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn centerButtons<T: QDialogButtonBox_centerButtons>(&mut self, value: T) -> i32 {
    value.centerButtons(self);
    return 1;
  }
}

pub trait QDialogButtonBox_centerButtons {
  fn centerButtons(self, this: &mut QDialogButtonBox) -> i32;
}

// proto: bool QDialogButtonBox::centerButtons();
impl<'a> /*trait*/ QDialogButtonBox_centerButtons for () {
  fn centerButtons(self, this: &mut QDialogButtonBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDialogButtonBox13centerButtonsEv()};
    unsafe {_ZNK16QDialogButtonBox13centerButtonsEv()};
    return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn metaObject<T: QDialogButtonBox_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QDialogButtonBox_metaObject {
  fn metaObject(self, this: &mut QDialogButtonBox) -> i32;
}

// proto: const QMetaObject * QDialogButtonBox::metaObject();
impl<'a> /*trait*/ QDialogButtonBox_metaObject for () {
  fn metaObject(self, this: &mut QDialogButtonBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDialogButtonBox10metaObjectEv()};
    unsafe {_ZNK16QDialogButtonBox10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn FreeQDialogButtonBox<T: QDialogButtonBox_FreeQDialogButtonBox>(&mut self, value: T) -> i32 {
    value.FreeQDialogButtonBox(self);
    return 1;
  }
}

pub trait QDialogButtonBox_FreeQDialogButtonBox {
  fn FreeQDialogButtonBox(self, this: &mut QDialogButtonBox) -> i32;
}

// proto: void QDialogButtonBox::FreeQDialogButtonBox();
impl<'a> /*trait*/ QDialogButtonBox_FreeQDialogButtonBox for () {
  fn FreeQDialogButtonBox(self, this: &mut QDialogButtonBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBoxD0Ev()};
    unsafe {_ZN16QDialogButtonBoxD0Ev()};
    return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn NewQDialogButtonBox<T: QDialogButtonBox_NewQDialogButtonBox>(value: T) -> QDialogButtonBox {
    let rsthis = value.NewQDialogButtonBox();
    return rsthis;
    // return 1;
  }
}

pub trait QDialogButtonBox_NewQDialogButtonBox {
  fn NewQDialogButtonBox(self) -> QDialogButtonBox;
}

// proto: void QDialogButtonBox::NewQDialogButtonBox(QWidget * parent);
impl<'a> /*trait*/ QDialogButtonBox_NewQDialogButtonBox for (&'a mut QWidget) {
  fn NewQDialogButtonBox(self) -> QDialogButtonBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBoxC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QDialogButtonBoxC1EP7QWidget(qthis, arg0)};
    let rsthis = QDialogButtonBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn accepted<T: QDialogButtonBox_accepted>(&mut self, value: T) -> i32 {
    value.accepted(self);
    return 1;
  }
}

pub trait QDialogButtonBox_accepted {
  fn accepted(self, this: &mut QDialogButtonBox) -> i32;
}

// proto: void QDialogButtonBox::accepted();
impl<'a> /*trait*/ QDialogButtonBox_accepted for () {
  fn accepted(self, this: &mut QDialogButtonBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox8acceptedEv()};
    unsafe {_ZN16QDialogButtonBox8acceptedEv()};
    return 1;
  }
}

// proto: void QDialogButtonBox::NewQDialogButtonBox(const QDialogButtonBox & );
impl<'a> /*trait*/ QDialogButtonBox_NewQDialogButtonBox for (&'a  QDialogButtonBox) {
  fn NewQDialogButtonBox(self) -> QDialogButtonBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QDialogButtonBoxC1ERKS_(qthis, arg0)};
    let rsthis = QDialogButtonBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn clear<T: QDialogButtonBox_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QDialogButtonBox_clear {
  fn clear(self, this: &mut QDialogButtonBox) -> i32;
}

// proto: void QDialogButtonBox::clear();
impl<'a> /*trait*/ QDialogButtonBox_clear for () {
  fn clear(self, this: &mut QDialogButtonBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox5clearEv()};
    unsafe {_ZN16QDialogButtonBox5clearEv()};
    return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn rejected<T: QDialogButtonBox_rejected>(&mut self, value: T) -> i32 {
    value.rejected(self);
    return 1;
  }
}

pub trait QDialogButtonBox_rejected {
  fn rejected(self, this: &mut QDialogButtonBox) -> i32;
}

// proto: void QDialogButtonBox::rejected();
impl<'a> /*trait*/ QDialogButtonBox_rejected for () {
  fn rejected(self, this: &mut QDialogButtonBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox8rejectedEv()};
    unsafe {_ZN16QDialogButtonBox8rejectedEv()};
    return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn helpRequested<T: QDialogButtonBox_helpRequested>(&mut self, value: T) -> i32 {
    value.helpRequested(self);
    return 1;
  }
}

pub trait QDialogButtonBox_helpRequested {
  fn helpRequested(self, this: &mut QDialogButtonBox) -> i32;
}

// proto: void QDialogButtonBox::helpRequested();
impl<'a> /*trait*/ QDialogButtonBox_helpRequested for () {
  fn helpRequested(self, this: &mut QDialogButtonBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox13helpRequestedEv()};
    unsafe {_ZN16QDialogButtonBox13helpRequestedEv()};
    return 1;
  }
}

