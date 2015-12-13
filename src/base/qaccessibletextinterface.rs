// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QAccessibleTextInterface::selection(int selectionIndex, int * startOffset, int * endOffset);
  fn _ZNK24QAccessibleTextInterface9selectionEiPiS0_(arg0: c_int, arg1: *mut c_int, arg2: *mut c_int) -> i32;
  // proto: void QAccessibleTextInterface::setCursorPosition(int position);
  fn _ZN24QAccessibleTextInterface17setCursorPositionEi(arg0: c_int) -> i32;
  // proto: int QAccessibleTextInterface::offsetAtPoint(const QPoint & point);
  fn _ZNK24QAccessibleTextInterface13offsetAtPointERK6QPoint(arg0: *const c_void) -> i32;
  // proto: QString QAccessibleTextInterface::attributes(int offset, int * startOffset, int * endOffset);
  fn _ZNK24QAccessibleTextInterface10attributesEiPiS0_(arg0: c_int, arg1: *mut c_int, arg2: *mut c_int) -> i32;
  // proto: int QAccessibleTextInterface::selectionCount();
  fn _ZNK24QAccessibleTextInterface14selectionCountEv() -> i32;
  // proto: int QAccessibleTextInterface::characterCount();
  fn _ZNK24QAccessibleTextInterface14characterCountEv() -> i32;
  // proto: void QAccessibleTextInterface::FreeQAccessibleTextInterface();
  fn _ZN24QAccessibleTextInterfaceD0Ev() -> i32;
  // proto: QString QAccessibleTextInterface::text(int startOffset, int endOffset);
  fn _ZNK24QAccessibleTextInterface4textEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: QRect QAccessibleTextInterface::characterRect(int offset);
  fn _ZNK24QAccessibleTextInterface13characterRectEi(arg0: c_int) -> i32;
  // proto: void QAccessibleTextInterface::removeSelection(int selectionIndex);
  fn _ZN24QAccessibleTextInterface15removeSelectionEi(arg0: c_int) -> i32;
  // proto: void QAccessibleTextInterface::addSelection(int startOffset, int endOffset);
  fn _ZN24QAccessibleTextInterface12addSelectionEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QAccessibleTextInterface::scrollToSubstring(int startIndex, int endIndex);
  fn _ZN24QAccessibleTextInterface17scrollToSubstringEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: int QAccessibleTextInterface::cursorPosition();
  fn _ZNK24QAccessibleTextInterface14cursorPositionEv() -> i32;
  // proto: void QAccessibleTextInterface::setSelection(int selectionIndex, int startOffset, int endOffset);
  fn _ZN24QAccessibleTextInterface12setSelectionEiii(arg0: c_int, arg1: c_int, arg2: c_int) -> i32;
}

// body block begin
// class sizeof(QAccessibleTextInterface)=8
pub struct QAccessibleTextInterface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn selection<T: QAccessibleTextInterface_selection>(&mut self, value: T) -> i32 {
    value.selection(self);
    return 1;
  }
}

pub trait QAccessibleTextInterface_selection {
  fn selection(self, this: &mut QAccessibleTextInterface) -> i32;
}

