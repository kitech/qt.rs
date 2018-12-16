

// mod ::gui::QAccessibleInterface
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
// extern C begin: 10
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
#[derive(Default)] // class sizeof(QAccessibleInterface)=8
pub struct QAccessibleInterface {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessibleInterface_ITF interface {
//    QAccessibleInterface_PTR() *QAccessibleInterface
//}
//func (ptr *QAccessibleInterface) QAccessibleInterface_PTR() *QAccessibleInterface { return ptr }

impl /*struct*/ QAccessibleInterface {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessibleInterface {
    return QAccessibleInterface{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessibleInterface {
//  type Target = QAccessibleInterfaceBASE;
//
//  fn deref(&self) -> &QAccessibleInterfaceBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleInterfaceBASE> for QAccessibleInterface {
//  fn as_ref(& self) -> & QAccessibleInterfaceBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessible.h:460
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void ~QAccessibleInterface()

/*

*/
pub fn DeleteQAccessibleInterface(this :*mut QAccessibleInterface) {
    // let rv = qtrt::InvokeQtFunc6("_ZN20QAccessibleInterfaceD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qaccessible.h:464
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [1] bool isValid() const

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn isValid_0<RetType, T: QAccessibleInterface_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QAccessibleInterface) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QAccessibleInterface7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:465
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QObject * object() const

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn object_0<RetType, T: QAccessibleInterface_object_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.object_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_object_0<RetType> {
  fn object_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_object_0<usize> for () {
  fn object_0(self , rsthis: & QAccessibleInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QAccessibleInterface6objectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:466
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QWindow * window() const

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn window_0<RetType, T: QAccessibleInterface_window_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.window_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_window_0<RetType> {
  fn window_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_window_0<usize> for () {
  fn window_0(self , rsthis: & QAccessibleInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QAccessibleInterface6windowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:470
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QAccessibleInterface * focusChild() const

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn focusChild_0<RetType, T: QAccessibleInterface_focusChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusChild_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_focusChild_0<RetType> {
  fn focusChild_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_focusChild_0<usize> for () {
  fn focusChild_0(self , rsthis: & QAccessibleInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QAccessibleInterface10focusChildEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:472
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QAccessibleInterface * childAt(int, int) const

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn childAt_0<RetType, T: QAccessibleInterface_childAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childAt_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_childAt_0<RetType> {
  fn childAt_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_childAt_0<usize> for (i32,i32) {
  fn childAt_0(self , rsthis: & QAccessibleInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QAccessibleInterface7childAtEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:475
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QAccessibleInterface * parent() const

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn parent_0<RetType, T: QAccessibleInterface_parent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parent_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_parent_0<RetType> {
  fn parent_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_parent_0<usize> for () {
  fn parent_0(self , rsthis: & QAccessibleInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QAccessibleInterface6parentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:476
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QAccessibleInterface * child(int) const

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn child_0<RetType, T: QAccessibleInterface_child_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.child_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_child_0<RetType> {
  fn child_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_child_0<usize> for (i32) {
  fn child_0(self , rsthis: & QAccessibleInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QAccessibleInterface5childEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:477
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int childCount() const

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn childCount_0<RetType, T: QAccessibleInterface_childCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childCount_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_childCount_0<RetType> {
  fn childCount_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_childCount_0<i32> for () {
  fn childCount_0(self , rsthis: & QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QAccessibleInterface10childCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:478
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int indexOfChild(const QAccessibleInterface *) const

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn indexOfChild_0<RetType, T: QAccessibleInterface_indexOfChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOfChild_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_indexOfChild_0<RetType> {
  fn indexOfChild_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_indexOfChild_0<i32> for (usize) {
  fn indexOfChild_0(self , rsthis: & QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QAccessibleInterface12indexOfChildEPKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:481
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QString text(QAccessible::Text) const

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn text_0<RetType, T: QAccessibleInterface_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_text_0<RetType> {
  fn text_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_text_0<usize> for (i32) {
  fn text_0(self , rsthis: & QAccessibleInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QAccessibleInterface4textEN11QAccessible4TextE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:482
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void setText(QAccessible::Text, const QString &)

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn setText_0<RetType, T: QAccessibleInterface_setText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setText_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_setText_0<RetType> {
  fn setText_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_setText_0<(/*void*/)> for (i32,usize) {
  fn setText_0(self , rsthis: & QAccessibleInterface) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QAccessibleInterface7setTextEN11QAccessible4TextERK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:483
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [16] QRect rect() const

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn rect_0<RetType, T: QAccessibleInterface_rect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rect_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_rect_0<RetType> {
  fn rect_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_rect_0<usize> for () {
  fn rect_0(self , rsthis: & QAccessibleInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QAccessibleInterface4rectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:484
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] QAccessible::Role role() const

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn role_0<RetType, T: QAccessibleInterface_role_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.role_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_role_0<RetType> {
  fn role_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_role_0<i32> for () {
  fn role_0(self , rsthis: & QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QAccessibleInterface4roleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:485
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QAccessible::State state() const

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn state_0<RetType, T: QAccessibleInterface_state_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.state_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_state_0<RetType> {
  fn state_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_state_0<i32> for () {
  fn state_0(self , rsthis: & QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QAccessibleInterface5stateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:487
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QColor foregroundColor() const

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn foregroundColor_0<RetType, T: QAccessibleInterface_foregroundColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.foregroundColor_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_foregroundColor_0<RetType> {
  fn foregroundColor_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_foregroundColor_0<usize> for () {
  fn foregroundColor_0(self , rsthis: & QAccessibleInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QAccessibleInterface15foregroundColorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:488
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QColor backgroundColor() const

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn backgroundColor_0<RetType, T: QAccessibleInterface_backgroundColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.backgroundColor_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_backgroundColor_0<RetType> {
  fn backgroundColor_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_backgroundColor_0<usize> for () {
  fn backgroundColor_0(self , rsthis: & QAccessibleInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QAccessibleInterface15backgroundColorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:490
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QAccessibleTextInterface * textInterface()

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn textInterface_0<RetType, T: QAccessibleInterface_textInterface_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textInterface_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_textInterface_0<RetType> {
  fn textInterface_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_textInterface_0<usize> for () {
  fn textInterface_0(self , rsthis: & QAccessibleInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QAccessibleInterface13textInterfaceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:493
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QAccessibleEditableTextInterface * editableTextInterface()

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn editableTextInterface_0<RetType, T: QAccessibleInterface_editableTextInterface_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.editableTextInterface_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_editableTextInterface_0<RetType> {
  fn editableTextInterface_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_editableTextInterface_0<usize> for () {
  fn editableTextInterface_0(self , rsthis: & QAccessibleInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QAccessibleInterface21editableTextInterfaceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:496
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QAccessibleValueInterface * valueInterface()

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn valueInterface_0<RetType, T: QAccessibleInterface_valueInterface_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.valueInterface_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_valueInterface_0<RetType> {
  fn valueInterface_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_valueInterface_0<usize> for () {
  fn valueInterface_0(self , rsthis: & QAccessibleInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QAccessibleInterface14valueInterfaceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:502
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QAccessibleImageInterface * imageInterface()

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn imageInterface_0<RetType, T: QAccessibleInterface_imageInterface_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.imageInterface_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_imageInterface_0<RetType> {
  fn imageInterface_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_imageInterface_0<usize> for () {
  fn imageInterface_0(self , rsthis: & QAccessibleInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QAccessibleInterface14imageInterfaceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:505
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QAccessibleTableInterface * tableInterface()

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn tableInterface_0<RetType, T: QAccessibleInterface_tableInterface_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tableInterface_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_tableInterface_0<RetType> {
  fn tableInterface_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_tableInterface_0<usize> for () {
  fn tableInterface_0(self , rsthis: & QAccessibleInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QAccessibleInterface14tableInterfaceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:508
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QAccessibleTableCellInterface * tableCellInterface()

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn tableCellInterface_0<RetType, T: QAccessibleInterface_tableCellInterface_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tableCellInterface_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_tableCellInterface_0<RetType> {
  fn tableCellInterface_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_tableCellInterface_0<usize> for () {
  fn tableCellInterface_0(self , rsthis: & QAccessibleInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QAccessibleInterface18tableCellInterfaceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:511
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void virtual_hook(int, void *)

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn virtual_hook_0<RetType, T: QAccessibleInterface_virtual_hook_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.virtual_hook_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_virtual_hook_0<RetType> {
  fn virtual_hook_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_virtual_hook_0<(/*void*/)> for (i32,usize) {
  fn virtual_hook_0(self , rsthis: & QAccessibleInterface) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QAccessibleInterface12virtual_hookEiPv", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:513
// index:0
// Public inline virtual Visibility=Default Availability=Available
// [8] void * interface_cast(QAccessible::InterfaceType)

/*

*/
impl /*struct*/ QAccessibleInterface {
  pub fn interface_cast_0<RetType, T: QAccessibleInterface_interface_cast_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.interface_cast_0(self);
    // return 1;
  }
}
pub trait QAccessibleInterface_interface_cast_0<RetType> {
  fn interface_cast_0(self , rsthis: & QAccessibleInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleInterface_interface_cast_0<usize> for (i32) {
  fn interface_cast_0(self , rsthis: & QAccessibleInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QAccessibleInterface14interface_castEN11QAccessible13InterfaceTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
