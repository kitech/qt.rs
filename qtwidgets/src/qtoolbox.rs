

// mod ::widgets::QToolBox
// package qtwidgets
// /usr/include/qt/QtWidgets/qtoolbox.h
// #include <qtoolbox.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 45
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

// bool event(QEvent *)
// func (this *QToolBox) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void itemInserted(int)
// func (this *QToolBox) InheritItemInserted(f func(index int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "itemInserted", f)
// }

// void itemRemoved(int)
// func (this *QToolBox) InheritItemRemoved(f func(index int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "itemRemoved", f)
// }

// void showEvent(QShowEvent *)
// func (this *QToolBox) InheritShowEvent(f func(e *qtgui.QShowEvent/*777 QShowEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "showEvent", f)
// }

// void changeEvent(QEvent *)
// func (this *QToolBox) InheritChangeEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QToolBox)=48
pub struct QToolBox {
  qbase: QFrame,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QToolBox_ITF interface {
//    QFrame_ITF
//    QToolBox_PTR() *QToolBox
//}
//func (ptr *QToolBox) QToolBox_PTR() *QToolBox { return ptr }

impl /*struct*/ QToolBox {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QToolBox {
    return QToolBox{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QToolBox {
//  type Target = QToolBoxBASE;
//
//  fn deref(&self) -> &QToolBoxBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QToolBoxBASE> for QToolBox {
//  fn as_ref(& self) -> & QToolBoxBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qtoolbox.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QToolBox {
  pub fn metaObject_0<RetType, T: QToolBox_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QToolBox_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QToolBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBox10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:60
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QToolBox(QWidget *, Qt::WindowFlags)

/*
Constructs a new toolbox with the given parent and the flags, f.
*/
// QToolBox(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QToolBox {
  pub fn QToolBox_0<T: QToolBox_QToolBox_0>(value: T) -> QToolBox {
    let rsthis = value.QToolBox_0();
    return rsthis;
    // return 1;
  }
}

pub trait QToolBox_QToolBox_0 {
  fn QToolBox_0(self) -> QToolBox;
}
// QToolBox(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QToolBox_QToolBox_0 for (usize,i32) {
  fn QToolBox_0(self) -> QToolBox {
    // unsafe{_ZN8QToolBoxC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QToolBoxC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QToolBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QToolBox()

/*

*/
pub fn DeleteQToolBox(this :*mut QToolBox) {
    // let rv = qtrt::InvokeQtFunc6("_ZN8QToolBoxD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qtoolbox.h:63
// index:0
// Public Visibility=Default Availability=Available
// [4] int addItem(QWidget *, const QString &)

/*
Adds the widget in a new tab at bottom of the toolbox. The new tab's text is set to text, and the iconSet is displayed to the left of the text. Returns the new tab's index.
*/
impl /*struct*/ QToolBox {
  pub fn addItem_0<RetType, T: QToolBox_addItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addItem_0(self);
    // return 1;
  }
}
pub trait QToolBox_addItem_0<RetType> {
  fn addItem_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_addItem_0<i32> for (usize,usize) {
  fn addItem_0(self , rsthis: & QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QToolBox7addItemEP7QWidgetRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:64
// index:1
// Public Visibility=Default Availability=Available
// [4] int addItem(QWidget *, const QIcon &, const QString &)

/*
Adds the widget in a new tab at bottom of the toolbox. The new tab's text is set to text, and the iconSet is displayed to the left of the text. Returns the new tab's index.
*/
impl /*struct*/ QToolBox {
  pub fn addItem_1<RetType, T: QToolBox_addItem_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addItem_1(self);
    // return 1;
  }
}
pub trait QToolBox_addItem_1<RetType> {
  fn addItem_1(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_addItem_1<i32> for (usize,usize,usize) {
  fn addItem_1(self , rsthis: & QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QToolBox7addItemEP7QWidgetRK5QIconRK7QString", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:65
// index:0
// Public Visibility=Default Availability=Available
// [4] int insertItem(int, QWidget *, const QString &)

/*
Inserts the widget at position index, or at the bottom of the toolbox if index is out of range. The new item's text is set to text, and the icon is displayed to the left of the text. Returns the new item's index.
*/
impl /*struct*/ QToolBox {
  pub fn insertItem_0<RetType, T: QToolBox_insertItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertItem_0(self);
    // return 1;
  }
}
pub trait QToolBox_insertItem_0<RetType> {
  fn insertItem_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_insertItem_0<i32> for (i32,usize,usize) {
  fn insertItem_0(self , rsthis: & QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QToolBox10insertItemEiP7QWidgetRK7QString", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:66
// index:1
// Public Visibility=Default Availability=Available
// [4] int insertItem(int, QWidget *, const QIcon &, const QString &)

/*
Inserts the widget at position index, or at the bottom of the toolbox if index is out of range. The new item's text is set to text, and the icon is displayed to the left of the text. Returns the new item's index.
*/
impl /*struct*/ QToolBox {
  pub fn insertItem_1<RetType, T: QToolBox_insertItem_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertItem_1(self);
    // return 1;
  }
}
pub trait QToolBox_insertItem_1<RetType> {
  fn insertItem_1(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_insertItem_1<i32> for (i32,usize,usize,usize) {
  fn insertItem_1(self , rsthis: & QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QToolBox10insertItemEiP7QWidgetRK5QIconRK7QString", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:68
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeItem(int)

/*
Removes the item at position index from the toolbox. Note that the widget is not deleted.
*/
impl /*struct*/ QToolBox {
  pub fn removeItem_0<RetType, T: QToolBox_removeItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeItem_0(self);
    // return 1;
  }
}
pub trait QToolBox_removeItem_0<RetType> {
  fn removeItem_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_removeItem_0<(/*void*/)> for (i32) {
  fn removeItem_0(self , rsthis: & QToolBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBox10removeItemEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:70
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemEnabled(int, bool)

/*
If enabled is true then the item at position index is enabled; otherwise the item at position index is disabled.

See also isItemEnabled().
*/
impl /*struct*/ QToolBox {
  pub fn setItemEnabled_0<RetType, T: QToolBox_setItemEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemEnabled_0(self);
    // return 1;
  }
}
pub trait QToolBox_setItemEnabled_0<RetType> {
  fn setItemEnabled_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_setItemEnabled_0<(/*void*/)> for (i32,bool) {
  fn setItemEnabled_0(self , rsthis: & QToolBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBox14setItemEnabledEib", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:71
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isItemEnabled(int) const

/*
Returns true if the item at position index is enabled; otherwise returns false.
*/
impl /*struct*/ QToolBox {
  pub fn isItemEnabled_0<RetType, T: QToolBox_isItemEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isItemEnabled_0(self);
    // return 1;
  }
}
pub trait QToolBox_isItemEnabled_0<RetType> {
  fn isItemEnabled_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_isItemEnabled_0<bool> for (i32) {
  fn isItemEnabled_0(self , rsthis: & QToolBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBox13isItemEnabledEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:73
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemText(int, const QString &)

/*
Sets the text of the item at position index to text.

If the provided text contains an ampersand character ('&'), a mnemonic is automatically created for it. The character that follows the '&' will be used as the shortcut key. Any previous mnemonic will be overwritten, or cleared if no mnemonic is defined by the text. See the QShortcut documentation for details (to display an actual ampersand, use '&&').

See also itemText().
*/
impl /*struct*/ QToolBox {
  pub fn setItemText_0<RetType, T: QToolBox_setItemText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemText_0(self);
    // return 1;
  }
}
pub trait QToolBox_setItemText_0<RetType> {
  fn setItemText_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_setItemText_0<(/*void*/)> for (i32,usize) {
  fn setItemText_0(self , rsthis: & QToolBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBox11setItemTextEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:74
// index:0
// Public Visibility=Default Availability=Available
// [8] QString itemText(int) const

/*
Returns the text of the item at position index, or an empty string if index is out of range.

See also setItemText().
*/
impl /*struct*/ QToolBox {
  pub fn itemText_0<RetType, T: QToolBox_itemText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemText_0(self);
    // return 1;
  }
}
pub trait QToolBox_itemText_0<RetType> {
  fn itemText_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_itemText_0<usize> for (i32) {
  fn itemText_0(self , rsthis: & QToolBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBox8itemTextEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:76
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemIcon(int, const QIcon &)

/*
Sets the icon of the item at position index to icon.

See also itemIcon().
*/
impl /*struct*/ QToolBox {
  pub fn setItemIcon_0<RetType, T: QToolBox_setItemIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemIcon_0(self);
    // return 1;
  }
}
pub trait QToolBox_setItemIcon_0<RetType> {
  fn setItemIcon_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_setItemIcon_0<(/*void*/)> for (i32,usize) {
  fn setItemIcon_0(self , rsthis: & QToolBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBox11setItemIconEiRK5QIcon", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:77
// index:0
// Public Visibility=Default Availability=Available
// [8] QIcon itemIcon(int) const

/*
Returns the icon of the item at position index, or a null icon if index is out of range.

See also setItemIcon().
*/
impl /*struct*/ QToolBox {
  pub fn itemIcon_0<RetType, T: QToolBox_itemIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemIcon_0(self);
    // return 1;
  }
}
pub trait QToolBox_itemIcon_0<RetType> {
  fn itemIcon_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_itemIcon_0<usize> for (i32) {
  fn itemIcon_0(self , rsthis: & QToolBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBox8itemIconEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemToolTip(int, const QString &)

/*
Sets the tooltip of the item at position index to toolTip.

See also itemToolTip().
*/
impl /*struct*/ QToolBox {
  pub fn setItemToolTip_0<RetType, T: QToolBox_setItemToolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemToolTip_0(self);
    // return 1;
  }
}
pub trait QToolBox_setItemToolTip_0<RetType> {
  fn setItemToolTip_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_setItemToolTip_0<(/*void*/)> for (i32,usize) {
  fn setItemToolTip_0(self , rsthis: & QToolBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBox14setItemToolTipEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:81
// index:0
// Public Visibility=Default Availability=Available
// [8] QString itemToolTip(int) const

/*
Returns the tooltip of the item at position index, or an empty string if index is out of range.

See also setItemToolTip().
*/
impl /*struct*/ QToolBox {
  pub fn itemToolTip_0<RetType, T: QToolBox_itemToolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemToolTip_0(self);
    // return 1;
  }
}
pub trait QToolBox_itemToolTip_0<RetType> {
  fn itemToolTip_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_itemToolTip_0<usize> for (i32) {
  fn itemToolTip_0(self , rsthis: & QToolBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBox11itemToolTipEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:84
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentIndex() const

/*

*/
impl /*struct*/ QToolBox {
  pub fn currentIndex_0<RetType, T: QToolBox_currentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentIndex_0(self);
    // return 1;
  }
}
pub trait QToolBox_currentIndex_0<RetType> {
  fn currentIndex_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_currentIndex_0<i32> for () {
  fn currentIndex_0(self , rsthis: & QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBox12currentIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:85
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * currentWidget() const

/*
Returns a pointer to the current widget, or 0 if there is no such item.

See also currentIndex() and setCurrentWidget().
*/
impl /*struct*/ QToolBox {
  pub fn currentWidget_0<RetType, T: QToolBox_currentWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentWidget_0(self);
    // return 1;
  }
}
pub trait QToolBox_currentWidget_0<RetType> {
  fn currentWidget_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_currentWidget_0<usize> for () {
  fn currentWidget_0(self , rsthis: & QToolBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBox13currentWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:86
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * widget(int) const

/*
Returns the widget at position index, or 0 if there is no such item.
*/
impl /*struct*/ QToolBox {
  pub fn widget_0<RetType, T: QToolBox_widget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widget_0(self);
    // return 1;
  }
}
pub trait QToolBox_widget_0<RetType> {
  fn widget_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_widget_0<usize> for (i32) {
  fn widget_0(self , rsthis: & QToolBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBox6widgetEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:87
// index:0
// Public Visibility=Default Availability=Available
// [4] int indexOf(QWidget *) const

/*
Returns the index of widget, or -1 if the item does not exist.
*/
impl /*struct*/ QToolBox {
  pub fn indexOf_0<RetType, T: QToolBox_indexOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_0(self);
    // return 1;
  }
}
pub trait QToolBox_indexOf_0<RetType> {
  fn indexOf_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_indexOf_0<i32> for (usize) {
  fn indexOf_0(self , rsthis: & QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBox7indexOfEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:88
// index:0
// Public Visibility=Default Availability=Available
// [4] int count() const

/*

*/
impl /*struct*/ QToolBox {
  pub fn count_0<RetType, T: QToolBox_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QToolBox_count_0<RetType> {
  fn count_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_count_0<i32> for () {
  fn count_0(self , rsthis: & QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBox5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentIndex(int)

/*

*/
impl /*struct*/ QToolBox {
  pub fn setCurrentIndex_0<RetType, T: QToolBox_setCurrentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex_0(self);
    // return 1;
  }
}
pub trait QToolBox_setCurrentIndex_0<RetType> {
  fn setCurrentIndex_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_setCurrentIndex_0<(/*void*/)> for (i32) {
  fn setCurrentIndex_0(self , rsthis: & QToolBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBox15setCurrentIndexEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentWidget(QWidget *)

/*
Makeswidget the current widget. The widget must be an item in this tool box.

See also addItem(), setCurrentIndex(), and currentWidget().
*/
impl /*struct*/ QToolBox {
  pub fn setCurrentWidget_0<RetType, T: QToolBox_setCurrentWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentWidget_0(self);
    // return 1;
  }
}
pub trait QToolBox_setCurrentWidget_0<RetType> {
  fn setCurrentWidget_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_setCurrentWidget_0<(/*void*/)> for (usize) {
  fn setCurrentWidget_0(self , rsthis: & QToolBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBox16setCurrentWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentChanged(int)

/*
This signal is emitted when the current item is changed. The new current item's index is passed in index, or -1 if there is no current item.

Note: Notifier signal for property currentIndex.
*/
impl /*struct*/ QToolBox {
  pub fn currentChanged_0<RetType, T: QToolBox_currentChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentChanged_0(self);
    // return 1;
  }
}
pub trait QToolBox_currentChanged_0<RetType> {
  fn currentChanged_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_currentChanged_0<(/*void*/)> for (i32) {
  fn currentChanged_0(self , rsthis: & QToolBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBox14currentChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:98
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QToolBox {
  pub fn event_0<RetType, T: QToolBox_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QToolBox_event_0<RetType> {
  fn event_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QToolBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QToolBox5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:99
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void itemInserted(int)

/*
This virtual handler is called after a new item was added or inserted at position index.

See also itemRemoved().
*/
impl /*struct*/ QToolBox {
  pub fn itemInserted_0<RetType, T: QToolBox_itemInserted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemInserted_0(self);
    // return 1;
  }
}
pub trait QToolBox_itemInserted_0<RetType> {
  fn itemInserted_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_itemInserted_0<(/*void*/)> for (i32) {
  fn itemInserted_0(self , rsthis: & QToolBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBox12itemInsertedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:100
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void itemRemoved(int)

/*
This virtual handler is called after an item was removed from position index.

See also itemInserted().
*/
impl /*struct*/ QToolBox {
  pub fn itemRemoved_0<RetType, T: QToolBox_itemRemoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemRemoved_0(self);
    // return 1;
  }
}
pub trait QToolBox_itemRemoved_0<RetType> {
  fn itemRemoved_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_itemRemoved_0<(/*void*/)> for (i32) {
  fn itemRemoved_0(self , rsthis: & QToolBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBox11itemRemovedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:101
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void showEvent(QShowEvent *)

/*
Reimplemented from QWidget::showEvent().
*/
impl /*struct*/ QToolBox {
  pub fn showEvent_0<RetType, T: QToolBox_showEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showEvent_0(self);
    // return 1;
  }
}
pub trait QToolBox_showEvent_0<RetType> {
  fn showEvent_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_showEvent_0<(/*void*/)> for (usize) {
  fn showEvent_0(self , rsthis: & QToolBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBox9showEventEP10QShowEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbox.h:102
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QToolBox {
  pub fn changeEvent_0<RetType, T: QToolBox_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QToolBox_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QToolBox) -> RetType;
}
impl<'a> /*trait*/ QToolBox_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QToolBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBox11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
