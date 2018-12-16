

// mod ::widgets::QStyleOptionTabBarBase
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
#[derive(Default)] // class sizeof(QStyleOptionTabBarBase)=104
pub struct QStyleOptionTabBarBase {
  qbase: QStyleOption,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionTabBarBase_ITF interface {
//    QStyleOption_ITF
//    QStyleOptionTabBarBase_PTR() *QStyleOptionTabBarBase
//}
//func (ptr *QStyleOptionTabBarBase) QStyleOptionTabBarBase_PTR() *QStyleOptionTabBarBase { return ptr }

impl /*struct*/ QStyleOptionTabBarBase {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionTabBarBase {
    return QStyleOptionTabBarBase{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionTabBarBase {
//  type Target = QStyleOptionTabBarBaseBASE;
//
//  fn deref(&self) -> &QStyleOptionTabBarBaseBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionTabBarBaseBASE> for QStyleOptionTabBarBase {
//  fn as_ref(& self) -> & QStyleOptionTabBarBaseBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:195
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionTabBarBase()

/*

*/
// QStyleOptionTabBarBase() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionTabBarBase {
  pub fn QStyleOptionTabBarBase_0<T: QStyleOptionTabBarBase_QStyleOptionTabBarBase_0>(value: T) -> QStyleOptionTabBarBase {
    let rsthis = value.QStyleOptionTabBarBase_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTabBarBase_QStyleOptionTabBarBase_0 {
  fn QStyleOptionTabBarBase_0(self) -> QStyleOptionTabBarBase;
}
// QStyleOptionTabBarBase() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionTabBarBase_QStyleOptionTabBarBase_0 for () {
  fn QStyleOptionTabBarBase_0(self) -> QStyleOptionTabBarBase {
    // unsafe{_ZN22QStyleOptionTabBarBaseC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN22QStyleOptionTabBarBaseC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionTabBarBase{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:199
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionTabBarBase(int)

/*

*/
// QStyleOptionTabBarBase(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionTabBarBase {
  pub fn QStyleOptionTabBarBase_1<T: QStyleOptionTabBarBase_QStyleOptionTabBarBase_1>(value: T) -> QStyleOptionTabBarBase {
    let rsthis = value.QStyleOptionTabBarBase_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTabBarBase_QStyleOptionTabBarBase_1 {
  fn QStyleOptionTabBarBase_1(self) -> QStyleOptionTabBarBase;
}
// QStyleOptionTabBarBase(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionTabBarBase_QStyleOptionTabBarBase_1 for (i32) {
  fn QStyleOptionTabBarBase_1(self) -> QStyleOptionTabBarBase {
    // unsafe{_ZN22QStyleOptionTabBarBaseC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN22QStyleOptionTabBarBaseC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionTabBarBase{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionTabBarBase(this :*mut QStyleOptionTabBarBase) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN22QStyleOptionTabBarBaseD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionTabBarBase__StyleOptionType = i32;
// 
pub const QStyleOptionTabBarBase__Type :QStyleOptionTabBarBase__StyleOptionType = 12;
pub fn QStyleOptionTabBarBase_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionTabBarBase__Type => // 12
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionTabBarBase_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionTabBarBase
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionTabBarBase_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionTabBarBase__StyleOptionVersion = i32;
// 1
pub const QStyleOptionTabBarBase__Version :QStyleOptionTabBarBase__StyleOptionVersion = 2;
pub fn QStyleOptionTabBarBase_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionTabBarBase__Version => // 2
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionTabBarBase_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionTabBarBase
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionTabBarBase_StyleOptionVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
