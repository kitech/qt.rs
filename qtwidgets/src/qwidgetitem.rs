

// mod ::widgets::QWidgetItem
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
// extern C begin: 12
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
#[derive(Default)] // class sizeof(QWidgetItem)=24
pub struct QWidgetItem {
  qbase: QLayoutItem,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QWidgetItem_ITF interface {
//    QLayoutItem_ITF
//    QWidgetItem_PTR() *QWidgetItem
//}
//func (ptr *QWidgetItem) QWidgetItem_PTR() *QWidgetItem { return ptr }

impl /*struct*/ QWidgetItem {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QWidgetItem {
    return QWidgetItem{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QWidgetItem {
//  type Target = QWidgetItemBASE;
//
//  fn deref(&self) -> &QWidgetItemBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QWidgetItemBASE> for QWidgetItem {
//  fn as_ref(& self) -> & QWidgetItemBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qlayoutitem.h:126
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QWidgetItem(QWidget *)

/*

*/
// QWidgetItem(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QWidgetItem {
  pub fn QWidgetItem_0<T: QWidgetItem_QWidgetItem_0>(value: T) -> QWidgetItem {
    let rsthis = value.QWidgetItem_0();
    return rsthis;
    // return 1;
  }
}

pub trait QWidgetItem_QWidgetItem_0 {
  fn QWidgetItem_0(self) -> QWidgetItem;
}
// QWidgetItem(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QWidgetItem_QWidgetItem_0 for (usize) {
  fn QWidgetItem_0(self) -> QWidgetItem {
    // unsafe{_ZN11QWidgetItemC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QWidgetItemC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:127
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QWidgetItem()

/*

*/
pub fn DeleteQWidgetItem(this :*mut QWidgetItem) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QWidgetItemD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qlayoutitem.h:129
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Implemented in subclasses to return the preferred size of this item.
*/
impl /*struct*/ QWidgetItem {
  pub fn sizeHint_0<RetType, T: QWidgetItem_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QWidgetItem_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QWidgetItem_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWidgetItem8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:130
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSize() const

/*
Implemented in subclasses to return the minimum size of this item.
*/
impl /*struct*/ QWidgetItem {
  pub fn minimumSize_0<RetType, T: QWidgetItem_minimumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSize_0(self);
    // return 1;
  }
}
pub trait QWidgetItem_minimumSize_0<RetType> {
  fn minimumSize_0(self , rsthis: & QWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QWidgetItem_minimumSize_0<usize> for () {
  fn minimumSize_0(self , rsthis: & QWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWidgetItem11minimumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:131
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize maximumSize() const

/*
Implemented in subclasses to return the maximum size of this item.
*/
impl /*struct*/ QWidgetItem {
  pub fn maximumSize_0<RetType, T: QWidgetItem_maximumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumSize_0(self);
    // return 1;
  }
}
pub trait QWidgetItem_maximumSize_0<RetType> {
  fn maximumSize_0(self , rsthis: & QWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QWidgetItem_maximumSize_0<usize> for () {
  fn maximumSize_0(self , rsthis: & QWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWidgetItem11maximumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:132
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::Orientations expandingDirections() const

/*
Returns whether this layout item can make use of more space than sizeHint(). A value of Qt::Vertical or Qt::Horizontal means that it wants to grow in only one dimension, whereas Qt::Vertical | Qt::Horizontal means that it wants to grow in both dimensions.
*/
impl /*struct*/ QWidgetItem {
  pub fn expandingDirections_0<RetType, T: QWidgetItem_expandingDirections_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.expandingDirections_0(self);
    // return 1;
  }
}
pub trait QWidgetItem_expandingDirections_0<RetType> {
  fn expandingDirections_0(self , rsthis: & QWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QWidgetItem_expandingDirections_0<i32> for () {
  fn expandingDirections_0(self , rsthis: & QWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWidgetItem19expandingDirectionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:133
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Implemented in subclasses to return whether this item is empty, i.e. whether it contains any widgets.
*/
impl /*struct*/ QWidgetItem {
  pub fn isEmpty_0<RetType, T: QWidgetItem_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QWidgetItem_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QWidgetItem_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QWidgetItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWidgetItem7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:134
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setGeometry(const QRect &)

/*
Implemented in subclasses to set this item's geometry to r.

See also geometry().
*/
impl /*struct*/ QWidgetItem {
  pub fn setGeometry_0<RetType, T: QWidgetItem_setGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_0(self);
    // return 1;
  }
}
pub trait QWidgetItem_setGeometry_0<RetType> {
  fn setGeometry_0(self , rsthis: & QWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QWidgetItem_setGeometry_0<(/*void*/)> for (usize) {
  fn setGeometry_0(self , rsthis: & QWidgetItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QWidgetItem11setGeometryERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:135
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QRect geometry() const

/*
Returns the rectangle covered by this layout item.

See also setGeometry().
*/
impl /*struct*/ QWidgetItem {
  pub fn geometry_0<RetType, T: QWidgetItem_geometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.geometry_0(self);
    // return 1;
  }
}
pub trait QWidgetItem_geometry_0<RetType> {
  fn geometry_0(self , rsthis: & QWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QWidgetItem_geometry_0<usize> for () {
  fn geometry_0(self , rsthis: & QWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWidgetItem8geometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:136
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QWidget * widget()

/*
If this item manages a QWidget, returns that widget. Otherwise, nullptr is returned.

Note: While the functions layout() and spacerItem() perform casts, this function returns another object: QLayout and QSpacerItem inherit QLayoutItem, while QWidget does not.

See also layout() and spacerItem().
*/
impl /*struct*/ QWidgetItem {
  pub fn widget_0<RetType, T: QWidgetItem_widget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widget_0(self);
    // return 1;
  }
}
pub trait QWidgetItem_widget_0<RetType> {
  fn widget_0(self , rsthis: & QWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QWidgetItem_widget_0<usize> for () {
  fn widget_0(self , rsthis: & QWidgetItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QWidgetItem6widgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:138
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool hasHeightForWidth() const

/*
Returns true if this layout's preferred height depends on its width; otherwise returns false. The default implementation returns false.

Reimplement this function in layout managers that support height for width.

See also heightForWidth() and QWidget::heightForWidth().
*/
impl /*struct*/ QWidgetItem {
  pub fn hasHeightForWidth_0<RetType, T: QWidgetItem_hasHeightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth_0(self);
    // return 1;
  }
}
pub trait QWidgetItem_hasHeightForWidth_0<RetType> {
  fn hasHeightForWidth_0(self , rsthis: & QWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QWidgetItem_hasHeightForWidth_0<bool> for () {
  fn hasHeightForWidth_0(self , rsthis: & QWidgetItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWidgetItem17hasHeightForWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:139
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
impl /*struct*/ QWidgetItem {
  pub fn heightForWidth_0<RetType, T: QWidgetItem_heightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth_0(self);
    // return 1;
  }
}
pub trait QWidgetItem_heightForWidth_0<RetType> {
  fn heightForWidth_0(self , rsthis: & QWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QWidgetItem_heightForWidth_0<i32> for (i32) {
  fn heightForWidth_0(self , rsthis: & QWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWidgetItem14heightForWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:140
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] QSizePolicy::ControlTypes controlTypes() const

/*
Returns the control type(s) for the layout item. For a QWidgetItem, the control type comes from the widget's size policy; for a QLayoutItem, the control types is derived from the layout's contents.

See also QSizePolicy::controlType().
*/
impl /*struct*/ QWidgetItem {
  pub fn controlTypes_0<RetType, T: QWidgetItem_controlTypes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.controlTypes_0(self);
    // return 1;
  }
}
pub trait QWidgetItem_controlTypes_0<RetType> {
  fn controlTypes_0(self , rsthis: & QWidgetItem) -> RetType;
}
impl<'a> /*trait*/ QWidgetItem_controlTypes_0<i32> for () {
  fn controlTypes_0(self , rsthis: & QWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWidgetItem12controlTypesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
