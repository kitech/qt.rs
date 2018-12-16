

// mod ::core::QListData
// package qtcore
// /usr/include/qt/QtCore/qlist.h
// #include <qlist.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 3
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QListData)=8
pub struct QListData {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QListData_ITF interface {
//    QListData_PTR() *QListData
//}
//func (ptr *QListData) QListData_PTR() *QListData { return ptr }

impl /*struct*/ QListData {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QListData {
    return QListData{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QListData {
//  type Target = QListDataBASE;
//
//  fn deref(&self) -> &QListDataBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QListDataBASE> for QListData {
//  fn as_ref(& self) -> & QListDataBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qlist.h:96
// index:0
// Public Visibility=Default Availability=Available
// [8] QListData::Data * detach(int)

/*

*/
impl /*struct*/ QListData {
  pub fn detach_0<RetType, T: QListData_detach_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.detach_0(self);
    // return 1;
  }
}
pub trait QListData_detach_0<RetType> {
  fn detach_0(self , rsthis: & QListData) -> RetType;
}
impl<'a> /*trait*/ QListData_detach_0<usize> for (i32) {
  fn detach_0(self , rsthis: & QListData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QListData6detachEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlist.h:97
// index:0
// Public Visibility=Default Availability=Available
// [8] QListData::Data * detach_grow(int *, int)

/*

*/
impl /*struct*/ QListData {
  pub fn detach_grow_0<RetType, T: QListData_detach_grow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.detach_grow_0(self);
    // return 1;
  }
}
pub trait QListData_detach_grow_0<RetType> {
  fn detach_grow_0(self , rsthis: & QListData) -> RetType;
}
impl<'a> /*trait*/ QListData_detach_grow_0<usize> for (usize,i32) {
  fn detach_grow_0(self , rsthis: & QListData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QListData11detach_growEPii", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlist.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void realloc(int)

/*

*/
impl /*struct*/ QListData {
  pub fn realloc_0<RetType, T: QListData_realloc_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.realloc_0(self);
    // return 1;
  }
}
pub trait QListData_realloc_0<RetType> {
  fn realloc_0(self , rsthis: & QListData) -> RetType;
}
impl<'a> /*trait*/ QListData_realloc_0<(/*void*/)> for (i32) {
  fn realloc_0(self , rsthis: & QListData) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QListData7reallocEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlist.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void realloc_grow(int)

/*

*/
impl /*struct*/ QListData {
  pub fn realloc_grow_0<RetType, T: QListData_realloc_grow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.realloc_grow_0(self);
    // return 1;
  }
}
pub trait QListData_realloc_grow_0<RetType> {
  fn realloc_grow_0(self , rsthis: & QListData) -> RetType;
}
impl<'a> /*trait*/ QListData_realloc_grow_0<(/*void*/)> for (i32) {
  fn realloc_grow_0(self , rsthis: & QListData) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QListData12realloc_growEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlist.h:100
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void dispose()

/*

*/
impl /*struct*/ QListData {
  pub fn dispose_0<RetType, T: QListData_dispose_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dispose_0(self);
    // return 1;
  }
}
pub trait QListData_dispose_0<RetType> {
  fn dispose_0(self , rsthis: & QListData) -> RetType;
}
impl<'a> /*trait*/ QListData_dispose_0<(/*void*/)> for () {
  fn dispose_0(self , rsthis: & QListData) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QListData7disposeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlist.h:104
// index:0
// Public Visibility=Default Availability=Available
// [8] void ** erase(void **)

/*
Removes the item associated with the iterator pos from the list, and returns an iterator to the next item in the list (which may be end()).

See also insert() and removeAt().
*/
impl /*struct*/ QListData {
  pub fn erase_0<RetType, T: QListData_erase_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.erase_0(self);
    // return 1;
  }
}
pub trait QListData_erase_0<RetType> {
  fn erase_0(self , rsthis: & QListData) -> RetType;
}
impl<'a> /*trait*/ QListData_erase_0<usize> for (usize) {
  fn erase_0(self , rsthis: & QListData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QListData5eraseEPPv", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlist.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void remove(int)

/*

*/
impl /*struct*/ QListData {
  pub fn remove_0<RetType, T: QListData_remove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remove_0(self);
    // return 1;
  }
}
pub trait QListData_remove_0<RetType> {
  fn remove_0(self , rsthis: & QListData) -> RetType;
}
impl<'a> /*trait*/ QListData_remove_0<(/*void*/)> for (i32) {
  fn remove_0(self , rsthis: & QListData) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QListData6removeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlist.h:111
// index:1
// Public Visibility=Default Availability=Available
// [-2] void remove(int, int)

/*

*/
impl /*struct*/ QListData {
  pub fn remove_1<RetType, T: QListData_remove_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remove_1(self);
    // return 1;
  }
}
pub trait QListData_remove_1<RetType> {
  fn remove_1(self , rsthis: & QListData) -> RetType;
}
impl<'a> /*trait*/ QListData_remove_1<(/*void*/)> for (i32,i32) {
  fn remove_1(self , rsthis: & QListData) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QListData6removeEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlist.h:112
// index:0
// Public Visibility=Default Availability=Available
// [-2] void move(int, int)

/*
Moves the item at index position from to index position to.

Example:


  QList<QString> list;
  list << "A" << "B" << "C" << "D" << "E" << "F";
  list.move(1, 4);
  // list: ["A", "C", "D", "E", "B", "F"]



This is the same as insert(to, takeAt(from)).This function assumes that both from and to are at least 0 but less than size(). To avoid failure, test that both from and to are at least 0 and less than size().

See also swap(), insert(), and takeAt().
*/
impl /*struct*/ QListData {
  pub fn move__0<RetType, T: QListData_move__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.move__0(self);
    // return 1;
  }
}
pub trait QListData_move__0<RetType> {
  fn move__0(self , rsthis: & QListData) -> RetType;
}
impl<'a> /*trait*/ QListData_move__0<(/*void*/)> for (i32,i32) {
  fn move__0(self , rsthis: & QListData) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QListData4moveEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlist.h:113
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int size() const

