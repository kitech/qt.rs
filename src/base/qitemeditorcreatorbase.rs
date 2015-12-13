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
  // proto: QByteArray QItemEditorCreatorBase::valuePropertyName();
  fn _ZNK22QItemEditorCreatorBase17valuePropertyNameEv() -> i32;
  // proto: QWidget * QItemEditorCreatorBase::createWidget(QWidget * parent);
  fn _ZNK22QItemEditorCreatorBase12createWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QItemEditorCreatorBase::FreeQItemEditorCreatorBase();
  fn _ZN22QItemEditorCreatorBaseD0Ev() -> i32;
}

// body block begin
// class sizeof(QItemEditorCreatorBase)=8
pub struct QItemEditorCreatorBase {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QItemEditorCreatorBase {
  pub fn valuePropertyName<T: QItemEditorCreatorBase_valuePropertyName>(&mut self, value: T) -> i32 {
    value.valuePropertyName(self);
    return 1;
  }
}

pub trait QItemEditorCreatorBase_valuePropertyName {
  fn valuePropertyName(self, this: &mut QItemEditorCreatorBase) -> i32;
}

// proto: QByteArray QItemEditorCreatorBase::valuePropertyName();
impl<'a> /*trait*/ QItemEditorCreatorBase_valuePropertyName for () {
  fn valuePropertyName(self, this: &mut QItemEditorCreatorBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QItemEditorCreatorBase17valuePropertyNameEv()};
    unsafe {_ZNK22QItemEditorCreatorBase17valuePropertyNameEv()};
    return 1;
  }
}

impl /*struct*/ QItemEditorCreatorBase {
  pub fn createWidget<T: QItemEditorCreatorBase_createWidget>(&mut self, value: T) -> i32 {
    value.createWidget(self);
    return 1;
  }
}

pub trait QItemEditorCreatorBase_createWidget {
  fn createWidget(self, this: &mut QItemEditorCreatorBase) -> i32;
}

// proto: QWidget * QItemEditorCreatorBase::createWidget(QWidget * parent);
impl<'a> /*trait*/ QItemEditorCreatorBase_createWidget for (&'a mut QWidget) {
  fn createWidget(self, this: &mut QItemEditorCreatorBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QItemEditorCreatorBase12createWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK22QItemEditorCreatorBase12createWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QItemEditorCreatorBase {
  pub fn FreeQItemEditorCreatorBase<T: QItemEditorCreatorBase_FreeQItemEditorCreatorBase>(&mut self, value: T) -> i32 {
    value.FreeQItemEditorCreatorBase(self);
    return 1;
  }
}

pub trait QItemEditorCreatorBase_FreeQItemEditorCreatorBase {
  fn FreeQItemEditorCreatorBase(self, this: &mut QItemEditorCreatorBase) -> i32;
}

// proto: void QItemEditorCreatorBase::FreeQItemEditorCreatorBase();
impl<'a> /*trait*/ QItemEditorCreatorBase_FreeQItemEditorCreatorBase for () {
  fn FreeQItemEditorCreatorBase(self, this: &mut QItemEditorCreatorBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QItemEditorCreatorBaseD0Ev()};
    unsafe {_ZN22QItemEditorCreatorBaseD0Ev()};
    return 1;
  }
}

