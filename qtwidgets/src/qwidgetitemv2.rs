

// mod ::widgets::QWidgetItemV2
// package qtwidgets
// /usr/include/qt/QtWidgets/qlayoutitem.h
// #include <qlayoutitem.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 13
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
#[derive(Default)] // class sizeof(QWidgetItemV2)=88
pub struct QWidgetItemV2 {
  qbase: QWidgetItem,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QWidgetItemV2_ITF interface {
//    QWidgetItem_ITF
//    QWidgetItemV2_PTR() *QWidgetItemV2
//}
//func (ptr *QWidgetItemV2) QWidgetItemV2_PTR() *QWidgetItemV2 { return ptr }

impl /*struct*/ QWidgetItemV2 {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QWidgetItemV2 {
    return QWidgetItemV2{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QWidgetItemV2 {
//  type Target = QWidgetItemV2BASE;
//
//  fn deref(&self) -> &QWidgetItemV2BASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QWidgetItemV2BASE> for QWidgetItemV2 {
//  fn as_ref(& self) -> & QWidgetItemV2BASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qlayoutitem.h:148
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QWidgetItemV2(QWidget *)

/*

*/
// QWidgetItemV2(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QWidgetItemV2 {
  pub fn QWidgetItemV2_0<T: QWidgetItemV2_QWidgetItemV2_0>(value: T) -> QWidgetItemV2 {
    let rsthis = value.QWidgetItemV2_0();
    return rsthis;
    // return 1;
  }
}

pub trait QWidgetItemV2_QWidgetItemV2_0 {
  fn QWidgetItemV2_0(self) -> QWidgetItemV2;
}
// QWidgetItemV2(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QWidgetItemV2_QWidgetItemV2_0 for (usize) {
  fn QWidgetItemV2_0(self) -> QWidgetItemV2 {
    // unsafe{_ZN13QWidgetItemV2C2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QWidgetItemV2C2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QWidgetItemV2{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:149
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QWidgetItemV2()

/*

*/
pub fn DeleteQWidgetItemV2(this :*mut QWidgetItemV2) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QWidgetItemV2D2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 88)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qlayoutitem.h:151
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Implemented in subclasses to return the preferred size of this item.
*/
impl /*struct*/ QWidgetItemV2 {
  pub fn sizeHint_0<RetType, T: QWidgetItemV2_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QWidgetItemV2_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QWidgetItemV2) -> RetType;
}
impl<'a> /*trait*/ QWidgetItemV2_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QWidgetItemV2) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QWidgetItemV28sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:152
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSize() const

/*
Implemented in subclasses to return the minimum size of this item.
*/
impl /*struct*/ QWidgetItemV2 {
  pub fn minimumSize_0<RetType, T: QWidgetItemV2_minimumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSize_0(self);
    // return 1;
  }
}
pub trait QWidgetItemV2_minimumSize_0<RetType> {
  fn minimumSize_0(self , rsthis: & QWidgetItemV2) -> RetType;
}
impl<'a> /*trait*/ QWidgetItemV2_minimumSize_0<usize> for () {
  fn minimumSize_0(self , rsthis: & QWidgetItemV2) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QWidgetItemV211minimumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:153
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize maximumSize() const

/*
Implemented in subclasses to return the maximum size of this item.
*/
impl /*struct*/ QWidgetItemV2 {
  pub fn maximumSize_0<RetType, T: QWidgetItemV2_maximumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumSize_0(self);
    // return 1;
  }
}
pub trait QWidgetItemV2_maximumSize_0<RetType> {
  fn maximumSize_0(self , rsthis: & QWidgetItemV2) -> RetType;
}
impl<'a> /*trait*/ QWidgetItemV2_maximumSize_0<usize> for () {
  fn maximumSize_0(self , rsthis: & QWidgetItemV2) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QWidgetItemV211maximumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:154
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int heightForWidth(int) const

/*
Returns the preferred height for this layout item, given the width w.

The default implementation returns -1, indicating that the preferred height is independent of the width of the item. Using the function hasHeightForWidth() will typically be much faster than calling this function and testing for -1.

Reimplement this function in layout managers that support height for width. A typical implementation will look like this:


  int MyLayout::heightForWidth(int w) const
  {
      if (cache_dirty || cached_width != w) {
          // not all C++ compilers support "mutable"
          MyLayout *that = (MyLayout*)this;
          int h = calculateHeightForWidth(w);
          that->cached_hfw = h;
          return h;
      }
      return cached_hfw;
  }



Caching is strongly recommended; without it layout will take exponential time.

See also hasHeightForWidth().
*/
impl /*struct*/ QWidgetItemV2 {
  pub fn heightForWidth_0<RetType, T: QWidgetItemV2_heightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth_0(self);
    // return 1;
  }
}
pub trait QWidgetItemV2_heightForWidth_0<RetType> {
  fn heightForWidth_0(self , rsthis: & QWidgetItemV2) -> RetType;
}
impl<'a> /*trait*/ QWidgetItemV2_heightForWidth_0<i32> for (i32) {
  fn heightForWidth_0(self , rsthis: & QWidgetItemV2) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QWidgetItemV214heightForWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


/*


*/
pub type QWidgetItemV2__ = i32;
// 
pub const QWidgetItemV2__Dirty :QWidgetItemV2__ = -123;
// 
pub const QWidgetItemV2__HfwCacheMaxSize :QWidgetItemV2__ = 3;
pub fn QWidgetItemV2_ItemName(val: i32) ->String {
  match val {
     QWidgetItemV2__Dirty => // -123
     {return String::from("Dirty");}
     QWidgetItemV2__HfwCacheMaxSize => // 3
     {return String::from("HfwCacheMaxSize");}
  _ => {return format!("{}", val);}
}
}
pub fn QWidgetItemV2_ItemName_s(val: i32) ->String {
  //var nilthis *QWidgetItemV2
  //return nilthis.ItemName(val);
  return QWidgetItemV2_ItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
