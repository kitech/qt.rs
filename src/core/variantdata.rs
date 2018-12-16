

// mod ::core::VariantData
// package qtcore
// /usr/include/qt/QtCore/qmetatype.h
// #include <qmetatype.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 0
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
#[derive(Default)] // class sizeof(VariantData)=24
pub struct VariantData {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type VariantData_ITF interface {
//    VariantData_PTR() *VariantData
//}
//func (ptr *VariantData) VariantData_PTR() *VariantData { return ptr }

impl /*struct*/ VariantData {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> VariantData {
    return VariantData{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for VariantData {
//  type Target = VariantDataBASE;
//
//  fn deref(&self) -> &VariantDataBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<VariantDataBASE> for VariantData {
//  fn as_ref(& self) -> & VariantDataBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qmetatype.h:798
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void VariantData(const int, const void *, const uint)

/*

*/
// VariantData(const int, const void *, const uint) ctx.fn_proto_cpp
impl /*struct*/ VariantData {
  pub fn VariantData_0<T: VariantData_VariantData_0>(value: T) -> VariantData {
    let rsthis = value.VariantData_0();
    return rsthis;
    // return 1;
  }
}

pub trait VariantData_VariantData_0 {
  fn VariantData_0(self) -> VariantData;
}
// VariantData(const int, const void *, const uint) ctx.fn_proto_cpp
impl<'a> /*trait*/ VariantData_VariantData_0 for (i32,usize,u32) {
  fn VariantData_0(self) -> VariantData {
    // unsafe{_ZN17QtMetaTypePrivate11VariantDataC2EiPKvj()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const u32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QtMetaTypePrivate11VariantDataC2EiPKvj", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = VariantData{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteVariantData(this :*mut VariantData) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN11VariantDataD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
