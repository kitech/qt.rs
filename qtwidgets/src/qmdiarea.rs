

// mod ::widgets::QMdiArea
// package qtwidgets
// /usr/include/qt/QtWidgets/qmdiarea.h
// #include <qmdiarea.h>
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

// void setupViewport(QWidget *)
// func (this *QMdiArea) InheritSetupViewport(f func(viewport *QWidget/*777 QWidget **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setupViewport", f)
// }

// bool event(QEvent *)
// func (this *QMdiArea) InheritEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// bool eventFilter(QObject *, QEvent *)
// func (this *QMdiArea) InheritEventFilter(f func(object *qtcore.QObject/*777 QObject **/, event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventFilter", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QMdiArea) InheritPaintEvent(f func(paintEvent *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void childEvent(QChildEvent *)
// func (this *QMdiArea) InheritChildEvent(f func(childEvent *qtcore.QChildEvent/*777 QChildEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "childEvent", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QMdiArea) InheritResizeEvent(f func(resizeEvent *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void timerEvent(QTimerEvent *)
// func (this *QMdiArea) InheritTimerEvent(f func(timerEvent *qtcore.QTimerEvent/*777 QTimerEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "timerEvent", f)
// }

// void showEvent(QShowEvent *)
// func (this *QMdiArea) InheritShowEvent(f func(showEvent *qtgui.QShowEvent/*777 QShowEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "showEvent", f)
// }

// bool viewportEvent(QEvent *)
// func (this *QMdiArea) InheritViewportEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "viewportEvent", f)
// }

