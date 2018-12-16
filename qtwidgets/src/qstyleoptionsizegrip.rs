

// mod ::widgets::QStyleOptionSizeGrip
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
#[derive(Default)] // class sizeof(QStyleOptionSizeGrip)=80
pub struct QStyleOptionSizeGrip {
  qbase: QStyleOptionComplex,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionSizeGrip_ITF interface {
//    QStyleOptionComplex_ITF
//    QStyleOptionSizeGrip_PTR() *QStyleOptionSizeGrip
//}
//func (ptr *QStyleOptionSizeGrip) QStyleOptionSizeGrip_PTR() *QStyleOptionSizeGrip { return ptr }

impl /*struct*/ QStyleOptionSizeGrip {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionSizeGrip {
    return QStyleOptionSizeGrip{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionSizeGrip {
//  type Target = QStyleOptionSizeGripBASE;
//
//  fn deref(&self) -> &QStyleOptionSizeGripBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionSizeGripBASE> for QStyleOptionSizeGrip {
//  fn as_ref(& self) -> & QStyleOptionSizeGripBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:653
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionSizeGrip()

/*

*/
// QStyleOptionSizeGrip() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionSizeGrip {
  pub fn QStyleOptionSizeGrip_0<T: QStyleOptionSizeGrip_QStyleOptionSizeGrip_0>(value: T) -> QStyleOptionSizeGrip {
    let rsthis = value.QStyleOptionSizeGrip_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionSizeGrip_QStyleOptionSizeGrip_0 {
  fn QStyleOptionSizeGrip_0(self) -> QStyleOptionSizeGrip;
}
// QStyleOptionSizeGrip() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionSizeGrip_QStyleOptionSizeGrip_0 for () {
  fn QStyleOptionSizeGrip_0(self) -> QStyleOptionSizeGrip {
    // unsafe{_ZN20QStyleOptionSizeGripC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN20QStyleOptionSizeGripC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionSizeGrip{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:656
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionSizeGrip(int)

/*

*/
// QStyleOptionSizeGrip(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionSizeGrip {
  pub fn QStyleOptionSizeGrip_1<T: QStyleOptionSizeGrip_QStyleOptionSizeGrip_1>(value: T) -> QStyleOptionSizeGrip {
    let rsthis = value.QStyleOptionSizeGrip_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionSizeGrip_QStyleOptionSizeGrip_1 {
  fn QStyleOptionSizeGrip_1(self) -> QStyleOptionSizeGrip;
}
// QStyleOptionSizeGrip(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionSizeGrip_QStyleOptionSizeGrip_1 for (i32) {
  fn QStyleOptionSizeGrip_1(self) -> QStyleOptionSizeGrip {
    // unsafe{_ZN20QStyleOptionSizeGripC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN20QStyleOptionSizeGripC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionSizeGrip{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionSizeGrip(this :*mut QStyleOptionSizeGrip) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN20QStyleOptionSizeGripD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionSizeGrip__StyleOptionType = i32;
// 
pub const QStyleOptionSizeGrip__Type :QStyleOptionSizeGrip__StyleOptionType = 983047;
pub fn QStyleOptionSizeGrip_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionSizeGrip__Type => // 983047
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionSizeGrip_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionSizeGrip
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionSizeGrip_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionSizeGrip__StyleOptionVersion = i32;
// 1
pub const QStyleOptionSizeGrip__Version :QStyleOptionSizeGrip__StyleOptionVersion = 1;
pub fn QStyleOptionSizeGrip_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionSizeGrip__Version => // 1
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionSizeGrip_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionSizeGrip
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionSizeGrip_StyleOptionVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
