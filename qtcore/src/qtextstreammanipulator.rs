

// mod ::core::QTextStreamManipulator
// package qtcore
// /usr/include/qt/QtCore/qtextstream.h
// #include <qtextstream.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 79
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
#[derive(Default)] // class sizeof(QTextStreamManipulator)=40
pub struct QTextStreamManipulator {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextStreamManipulator_ITF interface {
//    QTextStreamManipulator_PTR() *QTextStreamManipulator
//}
//func (ptr *QTextStreamManipulator) QTextStreamManipulator_PTR() *QTextStreamManipulator { return ptr }

impl /*struct*/ QTextStreamManipulator {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextStreamManipulator {
    return QTextStreamManipulator{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextStreamManipulator {
//  type Target = QTextStreamManipulatorBASE;
//
//  fn deref(&self) -> &QTextStreamManipulatorBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextStreamManipulatorBASE> for QTextStreamManipulator {
//  fn as_ref(& self) -> & QTextStreamManipulatorBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qtextstream.h:217
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void exec(QTextStream &)

/*

*/
impl /*struct*/ QTextStreamManipulator {
  pub fn exec_0<RetType, T: QTextStreamManipulator_exec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exec_0(self);
    // return 1;
  }
}
pub trait QTextStreamManipulator_exec_0<RetType> {
  fn exec_0(self , rsthis: & QTextStreamManipulator) -> RetType;
}
impl<'a> /*trait*/ QTextStreamManipulator_exec_0<(/*void*/)> for (usize) {
  fn exec_0(self , rsthis: & QTextStreamManipulator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN22QTextStreamManipulator4execER11QTextStream", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQTextStreamManipulator(this :*mut QTextStreamManipulator) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN22QTextStreamManipulatorD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
