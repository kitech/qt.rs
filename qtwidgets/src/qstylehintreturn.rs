

// mod ::widgets::QStyleHintReturn
// package qtwidgets
// /usr/include/qt/QtWidgets/qstyleoption.h
// #include <qstyleoption.h>
// #include <QtWidgets>

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
#[derive(Default)] // class sizeof(QStyleHintReturn)=8
pub struct QStyleHintReturn {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleHintReturn_ITF interface {
//    QStyleHintReturn_PTR() *QStyleHintReturn
//}
//func (ptr *QStyleHintReturn) QStyleHintReturn_PTR() *QStyleHintReturn { return ptr }

impl /*struct*/ QStyleHintReturn {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleHintReturn {
    return QStyleHintReturn{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleHintReturn {
//  type Target = QStyleHintReturnBASE;
//
//  fn deref(&self) -> &QStyleHintReturnBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleHintReturnBASE> for QStyleHintReturn {
//  fn as_ref(& self) -> & QStyleHintReturnBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:710
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleHintReturn(int, int)

/*

*/
// QStyleHintReturn(int, int) ctx.fn_proto_cpp
impl /*struct*/ QStyleHintReturn {
  pub fn QStyleHintReturn_0<T: QStyleHintReturn_QStyleHintReturn_0>(value: T) -> QStyleHintReturn {
    let rsthis = value.QStyleHintReturn_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleHintReturn_QStyleHintReturn_0 {
  fn QStyleHintReturn_0(self) -> QStyleHintReturn;
}
// QStyleHintReturn(int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleHintReturn_QStyleHintReturn_0 for (i32,i32) {
  fn QStyleHintReturn_0(self) -> QStyleHintReturn {
    // unsafe{_ZN16QStyleHintReturnC2Eii()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QStyleHintReturnC2Eii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleHintReturn{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:711
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QStyleHintReturn()

/*

*/
pub fn DeleteQStyleHintReturn(this :*mut QStyleHintReturn) {
    // let rv = qtrt::InvokeQtFunc6("_ZN16QStyleHintReturnD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QStyleHintReturn__HintReturnType = i32;
// 
pub const QStyleHintReturn__SH_Default :QStyleHintReturn__HintReturnType = 61440;
// 
pub const QStyleHintReturn__SH_Mask :QStyleHintReturn__HintReturnType = 61441;
// 
pub const QStyleHintReturn__SH_Variant :QStyleHintReturn__HintReturnType = 61442;
pub fn QStyleHintReturn_HintReturnTypeItemName(val: i32) ->String {
  match val {
     QStyleHintReturn__SH_Default => // 61440
     {return String::from("SH_Default");}
     QStyleHintReturn__SH_Mask => // 61441
     {return String::from("SH_Mask");}
     QStyleHintReturn__SH_Variant => // 61442
     {return String::from("SH_Variant");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleHintReturn_HintReturnTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleHintReturn
  //return nilthis.HintReturnTypeItemName(val);
  return QStyleHintReturn_HintReturnTypeItemName(val);
}


/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleHintReturn__StyleOptionType = i32;
// 
pub const QStyleHintReturn__Type :QStyleHintReturn__StyleOptionType = 61440;
pub fn QStyleHintReturn_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleHintReturn__Type => // 61440
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleHintReturn_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleHintReturn
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleHintReturn_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleHintReturn__StyleOptionVersion = i32;
// 1
pub const QStyleHintReturn__Version :QStyleHintReturn__StyleOptionVersion = 1;
pub fn QStyleHintReturn_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleHintReturn__Version => // 1
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleHintReturn_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleHintReturn
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleHintReturn_StyleOptionVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
