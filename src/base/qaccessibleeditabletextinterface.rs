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
  // proto:  void QAccessibleEditableTextInterface::insertText(int offset, const QString & text);
  fn _ZN32QAccessibleEditableTextInterface10insertTextEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QAccessibleEditableTextInterface::replaceText(int startOffset, int endOffset, const QString & text);
  fn _ZN32QAccessibleEditableTextInterface11replaceTextEiiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void);
  // proto:  void QAccessibleEditableTextInterface::deleteText(int startOffset, int endOffset);
  fn _ZN32QAccessibleEditableTextInterface10deleteTextEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QAccessibleEditableTextInterface::~QAccessibleEditableTextInterface();
  fn _ZN32QAccessibleEditableTextInterfaceD0Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QAccessibleEditableTextInterface)=8
pub struct QAccessibleEditableTextInterface {
  pub qclsinst: *mut c_void,
}

  // proto:  void QAccessibleEditableTextInterface::insertText(int offset, const QString & text);
impl /*struct*/ QAccessibleEditableTextInterface {
  pub fn insertText<RetType, T: QAccessibleEditableTextInterface_insertText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.insertText(self);
    // return 1;
  }
}

pub trait QAccessibleEditableTextInterface_insertText<RetType> {
  fn insertText(self , rsthis: &mut QAccessibleEditableTextInterface) -> RetType;
}

  // proto:  void QAccessibleEditableTextInterface::insertText(int offset, const QString & text);
impl<'a> /*trait*/ QAccessibleEditableTextInterface_insertText<()> for (i32, QString) {
  fn insertText(self , rsthis: &mut QAccessibleEditableTextInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QAccessibleEditableTextInterface10insertTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN32QAccessibleEditableTextInterface10insertTextEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QAccessibleEditableTextInterface::replaceText(int startOffset, int endOffset, const QString & text);
impl /*struct*/ QAccessibleEditableTextInterface {
  pub fn replaceText<RetType, T: QAccessibleEditableTextInterface_replaceText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.replaceText(self);
    // return 1;
  }
}

pub trait QAccessibleEditableTextInterface_replaceText<RetType> {
  fn replaceText(self , rsthis: &mut QAccessibleEditableTextInterface) -> RetType;
}

  // proto:  void QAccessibleEditableTextInterface::replaceText(int startOffset, int endOffset, const QString & text);
impl<'a> /*trait*/ QAccessibleEditableTextInterface_replaceText<()> for (i32, i32, QString) {
  fn replaceText(self , rsthis: &mut QAccessibleEditableTextInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QAccessibleEditableTextInterface11replaceTextEiiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN32QAccessibleEditableTextInterface11replaceTextEiiRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QAccessibleEditableTextInterface::deleteText(int startOffset, int endOffset);
impl /*struct*/ QAccessibleEditableTextInterface {
  pub fn deleteText<RetType, T: QAccessibleEditableTextInterface_deleteText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.deleteText(self);
    // return 1;
  }
}

pub trait QAccessibleEditableTextInterface_deleteText<RetType> {
  fn deleteText(self , rsthis: &mut QAccessibleEditableTextInterface) -> RetType;
}

  // proto:  void QAccessibleEditableTextInterface::deleteText(int startOffset, int endOffset);
impl<'a> /*trait*/ QAccessibleEditableTextInterface_deleteText<()> for (i32, i32) {
  fn deleteText(self , rsthis: &mut QAccessibleEditableTextInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QAccessibleEditableTextInterface10deleteTextEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN32QAccessibleEditableTextInterface10deleteTextEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QAccessibleEditableTextInterface::~QAccessibleEditableTextInterface();
impl /*struct*/ QAccessibleEditableTextInterface {
  pub fn FreeQAccessibleEditableTextInterface<RetType, T: QAccessibleEditableTextInterface_FreeQAccessibleEditableTextInterface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAccessibleEditableTextInterface(self);
    // return 1;
  }
}

pub trait QAccessibleEditableTextInterface_FreeQAccessibleEditableTextInterface<RetType> {
  fn FreeQAccessibleEditableTextInterface(self , rsthis: &mut QAccessibleEditableTextInterface) -> RetType;
}

  // proto:  void QAccessibleEditableTextInterface::~QAccessibleEditableTextInterface();
impl<'a> /*trait*/ QAccessibleEditableTextInterface_FreeQAccessibleEditableTextInterface<()> for () {
  fn FreeQAccessibleEditableTextInterface(self , rsthis: &mut QAccessibleEditableTextInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QAccessibleEditableTextInterfaceD0Ev()};
     unsafe {_ZN32QAccessibleEditableTextInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

