// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QAccessibleEditableTextInterface::insertText(int offset, const QString & text);
  fn _ZN32QAccessibleEditableTextInterface10insertTextEiRK7QString(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QAccessibleEditableTextInterface::replaceText(int startOffset, int endOffset, const QString & text);
  fn _ZN32QAccessibleEditableTextInterface11replaceTextEiiRK7QString(arg0: c_int, arg1: c_int, arg2: *const c_void) -> i32;
  // proto: void QAccessibleEditableTextInterface::deleteText(int startOffset, int endOffset);
  fn _ZN32QAccessibleEditableTextInterface10deleteTextEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QAccessibleEditableTextInterface::FreeQAccessibleEditableTextInterface();
  fn _ZN32QAccessibleEditableTextInterfaceD0Ev() -> i32;
}

// body block begin
// class sizeof(QAccessibleEditableTextInterface)=8
pub struct QAccessibleEditableTextInterface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleEditableTextInterface {
  pub fn insertText<T: QAccessibleEditableTextInterface_insertText>(&mut self, value: T) -> i32 {
    value.insertText(self);
    return 1;
  }
}

pub trait QAccessibleEditableTextInterface_insertText {
  fn insertText(self, this: &mut QAccessibleEditableTextInterface) -> i32;
}

// proto: void QAccessibleEditableTextInterface::insertText(int offset, const QString & text);
impl<'a> /*trait*/ QAccessibleEditableTextInterface_insertText for (i32, &'a  QString) {
  fn insertText(self, this: &mut QAccessibleEditableTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QAccessibleEditableTextInterface10insertTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN32QAccessibleEditableTextInterface10insertTextEiRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QAccessibleEditableTextInterface {
  pub fn replaceText<T: QAccessibleEditableTextInterface_replaceText>(&mut self, value: T) -> i32 {
    value.replaceText(self);
    return 1;
  }
}

pub trait QAccessibleEditableTextInterface_replaceText {
  fn replaceText(self, this: &mut QAccessibleEditableTextInterface) -> i32;
}

// proto: void QAccessibleEditableTextInterface::replaceText(int startOffset, int endOffset, const QString & text);
impl<'a> /*trait*/ QAccessibleEditableTextInterface_replaceText for (i32, i32, &'a  QString) {
  fn replaceText(self, this: &mut QAccessibleEditableTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QAccessibleEditableTextInterface11replaceTextEiiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN32QAccessibleEditableTextInterface11replaceTextEiiRK7QString(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QAccessibleEditableTextInterface {
  pub fn deleteText<T: QAccessibleEditableTextInterface_deleteText>(&mut self, value: T) -> i32 {
    value.deleteText(self);
    return 1;
  }
}

pub trait QAccessibleEditableTextInterface_deleteText {
  fn deleteText(self, this: &mut QAccessibleEditableTextInterface) -> i32;
}

// proto: void QAccessibleEditableTextInterface::deleteText(int startOffset, int endOffset);
impl<'a> /*trait*/ QAccessibleEditableTextInterface_deleteText for (i32, i32) {
  fn deleteText(self, this: &mut QAccessibleEditableTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QAccessibleEditableTextInterface10deleteTextEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN32QAccessibleEditableTextInterface10deleteTextEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QAccessibleEditableTextInterface {
  pub fn FreeQAccessibleEditableTextInterface<T: QAccessibleEditableTextInterface_FreeQAccessibleEditableTextInterface>(&mut self, value: T) -> i32 {
    value.FreeQAccessibleEditableTextInterface(self);
    return 1;
  }
}

pub trait QAccessibleEditableTextInterface_FreeQAccessibleEditableTextInterface {
  fn FreeQAccessibleEditableTextInterface(self, this: &mut QAccessibleEditableTextInterface) -> i32;
}

// proto: void QAccessibleEditableTextInterface::FreeQAccessibleEditableTextInterface();
impl<'a> /*trait*/ QAccessibleEditableTextInterface_FreeQAccessibleEditableTextInterface for () {
  fn FreeQAccessibleEditableTextInterface(self, this: &mut QAccessibleEditableTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QAccessibleEditableTextInterfaceD0Ev()};
    unsafe {_ZN32QAccessibleEditableTextInterfaceD0Ev()};
    return 1;
  }
}