// proto: void QAccessibleTextInterface::selection(int selectionIndex, int * startOffset, int * endOffset);
impl<'a> /*trait*/ QAccessibleTextInterface_selection for (i32, &'a mut i32, &'a mut i32) {
  fn selection(self, this: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface9selectionEiPiS0_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    unsafe {_ZNK24QAccessibleTextInterface9selectionEiPiS0_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn setCursorPosition<T: QAccessibleTextInterface_setCursorPosition>(&mut self, value: T) -> i32 {
    value.setCursorPosition(self);
    return 1;
  }
}

pub trait QAccessibleTextInterface_setCursorPosition {
  fn setCursorPosition(self, this: &mut QAccessibleTextInterface) -> i32;
}

// proto: void QAccessibleTextInterface::setCursorPosition(int position);
impl<'a> /*trait*/ QAccessibleTextInterface_setCursorPosition for (i32) {
  fn setCursorPosition(self, this: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAccessibleTextInterface17setCursorPositionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN24QAccessibleTextInterface17setCursorPositionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn offsetAtPoint<T: QAccessibleTextInterface_offsetAtPoint>(&mut self, value: T) -> i32 {
    value.offsetAtPoint(self);
    return 1;
  }
}

pub trait QAccessibleTextInterface_offsetAtPoint {
  fn offsetAtPoint(self, this: &mut QAccessibleTextInterface) -> i32;
}

// proto: int QAccessibleTextInterface::offsetAtPoint(const QPoint & point);
impl<'a> /*trait*/ QAccessibleTextInterface_offsetAtPoint for (&'a  QPoint) {
  fn offsetAtPoint(self, this: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface13offsetAtPointERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK24QAccessibleTextInterface13offsetAtPointERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn attributes<T: QAccessibleTextInterface_attributes>(&mut self, value: T) -> i32 {
    value.attributes(self);
    return 1;
  }
}

pub trait QAccessibleTextInterface_attributes {
  fn attributes(self, this: &mut QAccessibleTextInterface) -> i32;
}

// proto: QString QAccessibleTextInterface::attributes(int offset, int * startOffset, int * endOffset);
impl<'a> /*trait*/ QAccessibleTextInterface_attributes for (i32, &'a mut i32, &'a mut i32) {
  fn attributes(self, this: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface10attributesEiPiS0_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    unsafe {_ZNK24QAccessibleTextInterface10attributesEiPiS0_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn selectionCount<T: QAccessibleTextInterface_selectionCount>(&mut self, value: T) -> i32 {
    value.selectionCount(self);
    return 1;
  }
}

pub trait QAccessibleTextInterface_selectionCount {
  fn selectionCount(self, this: &mut QAccessibleTextInterface) -> i32;
}

// proto: int QAccessibleTextInterface::selectionCount();
impl<'a> /*trait*/ QAccessibleTextInterface_selectionCount for () {
  fn selectionCount(self, this: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface14selectionCountEv()};
    unsafe {_ZNK24QAccessibleTextInterface14selectionCountEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn characterCount<T: QAccessibleTextInterface_characterCount>(&mut self, value: T) -> i32 {
    value.characterCount(self);
    return 1;
  }
}

pub trait QAccessibleTextInterface_characterCount {
  fn characterCount(self, this: &mut QAccessibleTextInterface) -> i32;
}

// proto: int QAccessibleTextInterface::characterCount();
impl<'a> /*trait*/ QAccessibleTextInterface_characterCount for () {
  fn characterCount(self, this: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface14characterCountEv()};
    unsafe {_ZNK24QAccessibleTextInterface14characterCountEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn FreeQAccessibleTextInterface<T: QAccessibleTextInterface_FreeQAccessibleTextInterface>(&mut self, value: T) -> i32 {
    value.FreeQAccessibleTextInterface(self);
    return 1;
  }
}

pub trait QAccessibleTextInterface_FreeQAccessibleTextInterface {
  fn FreeQAccessibleTextInterface(self, this: &mut QAccessibleTextInterface) -> i32;
}

// proto: void QAccessibleTextInterface::FreeQAccessibleTextInterface();
impl<'a> /*trait*/ QAccessibleTextInterface_FreeQAccessibleTextInterface for () {
  fn FreeQAccessibleTextInterface(self, this: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAccessibleTextInterfaceD0Ev()};
    unsafe {_ZN24QAccessibleTextInterfaceD0Ev()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn text<T: QAccessibleTextInterface_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QAccessibleTextInterface_text {
  fn text(self, this: &mut QAccessibleTextInterface) -> i32;
}

// proto: QString QAccessibleTextInterface::text(int startOffset, int endOffset);
impl<'a> /*trait*/ QAccessibleTextInterface_text for (i32, i32) {
  fn text(self, this: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface4textEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK24QAccessibleTextInterface4textEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn characterRect<T: QAccessibleTextInterface_characterRect>(&mut self, value: T) -> i32 {
    value.characterRect(self);
    return 1;
  }
}

pub trait QAccessibleTextInterface_characterRect {
  fn characterRect(self, this: &mut QAccessibleTextInterface) -> i32;
}

// proto: QRect QAccessibleTextInterface::characterRect(int offset);
impl<'a> /*trait*/ QAccessibleTextInterface_characterRect for (i32) {
  fn characterRect(self, this: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface13characterRectEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK24QAccessibleTextInterface13characterRectEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn removeSelection<T: QAccessibleTextInterface_removeSelection>(&mut self, value: T) -> i32 {
    value.removeSelection(self);
    return 1;
  }
}

pub trait QAccessibleTextInterface_removeSelection {
  fn removeSelection(self, this: &mut QAccessibleTextInterface) -> i32;
}

// proto: void QAccessibleTextInterface::removeSelection(int selectionIndex);
impl<'a> /*trait*/ QAccessibleTextInterface_removeSelection for (i32) {
  fn removeSelection(self, this: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAccessibleTextInterface15removeSelectionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN24QAccessibleTextInterface15removeSelectionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn addSelection<T: QAccessibleTextInterface_addSelection>(&mut self, value: T) -> i32 {
    value.addSelection(self);
    return 1;
  }
}

pub trait QAccessibleTextInterface_addSelection {
  fn addSelection(self, this: &mut QAccessibleTextInterface) -> i32;
}

// proto: void QAccessibleTextInterface::addSelection(int startOffset, int endOffset);
impl<'a> /*trait*/ QAccessibleTextInterface_addSelection for (i32, i32) {
  fn addSelection(self, this: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAccessibleTextInterface12addSelectionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN24QAccessibleTextInterface12addSelectionEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn scrollToSubstring<T: QAccessibleTextInterface_scrollToSubstring>(&mut self, value: T) -> i32 {
    value.scrollToSubstring(self);
    return 1;
  }
}

pub trait QAccessibleTextInterface_scrollToSubstring {
  fn scrollToSubstring(self, this: &mut QAccessibleTextInterface) -> i32;
}

// proto: void QAccessibleTextInterface::scrollToSubstring(int startIndex, int endIndex);
impl<'a> /*trait*/ QAccessibleTextInterface_scrollToSubstring for (i32, i32) {
  fn scrollToSubstring(self, this: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAccessibleTextInterface17scrollToSubstringEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN24QAccessibleTextInterface17scrollToSubstringEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn cursorPosition<T: QAccessibleTextInterface_cursorPosition>(&mut self, value: T) -> i32 {
    value.cursorPosition(self);
    return 1;
  }
}

pub trait QAccessibleTextInterface_cursorPosition {
  fn cursorPosition(self, this: &mut QAccessibleTextInterface) -> i32;
}

// proto: int QAccessibleTextInterface::cursorPosition();
impl<'a> /*trait*/ QAccessibleTextInterface_cursorPosition for () {
  fn cursorPosition(self, this: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface14cursorPositionEv()};
    unsafe {_ZNK24QAccessibleTextInterface14cursorPositionEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn setSelection<T: QAccessibleTextInterface_setSelection>(&mut self, value: T) -> i32 {
    value.setSelection(self);
    return 1;
  }
}

pub trait QAccessibleTextInterface_setSelection {
  fn setSelection(self, this: &mut QAccessibleTextInterface) -> i32;
}

// proto: void QAccessibleTextInterface::setSelection(int selectionIndex, int startOffset, int endOffset);
impl<'a> /*trait*/ QAccessibleTextInterface_setSelection for (i32, i32, i32) {
  fn setSelection(self, this: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAccessibleTextInterface12setSelectionEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN24QAccessibleTextInterface12setSelectionEiii(arg0, arg1, arg2)};
    return 1;
  }
}

