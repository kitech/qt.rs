

// mod ::widgets::QLayoutItem
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
// extern C begin: 58
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
#[derive(Default)] // class sizeof(QLayoutItem)=16
pub struct QLayoutItem {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QLayoutItem_ITF interface {
//    QLayoutItem_PTR() *QLayoutItem
//}
//func (ptr *QLayoutItem) QLayoutItem_PTR() *QLayoutItem { return ptr }

impl /*struct*/ QLayoutItem {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QLayoutItem {
    return QLayoutItem{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QLayoutItem {
//  type Target = QLayoutItemBASE;
//
//  fn deref(&self) -> &QLayoutItemBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QLayoutItemBASE> for QLayoutItem {
//  fn as_ref(& self) -> & QLayoutItemBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qlayoutitem.h:63
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QLayoutItem(Qt::Alignment)

/*
Constructs a layout item with an alignment. Not all subclasses support alignment.
*/
// QLayoutItem(Qt::Alignment) ctx.fn_proto_cpp
impl /*struct*/ QLayoutItem {
  pub fn QLayoutItem_0<T: QLayoutItem_QLayoutItem_0>(value: T) -> QLayoutItem {
    let rsthis = value.QLayoutItem_0();
    return rsthis;
    // return 1;
  }
}

pub trait QLayoutItem_QLayoutItem_0 {
  fn QLayoutItem_0(self) -> QLayoutItem;
}
// QLayoutItem(Qt::Alignment) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLayoutItem_QLayoutItem_0 for (i32) {
  fn QLayoutItem_0(self) -> QLayoutItem {
    // unsafe{_ZN11QLayoutItemC2E6QFlagsIN2Qt13AlignmentFlagEE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QLayoutItemC2E6QFlagsIN2Qt13AlignmentFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLayoutItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:64
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QLayoutItem()

/*

*/
pub fn DeleteQLayoutItem(this :*mut QLayoutItem) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QLayoutItemD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qlayoutitem.h:65
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Implemented in subclasses to return the preferred size of this item.
*/
impl /*struct*/ QLayoutItem {
  pub fn sizeHint_0<RetType, T: QLayoutItem_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QLayoutItem_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QLayoutItem_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QLayoutItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QLayoutItem8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:66
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QSize minimumSize() const

/*
Implemented in subclasses to return the minimum size of this item.
*/
impl /*struct*/ QLayoutItem {
  pub fn minimumSize_0<RetType, T: QLayoutItem_minimumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSize_0(self);
    // return 1;
  }
}
pub trait QLayoutItem_minimumSize_0<RetType> {
  fn minimumSize_0(self , rsthis: & QLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QLayoutItem_minimumSize_0<usize> for () {
  fn minimumSize_0(self , rsthis: & QLayoutItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QLayoutItem11minimumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:67
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QSize maximumSize() const

/*
Implemented in subclasses to return the maximum size of this item.
*/
impl /*struct*/ QLayoutItem {
  pub fn maximumSize_0<RetType, T: QLayoutItem_maximumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumSize_0(self);
    // return 1;
  }
}
pub trait QLayoutItem_maximumSize_0<RetType> {
  fn maximumSize_0(self , rsthis: & QLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QLayoutItem_maximumSize_0<usize> for () {
  fn maximumSize_0(self , rsthis: & QLayoutItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QLayoutItem11maximumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:68
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] Qt::Orientations expandingDirections() const

/*
Returns whether this layout item can make use of more space than sizeHint(). A value of Qt::Vertical or Qt::Horizontal means that it wants to grow in only one dimension, whereas Qt::Vertical | Qt::Horizontal means that it wants to grow in both dimensions.
*/
impl /*struct*/ QLayoutItem {
  pub fn expandingDirections_0<RetType, T: QLayoutItem_expandingDirections_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.expandingDirections_0(self);
    // return 1;
  }
}
pub trait QLayoutItem_expandingDirections_0<RetType> {
  fn expandingDirections_0(self , rsthis: & QLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QLayoutItem_expandingDirections_0<i32> for () {
  fn expandingDirections_0(self , rsthis: & QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QLayoutItem19expandingDirectionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:69
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void setGeometry(const QRect &)

/*
Implemented in subclasses to set this item's geometry to r.

See also geometry().
*/
impl /*struct*/ QLayoutItem {
  pub fn setGeometry_0<RetType, T: QLayoutItem_setGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_0(self);
    // return 1;
  }
}
pub trait QLayoutItem_setGeometry_0<RetType> {
  fn setGeometry_0(self , rsthis: & QLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QLayoutItem_setGeometry_0<(/*void*/)> for (usize) {
  fn setGeometry_0(self , rsthis: & QLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QLayoutItem11setGeometryERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:70
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [16] QRect geometry() const

/*
Returns the rectangle covered by this layout item.

See also setGeometry().
*/
impl /*struct*/ QLayoutItem {
  pub fn geometry_0<RetType, T: QLayoutItem_geometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.geometry_0(self);
    // return 1;
  }
}
pub trait QLayoutItem_geometry_0<RetType> {
  fn geometry_0(self , rsthis: & QLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QLayoutItem_geometry_0<usize> for () {
  fn geometry_0(self , rsthis: & QLayoutItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QLayoutItem8geometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:71
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Implemented in subclasses to return whether this item is empty, i.e. whether it contains any widgets.
*/
impl /*struct*/ QLayoutItem {
  pub fn isEmpty_0<RetType, T: QLayoutItem_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QLayoutItem_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QLayoutItem_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QLayoutItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QLayoutItem7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool hasHeightForWidth() const

/*
Returns true if this layout's preferred height depends on its width; otherwise returns false. The default implementation returns false.

Reimplement this function in layout managers that support height for width.

See also heightForWidth() and QWidget::heightForWidth().
*/
impl /*struct*/ QLayoutItem {
  pub fn hasHeightForWidth_0<RetType, T: QLayoutItem_hasHeightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth_0(self);
    // return 1;
  }
}
pub trait QLayoutItem_hasHeightForWidth_0<RetType> {
  fn hasHeightForWidth_0(self , rsthis: & QLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QLayoutItem_hasHeightForWidth_0<bool> for () {
  fn hasHeightForWidth_0(self , rsthis: & QLayoutItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QLayoutItem17hasHeightForWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:73
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
impl /*struct*/ QLayoutItem {
  pub fn heightForWidth_0<RetType, T: QLayoutItem_heightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth_0(self);
    // return 1;
  }
}
pub trait QLayoutItem_heightForWidth_0<RetType> {
  fn heightForWidth_0(self , rsthis: & QLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QLayoutItem_heightForWidth_0<i32> for (i32) {
  fn heightForWidth_0(self , rsthis: & QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QLayoutItem14heightForWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int minimumHeightForWidth(int) const

/*
Returns the minimum height this widget needs for the given width, w. The default implementation simply returns heightForWidth(w).
*/
impl /*struct*/ QLayoutItem {
  pub fn minimumHeightForWidth_0<RetType, T: QLayoutItem_minimumHeightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumHeightForWidth_0(self);
    // return 1;
  }
}
pub trait QLayoutItem_minimumHeightForWidth_0<RetType> {
  fn minimumHeightForWidth_0(self , rsthis: & QLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QLayoutItem_minimumHeightForWidth_0<i32> for (i32) {
  fn minimumHeightForWidth_0(self , rsthis: & QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QLayoutItem21minimumHeightForWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:75
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void invalidate()

/*
Invalidates any cached information in this layout item.
*/
impl /*struct*/ QLayoutItem {
  pub fn invalidate_0<RetType, T: QLayoutItem_invalidate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invalidate_0(self);
    // return 1;
  }
}
pub trait QLayoutItem_invalidate_0<RetType> {
  fn invalidate_0(self , rsthis: & QLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QLayoutItem_invalidate_0<(/*void*/)> for () {
  fn invalidate_0(self , rsthis: & QLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QLayoutItem10invalidateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:77
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QWidget * widget()

/*
If this item manages a QWidget, returns that widget. Otherwise, nullptr is returned.

Note: While the functions layout() and spacerItem() perform casts, this function returns another object: QLayout and QSpacerItem inherit QLayoutItem, while QWidget does not.

See also layout() and spacerItem().
*/
impl /*struct*/ QLayoutItem {
  pub fn widget_0<RetType, T: QLayoutItem_widget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widget_0(self);
    // return 1;
  }
}
pub trait QLayoutItem_widget_0<RetType> {
  fn widget_0(self , rsthis: & QLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QLayoutItem_widget_0<usize> for () {
  fn widget_0(self , rsthis: & QLayoutItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QLayoutItem6widgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:78
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QLayout * layout()

/*
If this item is a QLayout, it is returned as a QLayout; otherwise 0 is returned. This function provides type-safe casting.

See also spacerItem() and widget().
*/
impl /*struct*/ QLayoutItem {
  pub fn layout_0<RetType, T: QLayoutItem_layout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.layout_0(self);
    // return 1;
  }
}
pub trait QLayoutItem_layout_0<RetType> {
  fn layout_0(self , rsthis: & QLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QLayoutItem_layout_0<usize> for () {
  fn layout_0(self , rsthis: & QLayoutItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QLayoutItem6layoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:79
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSpacerItem * spacerItem()

/*
If this item is a QSpacerItem, it is returned as a QSpacerItem; otherwise 0 is returned. This function provides type-safe casting.

See also layout() and widget().
*/
impl /*struct*/ QLayoutItem {
  pub fn spacerItem_0<RetType, T: QLayoutItem_spacerItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.spacerItem_0(self);
    // return 1;
  }
}
pub trait QLayoutItem_spacerItem_0<RetType> {
  fn spacerItem_0(self , rsthis: & QLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QLayoutItem_spacerItem_0<usize> for () {
  fn spacerItem_0(self , rsthis: & QLayoutItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QLayoutItem10spacerItemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:81
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::Alignment alignment() const

/*
Returns the alignment of this item.

See also setAlignment().
*/
impl /*struct*/ QLayoutItem {
  pub fn alignment_0<RetType, T: QLayoutItem_alignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alignment_0(self);
    // return 1;
  }
}
pub trait QLayoutItem_alignment_0<RetType> {
  fn alignment_0(self , rsthis: & QLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QLayoutItem_alignment_0<i32> for () {
  fn alignment_0(self , rsthis: & QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QLayoutItem9alignmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAlignment(Qt::Alignment)

/*
Sets the alignment of this item to alignment.

Note: Item alignment is only supported by QLayoutItem subclasses where it would have a visual effect. Except for QSpacerItem, which provides blank space for layouts, all public Qt classes that inherit QLayoutItem support item alignment.

See also alignment().
*/
impl /*struct*/ QLayoutItem {
  pub fn setAlignment_0<RetType, T: QLayoutItem_setAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlignment_0(self);
    // return 1;
  }
}
pub trait QLayoutItem_setAlignment_0<RetType> {
  fn setAlignment_0(self , rsthis: & QLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QLayoutItem_setAlignment_0<(/*void*/)> for (i32) {
  fn setAlignment_0(self , rsthis: & QLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QLayoutItem12setAlignmentE6QFlagsIN2Qt13AlignmentFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayoutitem.h:83
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] QSizePolicy::ControlTypes controlTypes() const

/*
Returns the control type(s) for the layout item. For a QWidgetItem, the control type comes from the widget's size policy; for a QLayoutItem, the control types is derived from the layout's contents.

See also QSizePolicy::controlType().
*/
impl /*struct*/ QLayoutItem {
  pub fn controlTypes_0<RetType, T: QLayoutItem_controlTypes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.controlTypes_0(self);
    // return 1;
  }
}
pub trait QLayoutItem_controlTypes_0<RetType> {
  fn controlTypes_0(self , rsthis: & QLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QLayoutItem_controlTypes_0<i32> for () {
  fn controlTypes_0(self , rsthis: & QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QLayoutItem12controlTypesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
