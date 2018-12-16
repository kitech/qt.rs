

// mod ::widgets::QStyleOptionRubberBand
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
#[derive(Default)] // class sizeof(QStyleOptionRubberBand)=72
pub struct QStyleOptionRubberBand {
  qbase: QStyleOption,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionRubberBand_ITF interface {
//    QStyleOption_ITF
//    QStyleOptionRubberBand_PTR() *QStyleOptionRubberBand
//}
//func (ptr *QStyleOptionRubberBand) QStyleOptionRubberBand_PTR() *QStyleOptionRubberBand { return ptr }

impl /*struct*/ QStyleOptionRubberBand {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionRubberBand {
    return QStyleOptionRubberBand{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionRubberBand {
//  type Target = QStyleOptionRubberBandBASE;
//
//  fn deref(&self) -> &QStyleOptionRubberBandBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionRubberBandBASE> for QStyleOptionRubberBand {
//  fn as_ref(& self) -> & QStyleOptionRubberBandBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:491
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionRubberBand()

/*

*/
// QStyleOptionRubberBand() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionRubberBand {
  pub fn QStyleOptionRubberBand_0<T: QStyleOptionRubberBand_QStyleOptionRubberBand_0>(value: T) -> QStyleOptionRubberBand {
    let rsthis = value.QStyleOptionRubberBand_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionRubberBand_QStyleOptionRubberBand_0 {
  fn QStyleOptionRubberBand_0(self) -> QStyleOptionRubberBand;
}
// QStyleOptionRubberBand() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionRubberBand_QStyleOptionRubberBand_0 for () {
  fn QStyleOptionRubberBand_0(self) -> QStyleOptionRubberBand {
    // unsafe{_ZN22QStyleOptionRubberBandC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN22QStyleOptionRubberBandC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionRubberBand{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:495
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionRubberBand(int)

/*

*/
// QStyleOptionRubberBand(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionRubberBand {
  pub fn QStyleOptionRubberBand_1<T: QStyleOptionRubberBand_QStyleOptionRubberBand_1>(value: T) -> QStyleOptionRubberBand {
    let rsthis = value.QStyleOptionRubberBand_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionRubberBand_QStyleOptionRubberBand_1 {
  fn QStyleOptionRubberBand_1(self) -> QStyleOptionRubberBand;
}
// QStyleOptionRubberBand(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionRubberBand_QStyleOptionRubberBand_1 for (i32) {
  fn QStyleOptionRubberBand_1(self) -> QStyleOptionRubberBand {
    // unsafe{_ZN22QStyleOptionRubberBandC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN22QStyleOptionRubberBandC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionRubberBand{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionRubberBand(this :*mut QStyleOptionRubberBand) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN22QStyleOptionRubberBandD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionRubberBand__StyleOptionType = i32;
// 
pub const QStyleOptionRubberBand__Type :QStyleOptionRubberBand__StyleOptionType = 13;
pub fn QStyleOptionRubberBand_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionRubberBand__Type => // 13
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionRubberBand_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionRubberBand
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionRubberBand_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionRubberBand__StyleOptionVersion = i32;
// 1
pub const QStyleOptionRubberBand__Version :QStyleOptionRubberBand__StyleOptionVersion = 1;
pub fn QStyleOptionRubberBand_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionRubberBand__Version => // 1
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionRubberBand_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionRubberBand
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionRubberBand_StyleOptionVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
