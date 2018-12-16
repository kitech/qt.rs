

// mod ::core::QGenericReturnArgument
// package qtcore
// /usr/include/qt/QtCore/qobjectdefs.h
// #include <qobjectdefs.h>
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
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QGenericReturnArgument)=16
pub struct QGenericReturnArgument {
  qbase: QGenericArgument,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGenericReturnArgument_ITF interface {
//    QGenericArgument_ITF
//    QGenericReturnArgument_PTR() *QGenericReturnArgument
//}
//func (ptr *QGenericReturnArgument) QGenericReturnArgument_PTR() *QGenericReturnArgument { return ptr }

impl /*struct*/ QGenericReturnArgument {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGenericReturnArgument {
    return QGenericReturnArgument{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGenericReturnArgument {
//  type Target = QGenericReturnArgumentBASE;
//
//  fn deref(&self) -> &QGenericReturnArgumentBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGenericReturnArgumentBASE> for QGenericReturnArgument {
//  fn as_ref(& self) -> & QGenericReturnArgumentBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qobjectdefs.h:310
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QGenericReturnArgument(const char *, void *)

/*

*/
// QGenericReturnArgument(const char *, void *) ctx.fn_proto_cpp
impl /*struct*/ QGenericReturnArgument {
  pub fn QGenericReturnArgument_0<T: QGenericReturnArgument_QGenericReturnArgument_0>(value: T) -> QGenericReturnArgument {
    let rsthis = value.QGenericReturnArgument_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGenericReturnArgument_QGenericReturnArgument_0 {
  fn QGenericReturnArgument_0(self) -> QGenericReturnArgument;
}
// QGenericReturnArgument(const char *, void *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGenericReturnArgument_QGenericReturnArgument_0 for (usize,usize) {
  fn QGenericReturnArgument_0(self) -> QGenericReturnArgument {
    // unsafe{_ZN22QGenericReturnArgumentC2EPKcPv()};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN22QGenericReturnArgumentC2EPKcPv", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGenericReturnArgument{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQGenericReturnArgument(this :*mut QGenericReturnArgument) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN22QGenericReturnArgumentD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
