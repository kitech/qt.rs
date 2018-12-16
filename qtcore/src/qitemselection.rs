

// mod ::core::QItemSelection
// package qtcore
// /usr/include/qt/QtCore/qitemselectionmodel.h
// #include <qitemselectionmodel.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 31
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QItemSelection)=8
pub struct QItemSelection {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QItemSelection_ITF interface {
//    QItemSelection_PTR() *QItemSelection
//}
//func (ptr *QItemSelection) QItemSelection_PTR() *QItemSelection { return ptr }

impl /*struct*/ QItemSelection {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QItemSelection {
    return QItemSelection{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QItemSelection {
//  type Target = QItemSelectionBASE;
//
//  fn deref(&self) -> &QItemSelectionBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QItemSelectionBASE> for QItemSelection {
//  fn as_ref(& self) -> & QItemSelectionBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qitemselectionmodel.h:250
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QItemSelection()

/*

*/
// QItemSelection() ctx.fn_proto_cpp
impl /*struct*/ QItemSelection {
  pub fn QItemSelection_0<T: QItemSelection_QItemSelection_0>(value: T) -> QItemSelection {
    let rsthis = value.QItemSelection_0();
    return rsthis;
    // return 1;
  }
}

pub trait QItemSelection_QItemSelection_0 {
  fn QItemSelection_0(self) -> QItemSelection;
}
// QItemSelection() ctx.fn_proto_cpp
impl<'a> /*trait*/ QItemSelection_QItemSelection_0 for () {
  fn QItemSelection_0(self) -> QItemSelection {
    // unsafe{_ZN14QItemSelectionC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QItemSelectionC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QItemSelection{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:251
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QItemSelection(const QModelIndex &, const QModelIndex &)

/*

*/
// QItemSelection(const QModelIndex &, const QModelIndex &) ctx.fn_proto_cpp
impl /*struct*/ QItemSelection {
  pub fn QItemSelection_1<T: QItemSelection_QItemSelection_1>(value: T) -> QItemSelection {
    let rsthis = value.QItemSelection_1();
    return rsthis;
    // return 1;
  }
}

pub trait QItemSelection_QItemSelection_1 {
  fn QItemSelection_1(self) -> QItemSelection;
}
// QItemSelection(const QModelIndex &, const QModelIndex &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QItemSelection_QItemSelection_1 for (usize,usize) {
  fn QItemSelection_1(self) -> QItemSelection {
    // unsafe{_ZN14QItemSelectionC2ERK11QModelIndexS2_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QItemSelectionC2ERK11QModelIndexS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QItemSelection{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:255
// index:0
// Public Visibility=Default Availability=Available
// [-2] void select(const QModelIndex &, const QModelIndex &)

/*
Selects the model item index using the specified command, and emits selectionChanged().

See also QItemSelectionModel::SelectionFlags.
*/
impl /*struct*/ QItemSelection {
  pub fn select__0<RetType, T: QItemSelection_select__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.select__0(self);
    // return 1;
  }
}
pub trait QItemSelection_select__0<RetType> {
  fn select__0(self , rsthis: & QItemSelection) -> RetType;
}
impl<'a> /*trait*/ QItemSelection_select__0<(/*void*/)> for (usize,usize) {
  fn select__0(self , rsthis: & QItemSelection) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QItemSelection6selectERK11QModelIndexS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:256
// index:0
// Public Visibility=Default Availability=Available
// [1] bool contains(const QModelIndex &) const

/*

*/
impl /*struct*/ QItemSelection {
  pub fn contains_0<RetType, T: QItemSelection_contains_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_0(self);
    // return 1;
  }
}
pub trait QItemSelection_contains_0<RetType> {
  fn contains_0(self , rsthis: & QItemSelection) -> RetType;
}
impl<'a> /*trait*/ QItemSelection_contains_0<bool> for (usize) {
  fn contains_0(self , rsthis: & QItemSelection) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QItemSelection8containsERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:257
// index:0
// Public Visibility=Default Availability=Available
// [8] QModelIndexList indexes() const

/*

*/
impl /*struct*/ QItemSelection {
  pub fn indexes_0<RetType, T: QItemSelection_indexes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexes_0(self);
    // return 1;
  }
}
pub trait QItemSelection_indexes_0<RetType> {
  fn indexes_0(self , rsthis: & QItemSelection) -> RetType;
}
impl<'a> /*trait*/ QItemSelection_indexes_0<usize> for () {
  fn indexes_0(self , rsthis: & QItemSelection) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QItemSelection7indexesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:258
// index:0
// Public Visibility=Default Availability=Available
// [-2] void merge(const QItemSelection &, QItemSelectionModel::SelectionFlags)

/*

*/
impl /*struct*/ QItemSelection {
  pub fn merge_0<RetType, T: QItemSelection_merge_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.merge_0(self);
    // return 1;
  }
}
pub trait QItemSelection_merge_0<RetType> {
  fn merge_0(self , rsthis: & QItemSelection) -> RetType;
}
impl<'a> /*trait*/ QItemSelection_merge_0<(/*void*/)> for (usize,i32) {
  fn merge_0(self , rsthis: & QItemSelection) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QItemSelection5mergeERKS_6QFlagsIN19QItemSelectionModel13SelectionFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:259
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void split(const QItemSelectionRange &, const QItemSelectionRange &, QItemSelection *)

/*

*/
impl /*struct*/ QItemSelection {
  pub fn split_0<RetType, T: QItemSelection_split_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.split_0();
    // return 1;
  }
}
pub trait QItemSelection_split_0<RetType> {
  fn split_0(self ) -> RetType;
}
impl<'a> /*trait*/ QItemSelection_split_0<(/*void*/)> for (usize,usize,usize) {
  fn split_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QItemSelection5splitERK19QItemSelectionRangeS2_PS_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQItemSelection(this :*mut QItemSelection) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN14QItemSelectionD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
