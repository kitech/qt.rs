// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;
use super::qstring::QString;
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QAccessibleTextInterface::selection(int selectionIndex, int * startOffset, int * endOffset);
  fn _ZNK24QAccessibleTextInterface9selectionEiPiS0_(qthis: *mut c_void, arg0: c_int, arg1: *mut c_int, arg2: *mut c_int) ;
  // proto:  void QAccessibleTextInterface::setCursorPosition(int position);
  fn _ZN24QAccessibleTextInterface17setCursorPositionEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QAccessibleTextInterface::offsetAtPoint(const QPoint & point);
  fn _ZNK24QAccessibleTextInterface13offsetAtPointERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QString QAccessibleTextInterface::attributes(int offset, int * startOffset, int * endOffset);
  fn _ZNK24QAccessibleTextInterface10attributesEiPiS0_(qthis: *mut c_void, arg0: c_int, arg1: *mut c_int, arg2: *mut c_int) -> *mut c_void;
  // proto:  int QAccessibleTextInterface::selectionCount();
  fn _ZNK24QAccessibleTextInterface14selectionCountEv(qthis: *mut c_void) -> c_int;
  // proto:  int QAccessibleTextInterface::characterCount();
  fn _ZNK24QAccessibleTextInterface14characterCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTextInterface::FreeQAccessibleTextInterface();
  fn _ZN24QAccessibleTextInterfaceD0Ev(qthis: *mut c_void) ;
  // proto:  QString QAccessibleTextInterface::text(int startOffset, int endOffset);
  fn _ZNK24QAccessibleTextInterface4textEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  QRect QAccessibleTextInterface::characterRect(int offset);
  fn _ZNK24QAccessibleTextInterface13characterRectEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QAccessibleTextInterface::removeSelection(int selectionIndex);
  fn _ZN24QAccessibleTextInterface15removeSelectionEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QAccessibleTextInterface::addSelection(int startOffset, int endOffset);
  fn _ZN24QAccessibleTextInterface12addSelectionEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QAccessibleTextInterface::scrollToSubstring(int startIndex, int endIndex);
  fn _ZN24QAccessibleTextInterface17scrollToSubstringEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  int QAccessibleTextInterface::cursorPosition();
  fn _ZNK24QAccessibleTextInterface14cursorPositionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTextInterface::setSelection(int selectionIndex, int startOffset, int endOffset);
  fn _ZN24QAccessibleTextInterface12setSelectionEiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int) ;
}

