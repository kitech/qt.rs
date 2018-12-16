

// mod ::gui::QAccessibleTextInterface
// package qtgui
// /usr/include/qt/QtGui/qaccessible.h
// #include <qaccessible.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 25
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QAccessibleTextInterface)=8
pub struct QAccessibleTextInterface {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessibleTextInterface_ITF interface {
//    QAccessibleTextInterface_PTR() *QAccessibleTextInterface
//}
//func (ptr *QAccessibleTextInterface) QAccessibleTextInterface_PTR() *QAccessibleTextInterface { return ptr }

impl /*struct*/ QAccessibleTextInterface {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessibleTextInterface {
    return QAccessibleTextInterface{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessibleTextInterface {
//  type Target = QAccessibleTextInterfaceBASE;
//
//  fn deref(&self) -> &QAccessibleTextInterfaceBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleTextInterfaceBASE> for QAccessibleTextInterface {
//  fn as_ref(& self) -> & QAccessibleTextInterfaceBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessible.h:523
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAccessibleTextInterface()

/*

*/
pub fn DeleteQAccessibleTextInterface(this :*mut QAccessibleTextInterface) {
    // let rv = qtrt::InvokeQtFunc6("_ZN24QAccessibleTextInterfaceD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qaccessible.h:525
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void selection(int, int *, int *) const

/*

*/
impl /*struct*/ QAccessibleTextInterface {
  pub fn selection_0<RetType, T: QAccessibleTextInterface_selection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selection_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextInterface_selection_0<RetType> {
  fn selection_0(self , rsthis: & QAccessibleTextInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextInterface_selection_0<(/*void*/)> for (i32,usize,usize) {
  fn selection_0(self , rsthis: & QAccessibleTextInterface) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK24QAccessibleTextInterface9selectionEiPiS0_", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:526
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int selectionCount() const

/*

*/
impl /*struct*/ QAccessibleTextInterface {
  pub fn selectionCount_0<RetType, T: QAccessibleTextInterface_selectionCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionCount_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextInterface_selectionCount_0<RetType> {
  fn selectionCount_0(self , rsthis: & QAccessibleTextInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextInterface_selectionCount_0<i32> for () {
  fn selectionCount_0(self , rsthis: & QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QAccessibleTextInterface14selectionCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:527
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void addSelection(int, int)

/*

*/
impl /*struct*/ QAccessibleTextInterface {
  pub fn addSelection_0<RetType, T: QAccessibleTextInterface_addSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addSelection_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextInterface_addSelection_0<RetType> {
  fn addSelection_0(self , rsthis: & QAccessibleTextInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextInterface_addSelection_0<(/*void*/)> for (i32,i32) {
  fn addSelection_0(self , rsthis: & QAccessibleTextInterface) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN24QAccessibleTextInterface12addSelectionEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:528
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void removeSelection(int)

/*

*/
impl /*struct*/ QAccessibleTextInterface {
  pub fn removeSelection_0<RetType, T: QAccessibleTextInterface_removeSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeSelection_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextInterface_removeSelection_0<RetType> {
  fn removeSelection_0(self , rsthis: & QAccessibleTextInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextInterface_removeSelection_0<(/*void*/)> for (i32) {
  fn removeSelection_0(self , rsthis: & QAccessibleTextInterface) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN24QAccessibleTextInterface15removeSelectionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:529
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void setSelection(int, int, int)

/*

*/
impl /*struct*/ QAccessibleTextInterface {
  pub fn setSelection_0<RetType, T: QAccessibleTextInterface_setSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelection_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextInterface_setSelection_0<RetType> {
  fn setSelection_0(self , rsthis: & QAccessibleTextInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextInterface_setSelection_0<(/*void*/)> for (i32,i32,i32) {
  fn setSelection_0(self , rsthis: & QAccessibleTextInterface) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN24QAccessibleTextInterface12setSelectionEiii", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:532
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int cursorPosition() const

/*

*/
impl /*struct*/ QAccessibleTextInterface {
  pub fn cursorPosition_0<RetType, T: QAccessibleTextInterface_cursorPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorPosition_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextInterface_cursorPosition_0<RetType> {
  fn cursorPosition_0(self , rsthis: & QAccessibleTextInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextInterface_cursorPosition_0<i32> for () {
  fn cursorPosition_0(self , rsthis: & QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QAccessibleTextInterface14cursorPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:533
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void setCursorPosition(int)

/*

*/
impl /*struct*/ QAccessibleTextInterface {
  pub fn setCursorPosition_0<RetType, T: QAccessibleTextInterface_setCursorPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCursorPosition_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextInterface_setCursorPosition_0<RetType> {
  fn setCursorPosition_0(self , rsthis: & QAccessibleTextInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextInterface_setCursorPosition_0<(/*void*/)> for (i32) {
  fn setCursorPosition_0(self , rsthis: & QAccessibleTextInterface) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN24QAccessibleTextInterface17setCursorPositionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:536
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QString text(int, int) const

/*

*/
impl /*struct*/ QAccessibleTextInterface {
  pub fn text_0<RetType, T: QAccessibleTextInterface_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextInterface_text_0<RetType> {
  fn text_0(self , rsthis: & QAccessibleTextInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextInterface_text_0<usize> for (i32,i32) {
  fn text_0(self , rsthis: & QAccessibleTextInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QAccessibleTextInterface4textEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:537
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QString textBeforeOffset(int, QAccessible::TextBoundaryType, int *, int *) const

/*

*/
impl /*struct*/ QAccessibleTextInterface {
  pub fn textBeforeOffset_0<RetType, T: QAccessibleTextInterface_textBeforeOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textBeforeOffset_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextInterface_textBeforeOffset_0<RetType> {
  fn textBeforeOffset_0(self , rsthis: & QAccessibleTextInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextInterface_textBeforeOffset_0<usize> for (i32,i32,usize,usize) {
  fn textBeforeOffset_0(self , rsthis: & QAccessibleTextInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QAccessibleTextInterface16textBeforeOffsetEiN11QAccessible16TextBoundaryTypeEPiS2_", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:539
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QString textAfterOffset(int, QAccessible::TextBoundaryType, int *, int *) const

/*

*/
impl /*struct*/ QAccessibleTextInterface {
  pub fn textAfterOffset_0<RetType, T: QAccessibleTextInterface_textAfterOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textAfterOffset_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextInterface_textAfterOffset_0<RetType> {
  fn textAfterOffset_0(self , rsthis: & QAccessibleTextInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextInterface_textAfterOffset_0<usize> for (i32,i32,usize,usize) {
  fn textAfterOffset_0(self , rsthis: & QAccessibleTextInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QAccessibleTextInterface15textAfterOffsetEiN11QAccessible16TextBoundaryTypeEPiS2_", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:541
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QString textAtOffset(int, QAccessible::TextBoundaryType, int *, int *) const

/*

*/
impl /*struct*/ QAccessibleTextInterface {
  pub fn textAtOffset_0<RetType, T: QAccessibleTextInterface_textAtOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textAtOffset_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextInterface_textAtOffset_0<RetType> {
  fn textAtOffset_0(self , rsthis: & QAccessibleTextInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextInterface_textAtOffset_0<usize> for (i32,i32,usize,usize) {
  fn textAtOffset_0(self , rsthis: & QAccessibleTextInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QAccessibleTextInterface12textAtOffsetEiN11QAccessible16TextBoundaryTypeEPiS2_", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:543
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int characterCount() const

/*

*/
impl /*struct*/ QAccessibleTextInterface {
  pub fn characterCount_0<RetType, T: QAccessibleTextInterface_characterCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.characterCount_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextInterface_characterCount_0<RetType> {
  fn characterCount_0(self , rsthis: & QAccessibleTextInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextInterface_characterCount_0<i32> for () {
  fn characterCount_0(self , rsthis: & QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QAccessibleTextInterface14characterCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:546
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [16] QRect characterRect(int) const

/*

*/
impl /*struct*/ QAccessibleTextInterface {
  pub fn characterRect_0<RetType, T: QAccessibleTextInterface_characterRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.characterRect_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextInterface_characterRect_0<RetType> {
  fn characterRect_0(self , rsthis: & QAccessibleTextInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextInterface_characterRect_0<usize> for (i32) {
  fn characterRect_0(self , rsthis: & QAccessibleTextInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QAccessibleTextInterface13characterRectEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:547
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int offsetAtPoint(const QPoint &) const

/*

*/
impl /*struct*/ QAccessibleTextInterface {
  pub fn offsetAtPoint_0<RetType, T: QAccessibleTextInterface_offsetAtPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.offsetAtPoint_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextInterface_offsetAtPoint_0<RetType> {
  fn offsetAtPoint_0(self , rsthis: & QAccessibleTextInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextInterface_offsetAtPoint_0<i32> for (usize) {
  fn offsetAtPoint_0(self , rsthis: & QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QAccessibleTextInterface13offsetAtPointERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:549
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void scrollToSubstring(int, int)

/*

*/
impl /*struct*/ QAccessibleTextInterface {
  pub fn scrollToSubstring_0<RetType, T: QAccessibleTextInterface_scrollToSubstring_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollToSubstring_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextInterface_scrollToSubstring_0<RetType> {
  fn scrollToSubstring_0(self , rsthis: & QAccessibleTextInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextInterface_scrollToSubstring_0<(/*void*/)> for (i32,i32) {
  fn scrollToSubstring_0(self , rsthis: & QAccessibleTextInterface) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN24QAccessibleTextInterface17scrollToSubstringEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:550
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QString attributes(int, int *, int *) const

/*

*/
impl /*struct*/ QAccessibleTextInterface {
  pub fn attributes_0<RetType, T: QAccessibleTextInterface_attributes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.attributes_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextInterface_attributes_0<RetType> {
  fn attributes_0(self , rsthis: & QAccessibleTextInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextInterface_attributes_0<usize> for (i32,usize,usize) {
  fn attributes_0(self , rsthis: & QAccessibleTextInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QAccessibleTextInterface10attributesEiPiS0_", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
