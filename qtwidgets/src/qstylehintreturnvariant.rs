

// mod ::widgets::QStyleHintReturnVariant
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
#[derive(Default)] // class sizeof(QStyleHintReturnVariant)=24
pub struct QStyleHintReturnVariant {
  qbase: QStyleHintReturn,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleHintReturnVariant_ITF interface {
//    QStyleHintReturn_ITF
//    QStyleHintReturnVariant_PTR() *QStyleHintReturnVariant
//}
//func (ptr *QStyleHintReturnVariant) QStyleHintReturnVariant_PTR() *QStyleHintReturnVariant { return ptr }

impl /*struct*/ QStyleHintReturnVariant {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleHintReturnVariant {
    return QStyleHintReturnVariant{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleHintReturnVariant {
//  type Target = QStyleHintReturnVariantBASE;
//
//  fn deref(&self) -> &QStyleHintReturnVariantBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleHintReturnVariantBASE> for QStyleHintReturnVariant {
//  fn as_ref(& self) -> & QStyleHintReturnVariantBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:733
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleHintReturnVariant()

/*

*/
// QStyleHintReturnVariant() ctx.fn_proto_cpp
impl /*struct*/ QStyleHintReturnVariant {
  pub fn QStyleHintReturnVariant_0<T: QStyleHintReturnVariant_QStyleHintReturnVariant_0>(value: T) -> QStyleHintReturnVariant {
    let rsthis = value.QStyleHintReturnVariant_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleHintReturnVariant_QStyleHintReturnVariant_0 {
  fn QStyleHintReturnVariant_0(self) -> QStyleHintReturnVariant;
}
// QStyleHintReturnVariant() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleHintReturnVariant_QStyleHintReturnVariant_0 for () {
  fn QStyleHintReturnVariant_0(self) -> QStyleHintReturnVariant {
    // unsafe{_ZN23QStyleHintReturnVariantC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN23QStyleHintReturnVariantC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleHintReturnVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:734
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QStyleHintReturnVariant()

/*

*/
pub fn DeleteQStyleHintReturnVariant(this :*mut QStyleHintReturnVariant) {
    // let rv = qtrt::InvokeQtFunc6("_ZN23QStyleHintReturnVariantD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleHintReturnVariant__StyleOptionType = i32;
// 
pub const QStyleHintReturnVariant__Type :QStyleHintReturnVariant__StyleOptionType = 61442;
pub fn QStyleHintReturnVariant_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleHintReturnVariant__Type => // 61442
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleHintReturnVariant_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleHintReturnVariant
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleHintReturnVariant_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleHintReturnVariant__StyleOptionVersion = i32;
// 1
pub const QStyleHintReturnVariant__Version :QStyleHintReturnVariant__StyleOptionVersion = 1;
pub fn QStyleHintReturnVariant_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleHintReturnVariant__Version => // 1
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleHintReturnVariant_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleHintReturnVariant
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleHintReturnVariant_StyleOptionVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