// body block begin
// class sizeof(QAccessibleTextInterface)=8
pub struct QAccessibleTextInterface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn selection<T: QAccessibleTextInterface_selection>(&mut self, value: T)  {
     value.selection(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_selection {
  fn selection(self, rsthis: &mut QAccessibleTextInterface) ;
}

// proto:  void QAccessibleTextInterface::selection(int selectionIndex, int * startOffset, int * endOffset);
impl<'a> /*trait*/ QAccessibleTextInterface_selection for (i32, &'a mut i32, &'a mut i32) {
  fn selection(self, rsthis: &mut QAccessibleTextInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface9selectionEiPiS0_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
     unsafe {_ZNK24QAccessibleTextInterface9selectionEiPiS0_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn setCursorPosition<T: QAccessibleTextInterface_setCursorPosition>(&mut self, value: T)  {
     value.setCursorPosition(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_setCursorPosition {
  fn setCursorPosition(self, rsthis: &mut QAccessibleTextInterface) ;
}

// proto:  void QAccessibleTextInterface::setCursorPosition(int position);
impl<'a> /*trait*/ QAccessibleTextInterface_setCursorPosition for (i32) {
  fn setCursorPosition(self, rsthis: &mut QAccessibleTextInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAccessibleTextInterface17setCursorPositionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN24QAccessibleTextInterface17setCursorPositionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn offsetAtPoint<T: QAccessibleTextInterface_offsetAtPoint>(&mut self, value: T) -> i32 {
    return value.offsetAtPoint(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_offsetAtPoint {
  fn offsetAtPoint(self, rsthis: &mut QAccessibleTextInterface) -> i32;
}

// proto:  int QAccessibleTextInterface::offsetAtPoint(const QPoint & point);
impl<'a> /*trait*/ QAccessibleTextInterface_offsetAtPoint for (&'a  QPoint) {
  fn offsetAtPoint(self, rsthis: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface13offsetAtPointERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK24QAccessibleTextInterface13offsetAtPointERK6QPoint(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn attributes<T: QAccessibleTextInterface_attributes>(&mut self, value: T) -> QString {
    return value.attributes(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_attributes {
  fn attributes(self, rsthis: &mut QAccessibleTextInterface) -> QString;
}

// proto:  QString QAccessibleTextInterface::attributes(int offset, int * startOffset, int * endOffset);
impl<'a> /*trait*/ QAccessibleTextInterface_attributes for (i32, &'a mut i32, &'a mut i32) {
  fn attributes(self, rsthis: &mut QAccessibleTextInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface10attributesEiPiS0_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let mut ret = unsafe {_ZNK24QAccessibleTextInterface10attributesEiPiS0_(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn selectionCount<T: QAccessibleTextInterface_selectionCount>(&mut self, value: T) -> i32 {
    return value.selectionCount(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_selectionCount {
  fn selectionCount(self, rsthis: &mut QAccessibleTextInterface) -> i32;
}

// proto:  int QAccessibleTextInterface::selectionCount();
impl<'a> /*trait*/ QAccessibleTextInterface_selectionCount for () {
  fn selectionCount(self, rsthis: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface14selectionCountEv()};
    let mut ret = unsafe {_ZNK24QAccessibleTextInterface14selectionCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn characterCount<T: QAccessibleTextInterface_characterCount>(&mut self, value: T) -> i32 {
    return value.characterCount(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_characterCount {
  fn characterCount(self, rsthis: &mut QAccessibleTextInterface) -> i32;
}

// proto:  int QAccessibleTextInterface::characterCount();
impl<'a> /*trait*/ QAccessibleTextInterface_characterCount for () {
  fn characterCount(self, rsthis: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface14characterCountEv()};
    let mut ret = unsafe {_ZNK24QAccessibleTextInterface14characterCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn FreeQAccessibleTextInterface<T: QAccessibleTextInterface_FreeQAccessibleTextInterface>(&mut self, value: T)  {
     value.FreeQAccessibleTextInterface(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_FreeQAccessibleTextInterface {
  fn FreeQAccessibleTextInterface(self, rsthis: &mut QAccessibleTextInterface) ;
}

// proto:  void QAccessibleTextInterface::FreeQAccessibleTextInterface();
impl<'a> /*trait*/ QAccessibleTextInterface_FreeQAccessibleTextInterface for () {
  fn FreeQAccessibleTextInterface(self, rsthis: &mut QAccessibleTextInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAccessibleTextInterfaceD0Ev()};
     unsafe {_ZN24QAccessibleTextInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn text<T: QAccessibleTextInterface_text>(&mut self, value: T) -> QString {
    return value.text(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_text {
  fn text(self, rsthis: &mut QAccessibleTextInterface) -> QString;
}

// proto:  QString QAccessibleTextInterface::text(int startOffset, int endOffset);
impl<'a> /*trait*/ QAccessibleTextInterface_text for (i32, i32) {
  fn text(self, rsthis: &mut QAccessibleTextInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface4textEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK24QAccessibleTextInterface4textEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn characterRect<T: QAccessibleTextInterface_characterRect>(&mut self, value: T) -> QRect {
    return value.characterRect(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_characterRect {
  fn characterRect(self, rsthis: &mut QAccessibleTextInterface) -> QRect;
}

// proto:  QRect QAccessibleTextInterface::characterRect(int offset);
impl<'a> /*trait*/ QAccessibleTextInterface_characterRect for (i32) {
  fn characterRect(self, rsthis: &mut QAccessibleTextInterface) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface13characterRectEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK24QAccessibleTextInterface13characterRectEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn removeSelection<T: QAccessibleTextInterface_removeSelection>(&mut self, value: T)  {
     value.removeSelection(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_removeSelection {
  fn removeSelection(self, rsthis: &mut QAccessibleTextInterface) ;
}

// proto:  void QAccessibleTextInterface::removeSelection(int selectionIndex);
impl<'a> /*trait*/ QAccessibleTextInterface_removeSelection for (i32) {
  fn removeSelection(self, rsthis: &mut QAccessibleTextInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAccessibleTextInterface15removeSelectionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN24QAccessibleTextInterface15removeSelectionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn addSelection<T: QAccessibleTextInterface_addSelection>(&mut self, value: T)  {
     value.addSelection(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_addSelection {
  fn addSelection(self, rsthis: &mut QAccessibleTextInterface) ;
}

// proto:  void QAccessibleTextInterface::addSelection(int startOffset, int endOffset);
impl<'a> /*trait*/ QAccessibleTextInterface_addSelection for (i32, i32) {
  fn addSelection(self, rsthis: &mut QAccessibleTextInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAccessibleTextInterface12addSelectionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN24QAccessibleTextInterface12addSelectionEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn scrollToSubstring<T: QAccessibleTextInterface_scrollToSubstring>(&mut self, value: T)  {
     value.scrollToSubstring(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_scrollToSubstring {
  fn scrollToSubstring(self, rsthis: &mut QAccessibleTextInterface) ;
}

// proto:  void QAccessibleTextInterface::scrollToSubstring(int startIndex, int endIndex);
impl<'a> /*trait*/ QAccessibleTextInterface_scrollToSubstring for (i32, i32) {
  fn scrollToSubstring(self, rsthis: &mut QAccessibleTextInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAccessibleTextInterface17scrollToSubstringEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN24QAccessibleTextInterface17scrollToSubstringEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn cursorPosition<T: QAccessibleTextInterface_cursorPosition>(&mut self, value: T) -> i32 {
    return value.cursorPosition(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_cursorPosition {
  fn cursorPosition(self, rsthis: &mut QAccessibleTextInterface) -> i32;
}

// proto:  int QAccessibleTextInterface::cursorPosition();
impl<'a> /*trait*/ QAccessibleTextInterface_cursorPosition for () {
  fn cursorPosition(self, rsthis: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface14cursorPositionEv()};
    let mut ret = unsafe {_ZNK24QAccessibleTextInterface14cursorPositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn setSelection<T: QAccessibleTextInterface_setSelection>(&mut self, value: T)  {
     value.setSelection(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_setSelection {
  fn setSelection(self, rsthis: &mut QAccessibleTextInterface) ;
}

// proto:  void QAccessibleTextInterface::setSelection(int selectionIndex, int startOffset, int endOffset);
impl<'a> /*trait*/ QAccessibleTextInterface_setSelection for (i32, i32, i32) {
  fn setSelection(self, rsthis: &mut QAccessibleTextInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAccessibleTextInterface12setSelectionEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN24QAccessibleTextInterface12setSelectionEiii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

