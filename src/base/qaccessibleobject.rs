// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QAccessibleObject::NewQAccessibleObject(QObject * object);
  fn _ZN17QAccessibleObjectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QObject * QAccessibleObject::object();
  fn _ZNK17QAccessibleObject6objectEv() -> i32;
  // proto: QRect QAccessibleObject::rect();
  fn _ZNK17QAccessibleObject4rectEv() -> i32;
  // proto: QAccessibleInterface * QAccessibleObject::childAt(int x, int y);
  fn _ZNK17QAccessibleObject7childAtEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QAccessibleObject::NewQAccessibleObject(const QAccessibleObject & );
  fn _ZN17QAccessibleObjectC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QAccessibleObject::isValid();
  fn _ZNK17QAccessibleObject7isValidEv() -> i32;
  // proto: void QAccessibleObject::FreeQAccessibleObject();
  fn _ZN17QAccessibleObjectD0Ev() -> i32;
}

// body block begin
// class sizeof(QAccessibleObject)=16
pub struct QAccessibleObject {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleObject {
  pub fn NewQAccessibleObject<T: QAccessibleObject_NewQAccessibleObject>(value: T) -> QAccessibleObject {
    let rsthis = value.NewQAccessibleObject();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleObject_NewQAccessibleObject {
  fn NewQAccessibleObject(self) -> QAccessibleObject;
}

// proto: void QAccessibleObject::NewQAccessibleObject(QObject * object);
impl<'a> /*trait*/ QAccessibleObject_NewQAccessibleObject for (&'a mut QObject) {
  fn NewQAccessibleObject(self) -> QAccessibleObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleObjectC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QAccessibleObjectC1EP7QObject(qthis, arg0)};
    let rsthis = QAccessibleObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleObject {
  pub fn object<T: QAccessibleObject_object>(&mut self, value: T) -> i32 {
    value.object(self);
    return 1;
  }
}

pub trait QAccessibleObject_object {
  fn object(self, this: &mut QAccessibleObject) -> i32;
}

// proto: QObject * QAccessibleObject::object();
impl<'a> /*trait*/ QAccessibleObject_object for () {
  fn object(self, this: &mut QAccessibleObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleObject6objectEv()};
    unsafe {_ZNK17QAccessibleObject6objectEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleObject {
  pub fn rect<T: QAccessibleObject_rect>(&mut self, value: T) -> i32 {
    value.rect(self);
    return 1;
  }
}

pub trait QAccessibleObject_rect {
  fn rect(self, this: &mut QAccessibleObject) -> i32;
}

// proto: QRect QAccessibleObject::rect();
impl<'a> /*trait*/ QAccessibleObject_rect for () {
  fn rect(self, this: &mut QAccessibleObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleObject4rectEv()};
    unsafe {_ZNK17QAccessibleObject4rectEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleObject {
  pub fn childAt<T: QAccessibleObject_childAt>(&mut self, value: T) -> i32 {
    value.childAt(self);
    return 1;
  }
}

pub trait QAccessibleObject_childAt {
  fn childAt(self, this: &mut QAccessibleObject) -> i32;
}

// proto: QAccessibleInterface * QAccessibleObject::childAt(int x, int y);
impl<'a> /*trait*/ QAccessibleObject_childAt for (i32, i32) {
  fn childAt(self, this: &mut QAccessibleObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleObject7childAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK17QAccessibleObject7childAtEii(arg0, arg1)};
    return 1;
  }
}

// proto: void QAccessibleObject::NewQAccessibleObject(const QAccessibleObject & );
impl<'a> /*trait*/ QAccessibleObject_NewQAccessibleObject for (&'a  QAccessibleObject) {
  fn NewQAccessibleObject(self) -> QAccessibleObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleObjectC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QAccessibleObjectC1ERKS_(qthis, arg0)};
    let rsthis = QAccessibleObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleObject {
  pub fn isValid<T: QAccessibleObject_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QAccessibleObject_isValid {
  fn isValid(self, this: &mut QAccessibleObject) -> i32;
}

// proto: bool QAccessibleObject::isValid();
impl<'a> /*trait*/ QAccessibleObject_isValid for () {
  fn isValid(self, this: &mut QAccessibleObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleObject7isValidEv()};
    unsafe {_ZNK17QAccessibleObject7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleObject {
  pub fn FreeQAccessibleObject<T: QAccessibleObject_FreeQAccessibleObject>(&mut self, value: T) -> i32 {
    value.FreeQAccessibleObject(self);
    return 1;
  }
}

pub trait QAccessibleObject_FreeQAccessibleObject {
  fn FreeQAccessibleObject(self, this: &mut QAccessibleObject) -> i32;
}

// proto: void QAccessibleObject::FreeQAccessibleObject();
impl<'a> /*trait*/ QAccessibleObject_FreeQAccessibleObject for () {
  fn FreeQAccessibleObject(self, this: &mut QAccessibleObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleObjectD0Ev()};
    unsafe {_ZN17QAccessibleObjectD0Ev()};
    return 1;
  }
}

