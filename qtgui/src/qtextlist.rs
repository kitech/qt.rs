

// mod ::gui::QTextList
// package qtgui
// /usr/include/qt/QtGui/qtextlist.h
// #include <qtextlist.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 14
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
#[derive(Default)] // class sizeof(QTextList)=16
pub struct QTextList {
  qbase: QTextBlockGroup,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextList_ITF interface {
//    QTextBlockGroup_ITF
//    QTextList_PTR() *QTextList
//}
//func (ptr *QTextList) QTextList_PTR() *QTextList { return ptr }

impl /*struct*/ QTextList {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextList {
    return QTextList{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextList {
//  type Target = QTextListBASE;
//
//  fn deref(&self) -> &QTextListBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextListBASE> for QTextList {
//  fn as_ref(& self) -> & QTextListBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextlist.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTextList {
  pub fn metaObject_0<RetType, T: QTextList_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTextList_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTextList) -> RetType;
}
impl<'a> /*trait*/ QTextList_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTextList) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextList10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlist.h:57
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextList(QTextDocument *)

/*

*/
// QTextList(QTextDocument *) ctx.fn_proto_cpp
impl /*struct*/ QTextList {
  pub fn QTextList_0<T: QTextList_QTextList_0>(value: T) -> QTextList {
    let rsthis = value.QTextList_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextList_QTextList_0 {
  fn QTextList_0(self) -> QTextList;
}
// QTextList(QTextDocument *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextList_QTextList_0 for (usize) {
  fn QTextList_0(self) -> QTextList {
    // unsafe{_ZN9QTextListC2EP13QTextDocument()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QTextListC2EP13QTextDocument", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextList{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlist.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTextList()

/*

*/
pub fn DeleteQTextList(this :*mut QTextList) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QTextListD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qtextlist.h:60
// index:0
// Public Visibility=Default Availability=Available
// [4] int count() const

/*
Returns the number of items in the list.
*/
impl /*struct*/ QTextList {
  pub fn count_0<RetType, T: QTextList_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QTextList_count_0<RetType> {
  fn count_0(self , rsthis: & QTextList) -> RetType;
}
impl<'a> /*trait*/ QTextList_count_0<i32> for () {
  fn count_0(self , rsthis: & QTextList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextList5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlist.h:62
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*

*/
impl /*struct*/ QTextList {
  pub fn isEmpty_0<RetType, T: QTextList_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QTextList_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QTextList) -> RetType;
}
impl<'a> /*trait*/ QTextList_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QTextList) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextList7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlist.h:65
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextBlock item(int) const

/*
Returns the i-th text block in the list.

See also count() and itemText().
*/
impl /*struct*/ QTextList {
  pub fn item_0<RetType, T: QTextList_item_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.item_0(self);
    // return 1;
  }
}
pub trait QTextList_item_0<RetType> {
  fn item_0(self , rsthis: & QTextList) -> RetType;
}
impl<'a> /*trait*/ QTextList_item_0<usize> for (i32) {
  fn item_0(self , rsthis: & QTextList) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextList4itemEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlist.h:67
// index:0
// Public Visibility=Default Availability=Available
// [4] int itemNumber(const QTextBlock &) const

/*
Returns the index of the list item that corresponds to the given block. Returns -1 if the block was not present in the list.
*/
impl /*struct*/ QTextList {
  pub fn itemNumber_0<RetType, T: QTextList_itemNumber_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemNumber_0(self);
    // return 1;
  }
}
pub trait QTextList_itemNumber_0<RetType> {
  fn itemNumber_0(self , rsthis: & QTextList) -> RetType;
}
impl<'a> /*trait*/ QTextList_itemNumber_0<i32> for (usize) {
  fn itemNumber_0(self , rsthis: & QTextList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextList10itemNumberERK10QTextBlock", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlist.h:68
// index:0
// Public Visibility=Default Availability=Available
// [8] QString itemText(const QTextBlock &) const

/*
Returns the text of the list item that corresponds to the given block.
*/
impl /*struct*/ QTextList {
  pub fn itemText_0<RetType, T: QTextList_itemText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemText_0(self);
    // return 1;
  }
}
pub trait QTextList_itemText_0<RetType> {
  fn itemText_0(self , rsthis: & QTextList) -> RetType;
}
impl<'a> /*trait*/ QTextList_itemText_0<usize> for (usize) {
  fn itemText_0(self , rsthis: & QTextList) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextList8itemTextERK10QTextBlock", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlist.h:70
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeItem(int)

/*
Removes the item at item position i from the list. When the last item in the list is removed, the list is automatically deleted by the QTextDocument that owns it.

See also add() and remove().
*/
impl /*struct*/ QTextList {
  pub fn removeItem_0<RetType, T: QTextList_removeItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeItem_0(self);
    // return 1;
  }
}
pub trait QTextList_removeItem_0<RetType> {
  fn removeItem_0(self , rsthis: & QTextList) -> RetType;
}
impl<'a> /*trait*/ QTextList_removeItem_0<(/*void*/)> for (i32) {
  fn removeItem_0(self , rsthis: & QTextList) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextList10removeItemEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlist.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void remove(const QTextBlock &)

/*
Removes the given block from the list.

See also add() and removeItem().
*/
impl /*struct*/ QTextList {
  pub fn remove_0<RetType, T: QTextList_remove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remove_0(self);
    // return 1;
  }
}
pub trait QTextList_remove_0<RetType> {
  fn remove_0(self , rsthis: & QTextList) -> RetType;
}
impl<'a> /*trait*/ QTextList_remove_0<(/*void*/)> for (usize) {
  fn remove_0(self , rsthis: & QTextList) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextList6removeERK10QTextBlock", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlist.h:73
// index:0
// Public Visibility=Default Availability=Available
// [-2] void add(const QTextBlock &)

/*
Makes the given block part of the list.

See also remove() and removeItem().
*/
impl /*struct*/ QTextList {
  pub fn add_0<RetType, T: QTextList_add_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.add_0(self);
    // return 1;
  }
}
pub trait QTextList_add_0<RetType> {
  fn add_0(self , rsthis: & QTextList) -> RetType;
}
impl<'a> /*trait*/ QTextList_add_0<(/*void*/)> for (usize) {
  fn add_0(self , rsthis: & QTextList) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextList3addERK10QTextBlock", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlist.h:75
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFormat(const QTextListFormat &)

/*
Sets the list's format to format.

See also format().
*/
impl /*struct*/ QTextList {
  pub fn setFormat_0<RetType, T: QTextList_setFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFormat_0(self);
    // return 1;
  }
}
pub trait QTextList_setFormat_0<RetType> {
  fn setFormat_0(self , rsthis: & QTextList) -> RetType;
}
impl<'a> /*trait*/ QTextList_setFormat_0<(/*void*/)> for (usize) {
  fn setFormat_0(self , rsthis: & QTextList) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextList9setFormatERK15QTextListFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlist.h:76
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QTextListFormat format() const

/*
Returns the list's format.

See also setFormat().
*/
impl /*struct*/ QTextList {
  pub fn format_0<RetType, T: QTextList_format_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.format_0(self);
    // return 1;
  }
}
pub trait QTextList_format_0<RetType> {
  fn format_0(self , rsthis: & QTextList) -> RetType;
}
impl<'a> /*trait*/ QTextList_format_0<usize> for () {
  fn format_0(self , rsthis: & QTextList) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextList6formatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