// void scrollContentsBy(int, int)
// func (this *QMdiArea) InheritScrollContentsBy(f func(dx int, dy int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "scrollContentsBy", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QMdiArea)=48
pub struct QMdiArea {
  qbase: QAbstractScrollArea,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMdiArea_ITF interface {
//    QAbstractScrollArea_ITF
//    QMdiArea_PTR() *QMdiArea
//}
//func (ptr *QMdiArea) QMdiArea_PTR() *QMdiArea { return ptr }

impl /*struct*/ QMdiArea {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMdiArea {
    return QMdiArea{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMdiArea {
//  type Target = QMdiAreaBASE;
//
//  fn deref(&self) -> &QMdiAreaBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMdiAreaBASE> for QMdiArea {
//  fn as_ref(& self) -> & QMdiAreaBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qmdiarea.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QMdiArea {
  pub fn metaObject_0<RetType, T: QMdiArea_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QMdiArea_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QMdiArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMdiArea10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QMdiArea(QWidget *)

/*
Constructs an empty mdi area. parent is passed to QWidget's constructor.
*/
// QMdiArea(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QMdiArea {
  pub fn QMdiArea_0<T: QMdiArea_QMdiArea_0>(value: T) -> QMdiArea {
    let rsthis = value.QMdiArea_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMdiArea_QMdiArea_0 {
  fn QMdiArea_0(self) -> QMdiArea;
}
// QMdiArea(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMdiArea_QMdiArea_0 for (usize) {
  fn QMdiArea_0(self) -> QMdiArea {
    // unsafe{_ZN8QMdiAreaC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QMdiAreaC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMdiArea{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:91
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QMdiArea()

/*

*/
pub fn DeleteQMdiArea(this :*mut QMdiArea) {
    // let rv = qtrt::InvokeQtFunc6("_ZN8QMdiAreaD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qmdiarea.h:93
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QMdiArea {
  pub fn sizeHint_0<RetType, T: QMdiArea_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QMdiArea_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QMdiArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMdiArea8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:94
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QMdiArea {
  pub fn minimumSizeHint_0<RetType, T: QMdiArea_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QMdiArea_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QMdiArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMdiArea15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:96
// index:0
// Public Visibility=Default Availability=Available
// [8] QMdiSubWindow * currentSubWindow() const

/*
Returns a pointer to the current subwindow, or 0 if there is no current subwindow.

This function will return the same as activeSubWindow() if the QApplication containing QMdiArea is active.

See also activeSubWindow() and QApplication::activeWindow().
*/
impl /*struct*/ QMdiArea {
  pub fn currentSubWindow_0<RetType, T: QMdiArea_currentSubWindow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentSubWindow_0(self);
    // return 1;
  }
}
pub trait QMdiArea_currentSubWindow_0<RetType> {
  fn currentSubWindow_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_currentSubWindow_0<usize> for () {
  fn currentSubWindow_0(self , rsthis: & QMdiArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMdiArea16currentSubWindowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:97
// index:0
// Public Visibility=Default Availability=Available
// [8] QMdiSubWindow * activeSubWindow() const

/*
Returns a pointer to the current active subwindow. If no window is currently active, 0 is returned.

Subwindows are treated as top-level windows with respect to window state, i.e., if a widget outside the MDI area is the active window, no subwindow will be active. Note that if a widget in the window in which the MDI area lives gains focus, the window will be activated.

See also setActiveSubWindow() and Qt::WindowState.
*/
impl /*struct*/ QMdiArea {
  pub fn activeSubWindow_0<RetType, T: QMdiArea_activeSubWindow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activeSubWindow_0(self);
    // return 1;
  }
}
pub trait QMdiArea_activeSubWindow_0<RetType> {
  fn activeSubWindow_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_activeSubWindow_0<usize> for () {
  fn activeSubWindow_0(self , rsthis: & QMdiArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMdiArea15activeSubWindowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:100
// index:0
// Public Visibility=Default Availability=Available
// [8] QMdiSubWindow * addSubWindow(QWidget *, Qt::WindowFlags)

/*
Adds widget as a new subwindow to the MDI area. If windowFlags are non-zero, they will override the flags set on the widget.

The widget can be either a QMdiSubWindow or another QWidget (in which case the MDI area will create a subwindow and set the widget as the internal widget).

Note: Once the subwindow has been added, its parent will be the viewport widget of the QMdiArea.


      QMdiArea mdiArea;
      QMdiSubWindow *subWindow1 = new QMdiSubWindow;
      subWindow1->setWidget(internalWidget1);
      subWindow1->setAttribute(Qt::WA_DeleteOnClose);
      mdiArea.addSubWindow(subWindow1);

      QMdiSubWindow *subWindow2 =
          mdiArea.addSubWindow(internalWidget2);



When you create your own subwindow, you must set the Qt::WA_DeleteOnClose widget attribute if you want the window to be deleted when closed in the MDI area. If not, the window will be hidden and the MDI area will not activate the next subwindow.

Returns the QMdiSubWindow that is added to the MDI area.

See also removeSubWindow().
*/
impl /*struct*/ QMdiArea {
  pub fn addSubWindow_0<RetType, T: QMdiArea_addSubWindow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addSubWindow_0(self);
    // return 1;
  }
}
pub trait QMdiArea_addSubWindow_0<RetType> {
  fn addSubWindow_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_addSubWindow_0<usize> for (usize,i32) {
  fn addSubWindow_0(self , rsthis: & QMdiArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMdiArea12addSubWindowEP7QWidget6QFlagsIN2Qt10WindowTypeEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeSubWindow(QWidget *)

/*
Removes widget from the MDI area. The widget must be either a QMdiSubWindow or a widget that is the internal widget of a subwindow. Note widget is never actually deleted by QMdiArea. If a QMdiSubWindow is passed in its parent is set to 0 and it is removed, but if an internal widget is passed in the child widget is set to 0 but the QMdiSubWindow is not removed.

See also addSubWindow().
*/
impl /*struct*/ QMdiArea {
  pub fn removeSubWindow_0<RetType, T: QMdiArea_removeSubWindow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeSubWindow_0(self);
    // return 1;
  }
}
pub trait QMdiArea_removeSubWindow_0<RetType> {
  fn removeSubWindow_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_removeSubWindow_0<(/*void*/)> for (usize) {
  fn removeSubWindow_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMdiArea15removeSubWindowEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:103
// index:0
// Public Visibility=Default Availability=Available
// [8] QBrush background() const

/*

*/
impl /*struct*/ QMdiArea {
  pub fn background_0<RetType, T: QMdiArea_background_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.background_0(self);
    // return 1;
  }
}
pub trait QMdiArea_background_0<RetType> {
  fn background_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_background_0<usize> for () {
  fn background_0(self , rsthis: & QMdiArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMdiArea10backgroundEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBackground(const QBrush &)

/*

*/
impl /*struct*/ QMdiArea {
  pub fn setBackground_0<RetType, T: QMdiArea_setBackground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBackground_0(self);
    // return 1;
  }
}
pub trait QMdiArea_setBackground_0<RetType> {
  fn setBackground_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_setBackground_0<(/*void*/)> for (usize) {
  fn setBackground_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMdiArea13setBackgroundERK6QBrush", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:106
// index:0
// Public Visibility=Default Availability=Available
// [4] QMdiArea::WindowOrder activationOrder() const

/*

*/
impl /*struct*/ QMdiArea {
  pub fn activationOrder_0<RetType, T: QMdiArea_activationOrder_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activationOrder_0(self);
    // return 1;
  }
}
pub trait QMdiArea_activationOrder_0<RetType> {
  fn activationOrder_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_activationOrder_0<i32> for () {
  fn activationOrder_0(self , rsthis: & QMdiArea) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMdiArea15activationOrderEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setActivationOrder(QMdiArea::WindowOrder)

/*

*/
impl /*struct*/ QMdiArea {
  pub fn setActivationOrder_0<RetType, T: QMdiArea_setActivationOrder_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setActivationOrder_0(self);
    // return 1;
  }
}
pub trait QMdiArea_setActivationOrder_0<RetType> {
  fn setActivationOrder_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_setActivationOrder_0<(/*void*/)> for (i32) {
  fn setActivationOrder_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QMdiArea18setActivationOrderENS_11WindowOrderE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOption(QMdiArea::AreaOption, bool)

/*
If on is true, option is enabled on the MDI area; otherwise it is disabled. See AreaOption for the effect of each option.

See also AreaOption and testOption().
*/
impl /*struct*/ QMdiArea {
  pub fn setOption_0<RetType, T: QMdiArea_setOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOption_0(self);
    // return 1;
  }
}
pub trait QMdiArea_setOption_0<RetType> {
  fn setOption_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_setOption_0<(/*void*/)> for (i32,bool) {
  fn setOption_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QMdiArea9setOptionENS_10AreaOptionEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:110
// index:0
// Public Visibility=Default Availability=Available
// [1] bool testOption(QMdiArea::AreaOption) const

/*
Returns true if option is enabled; otherwise returns false.

See also AreaOption and setOption().
*/
impl /*struct*/ QMdiArea {
  pub fn testOption_0<RetType, T: QMdiArea_testOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.testOption_0(self);
    // return 1;
  }
}
pub trait QMdiArea_testOption_0<RetType> {
  fn testOption_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_testOption_0<bool> for (i32) {
  fn testOption_0(self , rsthis: & QMdiArea) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMdiArea10testOptionENS_10AreaOptionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:112
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setViewMode(QMdiArea::ViewMode)

/*

*/
impl /*struct*/ QMdiArea {
  pub fn setViewMode_0<RetType, T: QMdiArea_setViewMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setViewMode_0(self);
    // return 1;
  }
}
pub trait QMdiArea_setViewMode_0<RetType> {
  fn setViewMode_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_setViewMode_0<(/*void*/)> for (i32) {
  fn setViewMode_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QMdiArea11setViewModeENS_8ViewModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:113
// index:0
// Public Visibility=Default Availability=Available
// [4] QMdiArea::ViewMode viewMode() const

/*

*/
impl /*struct*/ QMdiArea {
  pub fn viewMode_0<RetType, T: QMdiArea_viewMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewMode_0(self);
    // return 1;
  }
}
pub trait QMdiArea_viewMode_0<RetType> {
  fn viewMode_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_viewMode_0<i32> for () {
  fn viewMode_0(self , rsthis: & QMdiArea) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMdiArea8viewModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:116
// index:0
// Public Visibility=Default Availability=Available
// [1] bool documentMode() const

/*

*/
impl /*struct*/ QMdiArea {
  pub fn documentMode_0<RetType, T: QMdiArea_documentMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.documentMode_0(self);
    // return 1;
  }
}
pub trait QMdiArea_documentMode_0<RetType> {
  fn documentMode_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_documentMode_0<bool> for () {
  fn documentMode_0(self , rsthis: & QMdiArea) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMdiArea12documentModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDocumentMode(bool)

/*

*/
impl /*struct*/ QMdiArea {
  pub fn setDocumentMode_0<RetType, T: QMdiArea_setDocumentMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDocumentMode_0(self);
    // return 1;
  }
}
pub trait QMdiArea_setDocumentMode_0<RetType> {
  fn setDocumentMode_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_setDocumentMode_0<(/*void*/)> for (bool) {
  fn setDocumentMode_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QMdiArea15setDocumentModeEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:119
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabsClosable(bool)

/*

*/
impl /*struct*/ QMdiArea {
  pub fn setTabsClosable_0<RetType, T: QMdiArea_setTabsClosable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabsClosable_0(self);
    // return 1;
  }
}
pub trait QMdiArea_setTabsClosable_0<RetType> {
  fn setTabsClosable_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_setTabsClosable_0<(/*void*/)> for (bool) {
  fn setTabsClosable_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QMdiArea15setTabsClosableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:120
// index:0
// Public Visibility=Default Availability=Available
// [1] bool tabsClosable() const

/*

*/
impl /*struct*/ QMdiArea {
  pub fn tabsClosable_0<RetType, T: QMdiArea_tabsClosable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabsClosable_0(self);
    // return 1;
  }
}
pub trait QMdiArea_tabsClosable_0<RetType> {
  fn tabsClosable_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_tabsClosable_0<bool> for () {
  fn tabsClosable_0(self , rsthis: & QMdiArea) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMdiArea12tabsClosableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:122
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabsMovable(bool)

/*

*/
impl /*struct*/ QMdiArea {
  pub fn setTabsMovable_0<RetType, T: QMdiArea_setTabsMovable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabsMovable_0(self);
    // return 1;
  }
}
pub trait QMdiArea_setTabsMovable_0<RetType> {
  fn setTabsMovable_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_setTabsMovable_0<(/*void*/)> for (bool) {
  fn setTabsMovable_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QMdiArea14setTabsMovableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:123
// index:0
// Public Visibility=Default Availability=Available
// [1] bool tabsMovable() const

/*

*/
impl /*struct*/ QMdiArea {
  pub fn tabsMovable_0<RetType, T: QMdiArea_tabsMovable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabsMovable_0(self);
    // return 1;
  }
}
pub trait QMdiArea_tabsMovable_0<RetType> {
  fn tabsMovable_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_tabsMovable_0<bool> for () {
  fn tabsMovable_0(self , rsthis: & QMdiArea) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMdiArea11tabsMovableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabShape(QTabWidget::TabShape)

/*

*/
impl /*struct*/ QMdiArea {
  pub fn setTabShape_0<RetType, T: QMdiArea_setTabShape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabShape_0(self);
    // return 1;
  }
}
pub trait QMdiArea_setTabShape_0<RetType> {
  fn setTabShape_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_setTabShape_0<(/*void*/)> for (i32) {
  fn setTabShape_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QMdiArea11setTabShapeEN10QTabWidget8TabShapeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:127
// index:0
// Public Visibility=Default Availability=Available
// [4] QTabWidget::TabShape tabShape() const

/*

*/
impl /*struct*/ QMdiArea {
  pub fn tabShape_0<RetType, T: QMdiArea_tabShape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabShape_0(self);
    // return 1;
  }
}
pub trait QMdiArea_tabShape_0<RetType> {
  fn tabShape_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_tabShape_0<i32> for () {
  fn tabShape_0(self , rsthis: & QMdiArea) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMdiArea8tabShapeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:129
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabPosition(QTabWidget::TabPosition)

/*

*/
impl /*struct*/ QMdiArea {
  pub fn setTabPosition_0<RetType, T: QMdiArea_setTabPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabPosition_0(self);
    // return 1;
  }
}
pub trait QMdiArea_setTabPosition_0<RetType> {
  fn setTabPosition_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_setTabPosition_0<(/*void*/)> for (i32) {
  fn setTabPosition_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QMdiArea14setTabPositionEN10QTabWidget11TabPositionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:130
// index:0
// Public Visibility=Default Availability=Available
// [4] QTabWidget::TabPosition tabPosition() const

/*

*/
impl /*struct*/ QMdiArea {
  pub fn tabPosition_0<RetType, T: QMdiArea_tabPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabPosition_0(self);
    // return 1;
  }
}
pub trait QMdiArea_tabPosition_0<RetType> {
  fn tabPosition_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_tabPosition_0<i32> for () {
  fn tabPosition_0(self , rsthis: & QMdiArea) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMdiArea11tabPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:134
// index:0
// Public Visibility=Default Availability=Available
// [-2] void subWindowActivated(QMdiSubWindow *)

/*
QMdiArea emits this signal after window has been activated. When window is 0, QMdiArea has just deactivated its last active window, and there are no active windows on the workspace.

See also QMdiArea::activeSubWindow().
*/
impl /*struct*/ QMdiArea {
  pub fn subWindowActivated_0<RetType, T: QMdiArea_subWindowActivated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.subWindowActivated_0(self);
    // return 1;
  }
}
pub trait QMdiArea_subWindowActivated_0<RetType> {
  fn subWindowActivated_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_subWindowActivated_0<(/*void*/)> for (usize) {
  fn subWindowActivated_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMdiArea18subWindowActivatedEP13QMdiSubWindow", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:137
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setActiveSubWindow(QMdiSubWindow *)

/*
Activates the subwindow window. If window is 0, any current active window is deactivated.

See also activeSubWindow().
*/
impl /*struct*/ QMdiArea {
  pub fn setActiveSubWindow_0<RetType, T: QMdiArea_setActiveSubWindow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setActiveSubWindow_0(self);
    // return 1;
  }
}
pub trait QMdiArea_setActiveSubWindow_0<RetType> {
  fn setActiveSubWindow_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_setActiveSubWindow_0<(/*void*/)> for (usize) {
  fn setActiveSubWindow_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMdiArea18setActiveSubWindowEP13QMdiSubWindow", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:138
// index:0
// Public Visibility=Default Availability=Available
// [-2] void tileSubWindows()

/*
Arranges all child windows in a tile pattern.

See also cascadeSubWindows().
*/
impl /*struct*/ QMdiArea {
  pub fn tileSubWindows_0<RetType, T: QMdiArea_tileSubWindows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tileSubWindows_0(self);
    // return 1;
  }
}
pub trait QMdiArea_tileSubWindows_0<RetType> {
  fn tileSubWindows_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_tileSubWindows_0<(/*void*/)> for () {
  fn tileSubWindows_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QMdiArea14tileSubWindowsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:139
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cascadeSubWindows()

/*
Arranges all the child windows in a cascade pattern.

See also tileSubWindows().
*/
impl /*struct*/ QMdiArea {
  pub fn cascadeSubWindows_0<RetType, T: QMdiArea_cascadeSubWindows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cascadeSubWindows_0(self);
    // return 1;
  }
}
pub trait QMdiArea_cascadeSubWindows_0<RetType> {
  fn cascadeSubWindows_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_cascadeSubWindows_0<(/*void*/)> for () {
  fn cascadeSubWindows_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QMdiArea17cascadeSubWindowsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:140
// index:0
// Public Visibility=Default Availability=Available
// [-2] void closeActiveSubWindow()

/*
Closes the active subwindow.

See also closeAllSubWindows().
*/
impl /*struct*/ QMdiArea {
  pub fn closeActiveSubWindow_0<RetType, T: QMdiArea_closeActiveSubWindow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closeActiveSubWindow_0(self);
    // return 1;
  }
}
pub trait QMdiArea_closeActiveSubWindow_0<RetType> {
  fn closeActiveSubWindow_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_closeActiveSubWindow_0<(/*void*/)> for () {
  fn closeActiveSubWindow_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QMdiArea20closeActiveSubWindowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:141
// index:0
// Public Visibility=Default Availability=Available
// [-2] void closeAllSubWindows()

/*
Closes all subwindows by sending a QCloseEvent to each window. You may receive subWindowActivated() signals from subwindows before they are closed (if the MDI area activates the subwindow when another is closing).

Subwindows that ignore the close event will remain open.

See also closeActiveSubWindow().
*/
impl /*struct*/ QMdiArea {
  pub fn closeAllSubWindows_0<RetType, T: QMdiArea_closeAllSubWindows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closeAllSubWindows_0(self);
    // return 1;
  }
}
pub trait QMdiArea_closeAllSubWindows_0<RetType> {
  fn closeAllSubWindows_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_closeAllSubWindows_0<(/*void*/)> for () {
  fn closeAllSubWindows_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QMdiArea18closeAllSubWindowsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:142
// index:0
// Public Visibility=Default Availability=Available
// [-2] void activateNextSubWindow()

/*
Gives the keyboard focus to another window in the list of child windows. The window activated will be the next one determined by the current activation order.

See also activatePreviousSubWindow() and QMdiArea::WindowOrder.
*/
impl /*struct*/ QMdiArea {
  pub fn activateNextSubWindow_0<RetType, T: QMdiArea_activateNextSubWindow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activateNextSubWindow_0(self);
    // return 1;
  }
}
pub trait QMdiArea_activateNextSubWindow_0<RetType> {
  fn activateNextSubWindow_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_activateNextSubWindow_0<(/*void*/)> for () {
  fn activateNextSubWindow_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QMdiArea21activateNextSubWindowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:143
// index:0
// Public Visibility=Default Availability=Available
// [-2] void activatePreviousSubWindow()

/*
Gives the keyboard focus to another window in the list of child windows. The window activated will be the previous one determined by the current activation order.

See also activateNextSubWindow() and QMdiArea::WindowOrder.
*/
impl /*struct*/ QMdiArea {
  pub fn activatePreviousSubWindow_0<RetType, T: QMdiArea_activatePreviousSubWindow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activatePreviousSubWindow_0(self);
    // return 1;
  }
}
pub trait QMdiArea_activatePreviousSubWindow_0<RetType> {
  fn activatePreviousSubWindow_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_activatePreviousSubWindow_0<(/*void*/)> for () {
  fn activatePreviousSubWindow_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QMdiArea25activatePreviousSubWindowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:146
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void setupViewport(QWidget *)

/*
Reimplemented from QAbstractScrollArea::setupViewport().

This slot is called by QAbstractScrollArea after setViewport() has been called. Reimplement this function in a subclass of QMdiArea to initialize the new viewport before it is used.

See also setViewport().
*/
impl /*struct*/ QMdiArea {
  pub fn setupViewport_0<RetType, T: QMdiArea_setupViewport_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setupViewport_0(self);
    // return 1;
  }
}
pub trait QMdiArea_setupViewport_0<RetType> {
  fn setupViewport_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_setupViewport_0<(/*void*/)> for (usize) {
  fn setupViewport_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMdiArea13setupViewportEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:149
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QMdiArea {
  pub fn event_0<RetType, T: QMdiArea_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QMdiArea_event_0<RetType> {
  fn event_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QMdiArea) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMdiArea5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:150
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool eventFilter(QObject *, QEvent *)

/*
Reimplemented from QObject::eventFilter().
*/
impl /*struct*/ QMdiArea {
  pub fn eventFilter_0<RetType, T: QMdiArea_eventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventFilter_0(self);
    // return 1;
  }
}
pub trait QMdiArea_eventFilter_0<RetType> {
  fn eventFilter_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_eventFilter_0<bool> for (usize,usize) {
  fn eventFilter_0(self , rsthis: & QMdiArea) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMdiArea11eventFilterEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:151
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QMdiArea {
  pub fn paintEvent_0<RetType, T: QMdiArea_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QMdiArea_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMdiArea10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:152
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void childEvent(QChildEvent *)

/*
Reimplemented from QObject::childEvent().
*/
impl /*struct*/ QMdiArea {
  pub fn childEvent_0<RetType, T: QMdiArea_childEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childEvent_0(self);
    // return 1;
  }
}
pub trait QMdiArea_childEvent_0<RetType> {
  fn childEvent_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_childEvent_0<(/*void*/)> for (usize) {
  fn childEvent_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMdiArea10childEventEP11QChildEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:153
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QMdiArea {
  pub fn resizeEvent_0<RetType, T: QMdiArea_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QMdiArea_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMdiArea11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:154
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void timerEvent(QTimerEvent *)

/*
Reimplemented from QObject::timerEvent().
*/
impl /*struct*/ QMdiArea {
  pub fn timerEvent_0<RetType, T: QMdiArea_timerEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerEvent_0(self);
    // return 1;
  }
}
pub trait QMdiArea_timerEvent_0<RetType> {
  fn timerEvent_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_timerEvent_0<(/*void*/)> for (usize) {
  fn timerEvent_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMdiArea10timerEventEP11QTimerEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:155
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void showEvent(QShowEvent *)

/*
Reimplemented from QWidget::showEvent().
*/
impl /*struct*/ QMdiArea {
  pub fn showEvent_0<RetType, T: QMdiArea_showEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showEvent_0(self);
    // return 1;
  }
}
pub trait QMdiArea_showEvent_0<RetType> {
  fn showEvent_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_showEvent_0<(/*void*/)> for (usize) {
  fn showEvent_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMdiArea9showEventEP10QShowEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:156
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool viewportEvent(QEvent *)

/*
Reimplemented from QAbstractScrollArea::viewportEvent().
*/
impl /*struct*/ QMdiArea {
  pub fn viewportEvent_0<RetType, T: QMdiArea_viewportEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewportEvent_0(self);
    // return 1;
  }
}
pub trait QMdiArea_viewportEvent_0<RetType> {
  fn viewportEvent_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_viewportEvent_0<bool> for (usize) {
  fn viewportEvent_0(self , rsthis: & QMdiArea) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMdiArea13viewportEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdiarea.h:157
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void scrollContentsBy(int, int)

/*
Reimplemented from QAbstractScrollArea::scrollContentsBy().
*/
impl /*struct*/ QMdiArea {
  pub fn scrollContentsBy_0<RetType, T: QMdiArea_scrollContentsBy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollContentsBy_0(self);
    // return 1;
  }
}
pub trait QMdiArea_scrollContentsBy_0<RetType> {
  fn scrollContentsBy_0(self , rsthis: & QMdiArea) -> RetType;
}
impl<'a> /*trait*/ QMdiArea_scrollContentsBy_0<(/*void*/)> for (i32,i32) {
  fn scrollContentsBy_0(self , rsthis: & QMdiArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QMdiArea16scrollContentsByEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QMdiArea__AreaOption = i32;
// 
pub const QMdiArea__DontMaximizeSubWindowOnActivation :QMdiArea__AreaOption = 1;
pub fn QMdiArea_AreaOptionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QMdiArea", val);
}
pub fn QMdiArea_AreaOptionItemName_s(val: i32) ->String {
  //var nilthis *QMdiArea
  //return nilthis.AreaOptionItemName(val);
  return QMdiArea_AreaOptionItemName(val);
}


/*
Specifies the criteria to use for ordering the list of child windows returned by subWindowList(). The functions cascadeSubWindows() and tileSubWindows() follow this order when arranging the windows.



See also subWindowList().

*/
pub type QMdiArea__WindowOrder = i32;
// The windows are returned in the order of their creation.
pub const QMdiArea__CreationOrder :QMdiArea__WindowOrder = 0;
// The windows are returned in the order in which they are stacked, with the top-most window being last in the list.
pub const QMdiArea__StackingOrder :QMdiArea__WindowOrder = 1;
// The windows are returned in the order in which they were activated.
pub const QMdiArea__ActivationHistoryOrder :QMdiArea__WindowOrder = 2;
pub fn QMdiArea_WindowOrderItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QMdiArea", val);
}
pub fn QMdiArea_WindowOrderItemName_s(val: i32) ->String {
  //var nilthis *QMdiArea
  //return nilthis.WindowOrderItemName(val);
  return QMdiArea_WindowOrderItemName(val);
}


/*
This enum describes the view mode of the area; i.e. how sub-windows will be displayed.



This enum was introduced or modified in  Qt 4.4.

See also setViewMode().

*/
pub type QMdiArea__ViewMode = i32;
// Display sub-windows with window frames (default).
pub const QMdiArea__SubWindowView :QMdiArea__ViewMode = 0;
// Display sub-windows with tabs in a tab bar.
pub const QMdiArea__TabbedView :QMdiArea__ViewMode = 1;
pub fn QMdiArea_ViewModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QMdiArea", val);
}
pub fn QMdiArea_ViewModeItemName_s(val: i32) ->String {
  //var nilthis *QMdiArea
  //return nilthis.ViewModeItemName(val);
  return QMdiArea_ViewModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
