

// mod ::widgets::QStyleHintReturnMask
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
// extern C begin: 2
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
#[derive(Default)] // class sizeof(QStyleHintReturnMask)=16
pub struct QStyleHintReturnMask {
  qbase: QStyleHintReturn,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleHintReturnMask_ITF interface {
//    QStyleHintReturn_ITF
//    QStyleHintReturnMask_PTR() *QStyleHintReturnMask
//}
//func (ptr *QStyleHintReturnMask) QStyleHintReturnMask_PTR() *QStyleHintReturnMask { return ptr }

impl /*struct*/ QStyleHintReturnMask {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleHintReturnMask {
    return QStyleHintReturnMask{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleHintReturnMask {
//  type Target = QStyleHintReturnMaskBASE;
//
//  fn deref(&self) -> &QStyleHintReturnMaskBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleHintReturnMaskBASE> for QStyleHintReturnMask {
//  fn as_ref(& self) -> & QStyleHintReturnMaskBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:722
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleHintReturnMask()

/*

*/
// QStyleHintReturnMask() ctx.fn_proto_cpp
impl /*struct*/ QStyleHintReturnMask {
  pub fn QStyleHintReturnMask_0<T: QStyleHintReturnMask_QStyleHintReturnMask_0>(value: T) -> QStyleHintReturnMask {
    let rsthis = value.QStyleHintReturnMask_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleHintReturnMask_QStyleHintReturnMask_0 {
  fn QStyleHintReturnMask_0(self) -> QStyleHintReturnMask;
}
// QStyleHintReturnMask() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleHintReturnMask_QStyleHintReturnMask_0 for () {
  fn QStyleHintReturnMask_0(self) -> QStyleHintReturnMask {
    // unsafe{_ZN20QStyleHintReturnMaskC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN20QStyleHintReturnMaskC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleHintReturnMask{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:723
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QStyleHintReturnMask()

/*

*/
pub fn DeleteQStyleHintReturnMask(this :*mut QStyleHintReturnMask) {
    // let rv = qtrt::InvokeQtFunc6("_ZN20QStyleHintReturnMaskD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleHintReturnMask__StyleOptionType = i32;
// 
pub const QStyleHintReturnMask__Type :QStyleHintReturnMask__StyleOptionType = 61441;
pub fn QStyleHintReturnMask_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleHintReturnMask__Type => // 61441
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleHintReturnMask_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleHintReturnMask
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleHintReturnMask_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleHintReturnMask__StyleOptionVersion = i32;
// 1
pub const QStyleHintReturnMask__Version :QStyleHintReturnMask__StyleOptionVersion = 1;
pub fn QStyleHintReturnMask_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleHintReturnMask__Version => // 1
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleHintReturnMask_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleHintReturnMask
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleHintReturnMask_StyleOptionVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
