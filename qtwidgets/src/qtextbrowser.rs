

// mod ::widgets::QTextBrowser
// package qtwidgets
// /usr/include/qt/QtWidgets/qtextbrowser.h
// #include <qtextbrowser.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 77
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
// func (this *QTextBrowser) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QTextBrowser) InheritKeyPressEvent(f func(ev *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QTextBrowser) InheritMouseMoveEvent(f func(ev *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QTextBrowser) InheritMousePressEvent(f func(ev *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QTextBrowser) InheritMouseReleaseEvent(f func(ev *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void focusOutEvent(QFocusEvent *)
// func (this *QTextBrowser) InheritFocusOutEvent(f func(ev *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusOutEvent", f)
// }

// bool focusNextPrevChild(bool)
// func (this *QTextBrowser) InheritFocusNextPrevChild(f func(next bool) bool) {
//  qtrt.SetAllInheritCallback(this, "focusNextPrevChild", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QTextBrowser) InheritPaintEvent(f func(e *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QTextBrowser)=48
pub struct QTextBrowser {
  qbase: QTextEdit,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextBrowser_ITF interface {
//    QTextEdit_ITF
//    QTextBrowser_PTR() *QTextBrowser
//}
//func (ptr *QTextBrowser) QTextBrowser_PTR() *QTextBrowser { return ptr }

impl /*struct*/ QTextBrowser {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextBrowser {
    return QTextBrowser{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextBrowser {
//  type Target = QTextBrowserBASE;
//
//  fn deref(&self) -> &QTextBrowserBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextBrowserBASE> for QTextBrowser {
//  fn as_ref(& self) -> & QTextBrowserBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qtextbrowser.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTextBrowser {
  pub fn metaObject_0<RetType, T: QTextBrowser_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTextBrowser) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTextBrowser10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextBrowser(QWidget *)

/*
Constructs an empty QTextBrowser with parent parent.
*/
// QTextBrowser(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QTextBrowser {
  pub fn QTextBrowser_0<T: QTextBrowser_QTextBrowser_0>(value: T) -> QTextBrowser {
    let rsthis = value.QTextBrowser_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBrowser_QTextBrowser_0 {
  fn QTextBrowser_0(self) -> QTextBrowser;
}
// QTextBrowser(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextBrowser_QTextBrowser_0 for (usize) {
  fn QTextBrowser_0(self) -> QTextBrowser {
    // unsafe{_ZN12QTextBrowserC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QTextBrowserC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextBrowser{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:67
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTextBrowser()

/*

*/
pub fn DeleteQTextBrowser(this :*mut QTextBrowser) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QTextBrowserD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qtextbrowser.h:69
// index:0
// Public Visibility=Default Availability=Available
// [8] QUrl source() const

/*

*/
impl /*struct*/ QTextBrowser {
  pub fn source_0<RetType, T: QTextBrowser_source_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.source_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_source_0<RetType> {
  fn source_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_source_0<usize> for () {
  fn source_0(self , rsthis: & QTextBrowser) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTextBrowser6sourceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:71
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList searchPaths() const

/*

*/
impl /*struct*/ QTextBrowser {
  pub fn searchPaths_0<RetType, T: QTextBrowser_searchPaths_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.searchPaths_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_searchPaths_0<RetType> {
  fn searchPaths_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_searchPaths_0<usize> for () {
  fn searchPaths_0(self , rsthis: & QTextBrowser) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTextBrowser11searchPathsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSearchPaths(const QStringList &)

/*

*/
impl /*struct*/ QTextBrowser {
  pub fn setSearchPaths_0<RetType, T: QTextBrowser_setSearchPaths_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSearchPaths_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_setSearchPaths_0<RetType> {
  fn setSearchPaths_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_setSearchPaths_0<(/*void*/)> for (usize) {
  fn setSearchPaths_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser14setSearchPathsERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant loadResource(int, const QUrl &)

/*
Reimplemented from QTextEdit::loadResource().

This function is called when the document is loaded and for each image in the document. The type indicates the type of resource to be loaded. An invalid QVariant is returned if the resource cannot be loaded.

The default implementation ignores type and tries to locate the resources by interpreting name as a file name. If it is not an absolute path it tries to find the file in the paths of the searchPaths property and in the same directory as the current source. On success, the result is a QVariant that stores a QByteArray with the contents of the file.

If you reimplement this function, you can return other QVariant types. The table below shows which variant types are supported depending on the resource type:


 ResourceTypeQVariant::Type
QTextDocument::HtmlResourceQString or QByteArray
QTextDocument::ImageResourceQImage, QPixmap or QByteArray
QTextDocument::StyleSheetResourceQString or QByteArray
*/
impl /*struct*/ QTextBrowser {
  pub fn loadResource_0<RetType, T: QTextBrowser_loadResource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.loadResource_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_loadResource_0<RetType> {
  fn loadResource_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_loadResource_0<usize> for (i32,usize) {
  fn loadResource_0(self , rsthis: & QTextBrowser) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QTextBrowser12loadResourceEiRK4QUrl", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:76
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isBackwardAvailable() const

/*
Returns true if the text browser can go backward in the document history using backward().

This function was introduced in  Qt 4.2.

See also backwardAvailable() and backward().
*/
impl /*struct*/ QTextBrowser {
  pub fn isBackwardAvailable_0<RetType, T: QTextBrowser_isBackwardAvailable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isBackwardAvailable_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_isBackwardAvailable_0<RetType> {
  fn isBackwardAvailable_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_isBackwardAvailable_0<bool> for () {
  fn isBackwardAvailable_0(self , rsthis: & QTextBrowser) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTextBrowser19isBackwardAvailableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:77
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isForwardAvailable() const

/*
Returns true if the text browser can go forward in the document history using forward().

This function was introduced in  Qt 4.2.

See also forwardAvailable() and forward().
*/
impl /*struct*/ QTextBrowser {
  pub fn isForwardAvailable_0<RetType, T: QTextBrowser_isForwardAvailable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isForwardAvailable_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_isForwardAvailable_0<RetType> {
  fn isForwardAvailable_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_isForwardAvailable_0<bool> for () {
  fn isForwardAvailable_0(self , rsthis: & QTextBrowser) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTextBrowser18isForwardAvailableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:78
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearHistory()

/*
Clears the history of visited documents and disables the forward and backward navigation.

This function was introduced in  Qt 4.2.

See also backward() and forward().
*/
impl /*struct*/ QTextBrowser {
  pub fn clearHistory_0<RetType, T: QTextBrowser_clearHistory_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearHistory_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_clearHistory_0<RetType> {
  fn clearHistory_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_clearHistory_0<(/*void*/)> for () {
  fn clearHistory_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser12clearHistoryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:79
// index:0
// Public Visibility=Default Availability=Available
// [8] QString historyTitle(int) const

/*
Returns the documentTitle() of the HistoryItem.


 InputReturn
i < 0backward() history
i == 0current, see QTextBrowser::source()
i > 0forward() history



  backaction.setToolTip(browser.historyTitle(-1));
  forwardaction.setToolTip(browser.historyTitle(+1));



This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QTextBrowser {
  pub fn historyTitle_0<RetType, T: QTextBrowser_historyTitle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.historyTitle_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_historyTitle_0<RetType> {
  fn historyTitle_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_historyTitle_0<usize> for (i32) {
  fn historyTitle_0(self , rsthis: & QTextBrowser) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTextBrowser12historyTitleEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:80
// index:0
// Public Visibility=Default Availability=Available
// [8] QUrl historyUrl(int) const

/*
Returns the url of the HistoryItem.


 InputReturn
i < 0backward() history
i == 0current, see QTextBrowser::source()
i > 0forward() history


This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QTextBrowser {
  pub fn historyUrl_0<RetType, T: QTextBrowser_historyUrl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.historyUrl_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_historyUrl_0<RetType> {
  fn historyUrl_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_historyUrl_0<usize> for (i32) {
  fn historyUrl_0(self , rsthis: & QTextBrowser) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTextBrowser10historyUrlEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:81
// index:0
// Public Visibility=Default Availability=Available
// [4] int backwardHistoryCount() const

/*
Returns the number of locations backward in the history.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QTextBrowser {
  pub fn backwardHistoryCount_0<RetType, T: QTextBrowser_backwardHistoryCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.backwardHistoryCount_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_backwardHistoryCount_0<RetType> {
  fn backwardHistoryCount_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_backwardHistoryCount_0<i32> for () {
  fn backwardHistoryCount_0(self , rsthis: & QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTextBrowser20backwardHistoryCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:82
// index:0
// Public Visibility=Default Availability=Available
// [4] int forwardHistoryCount() const

/*
Returns the number of locations forward in the history.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QTextBrowser {
  pub fn forwardHistoryCount_0<RetType, T: QTextBrowser_forwardHistoryCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.forwardHistoryCount_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_forwardHistoryCount_0<RetType> {
  fn forwardHistoryCount_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_forwardHistoryCount_0<i32> for () {
  fn forwardHistoryCount_0(self , rsthis: & QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTextBrowser19forwardHistoryCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:84
// index:0
// Public Visibility=Default Availability=Available
// [1] bool openExternalLinks() const

/*

*/
impl /*struct*/ QTextBrowser {
  pub fn openExternalLinks_0<RetType, T: QTextBrowser_openExternalLinks_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.openExternalLinks_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_openExternalLinks_0<RetType> {
  fn openExternalLinks_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_openExternalLinks_0<bool> for () {
  fn openExternalLinks_0(self , rsthis: & QTextBrowser) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTextBrowser17openExternalLinksEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOpenExternalLinks(bool)

/*

*/
impl /*struct*/ QTextBrowser {
  pub fn setOpenExternalLinks_0<RetType, T: QTextBrowser_setOpenExternalLinks_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOpenExternalLinks_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_setOpenExternalLinks_0<RetType> {
  fn setOpenExternalLinks_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_setOpenExternalLinks_0<(/*void*/)> for (bool) {
  fn setOpenExternalLinks_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser20setOpenExternalLinksEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:87
// index:0
// Public Visibility=Default Availability=Available
// [1] bool openLinks() const

/*

*/
impl /*struct*/ QTextBrowser {
  pub fn openLinks_0<RetType, T: QTextBrowser_openLinks_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.openLinks_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_openLinks_0<RetType> {
  fn openLinks_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_openLinks_0<bool> for () {
  fn openLinks_0(self , rsthis: & QTextBrowser) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTextBrowser9openLinksEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:88
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOpenLinks(bool)

/*

*/
impl /*struct*/ QTextBrowser {
  pub fn setOpenLinks_0<RetType, T: QTextBrowser_setOpenLinks_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOpenLinks_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_setOpenLinks_0<RetType> {
  fn setOpenLinks_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_setOpenLinks_0<(/*void*/)> for (bool) {
  fn setOpenLinks_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser12setOpenLinksEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:91
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setSource(const QUrl &)

/*

*/
impl /*struct*/ QTextBrowser {
  pub fn setSource_0<RetType, T: QTextBrowser_setSource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSource_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_setSource_0<RetType> {
  fn setSource_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_setSource_0<(/*void*/)> for (usize) {
  fn setSource_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser9setSourceERK4QUrl", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:92
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void backward()

/*
Changes the document displayed to the previous document in the list of documents built by navigating links. Does nothing if there is no previous document.

See also forward() and backwardAvailable().
*/
impl /*struct*/ QTextBrowser {
  pub fn backward_0<RetType, T: QTextBrowser_backward_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.backward_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_backward_0<RetType> {
  fn backward_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_backward_0<(/*void*/)> for () {
  fn backward_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser8backwardEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:93
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void forward()

/*
Changes the document displayed to the next document in the list of documents built by navigating links. Does nothing if there is no next document.

See also backward() and forwardAvailable().
*/
impl /*struct*/ QTextBrowser {
  pub fn forward_0<RetType, T: QTextBrowser_forward_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.forward_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_forward_0<RetType> {
  fn forward_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_forward_0<(/*void*/)> for () {
  fn forward_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser7forwardEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:94
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void home()

/*
Changes the document displayed to be the first document from the history.
*/
impl /*struct*/ QTextBrowser {
  pub fn home_0<RetType, T: QTextBrowser_home_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.home_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_home_0<RetType> {
  fn home_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_home_0<(/*void*/)> for () {
  fn home_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser4homeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:95
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void reload()

/*
Reloads the current set source.
*/
impl /*struct*/ QTextBrowser {
  pub fn reload_0<RetType, T: QTextBrowser_reload_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reload_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_reload_0<RetType> {
  fn reload_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_reload_0<(/*void*/)> for () {
  fn reload_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser6reloadEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void backwardAvailable(bool)

/*
This signal is emitted when the availability of backward() changes. available is false when the user is at home(); otherwise it is true.
*/
impl /*struct*/ QTextBrowser {
  pub fn backwardAvailable_0<RetType, T: QTextBrowser_backwardAvailable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.backwardAvailable_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_backwardAvailable_0<RetType> {
  fn backwardAvailable_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_backwardAvailable_0<(/*void*/)> for (bool) {
  fn backwardAvailable_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser17backwardAvailableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void forwardAvailable(bool)

/*
This signal is emitted when the availability of forward() changes. available is true after the user navigates backward() and false when the user navigates or goes forward().
*/
impl /*struct*/ QTextBrowser {
  pub fn forwardAvailable_0<RetType, T: QTextBrowser_forwardAvailable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.forwardAvailable_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_forwardAvailable_0<RetType> {
  fn forwardAvailable_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_forwardAvailable_0<(/*void*/)> for (bool) {
  fn forwardAvailable_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser16forwardAvailableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void historyChanged()

/*
This signal is emitted when the history changes.

This function was introduced in  Qt 4.4.

See also historyTitle() and historyUrl().
*/
impl /*struct*/ QTextBrowser {
  pub fn historyChanged_0<RetType, T: QTextBrowser_historyChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.historyChanged_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_historyChanged_0<RetType> {
  fn historyChanged_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_historyChanged_0<(/*void*/)> for () {
  fn historyChanged_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser14historyChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sourceChanged(const QUrl &)

/*
This signal is emitted when the source has changed, src being the new source.

Source changes happen both programmatically when calling setSource(), forward(), backword() or home() or when the user clicks on links or presses the equivalent key sequences.
*/
impl /*struct*/ QTextBrowser {
  pub fn sourceChanged_0<RetType, T: QTextBrowser_sourceChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sourceChanged_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_sourceChanged_0<RetType> {
  fn sourceChanged_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_sourceChanged_0<(/*void*/)> for (usize) {
  fn sourceChanged_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser13sourceChangedERK4QUrl", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void highlighted(const QUrl &)

/*
This signal is emitted when the user has selected but not activated an anchor in the document. The URL referred to by the anchor is passed in link.

Note: Signal highlighted is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(textBrowser, QOverload<const QUrl &>::of(&QTextBrowser::highlighted),
      [=](const QUrl &link){ /-* ... *-/ });
*/
impl /*struct*/ QTextBrowser {
  pub fn highlighted_0<RetType, T: QTextBrowser_highlighted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.highlighted_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_highlighted_0<RetType> {
  fn highlighted_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_highlighted_0<(/*void*/)> for (usize) {
  fn highlighted_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser11highlightedERK4QUrl", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:103
// index:1
// Public Visibility=Default Availability=Available
// [-2] void highlighted(const QString &)

/*
This signal is emitted when the user has selected but not activated an anchor in the document. The URL referred to by the anchor is passed in link.

Note: Signal highlighted is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(textBrowser, QOverload<const QUrl &>::of(&QTextBrowser::highlighted),
      [=](const QUrl &link){ /-* ... *-/ });
*/
impl /*struct*/ QTextBrowser {
  pub fn highlighted_1<RetType, T: QTextBrowser_highlighted_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.highlighted_1(self);
    // return 1;
  }
}
pub trait QTextBrowser_highlighted_1<RetType> {
  fn highlighted_1(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_highlighted_1<(/*void*/)> for (usize) {
  fn highlighted_1(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser11highlightedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void anchorClicked(const QUrl &)

/*
This signal is emitted when the user clicks an anchor. The URL referred to by the anchor is passed in link.

Note that the browser will automatically handle navigation to the location specified by link unless the openLinks property is set to false or you call setSource() in a slot connected. This mechanism is used to override the default navigation features of the browser.
*/
impl /*struct*/ QTextBrowser {
  pub fn anchorClicked_0<RetType, T: QTextBrowser_anchorClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.anchorClicked_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_anchorClicked_0<RetType> {
  fn anchorClicked_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_anchorClicked_0<(/*void*/)> for (usize) {
  fn anchorClicked_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser13anchorClickedERK4QUrl", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:107
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QTextBrowser {
  pub fn event_0<RetType, T: QTextBrowser_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_event_0<RetType> {
  fn event_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QTextBrowser) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QTextBrowser5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:108
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().

The event ev is used to provide the following keyboard shortcuts:


 KeypressAction
Alt+Left Arrowbackward()
Alt+Right Arrowforward()
Alt+Up Arrowhome()
*/
impl /*struct*/ QTextBrowser {
  pub fn keyPressEvent_0<RetType, T: QTextBrowser_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:109
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QTextBrowser {
  pub fn mouseMoveEvent_0<RetType, T: QTextBrowser_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:110
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QTextBrowser {
  pub fn mousePressEvent_0<RetType, T: QTextBrowser_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:111
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QTextBrowser {
  pub fn mouseReleaseEvent_0<RetType, T: QTextBrowser_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:112
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusOutEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusOutEvent().
*/
impl /*struct*/ QTextBrowser {
  pub fn focusOutEvent_0<RetType, T: QTextBrowser_focusOutEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusOutEvent_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_focusOutEvent_0<RetType> {
  fn focusOutEvent_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_focusOutEvent_0<(/*void*/)> for (usize) {
  fn focusOutEvent_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser13focusOutEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:113
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool focusNextPrevChild(bool)

/*
Reimplemented from QWidget::focusNextPrevChild().
*/
impl /*struct*/ QTextBrowser {
  pub fn focusNextPrevChild_0<RetType, T: QTextBrowser_focusNextPrevChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusNextPrevChild_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_focusNextPrevChild_0<RetType> {
  fn focusNextPrevChild_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_focusNextPrevChild_0<bool> for (bool) {
  fn focusNextPrevChild_0(self , rsthis: & QTextBrowser) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QTextBrowser18focusNextPrevChildEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextbrowser.h:114
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QTextBrowser {
  pub fn paintEvent_0<RetType, T: QTextBrowser_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QTextBrowser_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QTextBrowser) -> RetType;
}
impl<'a> /*trait*/ QTextBrowser_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QTextBrowser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTextBrowser10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