/*
Returns the number of items in the list.

See also isEmpty() and count().
*/
impl /*struct*/ QListData {
  pub fn size_0<RetType, T: QListData_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QListData_size_0<RetType> {
  fn size_0(self , rsthis: & QListData) -> RetType;
}
impl<'a> /*trait*/ QListData_size_0<i32> for () {
  fn size_0(self , rsthis: & QListData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListData4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlist.h:114
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if the list contains no items; otherwise returns false.

See also size().
*/
impl /*struct*/ QListData {
  pub fn isEmpty_0<RetType, T: QListData_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QListData_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QListData) -> RetType;
}
impl<'a> /*trait*/ QListData_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QListData) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListData7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlist.h:115
// index:0
// Public inline Visibility=Default Availability=Available
// [8] void ** at(int) const

/*
Returns the item at index position i in the list. i must be a valid index position in the list (i.e., 0 <= i < size()).

This function is very fast (constant time).

See also value() and operator[]().
*/
impl /*struct*/ QListData {
  pub fn at_0<RetType, T: QListData_at_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.at_0(self);
    // return 1;
  }
}
pub trait QListData_at_0<RetType> {
  fn at_0(self , rsthis: & QListData) -> RetType;
}
impl<'a> /*trait*/ QListData_at_0<usize> for (i32) {
  fn at_0(self , rsthis: & QListData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListData2atEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlist.h:116
// index:0
// Public inline Visibility=Default Availability=Available
// [8] void ** begin() const

/*
Returns an STL-style iterator pointing to the first item in the list.

See also constBegin() and end().
*/
impl /*struct*/ QListData {
  pub fn begin_0<RetType, T: QListData_begin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_0(self);
    // return 1;
  }
}
pub trait QListData_begin_0<RetType> {
  fn begin_0(self , rsthis: & QListData) -> RetType;
}
impl<'a> /*trait*/ QListData_begin_0<usize> for () {
  fn begin_0(self , rsthis: & QListData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListData5beginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlist.h:117
// index:0
// Public inline Visibility=Default Availability=Available
// [8] void ** end() const

/*
Returns an STL-style iterator pointing to the imaginary item after the last item in the list.

See also begin() and constEnd().
*/
impl /*struct*/ QListData {
  pub fn end_0<RetType, T: QListData_end_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_0(self);
    // return 1;
  }
}
pub trait QListData_end_0<RetType> {
  fn end_0(self , rsthis: & QListData) -> RetType;
}
impl<'a> /*trait*/ QListData_end_0<usize> for () {
  fn end_0(self , rsthis: & QListData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListData3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQListData(this :*mut QListData) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN9QListDataD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QListData__ = i32;
// 
pub const QListData__DataHeaderSize :QListData__ = 16;
pub fn QListData_ItemName(val: i32) ->String {
  match val {
     QListData__DataHeaderSize => // 16
     {return String::from("DataHeaderSize");}
  _ => {return format!("{}", val);}
}
}
pub fn QListData_ItemName_s(val: i32) ->String {
  //var nilthis *QListData
  //return nilthis.ItemName(val);
  return QListData_ItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
