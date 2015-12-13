// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qaccessibleinterface::QAccessibleInterface;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QAccessibleApplication::NewQAccessibleApplication();
  fn _ZN22QAccessibleApplicationC1Ev(qthis: *mut c_void) -> i32;
  // proto: QWindow * QAccessibleApplication::window();
  fn _ZNK22QAccessibleApplication6windowEv() -> i32;
  // proto: QAccessibleInterface * QAccessibleApplication::child(int index);
  fn _ZNK22QAccessibleApplication5childEi(arg0: c_int) -> i32;
  // proto: int QAccessibleApplication::childCount();
  fn _ZNK22QAccessibleApplication10childCountEv() -> i32;
  // proto: QAccessibleInterface * QAccessibleApplication::parent();
  fn _ZNK22QAccessibleApplication6parentEv() -> i32;
  // proto: QAccessibleInterface * QAccessibleApplication::focusChild();
  fn _ZNK22QAccessibleApplication10focusChildEv() -> i32;
  // proto: int QAccessibleApplication::indexOfChild(const QAccessibleInterface * );
  fn _ZNK22QAccessibleApplication12indexOfChildEPK20QAccessibleInterface(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QAccessibleApplication)=16
pub struct QAccessibleApplication {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleApplication {
  pub fn NewQAccessibleApplication<T: QAccessibleApplication_NewQAccessibleApplication>(value: T) -> QAccessibleApplication {
    let rsthis = value.NewQAccessibleApplication();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleApplication_NewQAccessibleApplication {
  fn NewQAccessibleApplication(self) -> QAccessibleApplication;
}

// proto: void QAccessibleApplication::NewQAccessibleApplication();
impl<'a> /*trait*/ QAccessibleApplication_NewQAccessibleApplication for () {
  fn NewQAccessibleApplication(self) -> QAccessibleApplication {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QAccessibleApplicationC1Ev()};
    unsafe {_ZN22QAccessibleApplicationC1Ev(qthis)};
    let rsthis = QAccessibleApplication{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleApplication {
  pub fn window<T: QAccessibleApplication_window>(&mut self, value: T) -> i32 {
    value.window(self);
    return 1;
  }
}

pub trait QAccessibleApplication_window {
  fn window(self, this: &mut QAccessibleApplication) -> i32;
}

// proto: QWindow * QAccessibleApplication::window();
impl<'a> /*trait*/ QAccessibleApplication_window for () {
  fn window(self, this: &mut QAccessibleApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QAccessibleApplication6windowEv()};
    unsafe {_ZNK22QAccessibleApplication6windowEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleApplication {
  pub fn child<T: QAccessibleApplication_child>(&mut self, value: T) -> i32 {
    value.child(self);
    return 1;
  }
}

pub trait QAccessibleApplication_child {
  fn child(self, this: &mut QAccessibleApplication) -> i32;
}

// proto: QAccessibleInterface * QAccessibleApplication::child(int index);
impl<'a> /*trait*/ QAccessibleApplication_child for (i32) {
  fn child(self, this: &mut QAccessibleApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QAccessibleApplication5childEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK22QAccessibleApplication5childEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleApplication {
  pub fn childCount<T: QAccessibleApplication_childCount>(&mut self, value: T) -> i32 {
    value.childCount(self);
    return 1;
  }
}

pub trait QAccessibleApplication_childCount {
  fn childCount(self, this: &mut QAccessibleApplication) -> i32;
}

// proto: int QAccessibleApplication::childCount();
impl<'a> /*trait*/ QAccessibleApplication_childCount for () {
  fn childCount(self, this: &mut QAccessibleApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QAccessibleApplication10childCountEv()};
    unsafe {_ZNK22QAccessibleApplication10childCountEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleApplication {
  pub fn parent<T: QAccessibleApplication_parent>(&mut self, value: T) -> i32 {
    value.parent(self);
    return 1;
  }
}

pub trait QAccessibleApplication_parent {
  fn parent(self, this: &mut QAccessibleApplication) -> i32;
}

// proto: QAccessibleInterface * QAccessibleApplication::parent();
impl<'a> /*trait*/ QAccessibleApplication_parent for () {
  fn parent(self, this: &mut QAccessibleApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QAccessibleApplication6parentEv()};
    unsafe {_ZNK22QAccessibleApplication6parentEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleApplication {
  pub fn focusChild<T: QAccessibleApplication_focusChild>(&mut self, value: T) -> i32 {
    value.focusChild(self);
    return 1;
  }
}

pub trait QAccessibleApplication_focusChild {
  fn focusChild(self, this: &mut QAccessibleApplication) -> i32;
}

// proto: QAccessibleInterface * QAccessibleApplication::focusChild();
impl<'a> /*trait*/ QAccessibleApplication_focusChild for () {
  fn focusChild(self, this: &mut QAccessibleApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QAccessibleApplication10focusChildEv()};
    unsafe {_ZNK22QAccessibleApplication10focusChildEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleApplication {
  pub fn indexOfChild<T: QAccessibleApplication_indexOfChild>(&mut self, value: T) -> i32 {
    value.indexOfChild(self);
    return 1;
  }
}

pub trait QAccessibleApplication_indexOfChild {
  fn indexOfChild(self, this: &mut QAccessibleApplication) -> i32;
}

// proto: int QAccessibleApplication::indexOfChild(const QAccessibleInterface * );
impl<'a> /*trait*/ QAccessibleApplication_indexOfChild for (&'a  QAccessibleInterface) {
  fn indexOfChild(self, this: &mut QAccessibleApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QAccessibleApplication12indexOfChildEPK20QAccessibleInterface()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK22QAccessibleApplication12indexOfChildEPK20QAccessibleInterface(arg0)};
    return 1;
  }
}

