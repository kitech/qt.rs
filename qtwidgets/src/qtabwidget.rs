

// mod ::widgets::QTabWidget
// package qtwidgets
// /usr/include/qt/QtWidgets/qtabwidget.h
// #include <qtabwidget.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 81
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

// void tabInserted(int)
// func (this *QTabWidget) InheritTabInserted(f func(index int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "tabInserted", f)
// }

// void tabRemoved(int)
// func (this *QTabWidget) InheritTabRemoved(f func(index int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "tabRemoved", f)
// }

// void showEvent(QShowEvent *)
// func (this *QTabWidget) InheritShowEvent(f func(arg0 *qtgui.QShowEvent/*777 QShowEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "showEvent", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QTabWidget) InheritResizeEvent(f func(arg0 *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QTabWidget) InheritKeyPressEvent(f func(arg0 *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QTabWidget) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void setTabBar(QTabBar *)
// func (this *QTabWidget) InheritSetTabBar(f func(arg0 *QTabBar/*777 QTabBar **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setTabBar", f)
// }

// void changeEvent(QEvent *)
// func (this *QTabWidget) InheritChangeEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// bool event(QEvent *)
// func (this *QTabWidget) InheritEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void initStyleOption(QStyleOptionTabWidgetFrame *)
// func (this *QTabWidget) InheritInitStyleOption(f func(option *QStyleOptionTabWidgetFrame/*777 QStyleOptionTabWidgetFrame **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QTabWidget)=48
pub struct QTabWidget {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTabWidget_ITF interface {
//    QWidget_ITF
//    QTabWidget_PTR() *QTabWidget
//}
//func (ptr *QTabWidget) QTabWidget_PTR() *QTabWidget { return ptr }

impl /*struct*/ QTabWidget {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTabWidget {
    return QTabWidget{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTabWidget {
//  type Target = QTabWidgetBASE;
//
//  fn deref(&self) -> &QTabWidgetBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTabWidgetBASE> for QTabWidget {
//  fn as_ref(& self) -> & QTabWidgetBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qtabwidget.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTabWidget {
  pub fn metaObject_0<RetType, T: QTabWidget_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTabWidget_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTabWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTabWidget(QWidget *)

/*
Constructs a tabbed widget with parent parent.
*/
// QTabWidget(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QTabWidget {
  pub fn QTabWidget_0<T: QTabWidget_QTabWidget_0>(value: T) -> QTabWidget {
    let rsthis = value.QTabWidget_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTabWidget_QTabWidget_0 {
  fn QTabWidget_0(self) -> QTabWidget;
}
// QTabWidget(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTabWidget_QTabWidget_0 for (usize) {
  fn QTabWidget_0(self) -> QTabWidget {
    // unsafe{_ZN10QTabWidgetC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QTabWidgetC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTabWidget{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTabWidget()

/*

*/
pub fn DeleteQTabWidget(this :*mut QTabWidget) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QTabWidgetD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qtabwidget.h:74
// index:0
// Public Visibility=Default Availability=Available
// [4] int addTab(QWidget *, const QString &)

/*
Adds a tab with the given page and label to the tab widget, and returns the index of the tab in the tab bar.

If the tab's label contains an ampersand, the letter following the ampersand is used as a shortcut for the tab, e.g. if the label is "Bro&wse" then Alt+W becomes a shortcut which will move the focus to this tab.

Note: If you call addTab() after show(), the layout system will try to adjust to the changes in its widgets hierarchy and may cause flicker. To prevent this, you can set the QWidget::updatesEnabled property to false prior to changes; remember to set the property to true when the changes are done, making the widget receive paint events again.

See also insertTab().
*/
impl /*struct*/ QTabWidget {
  pub fn addTab_0<RetType, T: QTabWidget_addTab_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addTab_0(self);
    // return 1;
  }
}
pub trait QTabWidget_addTab_0<RetType> {
  fn addTab_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_addTab_0<i32> for (usize,usize) {
  fn addTab_0(self , rsthis: & QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTabWidget6addTabEP7QWidgetRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:75
// index:1
// Public Visibility=Default Availability=Available
// [4] int addTab(QWidget *, const QIcon &, const QString &)

/*
Adds a tab with the given page and label to the tab widget, and returns the index of the tab in the tab bar.

If the tab's label contains an ampersand, the letter following the ampersand is used as a shortcut for the tab, e.g. if the label is "Bro&wse" then Alt+W becomes a shortcut which will move the focus to this tab.

Note: If you call addTab() after show(), the layout system will try to adjust to the changes in its widgets hierarchy and may cause flicker. To prevent this, you can set the QWidget::updatesEnabled property to false prior to changes; remember to set the property to true when the changes are done, making the widget receive paint events again.

See also insertTab().
*/
impl /*struct*/ QTabWidget {
  pub fn addTab_1<RetType, T: QTabWidget_addTab_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addTab_1(self);
    // return 1;
  }
}
pub trait QTabWidget_addTab_1<RetType> {
  fn addTab_1(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_addTab_1<i32> for (usize,usize,usize) {
  fn addTab_1(self , rsthis: & QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTabWidget6addTabEP7QWidgetRK5QIconRK7QString", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:77
// index:0
// Public Visibility=Default Availability=Available
// [4] int insertTab(int, QWidget *, const QString &)

/*
Inserts a tab with the given label and page into the tab widget at the specified index, and returns the index of the inserted tab in the tab bar.

The label is displayed in the tab and may vary in appearance depending on the configuration of the tab widget.

If the tab's label contains an ampersand, the letter following the ampersand is used as a shortcut for the tab, e.g. if the label is "Bro&wse" then Alt+W becomes a shortcut which will move the focus to this tab.

If index is out of range, the tab is simply appended. Otherwise it is inserted at the specified position.

If the QTabWidget was empty before this function is called, the new page becomes the current page. Inserting a new tab at an index less than or equal to the current index will increment the current index, but keep the current page.

Note: If you call insertTab() after show(), the layout system will try to adjust to the changes in its widgets hierarchy and may cause flicker. To prevent this, you can set the QWidget::updatesEnabled property to false prior to changes; remember to set the property to true when the changes are done, making the widget receive paint events again.

See also addTab().
*/
impl /*struct*/ QTabWidget {
  pub fn insertTab_0<RetType, T: QTabWidget_insertTab_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertTab_0(self);
    // return 1;
  }
}
pub trait QTabWidget_insertTab_0<RetType> {
  fn insertTab_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_insertTab_0<i32> for (i32,usize,usize) {
  fn insertTab_0(self , rsthis: & QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTabWidget9insertTabEiP7QWidgetRK7QString", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:78
// index:1
// Public Visibility=Default Availability=Available
// [4] int insertTab(int, QWidget *, const QIcon &, const QString &)

/*
Inserts a tab with the given label and page into the tab widget at the specified index, and returns the index of the inserted tab in the tab bar.

The label is displayed in the tab and may vary in appearance depending on the configuration of the tab widget.

If the tab's label contains an ampersand, the letter following the ampersand is used as a shortcut for the tab, e.g. if the label is "Bro&wse" then Alt+W becomes a shortcut which will move the focus to this tab.

If index is out of range, the tab is simply appended. Otherwise it is inserted at the specified position.

If the QTabWidget was empty before this function is called, the new page becomes the current page. Inserting a new tab at an index less than or equal to the current index will increment the current index, but keep the current page.

Note: If you call insertTab() after show(), the layout system will try to adjust to the changes in its widgets hierarchy and may cause flicker. To prevent this, you can set the QWidget::updatesEnabled property to false prior to changes; remember to set the property to true when the changes are done, making the widget receive paint events again.

See also addTab().
*/
impl /*struct*/ QTabWidget {
  pub fn insertTab_1<RetType, T: QTabWidget_insertTab_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertTab_1(self);
    // return 1;
  }
}
pub trait QTabWidget_insertTab_1<RetType> {
  fn insertTab_1(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_insertTab_1<i32> for (i32,usize,usize,usize) {
  fn insertTab_1(self , rsthis: & QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTabWidget9insertTabEiP7QWidgetRK5QIconRK7QString", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeTab(int)

/*
Removes the tab at position index from this stack of widgets. The page widget itself is not deleted.

See also addTab() and insertTab().
*/
impl /*struct*/ QTabWidget {
  pub fn removeTab_0<RetType, T: QTabWidget_removeTab_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeTab_0(self);
    // return 1;
  }
}
pub trait QTabWidget_removeTab_0<RetType> {
  fn removeTab_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_removeTab_0<(/*void*/)> for (i32) {
  fn removeTab_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget9removeTabEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:82
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isTabEnabled(int) const

/*
Returns true if the page at position index is enabled; otherwise returns false.

See also setTabEnabled() and QWidget::isEnabled().
*/
impl /*struct*/ QTabWidget {
  pub fn isTabEnabled_0<RetType, T: QTabWidget_isTabEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isTabEnabled_0(self);
    // return 1;
  }
}
pub trait QTabWidget_isTabEnabled_0<RetType> {
  fn isTabEnabled_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_isTabEnabled_0<bool> for (i32) {
  fn isTabEnabled_0(self , rsthis: & QTabWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget12isTabEnabledEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabEnabled(int, bool)

/*
If enable is true, the page at position index is enabled; otherwise the page at position index is disabled. The page's tab is redrawn appropriately.

QTabWidget uses QWidget::setEnabled() internally, rather than keeping a separate flag.

Note that even a disabled tab/page may be visible. If the page is visible already, QTabWidget will not hide it; if all the pages are disabled, QTabWidget will show one of them.

See also isTabEnabled() and QWidget::setEnabled().
*/
impl /*struct*/ QTabWidget {
  pub fn setTabEnabled_0<RetType, T: QTabWidget_setTabEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabEnabled_0(self);
    // return 1;
  }
}
pub trait QTabWidget_setTabEnabled_0<RetType> {
  fn setTabEnabled_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_setTabEnabled_0<(/*void*/)> for (i32,bool) {
  fn setTabEnabled_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget13setTabEnabledEib", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:85
// index:0
// Public Visibility=Default Availability=Available
// [8] QString tabText(int) const

/*
Returns the label text for the tab on the page at position index.

See also setTabText().
*/
impl /*struct*/ QTabWidget {
  pub fn tabText_0<RetType, T: QTabWidget_tabText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabText_0(self);
    // return 1;
  }
}
pub trait QTabWidget_tabText_0<RetType> {
  fn tabText_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_tabText_0<usize> for (i32) {
  fn tabText_0(self , rsthis: & QTabWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget7tabTextEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabText(int, const QString &)

/*
Defines a new label for the page at position index's tab.

If the provided text contains an ampersand character ('&'), a shortcut is automatically created for it. The character that follows the '&' will be used as the shortcut key. Any previous shortcut will be overwritten, or cleared if no shortcut is defined by the text. See the QShortcut documentation for details (to display an actual ampersand, use '&&').

See also tabText().
*/
impl /*struct*/ QTabWidget {
  pub fn setTabText_0<RetType, T: QTabWidget_setTabText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabText_0(self);
    // return 1;
  }
}
pub trait QTabWidget_setTabText_0<RetType> {
  fn setTabText_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_setTabText_0<(/*void*/)> for (i32,usize) {
  fn setTabText_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget10setTabTextEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:88
// index:0
// Public Visibility=Default Availability=Available
// [8] QIcon tabIcon(int) const

/*
Returns the icon for the tab on the page at position index.

See also setTabIcon().
*/
impl /*struct*/ QTabWidget {
  pub fn tabIcon_0<RetType, T: QTabWidget_tabIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabIcon_0(self);
    // return 1;
  }
}
pub trait QTabWidget_tabIcon_0<RetType> {
  fn tabIcon_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_tabIcon_0<usize> for (i32) {
  fn tabIcon_0(self , rsthis: & QTabWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget7tabIconEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabIcon(int, const QIcon &)

/*
This is an overloaded function.

Sets the icon for the tab at position index.

See also tabIcon().
*/
impl /*struct*/ QTabWidget {
  pub fn setTabIcon_0<RetType, T: QTabWidget_setTabIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabIcon_0(self);
    // return 1;
  }
}
pub trait QTabWidget_setTabIcon_0<RetType> {
  fn setTabIcon_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_setTabIcon_0<(/*void*/)> for (i32,usize) {
  fn setTabIcon_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget10setTabIconEiRK5QIcon", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabToolTip(int, const QString &)

/*
Sets the tab tool tip for the page at position index to tip.

See also tabToolTip().
*/
impl /*struct*/ QTabWidget {
  pub fn setTabToolTip_0<RetType, T: QTabWidget_setTabToolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabToolTip_0(self);
    // return 1;
  }
}
pub trait QTabWidget_setTabToolTip_0<RetType> {
  fn setTabToolTip_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_setTabToolTip_0<(/*void*/)> for (i32,usize) {
  fn setTabToolTip_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget13setTabToolTipEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:93
// index:0
// Public Visibility=Default Availability=Available
// [8] QString tabToolTip(int) const

/*
Returns the tab tool tip for the page at position index or an empty string if no tool tip has been set.

See also setTabToolTip().
*/
impl /*struct*/ QTabWidget {
  pub fn tabToolTip_0<RetType, T: QTabWidget_tabToolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabToolTip_0(self);
    // return 1;
  }
}
pub trait QTabWidget_tabToolTip_0<RetType> {
  fn tabToolTip_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_tabToolTip_0<usize> for (i32) {
  fn tabToolTip_0(self , rsthis: & QTabWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget10tabToolTipEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:97
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabWhatsThis(int, const QString &)

/*
Sets the What's This help text for the page at position index to text.

This function was introduced in  Qt 4.1.

See also tabWhatsThis().
*/
impl /*struct*/ QTabWidget {
  pub fn setTabWhatsThis_0<RetType, T: QTabWidget_setTabWhatsThis_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabWhatsThis_0(self);
    // return 1;
  }
}
pub trait QTabWidget_setTabWhatsThis_0<RetType> {
  fn setTabWhatsThis_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_setTabWhatsThis_0<(/*void*/)> for (i32,usize) {
  fn setTabWhatsThis_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget15setTabWhatsThisEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:98
// index:0
// Public Visibility=Default Availability=Available
// [8] QString tabWhatsThis(int) const

/*
Returns the What's This help text for the page at position index, or an empty string if no help text has been set.

This function was introduced in  Qt 4.1.

See also setTabWhatsThis().
*/
impl /*struct*/ QTabWidget {
  pub fn tabWhatsThis_0<RetType, T: QTabWidget_tabWhatsThis_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabWhatsThis_0(self);
    // return 1;
  }
}
pub trait QTabWidget_tabWhatsThis_0<RetType> {
  fn tabWhatsThis_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_tabWhatsThis_0<usize> for (i32) {
  fn tabWhatsThis_0(self , rsthis: & QTabWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget12tabWhatsThisEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:101
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentIndex() const

/*

*/
impl /*struct*/ QTabWidget {
  pub fn currentIndex_0<RetType, T: QTabWidget_currentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentIndex_0(self);
    // return 1;
  }
}
pub trait QTabWidget_currentIndex_0<RetType> {
  fn currentIndex_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_currentIndex_0<i32> for () {
  fn currentIndex_0(self , rsthis: & QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget12currentIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:102
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * currentWidget() const

/*
Returns a pointer to the page currently being displayed by the tab dialog. The tab dialog does its best to make sure that this value is never 0 (but if you try hard enough, it can be).

See also currentIndex() and setCurrentWidget().
*/
impl /*struct*/ QTabWidget {
  pub fn currentWidget_0<RetType, T: QTabWidget_currentWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentWidget_0(self);
    // return 1;
  }
}
pub trait QTabWidget_currentWidget_0<RetType> {
  fn currentWidget_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_currentWidget_0<usize> for () {
  fn currentWidget_0(self , rsthis: & QTabWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget13currentWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:103
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * widget(int) const

/*
Returns the tab page at index position index or 0 if the index is out of range.
*/
impl /*struct*/ QTabWidget {
  pub fn widget_0<RetType, T: QTabWidget_widget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widget_0(self);
    // return 1;
  }
}
pub trait QTabWidget_widget_0<RetType> {
  fn widget_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_widget_0<usize> for (i32) {
  fn widget_0(self , rsthis: & QTabWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget6widgetEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:104
// index:0
// Public Visibility=Default Availability=Available
// [4] int indexOf(QWidget *) const

/*
Returns the index position of the page occupied by the widget w, or -1 if the widget cannot be found.
*/
impl /*struct*/ QTabWidget {
  pub fn indexOf_0<RetType, T: QTabWidget_indexOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_0(self);
    // return 1;
  }
}
pub trait QTabWidget_indexOf_0<RetType> {
  fn indexOf_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_indexOf_0<i32> for (usize) {
  fn indexOf_0(self , rsthis: & QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget7indexOfEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:105
// index:0
// Public Visibility=Default Availability=Available
// [4] int count() const

/*

*/
impl /*struct*/ QTabWidget {
  pub fn count_0<RetType, T: QTabWidget_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QTabWidget_count_0<RetType> {
  fn count_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_count_0<i32> for () {
  fn count_0(self , rsthis: & QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:109
// index:0
// Public Visibility=Default Availability=Available
// [4] QTabWidget::TabPosition tabPosition() const

/*

*/
impl /*struct*/ QTabWidget {
  pub fn tabPosition_0<RetType, T: QTabWidget_tabPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabPosition_0(self);
    // return 1;
  }
}
pub trait QTabWidget_tabPosition_0<RetType> {
  fn tabPosition_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_tabPosition_0<i32> for () {
  fn tabPosition_0(self , rsthis: & QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget11tabPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabPosition(QTabWidget::TabPosition)

/*

*/
impl /*struct*/ QTabWidget {
  pub fn setTabPosition_0<RetType, T: QTabWidget_setTabPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabPosition_0(self);
    // return 1;
  }
}
pub trait QTabWidget_setTabPosition_0<RetType> {
  fn setTabPosition_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_setTabPosition_0<(/*void*/)> for (i32) {
  fn setTabPosition_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget14setTabPositionENS_11TabPositionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:112
// index:0
// Public Visibility=Default Availability=Available
// [1] bool tabsClosable() const

/*

*/
impl /*struct*/ QTabWidget {
  pub fn tabsClosable_0<RetType, T: QTabWidget_tabsClosable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabsClosable_0(self);
    // return 1;
  }
}
pub trait QTabWidget_tabsClosable_0<RetType> {
  fn tabsClosable_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_tabsClosable_0<bool> for () {
  fn tabsClosable_0(self , rsthis: & QTabWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget12tabsClosableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:113
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabsClosable(bool)

/*

*/
impl /*struct*/ QTabWidget {
  pub fn setTabsClosable_0<RetType, T: QTabWidget_setTabsClosable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabsClosable_0(self);
    // return 1;
  }
}
pub trait QTabWidget_setTabsClosable_0<RetType> {
  fn setTabsClosable_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_setTabsClosable_0<(/*void*/)> for (bool) {
  fn setTabsClosable_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget15setTabsClosableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:115
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isMovable() const

/*

*/
impl /*struct*/ QTabWidget {
  pub fn isMovable_0<RetType, T: QTabWidget_isMovable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isMovable_0(self);
    // return 1;
  }
}
pub trait QTabWidget_isMovable_0<RetType> {
  fn isMovable_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_isMovable_0<bool> for () {
  fn isMovable_0(self , rsthis: & QTabWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget9isMovableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMovable(bool)

/*

*/
impl /*struct*/ QTabWidget {
  pub fn setMovable_0<RetType, T: QTabWidget_setMovable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMovable_0(self);
    // return 1;
  }
}
pub trait QTabWidget_setMovable_0<RetType> {
  fn setMovable_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_setMovable_0<(/*void*/)> for (bool) {
  fn setMovable_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget10setMovableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:120
// index:0
// Public Visibility=Default Availability=Available
// [4] QTabWidget::TabShape tabShape() const

/*

*/
impl /*struct*/ QTabWidget {
  pub fn tabShape_0<RetType, T: QTabWidget_tabShape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabShape_0(self);
    // return 1;
  }
}
pub trait QTabWidget_tabShape_0<RetType> {
  fn tabShape_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_tabShape_0<i32> for () {
  fn tabShape_0(self , rsthis: & QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget8tabShapeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:121
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabShape(QTabWidget::TabShape)

/*

*/
impl /*struct*/ QTabWidget {
  pub fn setTabShape_0<RetType, T: QTabWidget_setTabShape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabShape_0(self);
    // return 1;
  }
}
pub trait QTabWidget_setTabShape_0<RetType> {
  fn setTabShape_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_setTabShape_0<(/*void*/)> for (i32) {
  fn setTabShape_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget11setTabShapeENS_8TabShapeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:123
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QTabWidget {
  pub fn sizeHint_0<RetType, T: QTabWidget_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QTabWidget_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QTabWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:124
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().

Returns a suitable minimum size for the tab widget.
*/
impl /*struct*/ QTabWidget {
  pub fn minimumSizeHint_0<RetType, T: QTabWidget_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QTabWidget_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QTabWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:125
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int heightForWidth(int) const

/*
Reimplemented from QWidget::heightForWidth().
*/
impl /*struct*/ QTabWidget {
  pub fn heightForWidth_0<RetType, T: QTabWidget_heightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth_0(self);
    // return 1;
  }
}
pub trait QTabWidget_heightForWidth_0<RetType> {
  fn heightForWidth_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_heightForWidth_0<i32> for (i32) {
  fn heightForWidth_0(self , rsthis: & QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget14heightForWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:126
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool hasHeightForWidth() const

/*
Reimplemented from QWidget::hasHeightForWidth().
*/
impl /*struct*/ QTabWidget {
  pub fn hasHeightForWidth_0<RetType, T: QTabWidget_hasHeightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth_0(self);
    // return 1;
  }
}
pub trait QTabWidget_hasHeightForWidth_0<RetType> {
  fn hasHeightForWidth_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_hasHeightForWidth_0<bool> for () {
  fn hasHeightForWidth_0(self , rsthis: & QTabWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget17hasHeightForWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:128
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCornerWidget(QWidget *, Qt::Corner)

/*
Sets the given widget to be shown in the specified corner of the tab widget. The geometry of the widget is determined based on the widget's sizeHint() and the style().

Only the horizontal element of the corner will be used.

Passing 0 shows no widget in the corner.

Any previously set corner widget is hidden.

All widgets set here will be deleted by the tab widget when it is destroyed unless you separately reparent the widget after setting some other corner widget (or 0).

Note: Corner widgets are designed for North and South tab positions; other orientations are known to not work properly.

See also cornerWidget() and setTabPosition().
*/
impl /*struct*/ QTabWidget {
  pub fn setCornerWidget_0<RetType, T: QTabWidget_setCornerWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCornerWidget_0(self);
    // return 1;
  }
}
pub trait QTabWidget_setCornerWidget_0<RetType> {
  fn setCornerWidget_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_setCornerWidget_0<(/*void*/)> for (usize,i32) {
  fn setCornerWidget_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget15setCornerWidgetEP7QWidgetN2Qt6CornerE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:129
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * cornerWidget(Qt::Corner) const

/*
Returns the widget shown in the corner of the tab widget or 0.

See also setCornerWidget().
*/
impl /*struct*/ QTabWidget {
  pub fn cornerWidget_0<RetType, T: QTabWidget_cornerWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cornerWidget_0(self);
    // return 1;
  }
}
pub trait QTabWidget_cornerWidget_0<RetType> {
  fn cornerWidget_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_cornerWidget_0<usize> for (i32) {
  fn cornerWidget_0(self , rsthis: & QTabWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget12cornerWidgetEN2Qt6CornerE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:131
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::TextElideMode elideMode() const

/*

*/
impl /*struct*/ QTabWidget {
  pub fn elideMode_0<RetType, T: QTabWidget_elideMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.elideMode_0(self);
    // return 1;
  }
}
pub trait QTabWidget_elideMode_0<RetType> {
  fn elideMode_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_elideMode_0<i32> for () {
  fn elideMode_0(self , rsthis: & QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget9elideModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:132
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setElideMode(Qt::TextElideMode)

/*

*/
impl /*struct*/ QTabWidget {
  pub fn setElideMode_0<RetType, T: QTabWidget_setElideMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setElideMode_0(self);
    // return 1;
  }
}
pub trait QTabWidget_setElideMode_0<RetType> {
  fn setElideMode_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_setElideMode_0<(/*void*/)> for (i32) {
  fn setElideMode_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget12setElideModeEN2Qt13TextElideModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:134
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize iconSize() const

/*

*/
impl /*struct*/ QTabWidget {
  pub fn iconSize_0<RetType, T: QTabWidget_iconSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.iconSize_0(self);
    // return 1;
  }
}
pub trait QTabWidget_iconSize_0<RetType> {
  fn iconSize_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_iconSize_0<usize> for () {
  fn iconSize_0(self , rsthis: & QTabWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget8iconSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:135
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIconSize(const QSize &)

/*

*/
impl /*struct*/ QTabWidget {
  pub fn setIconSize_0<RetType, T: QTabWidget_setIconSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIconSize_0(self);
    // return 1;
  }
}
pub trait QTabWidget_setIconSize_0<RetType> {
  fn setIconSize_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_setIconSize_0<(/*void*/)> for (usize) {
  fn setIconSize_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget11setIconSizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:137
// index:0
// Public Visibility=Default Availability=Available
// [1] bool usesScrollButtons() const

/*

*/
impl /*struct*/ QTabWidget {
  pub fn usesScrollButtons_0<RetType, T: QTabWidget_usesScrollButtons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.usesScrollButtons_0(self);
    // return 1;
  }
}
pub trait QTabWidget_usesScrollButtons_0<RetType> {
  fn usesScrollButtons_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_usesScrollButtons_0<bool> for () {
  fn usesScrollButtons_0(self , rsthis: & QTabWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget17usesScrollButtonsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:138
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUsesScrollButtons(bool)

/*

*/
impl /*struct*/ QTabWidget {
  pub fn setUsesScrollButtons_0<RetType, T: QTabWidget_setUsesScrollButtons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUsesScrollButtons_0(self);
    // return 1;
  }
}
pub trait QTabWidget_setUsesScrollButtons_0<RetType> {
  fn setUsesScrollButtons_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_setUsesScrollButtons_0<(/*void*/)> for (bool) {
  fn setUsesScrollButtons_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget20setUsesScrollButtonsEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:140
// index:0
// Public Visibility=Default Availability=Available
// [1] bool documentMode() const

/*

*/
impl /*struct*/ QTabWidget {
  pub fn documentMode_0<RetType, T: QTabWidget_documentMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.documentMode_0(self);
    // return 1;
  }
}
pub trait QTabWidget_documentMode_0<RetType> {
  fn documentMode_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_documentMode_0<bool> for () {
  fn documentMode_0(self , rsthis: & QTabWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget12documentModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:141
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDocumentMode(bool)

/*

*/
impl /*struct*/ QTabWidget {
  pub fn setDocumentMode_0<RetType, T: QTabWidget_setDocumentMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDocumentMode_0(self);
    // return 1;
  }
}
pub trait QTabWidget_setDocumentMode_0<RetType> {
  fn setDocumentMode_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_setDocumentMode_0<(/*void*/)> for (bool) {
  fn setDocumentMode_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget15setDocumentModeEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:143
// index:0
// Public Visibility=Default Availability=Available
// [1] bool tabBarAutoHide() const

/*

*/
impl /*struct*/ QTabWidget {
  pub fn tabBarAutoHide_0<RetType, T: QTabWidget_tabBarAutoHide_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabBarAutoHide_0(self);
    // return 1;
  }
}
pub trait QTabWidget_tabBarAutoHide_0<RetType> {
  fn tabBarAutoHide_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_tabBarAutoHide_0<bool> for () {
  fn tabBarAutoHide_0(self , rsthis: & QTabWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget14tabBarAutoHideEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:144
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabBarAutoHide(bool)

/*

*/
impl /*struct*/ QTabWidget {
  pub fn setTabBarAutoHide_0<RetType, T: QTabWidget_setTabBarAutoHide_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabBarAutoHide_0(self);
    // return 1;
  }
}
pub trait QTabWidget_setTabBarAutoHide_0<RetType> {
  fn setTabBarAutoHide_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_setTabBarAutoHide_0<(/*void*/)> for (bool) {
  fn setTabBarAutoHide_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget17setTabBarAutoHideEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:146
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Removes all the pages, but does not delete them. Calling this function is equivalent to calling removeTab() until the tab widget is empty.
*/
impl /*struct*/ QTabWidget {
  pub fn clear_0<RetType, T: QTabWidget_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QTabWidget_clear_0<RetType> {
  fn clear_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QTabWidget5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:148
// index:0
// Public Visibility=Default Availability=Available
// [8] QTabBar * tabBar() const

/*
Returns the current QTabBar.

See also setTabBar().
*/
impl /*struct*/ QTabWidget {
  pub fn tabBar_0<RetType, T: QTabWidget_tabBar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabBar_0(self);
    // return 1;
  }
}
pub trait QTabWidget_tabBar_0<RetType> {
  fn tabBar_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_tabBar_0<usize> for () {
  fn tabBar_0(self , rsthis: & QTabWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTabWidget6tabBarEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:151
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentIndex(int)

/*

*/
impl /*struct*/ QTabWidget {
  pub fn setCurrentIndex_0<RetType, T: QTabWidget_setCurrentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex_0(self);
    // return 1;
  }
}
pub trait QTabWidget_setCurrentIndex_0<RetType> {
  fn setCurrentIndex_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_setCurrentIndex_0<(/*void*/)> for (i32) {
  fn setCurrentIndex_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget15setCurrentIndexEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:152
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentWidget(QWidget *)

/*
Makes widget the current widget. The widget used must be a page in this tab widget.

See also addTab(), setCurrentIndex(), and currentWidget().
*/
impl /*struct*/ QTabWidget {
  pub fn setCurrentWidget_0<RetType, T: QTabWidget_setCurrentWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentWidget_0(self);
    // return 1;
  }
}
pub trait QTabWidget_setCurrentWidget_0<RetType> {
  fn setCurrentWidget_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_setCurrentWidget_0<(/*void*/)> for (usize) {
  fn setCurrentWidget_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget16setCurrentWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:155
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentChanged(int)

/*
This signal is emitted whenever the current page index changes. The parameter is the new current page index position, or -1 if there isn't a new one (for example, if there are no widgets in the QTabWidget)

Note: Notifier signal for property currentIndex. 

See also currentWidget() and currentIndex.
*/
impl /*struct*/ QTabWidget {
  pub fn currentChanged_0<RetType, T: QTabWidget_currentChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentChanged_0(self);
    // return 1;
  }
}
pub trait QTabWidget_currentChanged_0<RetType> {
  fn currentChanged_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_currentChanged_0<(/*void*/)> for (i32) {
  fn currentChanged_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget14currentChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:156
// index:0
// Public Visibility=Default Availability=Available
// [-2] void tabCloseRequested(int)

/*
This signal is emitted when the close button on a tab is clicked. The index is the index that should be removed.

This function was introduced in  Qt 4.5.

See also setTabsClosable().
*/
impl /*struct*/ QTabWidget {
  pub fn tabCloseRequested_0<RetType, T: QTabWidget_tabCloseRequested_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabCloseRequested_0(self);
    // return 1;
  }
}
pub trait QTabWidget_tabCloseRequested_0<RetType> {
  fn tabCloseRequested_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_tabCloseRequested_0<(/*void*/)> for (i32) {
  fn tabCloseRequested_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget17tabCloseRequestedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:157
// index:0
// Public Visibility=Default Availability=Available
// [-2] void tabBarClicked(int)

/*
This signal is emitted when user clicks on a tab at an index.

index refers to the tab clicked, or -1 if no tab is under the cursor.

This function was introduced in  Qt 5.2.
*/
impl /*struct*/ QTabWidget {
  pub fn tabBarClicked_0<RetType, T: QTabWidget_tabBarClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabBarClicked_0(self);
    // return 1;
  }
}
pub trait QTabWidget_tabBarClicked_0<RetType> {
  fn tabBarClicked_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_tabBarClicked_0<(/*void*/)> for (i32) {
  fn tabBarClicked_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget13tabBarClickedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:158
// index:0
// Public Visibility=Default Availability=Available
// [-2] void tabBarDoubleClicked(int)

/*
This signal is emitted when the user double clicks on a tab at an index.

index is the index of a clicked tab, or -1 if no tab is under the cursor.

This function was introduced in  Qt 5.2.
*/
impl /*struct*/ QTabWidget {
  pub fn tabBarDoubleClicked_0<RetType, T: QTabWidget_tabBarDoubleClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabBarDoubleClicked_0(self);
    // return 1;
  }
}
pub trait QTabWidget_tabBarDoubleClicked_0<RetType> {
  fn tabBarDoubleClicked_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_tabBarDoubleClicked_0<(/*void*/)> for (i32) {
  fn tabBarDoubleClicked_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget19tabBarDoubleClickedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:161
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void tabInserted(int)

/*
This virtual handler is called after a new tab was added or inserted at position index.

See also tabRemoved().
*/
impl /*struct*/ QTabWidget {
  pub fn tabInserted_0<RetType, T: QTabWidget_tabInserted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabInserted_0(self);
    // return 1;
  }
}
pub trait QTabWidget_tabInserted_0<RetType> {
  fn tabInserted_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_tabInserted_0<(/*void*/)> for (i32) {
  fn tabInserted_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget11tabInsertedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:162
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void tabRemoved(int)

/*
This virtual handler is called after a tab was removed from position index.

See also tabInserted().
*/
impl /*struct*/ QTabWidget {
  pub fn tabRemoved_0<RetType, T: QTabWidget_tabRemoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabRemoved_0(self);
    // return 1;
  }
}
pub trait QTabWidget_tabRemoved_0<RetType> {
  fn tabRemoved_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_tabRemoved_0<(/*void*/)> for (i32) {
  fn tabRemoved_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget10tabRemovedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:164
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void showEvent(QShowEvent *)

/*
Reimplemented from QWidget::showEvent().
*/
impl /*struct*/ QTabWidget {
  pub fn showEvent_0<RetType, T: QTabWidget_showEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showEvent_0(self);
    // return 1;
  }
}
pub trait QTabWidget_showEvent_0<RetType> {
  fn showEvent_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_showEvent_0<(/*void*/)> for (usize) {
  fn showEvent_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget9showEventEP10QShowEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:165
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QTabWidget {
  pub fn resizeEvent_0<RetType, T: QTabWidget_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QTabWidget_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:166
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().
*/
impl /*struct*/ QTabWidget {
  pub fn keyPressEvent_0<RetType, T: QTabWidget_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QTabWidget_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:167
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().

Paints the tab widget's tab bar in response to the paint event.
*/
impl /*struct*/ QTabWidget {
  pub fn paintEvent_0<RetType, T: QTabWidget_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QTabWidget_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:168
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void setTabBar(QTabBar *)

/*
Replaces the dialog's QTabBar heading with the tab bar tb. Note that this must be called before any tabs have been added, or the behavior is undefined.

See also tabBar().
*/
impl /*struct*/ QTabWidget {
  pub fn setTabBar_0<RetType, T: QTabWidget_setTabBar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabBar_0(self);
    // return 1;
  }
}
pub trait QTabWidget_setTabBar_0<RetType> {
  fn setTabBar_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_setTabBar_0<(/*void*/)> for (usize) {
  fn setTabBar_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget9setTabBarEP7QTabBar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:169
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QTabWidget {
  pub fn changeEvent_0<RetType, T: QTabWidget_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QTabWidget_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTabWidget11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:170
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QTabWidget {
  pub fn event_0<RetType, T: QTabWidget_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QTabWidget_event_0<RetType> {
  fn event_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QTabWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTabWidget5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabwidget.h:171
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionTabWidgetFrame *) const

/*
Initialize option with the values from this QTabWidget. This method is useful for subclasses when they need a QStyleOptionTabWidgetFrame, but don't want to fill in all the information themselves.

See also QStyleOption::initFrom() and QTabBar::initStyleOption().
*/
impl /*struct*/ QTabWidget {
  pub fn initStyleOption_0<RetType, T: QTabWidget_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QTabWidget_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QTabWidget) -> RetType;
}
impl<'a> /*trait*/ QTabWidget_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QTabWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK10QTabWidget15initStyleOptionEP26QStyleOptionTabWidgetFrame", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum type defines where QTabWidget draws the tab row:


*/
pub type QTabWidget__TabPosition = i32;
// The tabs are drawn above the pages.
pub const QTabWidget__North :QTabWidget__TabPosition = 0;
// The tabs are drawn below the pages.
pub const QTabWidget__South :QTabWidget__TabPosition = 1;
// The tabs are drawn to the left of the pages.
pub const QTabWidget__West :QTabWidget__TabPosition = 2;
// The tabs are drawn to the right of the pages.
pub const QTabWidget__East :QTabWidget__TabPosition = 3;
pub fn QTabWidget_TabPositionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QTabWidget", val);
}
pub fn QTabWidget_TabPositionItemName_s(val: i32) ->String {
  //var nilthis *QTabWidget
  //return nilthis.TabPositionItemName(val);
  return QTabWidget_TabPositionItemName(val);
}


/*
This enum type defines the shape of the tabs:


*/
pub type QTabWidget__TabShape = i32;
// The tabs are drawn with a rounded look. This is the default shape.
pub const QTabWidget__Rounded :QTabWidget__TabShape = 0;
// The tabs are drawn with a triangular look.
pub const QTabWidget__Triangular :QTabWidget__TabShape = 1;
pub fn QTabWidget_TabShapeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QTabWidget", val);
}
pub fn QTabWidget_TabShapeItemName_s(val: i32) ->String {
  //var nilthis *QTabWidget
  //return nilthis.TabShapeItemName(val);
  return QTabWidget_TabShapeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
