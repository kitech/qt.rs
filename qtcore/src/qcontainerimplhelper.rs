

// mod ::core::QContainerImplHelper
// package qtcore
// /usr/include/qt/QtCore/qarraydata.h
// #include <qarraydata.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 8
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
#[derive(Default)] // class sizeof(QContainerImplHelper)=1
pub struct QContainerImplHelper {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QContainerImplHelper_ITF interface {
//    QContainerImplHelper_PTR() *QContainerImplHelper
//}
//func (ptr *QContainerImplHelper) QContainerImplHelper_PTR() *QContainerImplHelper { return ptr }

impl /*struct*/ QContainerImplHelper {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QContainerImplHelper {
    return QContainerImplHelper{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QContainerImplHelper {
//  type Target = QContainerImplHelperBASE;
//
//  fn deref(&self) -> &QContainerImplHelperBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QContainerImplHelperBASE> for QContainerImplHelper {
//  fn as_ref(& self) -> & QContainerImplHelperBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qarraydata.h:373
// index:0
// Public static Visibility=Default Availability=Available
// [4] QtPrivate::QContainerImplHelper::CutResult mid(int, int *, int *)

/*

*/
impl /*struct*/ QContainerImplHelper {
  pub fn mid_0<RetType, T: QContainerImplHelper_mid_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.mid_0();
    // return 1;
  }
}
pub trait QContainerImplHelper_mid_0<RetType> {
  fn mid_0(self ) -> RetType;
}
impl<'a> /*trait*/ QContainerImplHelper_mid_0<i32> for (i32,usize,usize) {
  fn mid_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QtPrivate20QContainerImplHelper3midEiPiS1_", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


pub fn DeleteQContainerImplHelper(this :*mut QContainerImplHelper) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN20QContainerImplHelperD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QContainerImplHelper__CutResult = i32;
// 
pub const QContainerImplHelper__Null :QContainerImplHelper__CutResult = 0;
// 
pub const QContainerImplHelper__Empty :QContainerImplHelper__CutResult = 1;
// 
pub const QContainerImplHelper__Full :QContainerImplHelper__CutResult = 2;
// 
pub const QContainerImplHelper__Subset :QContainerImplHelper__CutResult = 3;
pub fn QContainerImplHelper_CutResultItemName(val: i32) ->String {
  match val {
     QContainerImplHelper__Null => // 0
     {return String::from("Null");}
     QContainerImplHelper__Empty => // 1
     {return String::from("Empty");}
     QContainerImplHelper__Full => // 2
     {return String::from("Full");}
     QContainerImplHelper__Subset => // 3
     {return String::from("Subset");}
  _ => {return format!("{}", val);}
}
}
pub fn QContainerImplHelper_CutResultItemName_s(val: i32) ->String {
  //var nilthis *QContainerImplHelper
  //return nilthis.CutResultItemName(val);
  return QContainerImplHelper_CutResultItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
