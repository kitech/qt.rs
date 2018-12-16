

// mod ::core::QVariantComparisonHelper
// package qtcore
// /usr/include/qt/QtCore/qvariant.h
// #include <qvariant.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 109
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
#[derive(Default)] // class sizeof(QVariantComparisonHelper)=8
pub struct QVariantComparisonHelper {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QVariantComparisonHelper_ITF interface {
//    QVariantComparisonHelper_PTR() *QVariantComparisonHelper
//}
//func (ptr *QVariantComparisonHelper) QVariantComparisonHelper_PTR() *QVariantComparisonHelper { return ptr }

impl /*struct*/ QVariantComparisonHelper {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QVariantComparisonHelper {
    return QVariantComparisonHelper{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QVariantComparisonHelper {
//  type Target = QVariantComparisonHelperBASE;
//
//  fn deref(&self) -> &QVariantComparisonHelperBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QVariantComparisonHelperBASE> for QVariantComparisonHelper {
//  fn as_ref(& self) -> & QVariantComparisonHelperBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qvariant.h:560
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QVariantComparisonHelper(const QVariant &)

/*

*/
// QVariantComparisonHelper(const QVariant &) ctx.fn_proto_cpp
impl /*struct*/ QVariantComparisonHelper {
  pub fn QVariantComparisonHelper_0<T: QVariantComparisonHelper_QVariantComparisonHelper_0>(value: T) -> QVariantComparisonHelper {
    let rsthis = value.QVariantComparisonHelper_0();
    return rsthis;
    // return 1;
  }
}

pub trait QVariantComparisonHelper_QVariantComparisonHelper_0 {
  fn QVariantComparisonHelper_0(self) -> QVariantComparisonHelper;
}
// QVariantComparisonHelper(const QVariant &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariantComparisonHelper_QVariantComparisonHelper_0 for (usize) {
  fn QVariantComparisonHelper_0(self) -> QVariantComparisonHelper {
    // unsafe{_ZN24QVariantComparisonHelperC2ERK8QVariant()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN24QVariantComparisonHelperC2ERK8QVariant", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariantComparisonHelper{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQVariantComparisonHelper(this :*mut QVariantComparisonHelper) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN24QVariantComparisonHelperD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
