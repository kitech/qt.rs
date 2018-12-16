

// mod ::gui::QTextItem
// package qtgui
// /usr/include/qt/QtGui/qpaintengine.h
// #include <qpaintengine.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 203
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QTextItem)=1
pub struct QTextItem {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextItem_ITF interface {
//    QTextItem_PTR() *QTextItem
//}
//func (ptr *QTextItem) QTextItem_PTR() *QTextItem { return ptr }

impl /*struct*/ QTextItem {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextItem {
    return QTextItem{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextItem {
//  type Target = QTextItemBASE;
//
//  fn deref(&self) -> &QTextItemBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextItemBASE> for QTextItem {
//  fn as_ref(& self) -> & QTextItemBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qpaintengine.h:75
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal descent() const

/*

*/
impl /*struct*/ QTextItem {
  pub fn descent_0<RetType, T: QTextItem_descent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.descent_0(self);
    // return 1;
  }
}
pub trait QTextItem_descent_0<RetType> {
  fn descent_0(self , rsthis: & QTextItem) -> RetType;
}
impl<'a> /*trait*/ QTextItem_descent_0<f64> for () {
  fn descent_0(self , rsthis: & QTextItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextItem7descentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:76
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal ascent() const

/*

*/
impl /*struct*/ QTextItem {
  pub fn ascent_0<RetType, T: QTextItem_ascent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ascent_0(self);
    // return 1;
  }
}
pub trait QTextItem_ascent_0<RetType> {
  fn ascent_0(self , rsthis: & QTextItem) -> RetType;
}
impl<'a> /*trait*/ QTextItem_ascent_0<f64> for () {
  fn ascent_0(self , rsthis: & QTextItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextItem6ascentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:77
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal width() const

/*

*/
impl /*struct*/ QTextItem {
  pub fn width_0<RetType, T: QTextItem_width_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_0(self);
    // return 1;
  }
}
pub trait QTextItem_width_0<RetType> {
  fn width_0(self , rsthis: & QTextItem) -> RetType;
}
impl<'a> /*trait*/ QTextItem_width_0<f64> for () {
  fn width_0(self , rsthis: & QTextItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextItem5widthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:79
// index:0
// Public Visibility=Default Availability=Available
// [4] QTextItem::RenderFlags renderFlags() const

/*

*/
impl /*struct*/ QTextItem {
  pub fn renderFlags_0<RetType, T: QTextItem_renderFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.renderFlags_0(self);
    // return 1;
  }
}
pub trait QTextItem_renderFlags_0<RetType> {
  fn renderFlags_0(self , rsthis: & QTextItem) -> RetType;
}
impl<'a> /*trait*/ QTextItem_renderFlags_0<i32> for () {
  fn renderFlags_0(self , rsthis: & QTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextItem11renderFlagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:80
// index:0
// Public Visibility=Default Availability=Available
// [8] QString text() const

/*

*/
impl /*struct*/ QTextItem {
  pub fn text_0<RetType, T: QTextItem_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QTextItem_text_0<RetType> {
  fn text_0(self , rsthis: & QTextItem) -> RetType;
}
impl<'a> /*trait*/ QTextItem_text_0<usize> for () {
  fn text_0(self , rsthis: & QTextItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextItem4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:81
// index:0
// Public Visibility=Default Availability=Available
// [16] QFont font() const

/*

*/
impl /*struct*/ QTextItem {
  pub fn font_0<RetType, T: QTextItem_font_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.font_0(self);
    // return 1;
  }
}
pub trait QTextItem_font_0<RetType> {
  fn font_0(self , rsthis: & QTextItem) -> RetType;
}
impl<'a> /*trait*/ QTextItem_font_0<usize> for () {
  fn font_0(self , rsthis: & QTextItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextItem4fontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQTextItem(this :*mut QTextItem) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN9QTextItemD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QTextItem__RenderFlag = i32;
// 
pub const QTextItem__RightToLeft :QTextItem__RenderFlag = 1;
// 
pub const QTextItem__Overline :QTextItem__RenderFlag = 16;
// 
pub const QTextItem__Underline :QTextItem__RenderFlag = 32;
// 
pub const QTextItem__StrikeOut :QTextItem__RenderFlag = 64;
// 
pub const QTextItem__Dummy :QTextItem__RenderFlag = -1;
pub fn QTextItem_RenderFlagItemName(val: i32) ->String {
  match val {
     QTextItem__RightToLeft => // 1
     {return String::from("RightToLeft");}
     QTextItem__Overline => // 16
     {return String::from("Overline");}
     QTextItem__Underline => // 32
     {return String::from("Underline");}
     QTextItem__StrikeOut => // 64
     {return String::from("StrikeOut");}
     QTextItem__Dummy => // -1
     {return String::from("Dummy");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextItem_RenderFlagItemName_s(val: i32) ->String {
  //var nilthis *QTextItem
  //return nilthis.RenderFlagItemName(val);
  return QTextItem_RenderFlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
