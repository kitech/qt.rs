

// mod ::widgets::QWizard
// package qtwidgets
// /usr/include/qt/QtWidgets/qwizard.h
// #include <qwizard.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 11
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
// func (this *QWizard) InheritEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QWizard) InheritResizeEvent(f func(event *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QWizard) InheritPaintEvent(f func(event *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void done(int)
// func (this *QWizard) InheritDone(f func(result int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "done", f)
// }

// void initializePage(int)
// func (this *QWizard) InheritInitializePage(f func(id int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initializePage", f)
// }

// void cleanupPage(int)
// func (this *QWizard) InheritCleanupPage(f func(id int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "cleanupPage", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QWizard)=48
pub struct QWizard {
  qbase: QDialog,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QWizard_ITF interface {
//    QDialog_ITF
//    QWizard_PTR() *QWizard
//}
//func (ptr *QWizard) QWizard_PTR() *QWizard { return ptr }

impl /*struct*/ QWizard {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QWizard {
    return QWizard{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QWizard {
//  type Target = QWizardBASE;
//
//  fn deref(&self) -> &QWizardBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QWizardBASE> for QWizard {
//  fn as_ref(& self) -> & QWizardBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qwizard.h:56
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QWizard {
  pub fn metaObject_0<RetType, T: QWizard_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QWizard_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QWizard) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWizard10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:123
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QWizard(QWidget *, Qt::WindowFlags)

/*
Constructs a wizard with the given parent and window flags.

See also parent() and windowFlags().
*/
// QWizard(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QWizard {
  pub fn QWizard_0<T: QWizard_QWizard_0>(value: T) -> QWizard {
    let rsthis = value.QWizard_0();
    return rsthis;
    // return 1;
  }
}

pub trait QWizard_QWizard_0 {
  fn QWizard_0(self) -> QWizard;
}
// QWizard(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QWizard_QWizard_0 for (usize,i32) {
  fn QWizard_0(self) -> QWizard {
    // unsafe{_ZN7QWizardC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QWizardC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QWizard{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:124
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QWizard()

/*

*/
pub fn DeleteQWizard(this :*mut QWizard) {
    // let rv = qtrt::InvokeQtFunc6("_ZN7QWizardD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qwizard.h:126
// index:0
// Public Visibility=Default Availability=Available
// [4] int addPage(QWizardPage *)

/*
Adds the given page to the wizard, and returns the page's ID.

The ID is guaranteed to be larger than any other ID in the QWizard so far.

See also setPage(), page(), and pageAdded().
*/
impl /*struct*/ QWizard {
  pub fn addPage_0<RetType, T: QWizard_addPage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addPage_0(self);
    // return 1;
  }
}
pub trait QWizard_addPage_0<RetType> {
  fn addPage_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_addPage_0<i32> for (usize) {
  fn addPage_0(self , rsthis: & QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWizard7addPageEP11QWizardPage", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:127
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPage(int, QWizardPage *)

/*
Adds the given page to the wizard with the given id.

Note: Adding a page may influence the value of the startId property in case it was not set explicitly.

See also addPage(), page(), and pageAdded().
*/
impl /*struct*/ QWizard {
  pub fn setPage_0<RetType, T: QWizard_setPage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPage_0(self);
    // return 1;
  }
}
pub trait QWizard_setPage_0<RetType> {
  fn setPage_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_setPage_0<(/*void*/)> for (i32,usize) {
  fn setPage_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard7setPageEiP11QWizardPage", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:128
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removePage(int)

/*
Removes the page with the given id. cleanupPage() will be called if necessary.

Note: Removing a page may influence the value of the startId property.

This function was introduced in  Qt 4.5.

See also addPage(), setPage(), pageRemoved(), and startId().
*/
impl /*struct*/ QWizard {
  pub fn removePage_0<RetType, T: QWizard_removePage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removePage_0(self);
    // return 1;
  }
}
pub trait QWizard_removePage_0<RetType> {
  fn removePage_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_removePage_0<(/*void*/)> for (i32) {
  fn removePage_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard10removePageEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:129
// index:0
// Public Visibility=Default Availability=Available
// [8] QWizardPage * page(int) const

/*
Returns the page with the given id, or 0 if there is no such page.

See also addPage() and setPage().
*/
impl /*struct*/ QWizard {
  pub fn page_0<RetType, T: QWizard_page_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.page_0(self);
    // return 1;
  }
}
pub trait QWizard_page_0<RetType> {
  fn page_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_page_0<usize> for (i32) {
  fn page_0(self , rsthis: & QWizard) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWizard4pageEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:130
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasVisitedPage(int) const

/*
Returns true if the page history contains page id; otherwise, returns false.

Pressing Back marks the current page as "unvisited" again.

See also visitedPages().
*/
impl /*struct*/ QWizard {
  pub fn hasVisitedPage_0<RetType, T: QWizard_hasVisitedPage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasVisitedPage_0(self);
    // return 1;
  }
}
pub trait QWizard_hasVisitedPage_0<RetType> {
  fn hasVisitedPage_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_hasVisitedPage_0<bool> for (i32) {
  fn hasVisitedPage_0(self , rsthis: & QWizard) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWizard14hasVisitedPageEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:133
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStartId(int)

/*

*/
impl /*struct*/ QWizard {
  pub fn setStartId_0<RetType, T: QWizard_setStartId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStartId_0(self);
    // return 1;
  }
}
pub trait QWizard_setStartId_0<RetType> {
  fn setStartId_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_setStartId_0<(/*void*/)> for (i32) {
  fn setStartId_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard10setStartIdEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:134
// index:0
// Public Visibility=Default Availability=Available
// [4] int startId() const

/*

*/
impl /*struct*/ QWizard {
  pub fn startId_0<RetType, T: QWizard_startId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startId_0(self);
    // return 1;
  }
}
pub trait QWizard_startId_0<RetType> {
  fn startId_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_startId_0<i32> for () {
  fn startId_0(self , rsthis: & QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWizard7startIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:135
// index:0
// Public Visibility=Default Availability=Available
// [8] QWizardPage * currentPage() const

/*
Returns a pointer to the current page, or 0 if there is no current page (e.g., before the wizard is shown).

This is equivalent to calling page(currentId()).

See also page(), currentId(), and restart().
*/
impl /*struct*/ QWizard {
  pub fn currentPage_0<RetType, T: QWizard_currentPage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentPage_0(self);
    // return 1;
  }
}
pub trait QWizard_currentPage_0<RetType> {
  fn currentPage_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_currentPage_0<usize> for () {
  fn currentPage_0(self , rsthis: & QWizard) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWizard11currentPageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:136
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentId() const

/*

*/
impl /*struct*/ QWizard {
  pub fn currentId_0<RetType, T: QWizard_currentId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentId_0(self);
    // return 1;
  }
}
pub trait QWizard_currentId_0<RetType> {
  fn currentId_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_currentId_0<i32> for () {
  fn currentId_0(self , rsthis: & QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWizard9currentIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:138
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool validateCurrentPage()

/*
This virtual function is called by QWizard when the user clicks Next or Finish to perform some last-minute validation. If it returns true, the next page is shown (or the wizard finishes); otherwise, the current page stays up.

The default implementation calls QWizardPage::validatePage() on the currentPage().

When possible, it is usually better style to disable the Next or Finish button (by specifying mandatory fields or by reimplementing QWizardPage::isComplete()) than to reimplement validateCurrentPage().

See also QWizardPage::validatePage() and currentPage().
*/
impl /*struct*/ QWizard {
  pub fn validateCurrentPage_0<RetType, T: QWizard_validateCurrentPage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.validateCurrentPage_0(self);
    // return 1;
  }
}
pub trait QWizard_validateCurrentPage_0<RetType> {
  fn validateCurrentPage_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_validateCurrentPage_0<bool> for () {
  fn validateCurrentPage_0(self , rsthis: & QWizard) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWizard19validateCurrentPageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:139
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int nextId() const

/*
This virtual function is called by QWizard to find out which page to show when the user clicks the Next button.

The return value is the ID of the next page, or -1 if no page follows.

The default implementation calls QWizardPage::nextId() on the currentPage().

By reimplementing this function, you can specify a dynamic page order.

See also QWizardPage::nextId() and currentPage().
*/
impl /*struct*/ QWizard {
  pub fn nextId_0<RetType, T: QWizard_nextId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nextId_0(self);
    // return 1;
  }
}
pub trait QWizard_nextId_0<RetType> {
  fn nextId_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_nextId_0<i32> for () {
  fn nextId_0(self , rsthis: & QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWizard6nextIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:141
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setField(const QString &, const QVariant &)

/*
Sets the value of the field called name to value.

This function can be used to set fields on any page of the wizard.

See also QWizardPage::registerField(), QWizardPage::setField(), and field().
*/
impl /*struct*/ QWizard {
  pub fn setField_0<RetType, T: QWizard_setField_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setField_0(self);
    // return 1;
  }
}
pub trait QWizard_setField_0<RetType> {
  fn setField_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_setField_0<(/*void*/)> for (usize,usize) {
  fn setField_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard8setFieldERK7QStringRK8QVariant", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:142
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant field(const QString &) const

/*
Returns the value of the field called name.

This function can be used to access fields on any page of the wizard.

See also QWizardPage::registerField(), QWizardPage::field(), and setField().
*/
impl /*struct*/ QWizard {
  pub fn field_0<RetType, T: QWizard_field_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.field_0(self);
    // return 1;
  }
}
pub trait QWizard_field_0<RetType> {
  fn field_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_field_0<usize> for (usize) {
  fn field_0(self , rsthis: & QWizard) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWizard5fieldERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:144
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWizardStyle(QWizard::WizardStyle)

/*

*/
impl /*struct*/ QWizard {
  pub fn setWizardStyle_0<RetType, T: QWizard_setWizardStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWizardStyle_0(self);
    // return 1;
  }
}
pub trait QWizard_setWizardStyle_0<RetType> {
  fn setWizardStyle_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_setWizardStyle_0<(/*void*/)> for (i32) {
  fn setWizardStyle_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard14setWizardStyleENS_11WizardStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:145
// index:0
// Public Visibility=Default Availability=Available
// [4] QWizard::WizardStyle wizardStyle() const

/*

*/
impl /*struct*/ QWizard {
  pub fn wizardStyle_0<RetType, T: QWizard_wizardStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wizardStyle_0(self);
    // return 1;
  }
}
pub trait QWizard_wizardStyle_0<RetType> {
  fn wizardStyle_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_wizardStyle_0<i32> for () {
  fn wizardStyle_0(self , rsthis: & QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWizard11wizardStyleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:147
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOption(QWizard::WizardOption, bool)

/*
Sets the given option to be enabled if on is true; otherwise, clears the given option.

See also options, testOption(), and setWizardStyle().
*/
impl /*struct*/ QWizard {
  pub fn setOption_0<RetType, T: QWizard_setOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOption_0(self);
    // return 1;
  }
}
pub trait QWizard_setOption_0<RetType> {
  fn setOption_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_setOption_0<(/*void*/)> for (i32,bool) {
  fn setOption_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard9setOptionENS_12WizardOptionEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:148
// index:0
// Public Visibility=Default Availability=Available
// [1] bool testOption(QWizard::WizardOption) const

/*
Returns true if the given option is enabled; otherwise, returns false.

See also options, setOption(), and setWizardStyle().
*/
impl /*struct*/ QWizard {
  pub fn testOption_0<RetType, T: QWizard_testOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.testOption_0(self);
    // return 1;
  }
}
pub trait QWizard_testOption_0<RetType> {
  fn testOption_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_testOption_0<bool> for (i32) {
  fn testOption_0(self , rsthis: & QWizard) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWizard10testOptionENS_12WizardOptionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:149
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOptions(QWizard::WizardOptions)

/*

*/
impl /*struct*/ QWizard {
  pub fn setOptions_0<RetType, T: QWizard_setOptions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOptions_0(self);
    // return 1;
  }
}
pub trait QWizard_setOptions_0<RetType> {
  fn setOptions_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_setOptions_0<(/*void*/)> for (i32) {
  fn setOptions_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard10setOptionsE6QFlagsINS_12WizardOptionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:150
// index:0
// Public Visibility=Default Availability=Available
// [4] QWizard::WizardOptions options() const

/*

*/
impl /*struct*/ QWizard {
  pub fn options_0<RetType, T: QWizard_options_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.options_0(self);
    // return 1;
  }
}
pub trait QWizard_options_0<RetType> {
  fn options_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_options_0<i32> for () {
  fn options_0(self , rsthis: & QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWizard7optionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:152
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setButtonText(QWizard::WizardButton, const QString &)

/*
Sets the text on button which to be text.

By default, the text on buttons depends on the wizardStyle. For example, on macOS, the Next button is called Continue.

To add extra buttons to the wizard (e.g., a Print button), one way is to call setButtonText() with CustomButton1, CustomButton2, or CustomButton3 to set their text, and make the buttons visible using the HaveCustomButton1, HaveCustomButton2, and/or HaveCustomButton3 options.

Button texts may also be set on a per-page basis using QWizardPage::setButtonText().

See also buttonText(), setButton(), button(), setButtonLayout(), setOptions(), and QWizardPage::setButtonText().
*/
impl /*struct*/ QWizard {
  pub fn setButtonText_0<RetType, T: QWizard_setButtonText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setButtonText_0(self);
    // return 1;
  }
}
pub trait QWizard_setButtonText_0<RetType> {
  fn setButtonText_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_setButtonText_0<(/*void*/)> for (i32,usize) {
  fn setButtonText_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard13setButtonTextENS_12WizardButtonERK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:153
// index:0
// Public Visibility=Default Availability=Available
// [8] QString buttonText(QWizard::WizardButton) const

/*
Returns the text on button which.

If a text has ben set using setButtonText(), this text is returned.

By default, the text on buttons depends on the wizardStyle. For example, on macOS, the Next button is called Continue.

See also button(), setButton(), setButtonText(), QWizardPage::buttonText(), and QWizardPage::setButtonText().
*/
impl /*struct*/ QWizard {
  pub fn buttonText_0<RetType, T: QWizard_buttonText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttonText_0(self);
    // return 1;
  }
}
pub trait QWizard_buttonText_0<RetType> {
  fn buttonText_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_buttonText_0<usize> for (i32) {
  fn buttonText_0(self , rsthis: & QWizard) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWizard10buttonTextENS_12WizardButtonE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:155
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setButton(QWizard::WizardButton, QAbstractButton *)

/*
Sets the button corresponding to role which to button.

To add extra buttons to the wizard (e.g., a Print button), one way is to call setButton() with CustomButton1 to CustomButton3, and make the buttons visible using the HaveCustomButton1 to HaveCustomButton3 options.

See also button(), setButtonText(), setButtonLayout(), and options.
*/
impl /*struct*/ QWizard {
  pub fn setButton_0<RetType, T: QWizard_setButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setButton_0(self);
    // return 1;
  }
}
pub trait QWizard_setButton_0<RetType> {
  fn setButton_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_setButton_0<(/*void*/)> for (i32,usize) {
  fn setButton_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard9setButtonENS_12WizardButtonEP15QAbstractButton", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:156
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractButton * button(QWizard::WizardButton) const

/*
Returns the button corresponding to role which.

See also setButton() and setButtonText().
*/
impl /*struct*/ QWizard {
  pub fn button_0<RetType, T: QWizard_button_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.button_0(self);
    // return 1;
  }
}
pub trait QWizard_button_0<RetType> {
  fn button_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_button_0<usize> for (i32) {
  fn button_0(self , rsthis: & QWizard) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWizard6buttonENS_12WizardButtonE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:158
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTitleFormat(Qt::TextFormat)

/*

*/
impl /*struct*/ QWizard {
  pub fn setTitleFormat_0<RetType, T: QWizard_setTitleFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTitleFormat_0(self);
    // return 1;
  }
}
pub trait QWizard_setTitleFormat_0<RetType> {
  fn setTitleFormat_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_setTitleFormat_0<(/*void*/)> for (i32) {
  fn setTitleFormat_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard14setTitleFormatEN2Qt10TextFormatE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:159
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::TextFormat titleFormat() const

/*

*/
impl /*struct*/ QWizard {
  pub fn titleFormat_0<RetType, T: QWizard_titleFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.titleFormat_0(self);
    // return 1;
  }
}
pub trait QWizard_titleFormat_0<RetType> {
  fn titleFormat_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_titleFormat_0<i32> for () {
  fn titleFormat_0(self , rsthis: & QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWizard11titleFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:160
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSubTitleFormat(Qt::TextFormat)

/*

*/
impl /*struct*/ QWizard {
  pub fn setSubTitleFormat_0<RetType, T: QWizard_setSubTitleFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSubTitleFormat_0(self);
    // return 1;
  }
}
pub trait QWizard_setSubTitleFormat_0<RetType> {
  fn setSubTitleFormat_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_setSubTitleFormat_0<(/*void*/)> for (i32) {
  fn setSubTitleFormat_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard17setSubTitleFormatEN2Qt10TextFormatE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:161
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::TextFormat subTitleFormat() const

/*

*/
impl /*struct*/ QWizard {
  pub fn subTitleFormat_0<RetType, T: QWizard_subTitleFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.subTitleFormat_0(self);
    // return 1;
  }
}
pub trait QWizard_subTitleFormat_0<RetType> {
  fn subTitleFormat_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_subTitleFormat_0<i32> for () {
  fn subTitleFormat_0(self , rsthis: & QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWizard14subTitleFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:162
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPixmap(QWizard::WizardPixmap, const QPixmap &)

/*
Sets the pixmap for role which to pixmap.

The pixmaps are used by QWizard when displaying a page. Which pixmaps are actually used depend on the wizard style.

Pixmaps can also be set for a specific page using QWizardPage::setPixmap().

See also pixmap(), QWizardPage::setPixmap(), and Elements of a Wizard Page.
*/
impl /*struct*/ QWizard {
  pub fn setPixmap_0<RetType, T: QWizard_setPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPixmap_0(self);
    // return 1;
  }
}
pub trait QWizard_setPixmap_0<RetType> {
  fn setPixmap_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_setPixmap_0<(/*void*/)> for (i32,usize) {
  fn setPixmap_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard9setPixmapENS_12WizardPixmapERK7QPixmap", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:163
// index:0
// Public Visibility=Default Availability=Available
// [32] QPixmap pixmap(QWizard::WizardPixmap) const

/*
Returns the pixmap set for role which.

By default, the only pixmap that is set is the BackgroundPixmap on macOS.

See also setPixmap(), QWizardPage::pixmap(), and Elements of a Wizard Page.
*/
impl /*struct*/ QWizard {
  pub fn pixmap_0<RetType, T: QWizard_pixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixmap_0(self);
    // return 1;
  }
}
pub trait QWizard_pixmap_0<RetType> {
  fn pixmap_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_pixmap_0<usize> for (i32) {
  fn pixmap_0(self , rsthis: & QWizard) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWizard6pixmapENS_12WizardPixmapE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:165
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSideWidget(QWidget *)

/*
Sets the given widget to be shown on the left side of the wizard. For styles which use the WatermarkPixmap (ClassicStyle and ModernStyle) the side widget is displayed on top of the watermark, for other styles or when the watermark is not provided the side widget is displayed on the left side of the wizard.

Passing 0 shows no side widget.

When the widget is not 0 the wizard reparents it.

Any previous side widget is hidden.

You may call setSideWidget() with the same widget at different times.

All widgets set here will be deleted by the wizard when it is destroyed unless you separately reparent the widget after setting some other side widget (or 0).

By default, no side widget is present.

This function was introduced in  Qt 4.7.

See also sideWidget().
*/
impl /*struct*/ QWizard {
  pub fn setSideWidget_0<RetType, T: QWizard_setSideWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSideWidget_0(self);
    // return 1;
  }
}
pub trait QWizard_setSideWidget_0<RetType> {
  fn setSideWidget_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_setSideWidget_0<(/*void*/)> for (usize) {
  fn setSideWidget_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard13setSideWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:166
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * sideWidget() const

/*
Returns the widget on the left side of the wizard or 0.

By default, no side widget is present.

This function was introduced in  Qt 4.7.

See also setSideWidget().
*/
impl /*struct*/ QWizard {
  pub fn sideWidget_0<RetType, T: QWizard_sideWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sideWidget_0(self);
    // return 1;
  }
}
pub trait QWizard_sideWidget_0<RetType> {
  fn sideWidget_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_sideWidget_0<usize> for () {
  fn sideWidget_0(self , rsthis: & QWizard) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWizard10sideWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:168
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefaultProperty(const char *, const char *, const char *)

/*
Sets the default property for className to be property, and the associated change signal to be changedSignal.

The default property is used when an instance of className (or of one of its subclasses) is passed to QWizardPage::registerField() and no property is specified.

QWizard knows the most common Qt widgets. For these (or their subclasses), you don't need to specify a property or a changedSignal. The table below lists these widgets:


 WidgetPropertyChange Notification Signal
QAbstractButtonbool checkedtoggled()
QAbstractSliderint valuevalueChanged()
QComboBoxint currentIndexcurrentIndexChanged()
QDateTimeEditQDateTime dateTimedateTimeChanged()
QLineEditQString texttextChanged()
QListWidgetint currentRowcurrentRowChanged()
QSpinBoxint valuevalueChanged()


See also QWizardPage::registerField().
*/
impl /*struct*/ QWizard {
  pub fn setDefaultProperty_0<RetType, T: QWizard_setDefaultProperty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultProperty_0(self);
    // return 1;
  }
}
pub trait QWizard_setDefaultProperty_0<RetType> {
  fn setDefaultProperty_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_setDefaultProperty_0<(/*void*/)> for (usize,usize,usize) {
  fn setDefaultProperty_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (self.2) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard18setDefaultPropertyEPKcS1_S1_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:171
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setVisible(bool)

/*
Reimplemented from QWidget::setVisible().
*/
impl /*struct*/ QWizard {
  pub fn setVisible_0<RetType, T: QWizard_setVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVisible_0(self);
    // return 1;
  }
}
pub trait QWizard_setVisible_0<RetType> {
  fn setVisible_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_setVisible_0<(/*void*/)> for (bool) {
  fn setVisible_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard10setVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:172
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QWizard {
  pub fn sizeHint_0<RetType, T: QWizard_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QWizard_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QWizard) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWizard8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:175
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentIdChanged(int)

/*
This signal is emitted when the current page changes, with the new current id.

Note: Notifier signal for property currentId. 

See also currentId() and currentPage().
*/
impl /*struct*/ QWizard {
  pub fn currentIdChanged_0<RetType, T: QWizard_currentIdChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentIdChanged_0(self);
    // return 1;
  }
}
pub trait QWizard_currentIdChanged_0<RetType> {
  fn currentIdChanged_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_currentIdChanged_0<(/*void*/)> for (i32) {
  fn currentIdChanged_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard16currentIdChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:176
// index:0
// Public Visibility=Default Availability=Available
// [-2] void helpRequested()

/*
This signal is emitted when the user clicks the Help button.

By default, no Help button is shown. Call setOption(HaveHelpButton, true) to have one.

Example:


  LicenseWizard::LicenseWizard(QWidget *parent)
      : QWizard(parent)
  {
      ...
      setOption(HaveHelpButton, true);
      connect(this, &QWizard::helpRequested, this, &LicenseWizard::showHelp);
      ...
  }

  void LicenseWizard::showHelp()
  {
      static QString lastHelpMessage;

      QString message;

      switch (currentId()) {
      case Page_Intro:
          message = tr("The decision you make here will affect which page you "
                       "get to see next.");
          break;
      ...
      default:
          message = tr("This help is likely not to be of any help.");
      }

      QMessageBox::information(this, tr("License Wizard Help"), message);

  }



See also customButtonClicked().
*/
impl /*struct*/ QWizard {
  pub fn helpRequested_0<RetType, T: QWizard_helpRequested_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.helpRequested_0(self);
    // return 1;
  }
}
pub trait QWizard_helpRequested_0<RetType> {
  fn helpRequested_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_helpRequested_0<(/*void*/)> for () {
  fn helpRequested_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWizard13helpRequestedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:177
// index:0
// Public Visibility=Default Availability=Available
// [-2] void customButtonClicked(int)

/*
This signal is emitted when the user clicks a custom button. which can be CustomButton1, CustomButton2, or CustomButton3.

By default, no custom button is shown. Call setOption() with HaveCustomButton1, HaveCustomButton2, or HaveCustomButton3 to have one, and use setButtonText() or setButton() to configure it.

See also helpRequested().
*/
impl /*struct*/ QWizard {
  pub fn customButtonClicked_0<RetType, T: QWizard_customButtonClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.customButtonClicked_0(self);
    // return 1;
  }
}
pub trait QWizard_customButtonClicked_0<RetType> {
  fn customButtonClicked_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_customButtonClicked_0<(/*void*/)> for (i32) {
  fn customButtonClicked_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard19customButtonClickedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:178
// index:0
// Public Visibility=Default Availability=Available
// [-2] void pageAdded(int)

/*
This signal is emitted whenever a page is added to the wizard. The page's id is passed as parameter.

This function was introduced in  Qt 4.7.

See also addPage(), setPage(), and startId().
*/
impl /*struct*/ QWizard {
  pub fn pageAdded_0<RetType, T: QWizard_pageAdded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pageAdded_0(self);
    // return 1;
  }
}
pub trait QWizard_pageAdded_0<RetType> {
  fn pageAdded_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_pageAdded_0<(/*void*/)> for (i32) {
  fn pageAdded_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard9pageAddedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:179
// index:0
// Public Visibility=Default Availability=Available
// [-2] void pageRemoved(int)

/*
This signal is emitted whenever a page is removed from the wizard. The page's id is passed as parameter.

This function was introduced in  Qt 4.7.

See also removePage() and startId().
*/
impl /*struct*/ QWizard {
  pub fn pageRemoved_0<RetType, T: QWizard_pageRemoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pageRemoved_0(self);
    // return 1;
  }
}
pub trait QWizard_pageRemoved_0<RetType> {
  fn pageRemoved_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_pageRemoved_0<(/*void*/)> for (i32) {
  fn pageRemoved_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard11pageRemovedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:182
// index:0
// Public Visibility=Default Availability=Available
// [-2] void back()

/*
Goes back to the previous page.

This is equivalent to pressing the Back button.

See also next(), accept(), reject(), and restart().
*/
impl /*struct*/ QWizard {
  pub fn back_0<RetType, T: QWizard_back_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.back_0(self);
    // return 1;
  }
}
pub trait QWizard_back_0<RetType> {
  fn back_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_back_0<(/*void*/)> for () {
  fn back_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWizard4backEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:183
// index:0
// Public Visibility=Default Availability=Available
// [-2] void next()

/*
Advances to the next page.

This is equivalent to pressing the Next or Commit button.

See also nextId(), back(), accept(), reject(), and restart().
*/
impl /*struct*/ QWizard {
  pub fn next_0<RetType, T: QWizard_next_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.next_0(self);
    // return 1;
  }
}
pub trait QWizard_next_0<RetType> {
  fn next_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_next_0<(/*void*/)> for () {
  fn next_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWizard4nextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:184
// index:0
// Public Visibility=Default Availability=Available
// [-2] void restart()

/*
Restarts the wizard at the start page. This function is called automatically when the wizard is shown.

See also startId().
*/
impl /*struct*/ QWizard {
  pub fn restart_0<RetType, T: QWizard_restart_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.restart_0(self);
    // return 1;
  }
}
pub trait QWizard_restart_0<RetType> {
  fn restart_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_restart_0<(/*void*/)> for () {
  fn restart_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWizard7restartEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:187
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QWizard {
  pub fn event_0<RetType, T: QWizard_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QWizard_event_0<RetType> {
  fn event_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QWizard) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWizard5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:188
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QWizard {
  pub fn resizeEvent_0<RetType, T: QWizard_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QWizard_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:189
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QWizard {
  pub fn paintEvent_0<RetType, T: QWizard_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QWizard_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:193
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void done(int)

/*
Reimplemented from QDialog::done().
*/
impl /*struct*/ QWizard {
  pub fn done_0<RetType, T: QWizard_done_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.done_0(self);
    // return 1;
  }
}
pub trait QWizard_done_0<RetType> {
  fn done_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_done_0<(/*void*/)> for (i32) {
  fn done_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard4doneEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:194
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void initializePage(int)

/*
This virtual function is called by QWizard to prepare page id just before it is shown either as a result of QWizard::restart() being called, or as a result of the user clicking Next. (However, if the QWizard::IndependentPages option is set, this function is only called the first time the page is shown.)

By reimplementing this function, you can ensure that the page's fields are properly initialized based on fields from previous pages.

The default implementation calls QWizardPage::initializePage() on page(id).

See also QWizardPage::initializePage() and cleanupPage().
*/
impl /*struct*/ QWizard {
  pub fn initializePage_0<RetType, T: QWizard_initializePage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initializePage_0(self);
    // return 1;
  }
}
pub trait QWizard_initializePage_0<RetType> {
  fn initializePage_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_initializePage_0<(/*void*/)> for (i32) {
  fn initializePage_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard14initializePageEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:195
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void cleanupPage(int)

/*
This virtual function is called by QWizard to clean up page id just before the user leaves it by clicking Back (unless the QWizard::IndependentPages option is set).

The default implementation calls QWizardPage::cleanupPage() on page(id).

See also QWizardPage::cleanupPage() and initializePage().
*/
impl /*struct*/ QWizard {
  pub fn cleanupPage_0<RetType, T: QWizard_cleanupPage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cleanupPage_0(self);
    // return 1;
  }
}
pub trait QWizard_cleanupPage_0<RetType> {
  fn cleanupPage_0(self , rsthis: & QWizard) -> RetType;
}
impl<'a> /*trait*/ QWizard_cleanupPage_0<(/*void*/)> for (i32) {
  fn cleanupPage_0(self , rsthis: & QWizard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWizard11cleanupPageEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum specifies the buttons in a wizard.



The following value is only useful when calling setButtonLayout():



See also setButton(), setButtonText(), setButtonLayout(), and customButtonClicked().

*/
pub type QWizard__WizardButton = i32;
// The Back button (Go Back on macOS)
pub const QWizard__BackButton :QWizard__WizardButton = 0;
// The Next button (Continue on macOS)
pub const QWizard__NextButton :QWizard__WizardButton = 1;
// The Commit button
pub const QWizard__CommitButton :QWizard__WizardButton = 2;
// The Finish button (Done on macOS)
pub const QWizard__FinishButton :QWizard__WizardButton = 3;
// The Cancel button (see also NoCancelButton)
pub const QWizard__CancelButton :QWizard__WizardButton = 4;
// The Help button (see also HaveHelpButton)
pub const QWizard__HelpButton :QWizard__WizardButton = 5;
// 
pub const QWizard__CustomButton1 :QWizard__WizardButton = 6;
// 
pub const QWizard__CustomButton2 :QWizard__WizardButton = 7;
// 
pub const QWizard__CustomButton3 :QWizard__WizardButton = 8;
// A horizontal stretch in the button layout
pub const QWizard__Stretch :QWizard__WizardButton = 9;
// 
pub const QWizard__NoButton :QWizard__WizardButton = -1;
// 
pub const QWizard__NStandardButtons :QWizard__WizardButton = 6;
// 
pub const QWizard__NButtons :QWizard__WizardButton = 9;
pub fn QWizard_WizardButtonItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QWizard", val);
}
pub fn QWizard_WizardButtonItemName_s(val: i32) ->String {
  //var nilthis *QWizard
  //return nilthis.WizardButtonItemName(val);
  return QWizard_WizardButtonItemName(val);
}


/*
This enum specifies the pixmaps that can be associated with a page.



See also setPixmap(), QWizardPage::setPixmap(), and Elements of a Wizard Page.

*/
pub type QWizard__WizardPixmap = i32;
// The tall pixmap on the left side of a ClassicStyle or ModernStyle page
pub const QWizard__WatermarkPixmap :QWizard__WizardPixmap = 0;
// The small pixmap on the right side of a ClassicStyle or ModernStyle page header
pub const QWizard__LogoPixmap :QWizard__WizardPixmap = 1;
// The pixmap that occupies the background of a ModernStyle page header
pub const QWizard__BannerPixmap :QWizard__WizardPixmap = 2;
// The pixmap that occupies the background of a MacStyle wizard
pub const QWizard__BackgroundPixmap :QWizard__WizardPixmap = 3;
// 
pub const QWizard__NPixmaps :QWizard__WizardPixmap = 4;
pub fn QWizard_WizardPixmapItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QWizard", val);
}
pub fn QWizard_WizardPixmapItemName_s(val: i32) ->String {
  //var nilthis *QWizard
  //return nilthis.WizardPixmapItemName(val);
  return QWizard_WizardPixmapItemName(val);
}


/*
This enum specifies the different looks supported by QWizard.



See also setWizardStyle(), WizardOption, and Wizard Look and Feel.

*/
pub type QWizard__WizardStyle = i32;
// Classic Windows look
pub const QWizard__ClassicStyle :QWizard__WizardStyle = 0;
// Modern Windows look
pub const QWizard__ModernStyle :QWizard__WizardStyle = 1;
// macOS look
pub const QWizard__MacStyle :QWizard__WizardStyle = 2;
// Windows Aero look
pub const QWizard__AeroStyle :QWizard__WizardStyle = 3;
// 
pub const QWizard__NStyles :QWizard__WizardStyle = 4;
pub fn QWizard_WizardStyleItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QWizard", val);
}
pub fn QWizard_WizardStyleItemName_s(val: i32) ->String {
  //var nilthis *QWizard
  //return nilthis.WizardStyleItemName(val);
  return QWizard_WizardStyleItemName(val);
}


/*


*/
pub type QWizard__WizardOption = i32;
// 
pub const QWizard__IndependentPages :QWizard__WizardOption = 1;
// 
pub const QWizard__IgnoreSubTitles :QWizard__WizardOption = 2;
// 
pub const QWizard__ExtendedWatermarkPixmap :QWizard__WizardOption = 4;
// 
pub const QWizard__NoDefaultButton :QWizard__WizardOption = 8;
// 
pub const QWizard__NoBackButtonOnStartPage :QWizard__WizardOption = 16;
// 
pub const QWizard__NoBackButtonOnLastPage :QWizard__WizardOption = 32;
// 
pub const QWizard__DisabledBackButtonOnLastPage :QWizard__WizardOption = 64;
// 
pub const QWizard__HaveNextButtonOnLastPage :QWizard__WizardOption = 128;
// 
pub const QWizard__HaveFinishButtonOnEarlyPages :QWizard__WizardOption = 256;
// 
pub const QWizard__NoCancelButton :QWizard__WizardOption = 512;
// 
pub const QWizard__CancelButtonOnLeft :QWizard__WizardOption = 1024;
// 
pub const QWizard__HaveHelpButton :QWizard__WizardOption = 2048;
// 
pub const QWizard__HelpButtonOnRight :QWizard__WizardOption = 4096;
// 
pub const QWizard__HaveCustomButton1 :QWizard__WizardOption = 8192;
// 
pub const QWizard__HaveCustomButton2 :QWizard__WizardOption = 16384;
// 
pub const QWizard__HaveCustomButton3 :QWizard__WizardOption = 32768;
// 
pub const QWizard__NoCancelButtonOnLastPage :QWizard__WizardOption = 65536;
pub fn QWizard_WizardOptionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QWizard", val);
}
pub fn QWizard_WizardOptionItemName_s(val: i32) ->String {
  //var nilthis *QWizard
  //return nilthis.WizardOptionItemName(val);
  return QWizard_WizardOptionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
