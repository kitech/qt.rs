

// mod ::widgets::QItemEditorCreatorBase
// package qtwidgets
// /usr/include/qt/QtWidgets/qitemeditorfactory.h
// #include <qitemeditorfactory.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 27
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
#[derive(Default)] // class sizeof(QItemEditorCreatorBase)=8
pub struct QItemEditorCreatorBase {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QItemEditorCreatorBase_ITF interface {
//    QItemEditorCreatorBase_PTR() *QItemEditorCreatorBase
//}
//func (ptr *QItemEditorCreatorBase) QItemEditorCreatorBase_PTR() *QItemEditorCreatorBase { return ptr }

impl /*struct*/ QItemEditorCreatorBase {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QItemEditorCreatorBase {
    return QItemEditorCreatorBase{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QItemEditorCreatorBase {
//  type Target = QItemEditorCreatorBaseBASE;
//
//  fn deref(&self) -> &QItemEditorCreatorBaseBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QItemEditorCreatorBaseBASE> for QItemEditorCreatorBase {
//  fn as_ref(& self) -> & QItemEditorCreatorBaseBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qitemeditorfactory.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QItemEditorCreatorBase()

/*

*/
pub fn DeleteQItemEditorCreatorBase(this :*mut QItemEditorCreatorBase) {
    // let rv = qtrt::InvokeQtFunc6("_ZN22QItemEditorCreatorBaseD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qitemeditorfactory.h:60
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QWidget * createWidget(QWidget *) const

/*

*/
impl /*struct*/ QItemEditorCreatorBase {
  pub fn createWidget_0<RetType, T: QItemEditorCreatorBase_createWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createWidget_0(self);
    // return 1;
  }
}
pub trait QItemEditorCreatorBase_createWidget_0<RetType> {
  fn createWidget_0(self , rsthis: & QItemEditorCreatorBase) -> RetType;
}
impl<'a> /*trait*/ QItemEditorCreatorBase_createWidget_0<usize> for (usize) {
  fn createWidget_0(self , rsthis: & QItemEditorCreatorBase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QItemEditorCreatorBase12createWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qitemeditorfactory.h:61
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QByteArray valuePropertyName() const

/*
Returns the property name used to access data for the given userType of data.
*/
impl /*struct*/ QItemEditorCreatorBase {
  pub fn valuePropertyName_0<RetType, T: QItemEditorCreatorBase_valuePropertyName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.valuePropertyName_0(self);
    // return 1;
  }
}
pub trait QItemEditorCreatorBase_valuePropertyName_0<RetType> {
  fn valuePropertyName_0(self , rsthis: & QItemEditorCreatorBase) -> RetType;
}
impl<'a> /*trait*/ QItemEditorCreatorBase_valuePropertyName_0<usize> for () {
  fn valuePropertyName_0(self , rsthis: & QItemEditorCreatorBase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QItemEditorCreatorBase17valuePropertyNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
