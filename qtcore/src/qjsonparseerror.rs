

// mod ::core::QJsonParseError
// package qtcore
// /usr/include/qt/QtCore/qjsondocument.h
// #include <qjsondocument.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 37
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QJsonParseError)=8
pub struct QJsonParseError {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QJsonParseError_ITF interface {
//    QJsonParseError_PTR() *QJsonParseError
//}
//func (ptr *QJsonParseError) QJsonParseError_PTR() *QJsonParseError { return ptr }

impl /*struct*/ QJsonParseError {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QJsonParseError {
    return QJsonParseError{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QJsonParseError {
//  type Target = QJsonParseErrorBASE;
//
//  fn deref(&self) -> &QJsonParseErrorBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QJsonParseErrorBASE> for QJsonParseError {
//  fn as_ref(& self) -> & QJsonParseErrorBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qjsondocument.h:73
// index:0
// Public Visibility=Default Availability=Available
// [8] QString errorString() const

/*

*/
impl /*struct*/ QJsonParseError {
  pub fn errorString_0<RetType, T: QJsonParseError_errorString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.errorString_0(self);
    // return 1;
  }
}
pub trait QJsonParseError_errorString_0<RetType> {
  fn errorString_0(self , rsthis: & QJsonParseError) -> RetType;
}
impl<'a> /*trait*/ QJsonParseError_errorString_0<usize> for () {
  fn errorString_0(self , rsthis: & QJsonParseError) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QJsonParseError11errorStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQJsonParseError(this :*mut QJsonParseError) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN15QJsonParseErrorD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QJsonParseError__ParseError = i32;
// 
pub const QJsonParseError__NoError :QJsonParseError__ParseError = 0;
// 
pub const QJsonParseError__UnterminatedObject :QJsonParseError__ParseError = 1;
// 
pub const QJsonParseError__MissingNameSeparator :QJsonParseError__ParseError = 2;
// 
pub const QJsonParseError__UnterminatedArray :QJsonParseError__ParseError = 3;
// 
pub const QJsonParseError__MissingValueSeparator :QJsonParseError__ParseError = 4;
// 
pub const QJsonParseError__IllegalValue :QJsonParseError__ParseError = 5;
// 
pub const QJsonParseError__TerminationByNumber :QJsonParseError__ParseError = 6;
// 
pub const QJsonParseError__IllegalNumber :QJsonParseError__ParseError = 7;
// 
pub const QJsonParseError__IllegalEscapeSequence :QJsonParseError__ParseError = 8;
// 
pub const QJsonParseError__IllegalUTF8String :QJsonParseError__ParseError = 9;
// 
pub const QJsonParseError__UnterminatedString :QJsonParseError__ParseError = 10;
// 
pub const QJsonParseError__MissingObject :QJsonParseError__ParseError = 11;
// 
pub const QJsonParseError__DeepNesting :QJsonParseError__ParseError = 12;
// 
pub const QJsonParseError__DocumentTooLarge :QJsonParseError__ParseError = 13;
// 
pub const QJsonParseError__GarbageAtEnd :QJsonParseError__ParseError = 14;
pub fn QJsonParseError_ParseErrorItemName(val: i32) ->String {
  match val {
     QJsonParseError__NoError => // 0
     {return String::from("NoError");}
     QJsonParseError__UnterminatedObject => // 1
     {return String::from("UnterminatedObject");}
     QJsonParseError__MissingNameSeparator => // 2
     {return String::from("MissingNameSeparator");}
     QJsonParseError__UnterminatedArray => // 3
     {return String::from("UnterminatedArray");}
     QJsonParseError__MissingValueSeparator => // 4
     {return String::from("MissingValueSeparator");}
     QJsonParseError__IllegalValue => // 5
     {return String::from("IllegalValue");}
     QJsonParseError__TerminationByNumber => // 6
     {return String::from("TerminationByNumber");}
     QJsonParseError__IllegalNumber => // 7
     {return String::from("IllegalNumber");}
     QJsonParseError__IllegalEscapeSequence => // 8
     {return String::from("IllegalEscapeSequence");}
     QJsonParseError__IllegalUTF8String => // 9
     {return String::from("IllegalUTF8String");}
     QJsonParseError__UnterminatedString => // 10
     {return String::from("UnterminatedString");}
     QJsonParseError__MissingObject => // 11
     {return String::from("MissingObject");}
     QJsonParseError__DeepNesting => // 12
     {return String::from("DeepNesting");}
     QJsonParseError__DocumentTooLarge => // 13
     {return String::from("DocumentTooLarge");}
     QJsonParseError__GarbageAtEnd => // 14
     {return String::from("GarbageAtEnd");}
  _ => {return format!("{}", val);}
}
}
pub fn QJsonParseError_ParseErrorItemName_s(val: i32) ->String {
  //var nilthis *QJsonParseError
  //return nilthis.ParseErrorItemName(val);
  return QJsonParseError_ParseErrorItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
