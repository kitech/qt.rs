

// mod ::core::QScopedPointerPodDeleter
// package qtcore
// /usr/include/qt/QtCore/qscopedpointer.h
// #include <qscopedpointer.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 13
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
#[derive(Default)] // class sizeof(QScopedPointerPodDeleter)=1
pub struct QScopedPointerPodDeleter {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QScopedPointerPodDeleter_ITF interface {
//    QScopedPointerPodDeleter_PTR() *QScopedPointerPodDeleter
//}
//func (ptr *QScopedPointerPodDeleter) QScopedPointerPodDeleter_PTR() *QScopedPointerPodDeleter { return ptr }

impl /*struct*/ QScopedPointerPodDeleter {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QScopedPointerPodDeleter {
    return QScopedPointerPodDeleter{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QScopedPointerPodDeleter {
//  type Target = QScopedPointerPodDeleterBASE;
//
//  fn deref(&self) -> &QScopedPointerPodDeleterBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QScopedPointerPodDeleterBASE> for QScopedPointerPodDeleter {
//  fn as_ref(& self) -> & QScopedPointerPodDeleterBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qscopedpointer.h:81
// index:0
// Public static inline Visibility=Default Availability=Available
// [-2] void cleanup(void *)

/*

*/
impl /*struct*/ QScopedPointerPodDeleter {
  pub fn cleanup_0<RetType, T: QScopedPointerPodDeleter_cleanup_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.cleanup_0();
    // return 1;
  }
}
pub trait QScopedPointerPodDeleter_cleanup_0<RetType> {
  fn cleanup_0(self ) -> RetType;
}
impl<'a> /*trait*/ QScopedPointerPodDeleter_cleanup_0<(/*void*/)> for (usize) {
  fn cleanup_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QScopedPointerPodDeleter7cleanupEPv", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQScopedPointerPodDeleter(this :*mut QScopedPointerPodDeleter) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN24QScopedPointerPodDeleterD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
