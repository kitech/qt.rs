

// mod ::widgets::QTreeWidgetItemIterator
// package qtwidgets
// /usr/include/qt/QtWidgets/qtreewidgetitemiterator.h
// #include <qtreewidgetitemiterator.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 101
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QTreeWidgetItemIterator)=24
pub struct QTreeWidgetItemIterator {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTreeWidgetItemIterator_ITF interface {
//    QTreeWidgetItemIterator_PTR() *QTreeWidgetItemIterator
//}
//func (ptr *QTreeWidgetItemIterator) QTreeWidgetItemIterator_PTR() *QTreeWidgetItemIterator { return ptr }

impl /*struct*/ QTreeWidgetItemIterator {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTreeWidgetItemIterator {
    return QTreeWidgetItemIterator{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTreeWidgetItemIterator {
//  type Target = QTreeWidgetItemIteratorBASE;
//
//  fn deref(&self) -> &QTreeWidgetItemIteratorBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTreeWidgetItemIteratorBASE> for QTreeWidgetItemIterator {
//  fn as_ref(& self) -> & QTreeWidgetItemIteratorBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qtreewidgetitemiterator.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTreeWidgetItemIterator(QTreeWidget *, QTreeWidgetItemIterator::IteratorFlags)

/*
Constructs an iterator for the same QTreeWidget as it. The current iterator item is set to point on the current item of it.
*/
// QTreeWidgetItemIterator(QTreeWidget *, QTreeWidgetItemIterator::IteratorFlags) ctx.fn_proto_cpp
impl /*struct*/ QTreeWidgetItemIterator {
  pub fn QTreeWidgetItemIterator_0<T: QTreeWidgetItemIterator_QTreeWidgetItemIterator_0>(value: T) -> QTreeWidgetItemIterator {
    let rsthis = value.QTreeWidgetItemIterator_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeWidgetItemIterator_QTreeWidgetItemIterator_0 {
  fn QTreeWidgetItemIterator_0(self) -> QTreeWidgetItemIterator;
}
// QTreeWidgetItemIterator(QTreeWidget *, QTreeWidgetItemIterator::IteratorFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTreeWidgetItemIterator_QTreeWidgetItemIterator_0 for (usize,i32) {
  fn QTreeWidgetItemIterator_0(self) -> QTreeWidgetItemIterator {
    // unsafe{_ZN23QTreeWidgetItemIteratorC2EP11QTreeWidget6QFlagsINS_12IteratorFlagEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN23QTreeWidgetItemIteratorC2EP11QTreeWidget6QFlagsINS_12IteratorFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTreeWidgetItemIterator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidgetitemiterator.h:86
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTreeWidgetItemIterator(QTreeWidgetItem *, QTreeWidgetItemIterator::IteratorFlags)

/*
Constructs an iterator for the same QTreeWidget as it. The current iterator item is set to point on the current item of it.
*/
// QTreeWidgetItemIterator(QTreeWidgetItem *, QTreeWidgetItemIterator::IteratorFlags) ctx.fn_proto_cpp
impl /*struct*/ QTreeWidgetItemIterator {
  pub fn QTreeWidgetItemIterator_1<T: QTreeWidgetItemIterator_QTreeWidgetItemIterator_1>(value: T) -> QTreeWidgetItemIterator {
    let rsthis = value.QTreeWidgetItemIterator_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeWidgetItemIterator_QTreeWidgetItemIterator_1 {
  fn QTreeWidgetItemIterator_1(self) -> QTreeWidgetItemIterator;
}
// QTreeWidgetItemIterator(QTreeWidgetItem *, QTreeWidgetItemIterator::IteratorFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTreeWidgetItemIterator_QTreeWidgetItemIterator_1 for (usize,i32) {
  fn QTreeWidgetItemIterator_1(self) -> QTreeWidgetItemIterator {
    // unsafe{_ZN23QTreeWidgetItemIteratorC2EP15QTreeWidgetItem6QFlagsINS_12IteratorFlagEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN23QTreeWidgetItemIteratorC2EP15QTreeWidgetItem6QFlagsINS_12IteratorFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTreeWidgetItemIterator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreewidgetitemiterator.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QTreeWidgetItemIterator()

/*

*/
pub fn DeleteQTreeWidgetItemIterator(this :*mut QTreeWidgetItemIterator) {
    // let rv = qtrt::InvokeQtFunc6("_ZN23QTreeWidgetItemIteratorD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qtreewidgetitemiterator.h:89
// index:0
// Public Visibility=Default Availability=Available
// [24] QTreeWidgetItemIterator & operator=(const QTreeWidgetItemIterator &)

/*

*/
impl /*struct*/ QTreeWidgetItemIterator {
  pub fn operator_equal_0<RetType, T: QTreeWidgetItemIterator_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItemIterator_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QTreeWidgetItemIterator) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItemIterator_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QTreeWidgetItemIterator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN23QTreeWidgetItemIteratoraSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidgetitemiterator.h:91
// index:0
// Public Visibility=Default Availability=Available
// [24] QTreeWidgetItemIterator & operator++()

/*

*/
impl /*struct*/ QTreeWidgetItemIterator {
  pub fn operator_add_add_0<RetType, T: QTreeWidgetItemIterator_operator_add_add_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_add_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItemIterator_operator_add_add_0<RetType> {
  fn operator_add_add_0(self , rsthis: & QTreeWidgetItemIterator) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItemIterator_operator_add_add_0<usize> for () {
  fn operator_add_add_0(self , rsthis: & QTreeWidgetItemIterator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN23QTreeWidgetItemIteratorppEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidgetitemiterator.h:92
// index:1
// Public inline Visibility=Default Availability=Available
// [24] const QTreeWidgetItemIterator operator++(int)

/*

*/
impl /*struct*/ QTreeWidgetItemIterator {
  pub fn operator_add_add_1<RetType, T: QTreeWidgetItemIterator_operator_add_add_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_add_1(self);
    // return 1;
  }
}
pub trait QTreeWidgetItemIterator_operator_add_add_1<RetType> {
  fn operator_add_add_1(self , rsthis: & QTreeWidgetItemIterator) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItemIterator_operator_add_add_1<usize> for (i32) {
  fn operator_add_add_1(self , rsthis: & QTreeWidgetItemIterator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN23QTreeWidgetItemIteratorppEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidgetitemiterator.h:93
// index:0
// Public inline Visibility=Default Availability=Available
// [24] QTreeWidgetItemIterator & operator+=(int)

/*

*/
impl /*struct*/ QTreeWidgetItemIterator {
  pub fn operator_add_equal_0<RetType, T: QTreeWidgetItemIterator_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItemIterator_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QTreeWidgetItemIterator) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItemIterator_operator_add_equal_0<usize> for (i32) {
  fn operator_add_equal_0(self , rsthis: & QTreeWidgetItemIterator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN23QTreeWidgetItemIteratorpLEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidgetitemiterator.h:95
// index:0
// Public Visibility=Default Availability=Available
// [24] QTreeWidgetItemIterator & operator--()

/*
The prefix -- operator (--it) advances the iterator to the previous matching item and returns a reference to the resulting iterator. Sets the current pointer to 0 if the current item is the first matching item.
*/
impl /*struct*/ QTreeWidgetItemIterator {
  pub fn operator_minus_minus_0<RetType, T: QTreeWidgetItemIterator_operator_minus_minus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_minus_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItemIterator_operator_minus_minus_0<RetType> {
  fn operator_minus_minus_0(self , rsthis: & QTreeWidgetItemIterator) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItemIterator_operator_minus_minus_0<usize> for () {
  fn operator_minus_minus_0(self , rsthis: & QTreeWidgetItemIterator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN23QTreeWidgetItemIteratormmEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidgetitemiterator.h:96
// index:1
// Public inline Visibility=Default Availability=Available
// [24] const QTreeWidgetItemIterator operator--(int)

/*
The prefix -- operator (--it) advances the iterator to the previous matching item and returns a reference to the resulting iterator. Sets the current pointer to 0 if the current item is the first matching item.
*/
impl /*struct*/ QTreeWidgetItemIterator {
  pub fn operator_minus_minus_1<RetType, T: QTreeWidgetItemIterator_operator_minus_minus_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_minus_1(self);
    // return 1;
  }
}
pub trait QTreeWidgetItemIterator_operator_minus_minus_1<RetType> {
  fn operator_minus_minus_1(self , rsthis: & QTreeWidgetItemIterator) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItemIterator_operator_minus_minus_1<usize> for (i32) {
  fn operator_minus_minus_1(self , rsthis: & QTreeWidgetItemIterator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN23QTreeWidgetItemIteratormmEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidgetitemiterator.h:97
// index:0
// Public inline Visibility=Default Availability=Available
// [24] QTreeWidgetItemIterator & operator-=(int)

/*

*/
impl /*struct*/ QTreeWidgetItemIterator {
  pub fn operator_minus_equal_0<RetType, T: QTreeWidgetItemIterator_operator_minus_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_equal_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItemIterator_operator_minus_equal_0<RetType> {
  fn operator_minus_equal_0(self , rsthis: & QTreeWidgetItemIterator) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItemIterator_operator_minus_equal_0<usize> for (i32) {
  fn operator_minus_equal_0(self , rsthis: & QTreeWidgetItemIterator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN23QTreeWidgetItemIteratormIEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreewidgetitemiterator.h:99
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QTreeWidgetItem * operator*() const

/*

*/
impl /*struct*/ QTreeWidgetItemIterator {
  pub fn operator_mul_0<RetType, T: QTreeWidgetItemIterator_operator_mul_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_0(self);
    // return 1;
  }
}
pub trait QTreeWidgetItemIterator_operator_mul_0<RetType> {
  fn operator_mul_0(self , rsthis: & QTreeWidgetItemIterator) -> RetType;
}
impl<'a> /*trait*/ QTreeWidgetItemIterator_operator_mul_0<usize> for () {
  fn operator_mul_0(self , rsthis: & QTreeWidgetItemIterator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QTreeWidgetItemIteratordeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*


*/
pub type QTreeWidgetItemIterator__IteratorFlag = i32;
// 
pub const QTreeWidgetItemIterator__All :QTreeWidgetItemIterator__IteratorFlag = 0;
// 
pub const QTreeWidgetItemIterator__Hidden :QTreeWidgetItemIterator__IteratorFlag = 1;
// 
pub const QTreeWidgetItemIterator__NotHidden :QTreeWidgetItemIterator__IteratorFlag = 2;
// 
pub const QTreeWidgetItemIterator__Selected :QTreeWidgetItemIterator__IteratorFlag = 4;
// 
pub const QTreeWidgetItemIterator__Unselected :QTreeWidgetItemIterator__IteratorFlag = 8;
// 
pub const QTreeWidgetItemIterator__Selectable :QTreeWidgetItemIterator__IteratorFlag = 16;
// 
pub const QTreeWidgetItemIterator__NotSelectable :QTreeWidgetItemIterator__IteratorFlag = 32;
// 
pub const QTreeWidgetItemIterator__DragEnabled :QTreeWidgetItemIterator__IteratorFlag = 64;
// 
pub const QTreeWidgetItemIterator__DragDisabled :QTreeWidgetItemIterator__IteratorFlag = 128;
// 
pub const QTreeWidgetItemIterator__DropEnabled :QTreeWidgetItemIterator__IteratorFlag = 256;
// 
pub const QTreeWidgetItemIterator__DropDisabled :QTreeWidgetItemIterator__IteratorFlag = 512;
// 
pub const QTreeWidgetItemIterator__HasChildren :QTreeWidgetItemIterator__IteratorFlag = 1024;
// 
pub const QTreeWidgetItemIterator__NoChildren :QTreeWidgetItemIterator__IteratorFlag = 2048;
// 
pub const QTreeWidgetItemIterator__Checked :QTreeWidgetItemIterator__IteratorFlag = 4096;
// 
pub const QTreeWidgetItemIterator__NotChecked :QTreeWidgetItemIterator__IteratorFlag = 8192;
// 
pub const QTreeWidgetItemIterator__Enabled :QTreeWidgetItemIterator__IteratorFlag = 16384;
// 
pub const QTreeWidgetItemIterator__Disabled :QTreeWidgetItemIterator__IteratorFlag = 32768;
// 
pub const QTreeWidgetItemIterator__Editable :QTreeWidgetItemIterator__IteratorFlag = 65536;
// 
pub const QTreeWidgetItemIterator__NotEditable :QTreeWidgetItemIterator__IteratorFlag = 131072;
// 
pub const QTreeWidgetItemIterator__UserFlag :QTreeWidgetItemIterator__IteratorFlag = 16777216;
pub fn QTreeWidgetItemIterator_IteratorFlagItemName(val: i32) ->String {
  match val {
     QTreeWidgetItemIterator__All => // 0
     {return String::from("All");}
     QTreeWidgetItemIterator__Hidden => // 1
     {return String::from("Hidden");}
     QTreeWidgetItemIterator__NotHidden => // 2
     {return String::from("NotHidden");}
     QTreeWidgetItemIterator__Selected => // 4
     {return String::from("Selected");}
     QTreeWidgetItemIterator__Unselected => // 8
     {return String::from("Unselected");}
     QTreeWidgetItemIterator__Selectable => // 16
     {return String::from("Selectable");}
     QTreeWidgetItemIterator__NotSelectable => // 32
     {return String::from("NotSelectable");}
     QTreeWidgetItemIterator__DragEnabled => // 64
     {return String::from("DragEnabled");}
     QTreeWidgetItemIterator__DragDisabled => // 128
     {return String::from("DragDisabled");}
     QTreeWidgetItemIterator__DropEnabled => // 256
     {return String::from("DropEnabled");}
     QTreeWidgetItemIterator__DropDisabled => // 512
     {return String::from("DropDisabled");}
     QTreeWidgetItemIterator__HasChildren => // 1024
     {return String::from("HasChildren");}
     QTreeWidgetItemIterator__NoChildren => // 2048
     {return String::from("NoChildren");}
     QTreeWidgetItemIterator__Checked => // 4096
     {return String::from("Checked");}
     QTreeWidgetItemIterator__NotChecked => // 8192
     {return String::from("NotChecked");}
     QTreeWidgetItemIterator__Enabled => // 16384
     {return String::from("Enabled");}
     QTreeWidgetItemIterator__Disabled => // 32768
     {return String::from("Disabled");}
     QTreeWidgetItemIterator__Editable => // 65536
     {return String::from("Editable");}
     QTreeWidgetItemIterator__NotEditable => // 131072
     {return String::from("NotEditable");}
     QTreeWidgetItemIterator__UserFlag => // 16777216
     {return String::from("UserFlag");}
  _ => {return format!("{}", val);}
}
}
pub fn QTreeWidgetItemIterator_IteratorFlagItemName_s(val: i32) ->String {
  //var nilthis *QTreeWidgetItemIterator
  //return nilthis.IteratorFlagItemName(val);
  return QTreeWidgetItemIterator_IteratorFlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
