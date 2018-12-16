

// mod ::widgets::QStyleOptionComplex
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
#[derive(Default)] // class sizeof(QStyleOptionComplex)=72
pub struct QStyleOptionComplex {
  qbase: QStyleOption,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionComplex_ITF interface {
//    QStyleOption_ITF
//    QStyleOptionComplex_PTR() *QStyleOptionComplex
//}
//func (ptr *QStyleOptionComplex) QStyleOptionComplex_PTR() *QStyleOptionComplex { return ptr }

impl /*struct*/ QStyleOptionComplex {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionComplex {
    return QStyleOptionComplex{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionComplex {
//  type Target = QStyleOptionComplexBASE;
//
//  fn deref(&self) -> &QStyleOptionComplexBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionComplexBASE> for QStyleOptionComplex {
//  fn as_ref(& self) -> & QStyleOptionComplexBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:509
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionComplex(int, int)

/*

*/
// QStyleOptionComplex(int, int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionComplex {
  pub fn QStyleOptionComplex_0<T: QStyleOptionComplex_QStyleOptionComplex_0>(value: T) -> QStyleOptionComplex {
    let rsthis = value.QStyleOptionComplex_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionComplex_QStyleOptionComplex_0 {
  fn QStyleOptionComplex_0(self) -> QStyleOptionComplex;
}
// QStyleOptionComplex(int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionComplex_QStyleOptionComplex_0 for (i32,i32) {
  fn QStyleOptionComplex_0(self) -> QStyleOptionComplex {
    // unsafe{_ZN19QStyleOptionComplexC2Eii()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QStyleOptionComplexC2Eii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionComplex{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionComplex(this :*mut QStyleOptionComplex) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN19QStyleOptionComplexD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionComplex__StyleOptionType = i32;
// 
pub const QStyleOptionComplex__Type :QStyleOptionComplex__StyleOptionType = 983040;
pub fn QStyleOptionComplex_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionComplex__Type => // 983040
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionComplex_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionComplex
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionComplex_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionComplex__StyleOptionVersion = i32;
// 1
pub const QStyleOptionComplex__Version :QStyleOptionComplex__StyleOptionVersion = 1;
pub fn QStyleOptionComplex_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionComplex__Version => // 1
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionComplex_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionComplex
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionComplex_StyleOptionVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
